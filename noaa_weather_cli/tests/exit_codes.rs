//! The exit codes and the JSON error line, end to end.
//!
//! Both are contracts a script depends on, and neither is visible from
//! inside the process: `src/exit.rs` can decide a code but only the binary
//! can return one, and only a mock server can produce the NOAA response the
//! line is supposed to describe.

mod common;

use std::process::Output;

use common::fixtures::{ALERT_COUNT, GEO_JSON, JSON_LD};
use common::noaa_weather;
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// An RFC 7807 body carrying every member `ProblemDetail` declares.
///
/// Written to that model rather than captured, because the point here is
/// that whatever the client parsed reaches the JSON line unflattened; the
/// shapes NOAA actually sends are what the fixtures under
/// `noaa_weather_client/tests/fixtures` are for.
const PROBLEM: &str = r#"{
  "correlationId": "1a2b3c4d",
  "title": "Not Found",
  "type": "urn:noaa:nws:api:NotFound",
  "status": 404,
  "detail": "Alert Does Not Exist",
  "instance": "urn:noaa:nws:api:request:1a2b3c4d"
}"#;

/// Runs the built binary off the runtime's worker thread.
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

async fn run_against(server: &MockServer, arguments: &[&str]) -> Output {
    let mut all: Vec<&str> = arguments.to_vec();
    let uri = server.uri();
    all.push("--base-url");
    all.push(&uri);
    run(&all).await
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// Parses standard error as the one JSON line the contract promises.
///
/// Requiring the whole stream to parse is the assertion that matters: a
/// human-readable line printed alongside would leave a caller running
/// `2>&1 | jq` with a parse error rather than a report.
fn error_line(output: &Output) -> Value {
    let text = stderr(output);
    assert!(
        text.ends_with('\n'),
        "the JSON error line must be newline-terminated: {text:?}"
    );
    assert_eq!(
        text.lines().count(),
        1,
        "standard error must carry exactly one line: {text:?}"
    );
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!("standard error did not parse as JSON: {error}\n{text}");
    })
}

/// Answers `route` with `status` and a NOAA problem body.
async fn refusing(route: &'static str, status: u16) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(route))
        .respond_with(
            ResponseTemplate::new(status)
                .insert_header("Retry-After", "30")
                .insert_header("X-Correlation-Id", "corr-9")
                .insert_header("X-Request-Id", "req-9")
                .set_body_raw(PROBLEM, "application/problem+json"),
        )
        .mount(&server)
        .await;
    server
}

#[tokio::test]
async fn a_command_that_works_exits_zero_and_says_nothing() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/alerts/active/count"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(ALERT_COUNT, JSON_LD))
        .mount(&server)
        .await;

    let output = run_against(&server, &["alerts", "count"]).await;

    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stderr.is_empty(), "{}", stderr(&output));
}

#[tokio::test]
async fn a_refused_request_exits_three() {
    let server = refusing("/alerts/active/count", 404).await;

    let output = run_against(&server, &["alerts", "count", "--retries", "0"]).await;

    assert_eq!(output.status.code(), Some(3), "{}", stderr(&output));
    assert!(stderr(&output).contains("404"), "{}", stderr(&output));
}

/// The whole line, on the response that carries every optional fact.
#[tokio::test]
async fn a_refused_request_reports_every_fact_it_has_under_json() {
    let server = refusing("/alerts/active/count", 503).await;

    let output = run_against(&server, &["alerts", "count", "--retries", "0", "--json"]).await;

    assert_eq!(output.status.code(), Some(3), "{}", stderr(&output));
    let report = &error_line(&output)["error"];
    assert_eq!(report["kind"], "noaa", "{report}");
    assert_eq!(report["status"], 503, "{report}");
    assert_eq!(report["retry_after"], 30, "{report}");
    assert_eq!(report["correlation_id"], "corr-9", "{report}");
    assert_eq!(report["request_id"], "req-9", "{report}");
    assert!(
        report["url"]
            .as_str()
            .is_some_and(|url| url.ends_with("/alerts/active/count")),
        "{report}"
    );
    assert!(
        report["message"]
            .as_str()
            .is_some_and(|message| message.contains("alert count")),
        "the message must keep the operation the command named: {report}"
    );

    // Embedded whole rather than flattened: NOAA's own `status` survives
    // beside the client's, and so does the correlation id it carries.
    assert_eq!(report["problem"]["title"], "Not Found", "{report}");
    assert_eq!(report["problem"]["status"], 404.0, "{report}");
    assert_eq!(
        report["problem"]["detail"], "Alert Does Not Exist",
        "{report}"
    );
    assert_eq!(report["problem"]["correlationId"], "1a2b3c4d", "{report}");
}

#[tokio::test]
async fn an_unreachable_server_exits_four_and_reports_a_network_kind() {
    // Nothing is listening on port 1, and binding it would need root.
    let arguments = ["alerts", "count", "--base-url", "http://127.0.0.1:1"];

    let output = run(&[&arguments[..], &["--retries", "0"]].concat()).await;
    assert_eq!(output.status.code(), Some(4), "{}", stderr(&output));

    let output = run(&[&arguments[..], &["--retries", "0", "--json"]].concat()).await;
    assert_eq!(output.status.code(), Some(4), "{}", stderr(&output));
    let report = &error_line(&output)["error"];
    assert_eq!(report["kind"], "network", "{report}");
    // A transport failure has no response, so it has none of these.
    for absent in [
        "status",
        "retry_after",
        "correlation_id",
        "request_id",
        "problem",
    ] {
        assert!(
            report.get(absent).is_none(),
            "{absent} should be omitted: {report}"
        );
    }
}

/// A body that arrives and does not decode is 1, not 3.
///
/// The distinction is the point of the two codes: 3 says NOAA refused the
/// request and the status says why, and a caller acting on that would be
/// wrong about a 200 that was cut off mid-array.
#[tokio::test]
async fn a_truncated_body_exits_one_and_reports_an_internal_kind() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/alerts/active"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(r#"{"type":"FeatureCollection","features":[{"#, GEO_JSON),
        )
        .mount(&server)
        .await;

    let output = run_against(&server, &["alerts", "active"]).await;
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));

    let output = run_against(&server, &["alerts", "active", "--json"]).await;
    assert_eq!(output.status.code(), Some(1), "{}", stderr(&output));
    let report = &error_line(&output)["error"];
    assert_eq!(report["kind"], "internal", "{report}");
    assert!(report.get("status").is_none(), "{report}");
}

/// A destination that cannot take the bytes is 5, whatever went wrong there.
#[tokio::test]
async fn an_unwritable_destination_exits_five_and_reports_an_output_kind() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/alerts/active/count"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(ALERT_COUNT, JSON_LD))
        .mount(&server)
        .await;
    let directory = tempfile::tempdir().expect("temporary output directory");
    let unwritable = directory.path().join("no-such-directory/count.json");
    let unwritable = unwritable.to_str().expect("UTF-8 temporary path");

    let output = run_against(&server, &["alerts", "count", "--output", unwritable]).await;
    assert_eq!(output.status.code(), Some(5), "{}", stderr(&output));

    let output = run_against(
        &server,
        &["alerts", "count", "--output", unwritable, "--json"],
    )
    .await;
    assert_eq!(output.status.code(), Some(5), "{}", stderr(&output));
    assert_eq!(error_line(&output)["error"]["kind"], "output");
}

/// An output request argv alone makes impossible is a usage error.
///
/// All three of these fail before any request is made, and none of them
/// touches the filesystem: `StdoutDestination::validate` refuses binary
/// bytes without consulting whether stdout is a terminal, so they would
/// fail identically on every machine. That is exit 2, not 5 — telling a
/// script its disk is bad when its command line is would be worse than
/// useless.
///
/// The `--output /tmp/...` case is the one that settles it: a perfectly
/// writable path, never attempted, because `--json` was refused first.
#[tokio::test]
async fn output_requests_argv_alone_forbids_exit_two() {
    let directory = tempfile::tempdir().expect("temporary output directory");
    let writable = directory.path().join("briefing.pdf");
    let writable = writable.to_str().expect("UTF-8 temporary path");

    for arguments in [
        vec!["offices", "briefing-download-latest", "--id", "PSR"],
        vec![
            "offices",
            "briefing-download-latest",
            "--id",
            "PSR",
            "--output",
            "-",
        ],
        vec![
            "offices",
            "briefing-download-latest",
            "--id",
            "PSR",
            "--output",
            writable,
            "--json",
        ],
    ] {
        // A base URL nothing is listening on: a request would exit 4, so
        // exit 2 proves none was made.
        let output = run(&[&arguments[..], &["--base-url", "http://127.0.0.1:1"]].concat()).await;
        assert_eq!(
            output.status.code(),
            Some(2),
            "{arguments:?}: {}",
            stderr(&output)
        );
        // Exit 2 carries no `kind`, so no JSON line even though `--format`
        // was parsed and the last of these asked for JSON.
        assert!(
            serde_json::from_str::<Value>(&stderr(&output)).is_err(),
            "{arguments:?} wrote a JSON line: {}",
            stderr(&output)
        );
    }

    assert!(
        !std::path::Path::new(writable).exists(),
        "the refused download must not have created its target"
    );
}

/// The documented hole: exit 2 never writes a JSON line.
///
/// clap reports a bad value itself and exits before this program can, so
/// under `--json` a usage error still writes clap's own text. The half that
/// is this program's own — a value the client rejects, or a base URL that is
/// not a URL — writes the human line for the same reason: there is no
/// `"usage"` kind, and adding one would give the four kinds a fifth member
/// that no exit code above matches.
#[tokio::test]
async fn usage_failures_exit_two_without_a_json_line() {
    for arguments in [
        vec!["points", "metadata", "91,-97.0892", "--json"],
        vec!["alerts", "count", "--base-url", "not a url", "--json"],
        vec!["alerts", "count", "--timeout", "0s", "--json"],
    ] {
        let output = run(&arguments).await;
        assert_eq!(
            output.status.code(),
            Some(2),
            "{arguments:?}: {}",
            stderr(&output)
        );
        assert!(
            serde_json::from_str::<Value>(&stderr(&output)).is_err(),
            "{arguments:?} wrote a JSON line, so the documented hole has \
             closed and docs/README.md is now wrong: {}",
            stderr(&output)
        );
    }
}

/// Without `--json` a failure is prose, whatever its code.
#[tokio::test]
async fn the_human_line_is_still_prose_when_json_was_not_asked_for() {
    let server = refusing("/alerts/active/count", 503).await;

    let output = run_against(&server, &["alerts", "count", "--retries", "0"]).await;

    let text = stderr(&output);
    assert!(text.starts_with("noaa-weather: "), "{text}");
    assert!(serde_json::from_str::<Value>(&text).is_err(), "{text}");
}

/// The raw-JSON seam validates its destination too.
///
/// `radar wind-profiler` is the CLI's only `Output::raw_json` command, and
/// that method has its own `validate` call: the one in `show` covers nothing
/// for it. Without this, the branch was reachable and untested.
///
/// Four `OutputFailure::wrap` sites remain unpinned — the `write_presentation`
/// and `write_json` arms of `show`, the `write_json` of `raw_json`, and the
/// `write_document` of `download`. Reaching any of them needs a destination
/// that passes `validate` and then fails mid-write, which means a full disk,
/// a revoked permission between the two calls, or an injected writer the
/// binary has no flag for. Saying so is more honest than a test that claims
/// to build one and does not; `src/output/tests.rs` exercises the same four
/// paths in-process against failing writers.
#[tokio::test]
async fn the_raw_json_destination_is_validated_before_the_request() {
    let directory = tempfile::tempdir().expect("temporary output directory");
    let unwritable = directory.path().join("no-such-directory/profiler.json");
    let unwritable = unwritable.to_str().expect("UTF-8 temporary path");

    // Nothing is listening on port 1: reaching the request at all would fail
    // with 4, so exit 5 proves the destination was checked first.
    let output = run(&[
        "radar",
        "wind-profiler",
        "--id",
        "HWPA2",
        "--output",
        unwritable,
        "--base-url",
        "http://127.0.0.1:1",
    ])
    .await;

    assert_eq!(output.status.code(), Some(5), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("no-such-directory"),
        "{}",
        stderr(&output)
    );
}
