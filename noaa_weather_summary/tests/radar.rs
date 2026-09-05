//! Radar summaries against every captured response shape.

use noaa_weather_client::models::{
    RadarQueuesResponse, RadarServerTelemetry, RadarServersResponse, RadarSpgdsResponse,
    RadarStationAlarmsResponse, RadarStationTelemetry, RadarStationsResponse,
};
use noaa_weather_summary::{Summarize, SummaryOptions, coverage_gaps};

const KFSX: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/KFSX.json");
const KLNX: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/KLNX.json");
const TSLC: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/TSLC.json");
const STATION: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/station.json");
const STATIONS: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/stations.json");
const QUEUE: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/queue.json");
const SERVER: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/server.json");
const SERVERS: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/servers.json");
const ALARMS: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/alarms.json");
const SPGDS: &str = include_str!("../../noaa_weather_client/tests/fixtures/radar/spgds.json");

fn decode<T: serde::de::DeserializeOwned>(source: &str) -> T {
    serde_json::from_str(source).expect("radar fixture decodes")
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "radar fields neither shown nor deliberately omitted: {gaps:?}"
    );
}

#[test]
fn station_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<RadarStationTelemetry>(STATION).summarize(&SummaryOptions::default())
    );
}

#[test]
fn stations_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<RadarStationsResponse>(STATIONS).summarize(&SummaryOptions::default())
    );
}

#[test]
fn queue_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<RadarQueuesResponse>(QUEUE).summarize(&SummaryOptions::default())
    );
}

#[test]
fn server_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<RadarServerTelemetry>(SERVER).summarize(&SummaryOptions::default())
    );
}

#[test]
fn servers_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<RadarServersResponse>(SERVERS).summarize(&SummaryOptions::default())
    );
}

#[test]
fn alarms_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<RadarStationAlarmsResponse>(ALARMS).summarize(&SummaryOptions::default())
    );
}

#[test]
fn spgds_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<RadarSpgdsResponse>(SPGDS).summarize(&SummaryOptions::default())
    );
}

#[test]
fn every_radar_fixture_has_complete_summary_coverage() {
    for source in [KFSX, KLNX, TSLC, STATION] {
        assert_no_gaps(&decode::<RadarStationTelemetry>(source));
    }
    assert_no_gaps(&decode::<RadarStationsResponse>(STATIONS));
    assert_no_gaps(&decode::<RadarQueuesResponse>(QUEUE));
    assert_no_gaps(&decode::<RadarServerTelemetry>(SERVER));
    assert_no_gaps(&decode::<RadarServersResponse>(SERVERS));
    assert_no_gaps(&decode::<RadarStationAlarmsResponse>(ALARMS));
    assert_no_gaps(&decode::<RadarSpgdsResponse>(SPGDS));
}
