//! The `glossary` family, end to end.
//!
//! The destination tests live here rather than with the other output tests
//! because the glossary is the smallest command that produces a table, JSON,
//! and a file. They are checked twice: against a mock server, where the
//! assertions about the bytes written can be exact, and against NOAA, where
//! the point is only that a real response of that size still reaches a file
//! and an explicit standard output.

mod common;

use std::fs;

use common::fixtures::{GLOSSARY, JSON_LD};
use common::noaa_weather;
use common::runner::{family, hermetic, live, run_against, stderr};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn every_glossary_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("glossary")).await;
}

#[test]
fn test_glossary_live_noaa_answers_every_tabled_invocation() {
    live(family("glossary"));
}

/// The two destination shapes, sent at NOAA.
///
/// The mock-server tests below make the same two requests and check the
/// bytes far more closely. These exist because `--output` is the one global
/// flag whose behaviour depends on the size of the response: the glossary is
/// most of a megabyte, and a file destination that only ever saw a
/// two-kilobyte fixture would not have been asked the interesting question.
#[test]
fn test_glossary_live_writes_to_a_file_and_to_explicit_stdout() {
    let directory = tempfile::tempdir().expect("temporary output directory");
    let path = directory.path().join("glossary.txt");
    let path_argument = path.to_str().expect("UTF-8 temporary path");

    let output = noaa_weather()
        .args(["glossary", "--output", path_argument])
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        output.status.code(),
        Some(0),
        "`glossary --output` failed against NOAA: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stdout.is_empty(),
        "a file destination wrote to stdout"
    );
    let written = fs::read_to_string(&path).expect("the glossary file");
    assert!(
        written.contains("Definition"),
        "{}",
        &written[..200.min(written.len())]
    );

    let output = noaa_weather()
        .args(["glossary", "--json", "--output", "-"])
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        output.status.code(),
        Some(0),
        "`glossary --json --output -` failed against NOAA: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("explicit stdout JSON");
    assert!(
        value["glossary"]
            .as_array()
            .is_some_and(|terms| !terms.is_empty()),
        "NOAA returned an empty glossary, which does not change with the weather"
    );
}

/// Answers `/glossary` as many times as it is asked.
async fn glossary_server() -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/glossary"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(GLOSSARY, JSON_LD))
        .mount(&server)
        .await;
    server
}

#[tokio::test]
async fn glossary_supports_table_and_json_output() {
    let server = glossary_server().await;

    let table = run_against(&server, &["glossary"]).await;
    assert_eq!(table.status.code(), Some(0), "{}", stderr(&table));
    assert!(String::from_utf8_lossy(&table.stdout).contains("Definition"));

    let json = run_against(&server, &["glossary", "--json"]).await;
    assert_eq!(json.status.code(), Some(0), "{}", stderr(&json));
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).expect("glossary JSON");
    let glossary = value["glossary"].as_array().expect("glossary array");
    let first = glossary
        .first()
        .expect("the glossary fixture carries at least one term");
    assert!(
        first["term"].is_string(),
        "unexpected glossary item: {first}"
    );
}

#[tokio::test]
async fn glossary_supports_file_and_explicit_stdout_destinations() {
    let server = glossary_server().await;
    let directory = tempfile::tempdir().expect("temporary output directory");
    let table_path = directory.path().join("glossary.txt");
    let table_path_arg = table_path.to_str().expect("UTF-8 temporary path");

    let table = run_against(&server, &["glossary", "--output", table_path_arg]).await;
    assert_eq!(table.status.code(), Some(0), "{}", stderr(&table));
    assert!(table.stdout.is_empty());
    let table_text = fs::read_to_string(&table_path).expect("table output file");
    assert!(table_text.contains("Definition"));
    assert!(table_text.ends_with('\n'));
    assert!(!table_text.ends_with("\n\n"));

    let json = run_against(&server, &["glossary", "--json", "--output", "-"]).await;
    assert_eq!(json.status.code(), Some(0), "{}", stderr(&json));
    assert!(json.stdout.ends_with(b"\n"));
    assert!(!json.stdout.ends_with(b"\n\n"));
    let value: serde_json::Value =
        serde_json::from_slice(&json.stdout).expect("explicit stdout JSON");
    assert!(value["glossary"].is_array());
}
