//! The `points` family, end to end.

mod common;

use common::noaa_weather;
use common::runner::{family, hermetic, live};

#[tokio::test]
async fn every_points_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("points")).await;
}

#[test]
fn test_points_live_noaa_answers_every_tabled_invocation() {
    live(family("points"));
}

#[test]
fn test_points_command_failure_invalid_point() {
    for value in ["test", "39.7456", "91,-97.0892", "39.7456,-181"] {
        let output = noaa_weather()
            .args(["points", "metadata", value])
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{value}: {stderr}");
        assert!(stderr.contains("invalid coordinates"), "{value}: {stderr}");
    }
}

#[test]
fn test_points_command_rejects_two_positional_values() {
    let output = noaa_weather()
        .args(["points", "metadata", "39.7456", "--", "-97.0892"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unexpected argument '-97.0892'"),
        "{stderr}"
    );
}

#[test]
fn test_points_command_rejects_removed_stations_subcommand() {
    let output = noaa_weather()
        .args(["points", "stations", "33.4484,-112.0740"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unrecognized subcommand 'stations'"),
        "{stderr}"
    );
}
