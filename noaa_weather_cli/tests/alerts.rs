//! The `alerts` family, end to end.
//!
//! Three kinds of test live here. The hermetic ones point the binary at a
//! `wiremock` server and check the request it made: no other test in the
//! workspace sees the URL a command line turns into. The usage ones never
//! reach the network at all. The live ones at the bottom send the same
//! argument lists at real NOAA, which is the only way to notice that a route
//! moved or a response stopped decoding.
//!
//! Rendered output is snapshotted once, for one command. `noaa_weather_summary`
//! already snapshots `Summary` values and `output::render` already snapshots
//! rendered bytes, so a third set here would only mean three snapshot files to
//! review for every wording change.

mod common;

use std::process::Output;
use std::time::{Duration, Instant};

use assert_cmd::prelude::*;
use common::fixtures::ALERT_ID;
use common::noaa_weather;
use common::table::{ALERTS, Expectation, Invocation, Live, Query};
use jiff::{Span, Timestamp};
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

/// Runs the built binary with `arguments`, off the runtime's worker thread.
///
/// `MockServer::start` needs a tokio runtime, and the binary is driven with
/// blocking `std::process` calls, so the two have to be kept apart.
async fn run(arguments: &[&str]) -> Output {
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
async fn run_against(server: &MockServer, arguments: &[&str]) -> Output {
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
fn asked_for(path: &'static str, query: &'static str) -> impl Fn(&Request) -> bool {
    matching(path, Query::Exact(query))
}

/// Matches a request against an invocation's path and [`Query`].
fn matching(path: &'static str, query: Query) -> impl Fn(&Request) -> bool {
    move |request: &Request| {
        if request.url.path() != path {
            return false;
        }
        let seen = request.url.query().unwrap_or_default();
        match query {
            Query::Exact(text) => seen == text,
            Query::Clock => !seen.is_empty(),
        }
    }
}

/// Serves `invocation`'s fixture, and only to the request it should make.
async fn expect_request(server: &MockServer, invocation: &Invocation) {
    Mock::given(method("GET"))
        .and(matching(invocation.path, invocation.query))
        .respond_with(ResponseTemplate::new(200).set_body_raw(invocation.body, invocation.media))
        .expect(1)
        .mount(server)
        .await;
}

/// Describes what the server was actually asked for, for failure messages.
async fn requests_seen(server: &MockServer) -> String {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .map(|request| format!("\n  {} {}", request.method, request.url))
        .collect()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[tokio::test]
async fn every_alerts_invocation_asks_for_the_path_and_query_the_table_records() {
    for invocation in ALERTS {
        let server = MockServer::start().await;
        expect_request(&server, invocation).await;

        let output = run_against(&server, &invocation.argv()).await;

        assert_eq!(
            output.status.code(),
            Some(0),
            "`{}` failed.\nstderr: {}\nserver saw:{}",
            invocation.display(),
            stderr(&output),
            requests_seen(&server).await
        );
        server.verify().await;
    }
}

/// The one query the table cannot spell, checked where the clock can be read.
///
/// `--start 6h` becomes an absolute RFC 3339 instant, and a malformed one
/// comes back from NOAA as a 400. Asserting the offsets is stronger than any
/// fixed string could be, because it pins the arithmetic and not just the
/// shape.
#[tokio::test]
async fn relative_ages_become_absolute_timestamps() {
    let server = MockServer::start().await;
    let invocation = ALERTS
        .iter()
        .find(|invocation| matches!(invocation.query, Query::Clock))
        .expect("the table must still carry the relative-age invocation");
    expect_request(&server, invocation).await;

    let before = Timestamp::now();
    let output = run_against(&server, &invocation.argv()).await;
    let after = Timestamp::now();
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));

    let request = &server.received_requests().await.expect("recorded requests")[0];
    let parameters: Vec<(String, String)> = request
        .url
        .query_pairs()
        .map(|(name, value)| (name.into_owned(), value.into_owned()))
        .collect();
    let named: Vec<&str> = parameters.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(
        named,
        ["start", "end", "limit"],
        "the query lost or reordered a parameter: {:?}",
        request.url.query()
    );

    let at = |name: &str| -> Timestamp {
        let raw = parameters
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
            .unwrap_or_else(|| panic!("no {name} parameter in {:?}", request.url.query()));
        raw.parse()
            .unwrap_or_else(|error| panic!("{name}={raw:?} is not an RFC 3339 instant: {error}"))
    };
    // The binary reads the clock somewhere between these two, so each age
    // lands in a window that width, and nowhere else. The extra second is
    // truncation: the query carries whole seconds, which is the only
    // precision NOAA accepts.
    for (name, age) in [("start", 6), ("end", 1)] {
        let expected_earliest = before - Span::new().hours(age).seconds(1);
        let expected_latest = after - Span::new().hours(age);
        let seen = at(name);
        assert!(
            seen >= expected_earliest && seen <= expected_latest,
            "{name} should be {age}h before the run, but {seen} is outside \
             {expected_earliest}..={expected_latest}"
        );
    }
    assert!(at("start") < at("end"), "start must precede end");
    server.verify().await;
}

/// One rendered command, pinned. Color, width, and time zone are all fixed:
/// the first two follow the terminal and the third follows the machine's
/// zone, and a snapshot that moved with any of them would be a snapshot of
/// the test host.
#[tokio::test]
async fn a_single_alert_renders_the_same_table_every_time() {
    let server = MockServer::start().await;
    let invocation = ALERTS
        .iter()
        .find(|invocation| invocation.command == ["alerts", "alert"])
        .expect("the table must still cover `alerts alert`");
    expect_request(&server, invocation).await;

    let output = run_against(
        &server,
        &[
            "alerts",
            "alert",
            "--id",
            ALERT_ID,
            "--color",
            "never",
            "--width",
            "100",
            "--time-zone",
            "UTC",
        ],
    )
    .await;

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    insta::assert_snapshot!(stdout(&output));
}

#[tokio::test]
async fn a_server_error_fails_the_command() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(asked_for("/alerts/active/count", ""))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;

    let output = run_against(&server, &["alerts", "count", "--retries", "0"]).await;

    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
    assert!(stderr(&output).contains("503"), "{}", stderr(&output));
    server.verify().await;
}

/// `--timeout` has to reach the builder, not merely parse.
///
/// A reply held longer than the flag allows but well under the 30-second
/// default is the only thing that separates a timeout that was applied from
/// one that was dropped on the floor: with the flag wired the command fails
/// in milliseconds, without it the reply arrives and the command succeeds.
#[tokio::test]
async fn the_timeout_flag_reaches_the_client() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(asked_for("/alerts/types", ""))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_secs(5))
                .set_body_raw(
                    r#"{"@context":[],"eventTypes":["Tornado Warning"]}"#,
                    "application/ld+json",
                ),
        )
        .expect(1)
        .mount(&server)
        .await;

    let started = Instant::now();
    let output = run_against(
        &server,
        &["alerts", "types", "--timeout", "50ms", "--retries", "0"],
    )
    .await;
    let waited = started.elapsed();

    assert_eq!(
        output.status.code(),
        Some(1),
        "the delayed reply should have timed out: {}",
        stdout(&output)
    );
    assert!(
        waited < Duration::from_secs(4),
        "gave up after {waited:?}, which is the server's delay rather than \
         the timeout the flag asked for"
    );
    server.verify().await;
}

/// `--retries 0` has to reach the policy, not just be accepted: the previous
/// test only proves one attempt happened, and one attempt is also what a
/// broken flag would produce if the default were one.
#[tokio::test]
async fn retries_asks_for_the_response_again() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(asked_for("/alerts/active/count", ""))
        .respond_with(ResponseTemplate::new(503))
        .expect(2)
        .mount(&server)
        .await;

    let output = run_against(&server, &["alerts", "count", "--retries", "2"]).await;

    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
    server.verify().await;
}

#[tokio::test]
async fn the_base_url_environment_variable_redirects_the_program() {
    let server = MockServer::start().await;
    let uri = server.uri();
    Mock::given(method("GET"))
        .and(asked_for("/alerts/types", ""))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"@context":[],"eventTypes":["Tornado Warning"]}"#,
            "application/ld+json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let output = tokio::task::spawn_blocking(move || {
        noaa_weather()
            .args(["alerts", "types"])
            .env("NOAA_WEATHER_BASE_URL", uri)
            .output()
            .expect("the built binary must be runnable")
    })
    .await
    .expect("the subprocess task must not panic");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    server.verify().await;
}

/// A flag beats the variable, which is the half of the precedence rule a
/// caller notices when a stale export is sitting in their shell.
#[tokio::test]
async fn the_base_url_flag_overrides_the_environment_variable() {
    let server = MockServer::start().await;
    let uri = server.uri();
    Mock::given(method("GET"))
        .and(asked_for("/alerts/types", ""))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"@context":[],"eventTypes":["Tornado Warning"]}"#,
            "application/ld+json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let output = tokio::task::spawn_blocking(move || {
        noaa_weather()
            .args(["alerts", "types", "--base-url", &uri])
            // Binding below port 1024 needs root and nothing in this suite
            // does, so a run that took the variable would fail rather than
            // quietly succeed somewhere else.
            .env("NOAA_WEATHER_BASE_URL", "http://127.0.0.1:1")
            .output()
            .expect("the built binary must be runnable")
    })
    .await
    .expect("the subprocess task must not panic");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    server.verify().await;
}

/// The user agent is what NOAA identifies callers by, so it has to arrive on
/// the wire and not merely be accepted by the parser.
#[tokio::test]
async fn the_user_agent_flag_reaches_the_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(asked_for("/alerts/types", ""))
        .and(wiremock::matchers::header(
            "user-agent",
            "fixture-client/9.9 (+test)",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"@context":[],"eventTypes":["Tornado Warning"]}"#,
            "application/ld+json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let output = run_against(
        &server,
        &[
            "alerts",
            "types",
            "--user-agent",
            "fixture-client/9.9 (+test)",
        ],
    )
    .await;

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    server.verify().await;
}

/// The API key deliberately has no flag, so a variable is the only way to
/// supply one and this is the only test that carries it end to end.
#[tokio::test]
async fn the_api_key_variable_reaches_the_request_as_a_header() {
    let server = MockServer::start().await;
    let uri = server.uri();
    Mock::given(method("GET"))
        .and(asked_for("/alerts/types", ""))
        .and(wiremock::matchers::header("x-api-key", "fixture-key"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"@context":[],"eventTypes":["Tornado Warning"]}"#,
            "application/ld+json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let output = tokio::task::spawn_blocking(move || {
        noaa_weather()
            .args(["alerts", "types", "--base-url", &uri])
            .env("NOAA_WEATHER_API_KEY", "fixture-key")
            .output()
            .expect("the built binary must be runnable")
    })
    .await
    .expect("the subprocess task must not panic");

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    server.verify().await;
}

#[tokio::test]
async fn an_unreachable_base_url_fails_without_retrying() {
    // No server: the CLI should report the connection failure rather than
    // hang or succeed.
    let output = run(&[
        "alerts",
        "count",
        "--base-url",
        "http://127.0.0.1:1",
        "--retries",
        "0",
    ])
    .await;

    assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
}

#[tokio::test]
async fn a_base_url_that_is_not_a_url_is_a_usage_error() {
    let output = run(&["alerts", "count", "--base-url", "not a url"]).await;

    let stderr = stderr(&output);
    assert!(stderr.contains("--base-url"), "{stderr}");
    assert!(stderr.contains("NOAA_WEATHER_BASE_URL"), "{stderr}");
}

/// A timeout that is not a positive duration is a usage error, and says so.
///
/// This has to go through the binary. The parser's own unit test calls
/// `parse_timeout("-5s")` directly and so never sees clap, which is where the
/// interesting half happens: without `allow_hyphen_values` on the argument,
/// clap claims `-5` as an unknown flag and answers `unexpected argument '-5'
/// found`. Still exit 2, so only the message distinguishes the two.
#[test]
fn a_timeout_that_is_not_a_positive_duration_names_the_timeout() {
    for argument in ["-5s", "-0s", "0s"] {
        let output = noaa_weather()
            .args(["alerts", "count", "--timeout", argument])
            .output()
            .expect("the built binary must be runnable");
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert_eq!(
            output.status.code(),
            Some(2),
            "--timeout {argument}: {stderr}"
        );
        assert!(
            stderr.contains("must be greater than zero"),
            "--timeout {argument} should complain about the duration, not the \
             argument grammar: {stderr}"
        );
    }

    // A bare number has no unit, so this one is the duration parser talking
    // rather than the sign check; it still has to name the flag.
    let output = noaa_weather()
        .args(["alerts", "count", "--timeout", "0"])
        .output()
        .expect("the built binary must be runnable");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("--timeout"), "{stderr}");
}

#[test]
fn test_alerts_command_list_rejects_removed_active_option() {
    let mut cmd = noaa_weather();
    cmd.args(["alerts", "list", "--active", "true"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unexpected argument '--active'"),
        "{stderr}"
    );
}

#[test]
fn test_alerts_command_failure_invalid_command() {
    let mut cmd = noaa_weather();
    cmd.arg("alerts");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_command_area_failure_invalid_area() {
    let mut cmd = noaa_weather();
    cmd.arg("alerts");
    cmd.arg("area");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_command_marine_region_failure_invalid_region() {
    let mut cmd = noaa_weather();
    cmd.arg("alerts");
    cmd.arg("marine-region");
    cmd.arg("--marine-region");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_list_rejects_malformed_zone() {
    let mut cmd = noaa_weather();
    cmd.args(["alerts", "zone", "--zone-id", "CAZ 043"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid zone id"), "{stderr}");
}

/// Holds one live `--json` payload to what the table says NOAA must send.
///
/// Exit 0 already covers the fields the curated models require, since one of
/// those going missing fails the decode. This closes the two gaps that
/// leaves: a collection that came back empty, and an optional field NOAA
/// renamed or dropped, which arrives as a silent `null`.
fn check_payload(invocation: &Invocation, expectation: &Expectation, body: &[u8]) {
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
        other => panic!(
            "{} in `{where_} --json` is neither an array nor an object: {other}",
            expectation.payload
        ),
    };
    assert!(
        length > 0 || !expectation.non_empty,
        "NOAA returned a well-formed but empty {} for `{where_}`, and this \
         invocation is one that should always have something in it",
        expectation.payload
    );

    let Some(first) = payload.as_array().and_then(|items| items.first()) else {
        return;
    };
    let properties = first.get("properties").unwrap_or(first);
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
             alert when the table was written, so this is drift"
        );
    }
}

/// The live half of the shared table: the same argument lists, sent at real
/// NOAA. The hermetic tests prove the CLI asks for the right URL; only this
/// notices that the URL stopped answering or the answer stopped decoding.
///
/// Each invocation runs twice, because the two forms fail differently: the
/// table form drives the renderer over live data, and the `--json` form is
/// what [`check_payload`] can inspect.
#[test]
fn test_alerts_live_noaa_answers_every_tabled_invocation() {
    for invocation in ALERTS {
        let Live::Check(expectation) = &invocation.live else {
            continue;
        };
        for extra in [&[][..], &["--json"]] {
            let output = noaa_weather()
                .args(invocation.argv())
                .args(extra)
                .output()
                .expect("the built binary must be runnable");
            assert_eq!(
                output.status.code(),
                Some(0),
                "`{} {}` failed against NOAA: {}",
                invocation.display(),
                extra.join(" "),
                String::from_utf8_lossy(&output.stderr)
            );
            if !extra.is_empty() {
                check_payload(invocation, expectation, &output.stdout);
            }
        }
    }
}

/// Fetches a single alert by an id resolved at run time.
///
/// The id used to be hardcoded, and the test was ignored once NOAA stopped
/// serving that alert. Resolving through `alerts list --json` keeps the whole
/// test in the CLI's own vocabulary: a `--json` that stopped carrying ids
/// breaks resolution here rather than passing unnoticed.
///
/// Nothing below asserts the envelope's `type`. The CLI writes that string on
/// serialize instead of reading it off the response, so the assertion would
/// hold even against a reply that had no `type` at all.
#[test]
fn test_alerts_command_get_success() {
    let listing = noaa_weather()
        .args(["alerts", "list", "--json", "--limit", "1"])
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        listing.status.code(),
        Some(0),
        "`alerts list --json --limit 1` failed: {}",
        String::from_utf8_lossy(&listing.stderr)
    );

    let envelope: serde_json::Value = serde_json::from_slice(&listing.stdout).unwrap_or_else(|e| {
        panic!(
            "`alerts list --json` did not emit JSON: {e}\n{}",
            String::from_utf8_lossy(&listing.stdout)
        )
    });
    let features = envelope["features"]
        .as_array()
        .unwrap_or_else(|| panic!("`alerts list --json` has no `features` array: {envelope}"));
    let first = features.first().unwrap_or_else(|| {
        panic!(
            "NOAA returned a well-formed but empty alert listing, so there was \
             no id to look up; this test cannot check `alerts alert` until \
             NOAA is serving at least one alert again"
        )
    });
    let id = first["properties"]["id"]
        .as_str()
        .unwrap_or_else(|| panic!("the first listed alert has no `properties.id`: {first}"));

    let alert = noaa_weather()
        .args(["alerts", "alert", "--id", id, "--json"])
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        alert.status.code(),
        Some(0),
        "`alerts alert --id {id}` failed: {}",
        String::from_utf8_lossy(&alert.stderr)
    );

    let fetched: serde_json::Value = serde_json::from_slice(&alert.stdout).unwrap_or_else(|e| {
        panic!(
            "`alerts alert --json` did not emit JSON: {e}\n{}",
            String::from_utf8_lossy(&alert.stdout)
        )
    });
    assert_eq!(
        fetched["properties"]["id"], id,
        "NOAA answered `alerts alert --id {id}` with a different alert"
    );
}
