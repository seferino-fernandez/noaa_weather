//! The gridpoint and forecast summaries against the captured NOAA fixtures.
//!
//! Every fixture is snapshotted in both unit systems and checked for coverage
//! gaps. The raw gridpoint is where the 59-layer census has to hold: a layer
//! that arrives under a name this crate does not know must show up as a gap
//! rather than pass unnoticed, and the last test here proves it does.

use noaa_weather_client::models::{Forecast, Gridpoint};
use noaa_weather_client::{Feature, Interval};
use noaa_weather_summary::{
    Emphasis, Section, Summarize, SummaryOptions, UnitSystem, Value, coverage_gaps,
};

const GRIDPOINT: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/gridpoints/gridpoint.json");
const FORECAST: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/gridpoints/forecast.json");
const HOURLY: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/gridpoints/hourly.json");

fn gridpoint() -> Feature<Gridpoint> {
    serde_json::from_str(GRIDPOINT).expect("gridpoint.json decodes")
}

fn forecast() -> Feature<Forecast> {
    serde_json::from_str(FORECAST).expect("forecast.json decodes")
}

fn hourly() -> Feature<Forecast> {
    serde_json::from_str(HOURLY).expect("hourly.json decodes")
}

fn options(units: UnitSystem) -> SummaryOptions {
    SummaryOptions { units }
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn gridpoint_summary_snapshot_us() {
    insta::assert_yaml_snapshot!(gridpoint().summarize(&options(UnitSystem::Us)));
}

#[test]
fn gridpoint_summary_snapshot_si() {
    insta::assert_yaml_snapshot!(gridpoint().summarize(&options(UnitSystem::Si)));
}

#[test]
fn gridpoint_covers_every_property() {
    assert_no_gaps(&gridpoint());
}

#[test]
fn forecast_summary_snapshot_us() {
    insta::assert_yaml_snapshot!(forecast().summarize(&options(UnitSystem::Us)));
}

#[test]
fn forecast_summary_snapshot_si() {
    insta::assert_yaml_snapshot!(forecast().summarize(&options(UnitSystem::Si)));
}

#[test]
fn forecast_covers_every_property() {
    assert_no_gaps(&forecast());
}

#[test]
fn hourly_summary_snapshot_us() {
    insta::assert_yaml_snapshot!(hourly().summarize(&options(UnitSystem::Us)));
}

#[test]
fn hourly_summary_snapshot_si() {
    insta::assert_yaml_snapshot!(hourly().summarize(&options(UnitSystem::Si)));
}

#[test]
fn hourly_covers_every_property() {
    assert_no_gaps(&hourly());
}

/// The twelve-hour wind is the reason [`Value::Range`] exists: NOAA sends it
/// as `minValue`/`maxValue` with a null `value`, and the CLI used to print
/// `N/A S gust 40.2336 km_h-1` for it.
#[test]
fn the_twelve_hour_wind_reads_as_a_range() {
    let summary = forecast().summarize(&options(UnitSystem::Us));
    let Some(Section::Table { rows, .. }) = summary.sections.last() else {
        panic!("the twelve-hour forecast ends in its table");
    };
    let winds: Vec<&Value> = rows.iter().map(|row| &row[3].value).collect();
    let ranged = winds
        .iter()
        .filter_map(|value| match value {
            Value::Text(text) if text.contains(" to ") => Some(text.as_str()),
            _ => None,
        })
        .next()
        .expect("at least one period sends bounds rather than a value");
    assert_eq!(ranged, "10 to 15 mph S gust 25 mph");
    for wind in &winds {
        let Value::Text(text) = wind else {
            panic!("a wind is one phrase, got {wind:?}");
        };
        assert!(!text.contains("N/A"), "{text}");
        assert!(!text.contains("km_h-1"), "{text}");
    }
}

/// A hazard period is the one thing in a gridpoint worth coloring.
#[test]
fn a_hazard_period_carries_emphasis() {
    let summary = gridpoint().summarize(&SummaryOptions::default());
    let hazards = summary
        .sections
        .iter()
        .find_map(|section| match section {
            Section::Table { heading, rows, .. } if heading.as_deref() == Some("Hazards") => {
                Some(rows)
            }
            _ => None,
        })
        .expect("the fixture grid has a hazard");
    for row in hazards {
        for cell in row {
            assert!(
                matches!(cell.emphasis, Emphasis::Warning | Emphasis::Danger),
                "{cell:?}"
            );
        }
    }
    assert_eq!(
        hazards[0][1].value,
        Value::Lines(vec![Value::Text("Heat Advisory".to_owned())])
    );
}

/// The 59 known keys ride on the layers table's `also`; a sixtieth does not,
/// which is the whole point of listing them rather than waving them through.
#[test]
fn a_layer_this_crate_does_not_know_surfaces_as_a_gap() {
    let mut grid = gridpoint();
    let sunshine = serde_json::from_value(serde_json::json!({
        "uom": "wmoUnit:percent",
        "values": [{"validTime": "2026-09-02T00:00:00+00:00/PT1H", "value": 70}],
    }))
    .expect("a layer decodes");
    grid.properties
        .other
        .insert("sunshinePercentage".to_owned(), sunshine);

    assert_eq!(coverage_gaps(&grid), vec!["sunshinePercentage".to_owned()]);

    // It is still rendered — a gap is an alarm about the key list, not a
    // reason to drop the data.
    let summary = grid.summarize(&SummaryOptions::default());
    let Some(Section::Table { rows, .. }) = summary
        .sections
        .iter()
        .find(|section| matches!(section, Section::Table { heading, .. } if heading.as_deref() == Some("Layers")))
    else {
        panic!("the layers table is there");
    };
    assert!(
        rows.iter()
            .any(|row| row[0].value == Value::Text("sunshinePercentage".to_owned())),
        "an unknown layer still gets a row"
    );
}

/// `validTimes` arrives in the `start/duration` form, the only form NOAA
/// writes it in, so the covered-interval fact needs `resolved_end` to have an
/// end at all.
#[test]
fn the_covered_interval_resolves_its_end() {
    let summary = gridpoint().summarize(&SummaryOptions::default());
    let Some(Section::Facts { facts, .. }) = summary.sections.first() else {
        panic!("a gridpoint opens with its facts");
    };
    let covers = facts
        .iter()
        .find(|fact| fact.key == Some("validTimes"))
        .expect("the covered interval is a fact");
    let Value::Interval { end, .. } = covers.value else {
        panic!(
            "the covered interval is an interval, got {:?}",
            covers.value
        );
    };
    let valid_times: Interval = "2026-09-02T00:00:00+00:00/P8DT1H".parse().unwrap();
    assert_eq!(
        end,
        valid_times.resolved_end().map(Into::into),
        "the end is the start plus the span"
    );
    assert!(end.is_some(), "the Starting form still has an end");
}
