//! The point summary against the captured NOAA fixture.
//!
//! Snapshotted in both unit systems, because the unit system is the one
//! meaning choice a caller makes and a [`Summary`] is where it has to show
//! up: the distance in the title is `4.2 mi` under US and `6.7 km` under SI.

use noaa_weather_client::Feature;
use noaa_weather_client::models::Point;
use noaa_weather_summary::{Summarize, SummaryOptions, UnitSystem, coverage_gaps};

const POINT: &str = include_str!("../../noaa_weather_client/tests/fixtures/points/point.json");

fn point() -> Feature<Point> {
    serde_json::from_str(POINT).expect("point.json decodes")
}

fn options(units: UnitSystem) -> SummaryOptions {
    SummaryOptions { units }
}

#[test]
fn point_summary_snapshot_us() {
    insta::assert_yaml_snapshot!(point().summarize(&options(UnitSystem::Us)));
}

#[test]
fn point_summary_snapshot_si() {
    insta::assert_yaml_snapshot!(point().summarize(&options(UnitSystem::Si)));
}

#[test]
fn point_covers_every_property() {
    let gaps = coverage_gaps(&point());
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

/// The unit system has to reach the [`Summary`], not just the rendering, or
/// `--units` would be a rendering flag pretending to be a meaning flag.
#[test]
fn the_two_unit_systems_disagree() {
    let us = point().summarize(&options(UnitSystem::Us));
    let si = point().summarize(&options(UnitSystem::Si));
    assert_ne!(us, si);
    assert!(us.title.contains(" mi "), "{}", us.title);
    assert!(si.title.contains(" km "), "{}", si.title);
}
