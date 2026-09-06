use std::collections::BTreeSet;

use noaa_weather_client::radar::{
    RadarQueuesResponse, RadarServerTelemetry, RadarServersResponse, RadarSpgdsResponse,
    RadarStationAlarmsResponse, RadarStationTelemetry, RadarStationsResponse,
};
use rmcp::handler::server::tool::schema_for_output;
use rmcp::model::Tool;
use schemars::JsonSchema;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{
    assert_json_success, assert_metadata, call, result_text, server_with_router,
};

const QUEUE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radar/queue.json"
));
const SERVERS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radar/servers.json"
));
const SERVER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radar/server.json"
));
const STATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radar/stations.json"
));
const STATION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radar/station.json"
));
const ALARMS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radar/alarms.json"
));
const SPGDS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radar/spgds.json"
));

fn tool<'a>(tools: &'a [Tool], name: &str) -> &'a Tool {
    tools
        .iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("missing {name} tool"))
}

fn properties(tool: &Tool) -> BTreeSet<&str> {
    tool.input_schema["properties"]
        .as_object()
        .expect("properties must be an object")
        .keys()
        .map(String::as_str)
        .collect()
}

fn required(tool: &Tool) -> BTreeSet<&str> {
    tool.input_schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|name| name.as_str().expect("required name must be text"))
        .collect()
}

fn assert_contract<T: JsonSchema + 'static>(
    tools: &[Tool],
    name: &str,
    expected_properties: &[&str],
    expected_required: &[&str],
    title: &str,
) {
    let tool = tool(tools, name);
    assert_eq!(
        properties(tool),
        expected_properties.iter().copied().collect()
    );
    assert_eq!(required(tool), expected_required.iter().copied().collect());
    assert_eq!(tool.output_schema.as_ref(), Some(&schema_for_output::<T>()));
    assert_metadata(tool, title);
}

async fn mount(upstream: &MockServer, route: &str, fixture: &'static str, media: &'static str) {
    Mock::given(method("GET"))
        .and(path(route))
        .and(header("Accept", media))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture, media))
        .expect(1)
        .mount(upstream)
        .await;
}

#[test]
fn router_publishes_exact_flat_typed_radar_contracts() {
    let tools = NoaaWeatherServer::radar_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "radar_queue",
            "radar_server_get",
            "radar_servers",
            "radar_spgds",
            "radar_station_alarms",
            "radar_station_get",
            "radar_stations",
        ]
    );

    assert_contract::<RadarQueuesResponse>(
        &tools,
        "radar_queue",
        &[
            "host",
            "limit",
            "arrived",
            "created",
            "published",
            "station",
            "type",
            "feed",
            "resolution",
        ],
        &["host"],
        "Get Radar Queue",
    );
    assert!(!properties(tool(&tools, "radar_queue")).contains("dataType"));
    assert_contract::<RadarServersResponse>(
        &tools,
        "radar_servers",
        &["reportingHost"],
        &[],
        "List Radar Servers",
    );
    assert_contract::<RadarServerTelemetry>(
        &tools,
        "radar_server_get",
        &["serverId", "reportingHost"],
        &["serverId"],
        "Get Radar Server",
    );
    assert_contract::<RadarStationsResponse>(
        &tools,
        "radar_stations",
        &["stationType", "reportingHost", "host"],
        &[],
        "List Radar Stations",
    );
    assert_contract::<RadarStationTelemetry>(
        &tools,
        "radar_station_get",
        &["stationId", "reportingHost", "host"],
        &["stationId"],
        "Get Radar Station",
    );
    assert_contract::<RadarStationAlarmsResponse>(
        &tools,
        "radar_station_alarms",
        &["stationId"],
        &["stationId"],
        "List Radar Station Alarms",
    );
    assert_contract::<RadarSpgdsResponse>(
        &tools,
        "radar_spgds",
        &["published"],
        &[],
        "Get Radar SPGDS Telemetry",
    );
}

#[tokio::test]
async fn every_radar_tool_preserves_route_query_media_and_typed_json_parity() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::radar_router()).await;
    mount(&upstream, "/radar/queues/rds", QUEUE, "application/ld+json").await;
    mount(&upstream, "/radar/servers", SERVERS, "application/ld+json").await;
    mount(
        &upstream,
        "/radar/servers/ldm%2Fone%20host",
        SERVER,
        "application/ld+json",
    )
    .await;
    mount(
        &upstream,
        "/radar/stations",
        STATIONS,
        "application/geo+json",
    )
    .await;
    mount(
        &upstream,
        "/radar/stations/KXYZ",
        STATION,
        "application/geo+json",
    )
    .await;
    mount(
        &upstream,
        "/radar/stations/KABQ/alarms",
        ALARMS,
        "application/ld+json",
    )
    .await;
    mount(&upstream, "/radar/spgds", SPGDS, "application/ld+json").await;

    let result = call(
        &server,
        "radar_queue",
        json!({
            "host": "rds",
            "arrived": "2026-08-30T12:00:00Z/PT1H",
            "created": "PT15M/2026-08-30T12:00:00Z",
            "published": "PT30M",
            "station": "kiwa",
            "type": "LEVEL2",
            "feed": "level2",
            "resolution": 1
        }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadarQueuesResponse>(QUEUE).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "radar_servers",
        json!({ "reportingHost": "report/host" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadarServersResponse>(SERVERS).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "radar_server_get",
        json!({ "serverId": "ldm/one host", "reportingHost": "report/host" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadarServerTelemetry>(SERVER).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "radar_stations",
        json!({
            "stationType": ["WSR-88D", "TD/WR"],
            "reportingHost": "report/host",
            "host": "rds"
        }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadarStationsResponse>(STATIONS).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "radar_station_get",
        json!({
            "stationId": "kxyz",
            "reportingHost": "report/host",
            "host": "tds"
        }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadarStationTelemetry>(STATION).expect("fixture must decode"),
    );
    assert_eq!(
        result
            .structured_content
            .as_ref()
            .and_then(|value| value.pointer("/properties/id")),
        Some(&json!("KXYZ"))
    );

    let result = call(
        &server,
        "radar_station_alarms",
        json!({ "stationId": "kabq" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadarStationAlarmsResponse>(ALARMS).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "radar_spgds",
        json!({
            "published": "2026-01-01T00:00:00+00:00/2026-01-01T01:30:00+00:00"
        }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadarSpgdsResponse>(SPGDS).expect("fixture must decode"),
    );

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 7);
    assert_eq!(
        requests
            .iter()
            .map(|request| request.url.path())
            .collect::<Vec<_>>(),
        [
            "/radar/queues/rds",
            "/radar/servers",
            "/radar/servers/ldm%2Fone%20host",
            "/radar/stations",
            "/radar/stations/KXYZ",
            "/radar/stations/KABQ/alarms",
            "/radar/spgds",
        ]
    );
    assert_eq!(
        requests[0].url.query(),
        Some(
            "arrived=2026-08-30T12%3A00%3A00%2B00%3A00%2FPT1H\
             &created=PT15M%2F2026-08-30T12%3A00%3A00%2B00%3A00&published=PT30M\
             &station=KIWA&type=LEVEL2&feed=level2&resolution=1"
        )
    );
    assert!(!requests[0].url.query().unwrap().contains("limit="));
    assert_eq!(requests[1].url.query(), Some("reportingHost=report%2Fhost"));
    assert_eq!(requests[2].url.query(), Some("reportingHost=report%2Fhost"));
    assert_eq!(
        requests[3].url.query(),
        Some("stationType=WSR-88D%2CTD%2FWR&reportingHost=report%2Fhost&host=rds")
    );
    assert_eq!(
        requests[4].url.query(),
        Some("reportingHost=report%2Fhost&host=tds")
    );
    assert_eq!(requests[5].url.query(), None);
    assert_eq!(
        requests[6].url.query(),
        Some("published=2026-01-01T00%3A00%3A00%2B00%3A00%2F2026-01-01T01%3A30%3A00%2B00%3A00")
    );
}

#[tokio::test]
async fn invalid_radar_station_and_host_are_rejected_before_http() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::radar_router()).await;

    for (name, arguments) in [
        (
            "radar_station_get",
            json!({ "stationId": "ABC", "reportingHost": null, "host": null }),
        ),
        ("radar_queue", json!({ "host": "unknown" })),
    ] {
        let result = call(&server, name, arguments).await;
        assert_eq!(result.is_error, Some(true));
        assert!(result.structured_content.is_none());
        assert!(result_text(&result).contains("failed to deserialize parameters"));
    }

    assert!(
        upstream
            .received_requests()
            .await
            .expect("requests must be readable")
            .is_empty()
    );
}

#[tokio::test]
async fn radar_upstream_failure_is_bounded_json_without_body_leakage() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::radar_router()).await;
    Mock::given(method("GET"))
        .and(path("/radar/spgds"))
        .respond_with(ResponseTemplate::new(503).set_body_string("private radar body"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "radar_spgds", json!({})).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    let failure: Value = serde_json::from_str(text).expect("failure must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    assert!(!text.contains("private radar body"));
}
