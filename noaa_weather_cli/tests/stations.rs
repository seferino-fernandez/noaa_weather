//! The `stations` family, end to end.

mod common;

use std::process::Output;

use common::noaa_weather;
use common::runner::{family, hermetic, live};
use serde_json::Value;

#[tokio::test]
async fn every_stations_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("stations")).await;
}

#[test]
fn test_stations_live_noaa_answers_every_tabled_invocation() {
    live(family("stations"));
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

    let Some(first) = forecasts.first() else {
        eprintln!(
            "`{what}` returned a well-formed empty `@graph`, so KPHX has no \
             current TAF and there was no issue time to fetch. The listing \
             endpoint was checked; `terminal-aerodrome-forecast` was not."
        );
        return;
    };

    let id = first["id"]
        .as_str()
        .expect("NOAA returned at least one current KPHX TAF identifier");
    let mut segments = id.trim_end_matches('/').rsplit('/');
    let time = segments.next().expect("TAF identifier time segment");
    let date = segments.next().expect("TAF identifier date segment");
    let (hours, minutes) = time.split_at(2);
    let issued = format!("{date}T{hours}:{minutes}:00Z");

    let output = succeeding(&[
        "stations",
        "terminal-aerodrome-forecast",
        "--station-id",
        "KPHX",
        "--issued",
        &issued,
    ]);

    let table = String::from_utf8(output.stdout).unwrap();
    assert!(table.contains("KPHX"), "{table}");
    assert!(table.contains("Report state"), "{table}");
}
