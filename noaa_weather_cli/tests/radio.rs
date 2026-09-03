//! The `radio` family, end to end.

mod common;

use common::fixtures::{JSON_LD, RADIO_TRANSMITTERS};
use common::noaa_weather;
use common::runner::{family, hermetic, live, run_against, stderr};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn every_radio_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("radio")).await;
}

#[test]
fn test_radio_live_noaa_answers_every_tabled_invocation() {
    live(family("radio"));
}

#[test]
fn test_radio_station_rejects_malformed_call_sign() {
    let output = noaa_weather()
        .args(["radio", "station", "KE C94"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid call sign"), "{stderr}");
}

#[test]
fn test_radio_station_failure_missing_arg() {
    let output = noaa_weather().args(["radio", "station"]).output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("required arguments were not provided"),
        "{stderr}"
    );
}

/// The transmitter listing renders a table and JSON off one response.
#[tokio::test]
async fn test_radio_transmitters_support_table_and_json() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/radio"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(RADIO_TRANSMITTERS, JSON_LD))
        .mount(&server)
        .await;

    let table = run_against(&server, &["radio", "transmitters"]).await;
    assert_eq!(table.status.code(), Some(0), "{}", stderr(&table));
    assert!(String::from_utf8_lossy(&table.stdout).contains("Call Sign"));

    let json = run_against(&server, &["radio", "transmitters", "--json"]).await;
    assert_eq!(json.status.code(), Some(0), "{}", stderr(&json));
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    let transmitters = value["@graph"].as_array().expect("transmitter graph");
    let first = transmitters
        .first()
        .expect("the transmitter fixture carries at least one entry");
    assert!(first["callSign"].is_string(), "{first}");
}

/// A cursor NOAA published has to be usable as the next page's argument.
///
/// The value comes out of the previous page rather than the table, because a
/// cursor encodes an offset into a list that changes.
#[test]
fn test_radio_transmitters_follow_a_live_cursor() {
    let first = noaa_weather()
        .args(["radio", "transmitters", "--json"])
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        first.status.code(),
        Some(0),
        "`radio transmitters --json` failed: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    let page: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();

    let next = page["pagination"]["next"].as_str().unwrap_or_else(|| {
        panic!("NOAA stopped publishing a `pagination.next` for `/radio`: {page}")
    });
    let cursor = next
        .split("cursor=")
        .nth(1)
        .unwrap_or_else(|| panic!("`pagination.next` carries no cursor parameter: {next}"));

    let second = noaa_weather()
        .args(["radio", "transmitters", "--cursor", cursor, "--json"])
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        second.status.code(),
        Some(0),
        "`radio transmitters --cursor {cursor}` failed: {}",
        String::from_utf8_lossy(&second.stderr)
    );
    let page: serde_json::Value = serde_json::from_slice(&second.stdout).unwrap();
    assert!(
        page["@graph"]
            .as_array()
            .is_some_and(|page| !page.is_empty()),
        "the cursor NOAA published led to an empty page: {page}"
    );
}
