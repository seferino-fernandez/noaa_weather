//! The two runners every family's suite drives its table through.
//!
//! The hermetic runner points the binary at a `wiremock` server and checks
//! the request it made: no other test in the workspace sees the URL a command
//! line turns into. The live runner sends the same argument lists at real
//! NOAA, which is the only way to notice that a route moved or a response
//! stopped decoding.
//!
//! Both live here rather than in one family's file because there are eleven
//! families. A runner copied eleven times is a runner that gets fixed once.

use std::process::Output;

use jiff::{Span, Timestamp};
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

use super::noaa_weather;
use super::table::{Ages, Expectation, FAMILIES, Family, Invocation, Live, Query};

/// The table entry for one family.
///
/// Going through the shared list rather than naming the constant directly is
/// what makes a family that never got added to [`FAMILIES`] fail loudly here
/// instead of quietly running nothing.
pub fn family(name: &str) -> &'static Family {
    FAMILIES
        .iter()
        .find(|family| family.name == name)
        .unwrap_or_else(|| panic!("tests/common/table.rs has no {name:?} family"))
}

/// Runs the built binary with `arguments`, off the runtime's worker thread.
///
/// `MockServer::start` needs a tokio runtime, and the binary is driven with
/// blocking `std::process` calls, so the two have to be kept apart.
pub async fn run(arguments: &[&str]) -> Output {
    let arguments: Vec<String> = arguments.iter().map(|&part| part.to_owned()).collect();
    tokio::task::spawn_blocking(move || {
        noaa_weather()
            .args(&arguments)
            .output()
            .expect("the built binary must be runnable")
    })
    .await
    .expect("the subprocess task must not panic")
}

/// Runs `arguments` against `server`.
pub async fn run_against(server: &MockServer, arguments: &[&str]) -> Output {
    let mut all: Vec<&str> = arguments.to_vec();
    let uri = server.uri();
    all.push("--base-url");
    all.push(&uri);
    run(&all).await
}

/// Matches a request whose path and query string are exactly the expected
/// ones.
///
/// `wiremock::matchers::path` ignores the query, and `query_param` ignores
/// parameters it was not told about; comparing the whole thing is what makes
/// an accidental extra or renamed parameter show up.
pub fn asked_for(path: &'static str, query: &'static str) -> impl Fn(&Request) -> bool {
    matching(path, Query::Exact(query))
}

/// Matches a request against an invocation's path and [`Query`].
pub fn matching(path: &'static str, query: Query) -> impl Fn(&Request) -> bool {
    move |request: &Request| {
        if request.url.path() != path {
            return false;
        }
        let seen = request.url.query().unwrap_or_default();
        match query {
            Query::Exact(text) => seen == text,
            // The values move with the clock; the names and their order do
            // not, and checking them here is what stops `--start 6h` from
            // sending `start=banana` and passing. `check_ages` reads the
            // values themselves.
            Query::Clock(ages) => {
                let names: Vec<String> = request
                    .url
                    .query_pairs()
                    .map(|(name, _)| name.into_owned())
                    .collect();
                names == ages.parameters
            }
        }
    }
}

/// Serves `invocation`'s fixture, and only to the request it should make.
pub async fn expect_request(server: &MockServer, invocation: &Invocation) {
    Mock::given(method("GET"))
        .and(matching(invocation.path, invocation.query))
        .respond_with(ResponseTemplate::new(200).set_body_raw(invocation.body, invocation.media))
        .expect(1)
        .mount(server)
        .await;
}

/// Describes what the server was actually asked for, for failure messages.
pub async fn requests_seen(server: &MockServer) -> String {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .map(|request| format!("\n  {} {}", request.method, request.url))
        .collect()
}

pub fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

pub fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Drives every invocation in `family` against a mock server, one server per
/// invocation, and holds each to the path and query the table records.
pub async fn hermetic(family: &Family) {
    assert!(
        !family.invocations.is_empty(),
        "the {:?} family has no invocations",
        family.name
    );
    for invocation in family.invocations {
        let server = MockServer::start().await;
        expect_request(&server, invocation).await;

        // A binary response has nowhere to go but a file, and the table
        // cannot spell a temporary path.
        let directory = tempfile::tempdir().expect("temporary output directory");
        let target = directory.path().join("download");
        let mut arguments = invocation.argv();
        if invocation.binary {
            arguments.push("--output");
            arguments.push(target.to_str().expect("UTF-8 temporary path"));
        }

        // The clock is read on both sides of the run, so a relative age has
        // a window it must land in and nowhere else.
        let before = Timestamp::now();
        let output = run_against(&server, &arguments).await;
        let after = Timestamp::now();

        assert_eq!(
            output.status.code(),
            Some(0),
            "`{}` failed.\nstderr: {}\nserver saw:{}",
            invocation.display(),
            stderr(&output),
            requests_seen(&server).await
        );
        check_rendering(invocation, &stdout(&output));
        if let Query::Clock(ages) = invocation.query {
            let requests = server.received_requests().await.expect("recorded requests");
            check_ages(invocation, &ages, &requests[0], before, after);
        }
        server.verify().await;
    }
}

/// Holds a relative age to the instant it should have resolved to.
///
/// The names and their order are already pinned by [`matching`]. This is the
/// arithmetic: `--start 6h` has to be six hours before the moment the binary
/// read the clock, which is somewhere between `before` and `after`, and
/// nowhere else. A fixed string could not check this and a non-empty query
/// does not check it at all.
fn check_ages(
    invocation: &Invocation,
    ages: &Ages,
    request: &Request,
    before: Timestamp,
    after: Timestamp,
) {
    let where_ = invocation.display();
    let url = &request.url;
    let parameters: Vec<(String, String)> = url
        .query_pairs()
        .map(|(name, value)| (name.into_owned(), value.into_owned()))
        .collect();

    for (name, hours) in ages.relative {
        let raw = parameters
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
            .unwrap_or_else(|| panic!("`{where_}` sent no {name} parameter: {url}"));
        let seen: Timestamp = raw.parse().unwrap_or_else(|error| {
            panic!("`{where_}` sent {name}={raw:?}, not an instant: {error}")
        });

        // The extra second is truncation: the query carries whole seconds,
        // which is the only precision NOAA accepts.
        let earliest = before - Span::new().hours(*hours).seconds(1);
        let latest = after - Span::new().hours(*hours);
        assert!(
            seen >= earliest && seen <= latest,
            "`{where_}` should have sent {name} {hours}h before the run, but \
             {seen} is outside {earliest}..={latest}"
        );
    }
}

/// Holds the default presentation to the substrings the table records.
///
/// Exit 0 says the response decoded. It says nothing about whether anything
/// was drawn: a presenter that returned an empty table, or drew every header
/// and no rows, exits 0 just the same. For eight of the eleven families
/// nothing else in the workspace looks at these bytes at all.
fn check_rendering(invocation: &Invocation, rendered: &str) {
    if invocation.binary {
        assert!(
            rendered.is_empty(),
            "`{}` wrote {} bytes to standard output; binary responses go to \
             the file and nowhere else",
            invocation.display(),
            rendered.len()
        );
        return;
    }

    assert!(
        !invocation.renders.is_empty(),
        "`{}` records no expected output. Every non-binary invocation needs \
         at least one substring, or the table form is running untested.",
        invocation.display()
    );
    for expected in invocation.renders {
        assert!(
            rendered.contains(expected),
            "`{}` did not render {expected:?}.\nIt wrote:\n{rendered}",
            invocation.display()
        );
    }
}

/// Sends every checked invocation in `family` at real NOAA.
///
/// Each runs twice, because the two forms fail differently: the table form
/// drives the renderer over live data, and the `--json` form is what
/// [`check_payload`] can inspect. A binary invocation runs once, into a
/// temporary file, since `--json` is refused for it by design.
pub fn live(family: &Family) {
    for invocation in family.invocations {
        let Live::Check(expectation) = &invocation.live else {
            continue;
        };
        let directory = tempfile::tempdir().expect("temporary output directory");
        let target = directory.path().join("download");
        let target = target.to_str().expect("UTF-8 temporary path");

        let forms: &[&[&str]] = if invocation.binary {
            &[&["--output"]]
        } else {
            &[&[], &["--json"]]
        };
        for extra in forms {
            let mut arguments = invocation.argv();
            arguments.extend_from_slice(extra);
            if invocation.binary {
                arguments.push(target);
            }

            let output = noaa_weather()
                .args(&arguments)
                .output()
                .expect("the built binary must be runnable");
            assert_eq!(
                output.status.code(),
                Some(0),
                "`{} {}` failed against NOAA: {}",
                invocation.display(),
                extra.join(" "),
                stderr(&output)
            );
            if extra == &["--json"] {
                check_payload(invocation, expectation, &output.stdout);
            }
        }
    }
}

/// Holds one live `--json` payload to what the table says NOAA must send.
///
/// Exit 0 already covers the fields the curated models require, since one of
/// those going missing fails the decode. This closes the two gaps that
/// leaves: a collection that came back empty, and an optional field NOAA
/// renamed or dropped, which arrives as a silent `null`.
pub fn check_payload(invocation: &Invocation, expectation: &Expectation, body: &[u8]) {
    let where_ = invocation.display();
    let document: serde_json::Value = serde_json::from_slice(body).unwrap_or_else(|error| {
        panic!(
            "`{where_} --json` did not emit JSON: {error}\n{}",
            String::from_utf8_lossy(body)
        )
    });
    let payload = document.pointer(expectation.payload).unwrap_or_else(|| {
        panic!(
            "`{where_} --json` has nothing at {}: {document}",
            expectation.payload
        )
    });

    let length = match payload {
        serde_json::Value::Array(items) => items.len(),
        serde_json::Value::Object(members) => members.len(),
        serde_json::Value::String(text) => text.len(),
        other => panic!(
            "{} in `{where_} --json` is neither a collection nor a string: {other}",
            expectation.payload
        ),
    };
    assert!(
        length > 0 || !expectation.non_empty,
        "NOAA returned a well-formed but empty {} for `{where_}`, and this \
         invocation is one that should always have something in it",
        expectation.payload
    );

    // Identity, which is the only thing here that notices NOAA answering the
    // URL we asked for with a document about something else. Everything else
    // in this function asks whether a value is present, not whether it is
    // the right one.
    for (pointer, expected) in expectation.equals {
        let seen = document
            .pointer(pointer)
            .unwrap_or_else(|| panic!("`{where_} --json` has nothing at {pointer}: {document}"));
        let seen = seen
            .as_str()
            .unwrap_or_else(|| panic!("`{where_} --json` has a non-string at {pointer}: {seen}"));
        assert_eq!(
            seen, *expected,
            "`{where_} --json` answered with {pointer} = {seen:?}, but the \
             request named {expected:?}"
        );
    }

    // A single-object payload is checked directly; a collection is checked
    // through its first element, when it has one.
    let subject = match payload {
        serde_json::Value::Array(items) => match items.first() {
            Some(first) => first,
            None => return,
        },
        other => other,
    };
    let properties = subject.get("properties").unwrap_or(subject);
    for key in expectation.keys {
        let value = properties
            .get(key)
            .unwrap_or_else(|| panic!("`{where_} --json` dropped the {key:?} key: {properties}"));
        let populated = match value {
            serde_json::Value::Null => false,
            serde_json::Value::Array(items) => !items.is_empty(),
            serde_json::Value::Object(members) => !members.is_empty(),
            serde_json::Value::String(text) => !text.is_empty(),
            _ => true,
        };
        assert!(
            populated,
            "`{where_} --json` left {key:?} empty; NOAA populated it on every \
             response of this shape when the table was written, so this is \
             drift"
        );
    }
}
