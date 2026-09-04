//! Station, observation, and TAF summaries against captured NOAA fixtures.

use noaa_weather_client::models::{
    Observation, ObservationStation, TerminalAerodromeForecast, TerminalAerodromeForecastsResponse,
};
use noaa_weather_client::{Feature, FeatureCollection};
use noaa_weather_summary::stations::ZoneObservations;
use noaa_weather_summary::{Section, Summarize, SummaryOptions, coverage_gaps};

const STATION: &str = include_str!("../../noaa_weather_client/tests/fixtures/stations/single.json");
const STATIONS: &str = include_str!("../../noaa_weather_client/tests/fixtures/stations/list.json");
const OBSERVATION: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/stations/latest.json");
const OBSERVATIONS: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/stations/observations.json");
const ZONE_OBSERVATIONS: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/zones/observations.json");
const TAFS: &str = include_str!("../../noaa_weather_client/tests/fixtures/stations/tafs.json");
const TAF: &[u8] = include_bytes!("../../noaa_weather_client/tests/fixtures/stations/taf.xml");

fn station() -> Feature<ObservationStation> {
    serde_json::from_str(STATION).expect("single station decodes")
}

fn stations() -> FeatureCollection<ObservationStation> {
    serde_json::from_str(STATIONS).expect("station list decodes")
}

fn observation() -> Feature<Observation> {
    serde_json::from_str(OBSERVATION).expect("latest observation decodes")
}

fn observations() -> FeatureCollection<Observation> {
    serde_json::from_str(OBSERVATIONS).expect("observation history decodes")
}

fn tafs() -> TerminalAerodromeForecastsResponse {
    serde_json::from_str(TAFS).expect("TAF metadata decodes")
}

#[test]
fn station_summary_snapshot() {
    insta::assert_yaml_snapshot!(station().summarize(&SummaryOptions::default()));
}

#[test]
fn station_collection_summary_snapshot() {
    let mut stations = stations();
    stations.features.truncate(2);
    insta::assert_yaml_snapshot!(stations.summarize(&SummaryOptions::default()));
}

#[test]
fn observation_summary_snapshot() {
    insta::assert_yaml_snapshot!(observation().summarize(&SummaryOptions::default()));
}

#[test]
fn observation_collection_summary_snapshot() {
    let mut observations = observations();
    observations.features.truncate(2);
    insta::assert_yaml_snapshot!(observations.summarize(&SummaryOptions::default()));
}

#[test]
fn zone_observation_summary_snapshot() {
    let mut observations: FeatureCollection<Observation> =
        serde_json::from_str(ZONE_OBSERVATIONS).expect("zone observations decode");
    observations.features.truncate(2);
    insta::assert_yaml_snapshot!(
        ZoneObservations(observations).summarize(&SummaryOptions::default())
    );
}

#[test]
fn taf_metadata_summary_snapshot() {
    let mut tafs = tafs();
    tafs.forecasts.truncate(2);
    insta::assert_yaml_snapshot!(tafs.summarize(&SummaryOptions::default()));
}

#[test]
fn decoded_taf_summary_snapshot() {
    let taf = TerminalAerodromeForecast::from_iwxxm(TAF).expect("TAF IWXXM decodes");
    insta::assert_yaml_snapshot!(taf.summarize(&SummaryOptions::default()));
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn every_station_json_shape_covers_every_property() {
    assert_no_gaps(&station());
    assert_no_gaps(&stations());
    assert_no_gaps(&observation());
    assert_no_gaps(&observations());
    assert_no_gaps(&tafs());
    let zone: FeatureCollection<Observation> = serde_json::from_str(ZONE_OBSERVATIONS).unwrap();
    assert_no_gaps(&ZoneObservations(zone));
}

#[test]
fn empty_collections_explain_why_there_are_no_rows() {
    let empty_stations = FeatureCollection::<ObservationStation> {
        features: Vec::new(),
        title: None,
        updated: None,
        pagination: None,
    };
    let empty_observations = FeatureCollection::<Observation> {
        features: Vec::new(),
        title: None,
        updated: None,
        pagination: None,
    };
    let empty_tafs: TerminalAerodromeForecastsResponse =
        serde_json::from_str(r#"{"@graph": []}"#).unwrap();

    for summary in [
        empty_stations.summarize(&SummaryOptions::default()),
        empty_observations.summarize(&SummaryOptions::default()),
        empty_tafs.summarize(&SummaryOptions::default()),
    ] {
        assert!(matches!(
            summary.sections.as_slice(),
            [Section::Empty { .. }]
        ));
    }
}
