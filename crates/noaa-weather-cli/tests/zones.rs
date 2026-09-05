//! The `zones` family, end to end.

mod common;

use common::noaa_weather;
use common::runner::{family, hermetic, live};

#[tokio::test]
async fn every_zones_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("zones")).await;
}

#[test]
fn test_zones_live_noaa_answers_every_tabled_invocation() {
    live(family("zones"));
}

#[test]
fn test_zones_rejects_a_malformed_zone_and_an_unknown_type() {
    let output = noaa_weather()
        .args(["zones", "metadata", "--id", "AZZ 543", "--type", "public"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid zone id"), "{stderr}");

    let output = noaa_weather()
        .args(["zones", "metadata", "--id", "AZZ543", "--type", "weather"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("--type <TYPE>"), "{stderr}");
}
