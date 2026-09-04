//! Zone summaries against captured NOAA fixtures.

use noaa_weather_client::models::{Zone, ZoneForecast};
use noaa_weather_client::{Feature, FeatureCollection};
use noaa_weather_summary::{Section, Summarize, SummaryOptions, coverage_gaps};

const ZONE: &str = include_str!("../../noaa_weather_client/tests/fixtures/zones/single.json");
const ZONES: &str = include_str!("../../noaa_weather_client/tests/fixtures/zones/list.json");
const FORECAST: &str = include_str!("../../noaa_weather_client/tests/fixtures/zones/forecast.json");

fn zone() -> Feature<Zone> {
    serde_json::from_str(ZONE).expect("single zone decodes")
}

fn zones() -> FeatureCollection<Zone> {
    serde_json::from_str(ZONES).expect("zone list decodes")
}

fn forecast() -> Feature<ZoneForecast> {
    serde_json::from_str(FORECAST).expect("zone forecast decodes")
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn zone_summary_snapshot() {
    insta::assert_yaml_snapshot!(zone().summarize(&SummaryOptions::default()));
}

#[test]
fn zone_collection_summary_snapshot() {
    let mut zones = zones();
    zones.features.truncate(2);
    insta::assert_yaml_snapshot!(zones.summarize(&SummaryOptions::default()));
}

#[test]
fn zone_forecast_summary_snapshot() {
    let mut forecast = forecast();
    forecast.properties.periods.truncate(2);
    insta::assert_yaml_snapshot!(forecast.summarize(&SummaryOptions::default()));
}

#[test]
fn every_zone_shape_covers_every_property() {
    assert_no_gaps(&zone());
    assert_no_gaps(&zones());
    assert_no_gaps(&forecast());
}

#[test]
fn forecast_summary_accounts_for_every_period_property() {
    let summary = forecast().summarize(&SummaryOptions::default());
    let keys = summary.keys();
    for key in ["periods", "name", "detailedForecast"] {
        assert!(
            keys.contains(key),
            "forecast summary does not account for {key}"
        );
    }
    assert!(
        <Feature<ZoneForecast> as Summarize>::OMITTED
            .iter()
            .any(|(key, _)| *key == "number"),
        "period number must be shown or explicitly omitted"
    );
}

#[test]
fn empty_collections_and_forecasts_explain_why_there_are_no_rows() {
    let empty_zones = FeatureCollection::<Zone> {
        features: Vec::new(),
        title: None,
        updated: None,
        pagination: None,
    };
    let mut empty_forecast = forecast();
    empty_forecast.properties.periods.clear();

    for summary in [
        empty_zones.summarize(&SummaryOptions::default()),
        empty_forecast.summarize(&SummaryOptions::default()),
    ] {
        assert!(
            summary
                .sections
                .iter()
                .any(|section| matches!(section, Section::Empty { .. }))
        );
    }
}
