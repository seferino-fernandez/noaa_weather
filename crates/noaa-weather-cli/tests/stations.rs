//! The `stations` family, end to end.

mod common;

use std::process::Output;

use common::noaa_weather;
use common::runner::{check_payload, family, hermetic, live};
use common::table::Live;
use serde_json::{Value, json};

#[tokio::test]
async fn every_stations_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("stations")).await;
}

#[test]
fn test_stations_live_noaa_answers_every_tabled_invocation() {
    live(family("stations"));
}

#[test]
fn an_empty_current_taf_listing_is_valid_live_data() {
    let invocation = family("stations")
        .invocations
        .iter()
        .find(|invocation| invocation.command == ["stations", "terminal-aerodrome-forecasts"])
        .expect("the stations table must cover current TAF listings");
    let Live::Check(expectation) = &invocation.live else {
        panic!("the current TAF listing must remain covered by the live suite");
    };

    check_payload(invocation, expectation, br#"{"@graph":[]}"#);
}

fn succeeding(arguments: &[&str]) -> Output {
    let output = noaa_weather()
        .args(arguments)
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        output.status.code(),
        Some(0),
        "`{}` failed: {}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn json(output: &Output, what: &str) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "`{what}` did not emit JSON: {error}\n{}",
            String::from_utf8_lossy(&output.stdout)
        )
    })
}

#[derive(Debug, PartialEq, Eq)]
enum TafFetchError {
    NotFound,
    Failed(String),
}

fn taf_issue_time(forecast: &Value) -> String {
    let id = forecast["id"]
        .as_str()
        .expect("NOAA returned a current TAF without an identifier");
    let mut segments = id.trim_end_matches('/').rsplit('/');
    let time = segments.next().expect("TAF identifier time segment");
    let date = segments.next().expect("TAF identifier date segment");
    let (hours, minutes) = time.split_at(2);
    format!("{date}T{hours}:{minutes}:00Z")
}

fn first_available_taf<T>(
    forecasts: &[Value],
    mut fetch: impl FnMut(&str) -> Result<T, TafFetchError>,
) -> Result<Option<T>, TafFetchError> {
    for forecast in forecasts {
        match fetch(&taf_issue_time(forecast)) {
            Ok(output) => return Ok(Some(output)),
            Err(TafFetchError::NotFound) => {}
            Err(error @ TafFetchError::Failed(_)) => return Err(error),
        }
    }
    Ok(None)
}

fn fetch_taf(issued: &str) -> Result<Output, TafFetchError> {
    let arguments = [
        "stations",
        "terminal-aerodrome-forecast",
        "--station-id",
        "KPHX",
        "--issued",
        issued,
    ];
    let output = noaa_weather()
        .args(arguments)
        .output()
        .expect("the built binary must be runnable");
    if output.status.success() {
        return Ok(output);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.code() == Some(3) && stderr.contains("HTTP 404 Not Found") {
        Err(TafFetchError::NotFound)
    } else {
        Err(TafFetchError::Failed(format!(
            "`{}` failed: {stderr}",
            arguments.join(" ")
        )))
    }
}

#[test]
fn a_dangling_taf_identifier_does_not_hide_a_later_current_taf() {
    let listing = json!({
        "@graph": [
            {"id": "https://api.weather.gov/stations/KPHX/tafs/2026-08-30/2254"},
            {"id": "https://api.weather.gov/stations/KPHX/tafs/2026-09-06/2354"}
        ]
    });
    let forecasts = listing["@graph"].as_array().unwrap();
    let mut attempted = Vec::new();

    let fetched = first_available_taf(forecasts, |issued| {
        attempted.push(issued.to_owned());
        if issued == "2026-08-30T22:54:00Z" {
            Err(TafFetchError::NotFound)
        } else {
            Ok(issued.to_owned())
        }
    });

    assert_eq!(fetched, Ok(Some("2026-09-06T23:54:00Z".to_owned())));
    assert_eq!(attempted, ["2026-08-30T22:54:00Z", "2026-09-06T23:54:00Z"]);
}

#[test]
fn an_entirely_dangling_taf_listing_has_no_available_forecast() {
    let listing = json!({
        "@graph": [
            {"id": "https://api.weather.gov/stations/KPHX/tafs/2026-08-30/2254"},
            {"id": "https://api.weather.gov/stations/KPHX/tafs/2026-08-30/2251"}
        ]
    });
    let forecasts = listing["@graph"].as_array().unwrap();
    let mut attempted = Vec::new();

    let fetched: Result<Option<()>, TafFetchError> = first_available_taf(forecasts, |issued| {
        attempted.push(issued.to_owned());
        Err(TafFetchError::NotFound)
    });

    assert_eq!(fetched, Ok(None));
    assert_eq!(attempted, ["2026-08-30T22:54:00Z", "2026-08-30T22:51:00Z"]);
}

#[test]
fn test_stations_reject_malformed_station_id_and_time() {
    let output = noaa_weather()
        .args(["stations", "metadata", "--id", "K PHX"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid station id"), "{stderr}");

    let output = noaa_weather()
        .args([
            "stations",
            "observations",
            "--station-id",
            "KPHX",
            "--start",
            "yesterday",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("RFC 3339"), "{stderr}");
}

/// Fetches one observation by an instant resolved at run time.
///
/// `stations observation` addresses a single observation by the exact time
/// it was taken, so the only way to drive it is to read one out of the
/// listing first. Nothing else in the suite reaches this route.
#[test]
fn an_observation_is_fetched_by_a_time_resolved_at_run_time() {
    let what = "stations observations --station-id KPHX --limit 1 --json";
    let listing = json(
        &succeeding(&[
            "stations",
            "observations",
            "--station-id",
            "KPHX",
            "--limit",
            "1",
            "--json",
        ]),
        what,
    );
    let observations = listing["features"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` returned no `features` array: {listing}"));

    let Some(first) = observations.first() else {
        eprintln!(
            "`{what}` returned a well-formed empty `features` array, so KPHX \
             has reported nothing recently and there was no instant to fetch. \
             The listing endpoint was checked; `stations observation` was not."
        );
        return;
    };

    let timestamp = first["properties"]["timestamp"]
        .as_str()
        .unwrap_or_else(|| panic!("the first observation has no `properties.timestamp`: {first}"));

    let fetched = succeeding(&[
        "stations",
        "observation",
        "--station-id",
        "KPHX",
        "--time",
        timestamp,
        "--json",
    ]);
    let fetched = json(&fetched, "stations observation --json");
    assert_eq!(
        fetched["properties"]["timestamp"], first["properties"]["timestamp"],
        "NOAA answered `stations observation --time {timestamp}` with a \
         different observation"
    );
}

/// Fetches one TAF by the issue minute of a current one.
#[test]
fn test_stations_taf_success() {
    let what = "stations terminal-aerodrome-forecasts --station-id KPHX --json";
    let metadata = json(
        &succeeding(&[
            "stations",
            "terminal-aerodrome-forecasts",
            "--station-id",
            "KPHX",
            "--json",
        ]),
        what,
    );
    let forecasts = metadata["@graph"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` returned no `@graph` array: {metadata}"));

    let output = match first_available_taf(forecasts, fetch_taf) {
        Ok(Some(output)) => output,
        Ok(None) => {
            if forecasts.is_empty() {
                eprintln!(
                    "`{what}` returned a well-formed empty `@graph`, so KPHX has no \
                     current TAF and there was no issue time to fetch. The listing \
                     endpoint was checked; `terminal-aerodrome-forecast` was not."
                );
            } else {
                eprintln!(
                    "`{what}` advertised {} TAF entries, but every singleton \
                     returned 404. The listing endpoint was checked; \
                     `terminal-aerodrome-forecast` was unavailable.",
                    forecasts.len()
                );
            }
            return;
        }
        Err(TafFetchError::NotFound) => unreachable!("404s are exhausted as candidates"),
        Err(TafFetchError::Failed(message)) => panic!("{message}"),
    };

    let table = String::from_utf8(output.stdout).unwrap();
    assert!(table.contains("KPHX"), "{table}");
    assert!(table.contains("Report state"), "{table}");
}
