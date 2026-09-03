//! The `gridpoints` family, end to end.

mod common;

use common::noaa_weather;
use common::runner::{family, hermetic, live};

#[tokio::test]
async fn every_gridpoints_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("gridpoints")).await;
}

#[test]
fn test_gridpoints_live_noaa_answers_every_tabled_invocation() {
    live(family("gridpoints"));
}

#[test]
fn test_gridpoints_rejects_malformed_gridpoint_as_usage_error() {
    for (value, reason) in [
        ("PSR/159", "must be OFFICE/x,y"),
        ("PSR/-1,58", "grid x and y must be whole numbers"),
        ("P/159,58", "office code must be"),
    ] {
        let output = noaa_weather()
            .args(["gridpoints", "forecast", value])
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{value}: {stderr}");
        assert!(stderr.contains("invalid gridpoint"), "{value}: {stderr}");
        assert!(stderr.contains(reason), "{value}: {stderr}");
    }
}

#[test]
fn test_gridpoints_rejects_removed_office_and_coordinate_flags() {
    let output = noaa_weather()
        .args([
            "gridpoints",
            "forecast",
            "--forecast-office-id",
            "PSR",
            "--x",
            "159",
            "--y",
            "58",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unexpected argument '--forecast-office-id'"),
        "{stderr}"
    );
}
