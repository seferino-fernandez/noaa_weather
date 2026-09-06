use std::collections::BTreeSet;

use noaa_weather_client::geo::{Feature, FeatureCollection};
use noaa_weather_client::stations::{Observation, ObservationStation};
use noaa_weather_client::zones::{Zone, ZoneForecast};
use rmcp::handler::server::tool::schema_for_output;
use rmcp::model::{CallToolResult, Tool};
use schemars::JsonSchema;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{
    assert_json_success, assert_metadata, call, result_text, server_with_router,
};

const ZONE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/zones/single.json"
));
const FORECAST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/zones/forecast.json"
));
const ZONES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/zones/list.json"
));
const OBSERVATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/zones/observations.json"
));
const STATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/zones/stations.json"
));

fn tool<'a>(tools: &'a [Tool], name: &str) -> &'a Tool {
    tools
        .iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("missing {name} tool"))
}

fn names(value: Option<&Value>) -> BTreeSet<&str> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|name| name.as_str().expect("schema name must be text"))
        .collect()
}

fn assert_contract<T: JsonSchema + 'static>(
    tool: &Tool,
    title: &str,
    properties: &[&str],
    required: &[&str],
) {
    assert_metadata(tool, title);
    let actual_properties = tool.input_schema["properties"]
        .as_object()
        .expect("input properties must be an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_properties, properties.iter().copied().collect());
    assert_eq!(
        names(tool.input_schema.get("required")),
        required.iter().copied().collect()
    );
    assert!(!actual_properties.contains("query"));
    assert_eq!(tool.output_schema.as_ref(), Some(&schema_for_output::<T>()));
}

fn assert_parameter_failure(result: &CallToolResult) {
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    assert!(result_text(result).starts_with("failed to deserialize parameters:"));
}

async fn no_requests(upstream: &MockServer) {
    assert!(
        upstream
            .received_requests()
            .await
            .expect("recorded requests must be readable")
            .is_empty()
    );
}

#[tokio::test]
async fn router_inventory_has_exact_flat_schemas_outputs_and_annotations() {
    let (_upstream, server) = server_with_router(NoaaWeatherServer::zones_router()).await;
    let tools = server.tool_router.list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "zones_forecast",
            "zones_get",
            "zones_list",
            "zones_list_of_type",
            "zones_observations",
            "zones_stations",
        ]
    );

    assert_contract::<Feature<ZoneForecast>>(
        tool(&tools, "zones_forecast"),
        "Get NOAA Zone Forecast",
        &["zoneId", "zoneType"],
        &["zoneId", "zoneType"],
    );
    assert_contract::<Feature<Zone>>(
        tool(&tools, "zones_get"),
        "Get NOAA Zone",
        &["effective", "zoneId", "zoneType"],
        &["zoneId", "zoneType"],
    );
    assert_contract::<FeatureCollection<Zone>>(
        tool(&tools, "zones_list"),
        "List NOAA Zones",
        &[
            "area",
            "effective",
            "id",
            "includeGeometry",
            "limit",
            "point",
            "region",
            "type",
        ],
        &[],
    );
    assert_contract::<FeatureCollection<Zone>>(
        tool(&tools, "zones_list_of_type"),
        "List NOAA Zones of Type",
        &[
            "area",
            "effective",
            "id",
            "includeGeometry",
            "limit",
            "point",
            "region",
            "type",
            "zoneType",
        ],
        &["zoneType"],
    );
    assert_contract::<FeatureCollection<Observation>>(
        tool(&tools, "zones_observations"),
        "Get NOAA Zone Observations",
        &["end", "limit", "start", "zoneId"],
        &["zoneId"],
    );
    let stations = tool(&tools, "zones_stations");
    assert_contract::<FeatureCollection<ObservationStation>>(
        stations,
        "Get NOAA Zone Stations",
        &["cursor", "limit", "zoneId"],
        &["zoneId"],
    );
    let description = stations
        .description
        .as_deref()
        .expect("zone stations must describe broken pagination");
    assert!(description.contains("pagination.next"));
    assert!(description.contains("/stations"));
    assert!(description.contains("increase limit"));
    let cursor_description = stations.input_schema["properties"]["cursor"]["description"]
        .as_str()
        .expect("cursor must have standalone schema documentation");
    assert!(cursor_description.contains("/stations"));
    assert!(cursor_description.contains("empty page"));
    assert!(cursor_description.contains("Increase `limit` instead"));
}

#[tokio::test]
async fn tools_preserve_every_typed_result_and_exact_http_contract() {
    let zone_expected: Feature<Zone> =
        serde_json::from_str(ZONE).expect("zone fixture must decode");
    let forecast_expected: Feature<ZoneForecast> =
        serde_json::from_str(FORECAST).expect("forecast fixture must decode");
    let zones_expected: FeatureCollection<Zone> =
        serde_json::from_str(ZONES).expect("zones fixture must decode");
    let observations_expected: FeatureCollection<Observation> =
        serde_json::from_str(OBSERVATIONS).expect("observations fixture must decode");
    let stations_expected: FeatureCollection<ObservationStation> =
        serde_json::from_str(STATIONS).expect("stations fixture must decode");
    let (upstream, server) = server_with_router(NoaaWeatherServer::zones_router()).await;

    for (expected_path, body) in [
        ("/zones/forecast/AZZ540", ZONE),
        ("/zones/public/AZZ540/forecast", FORECAST),
        ("/zones", ZONES),
        ("/zones/public", ZONES),
        ("/zones/forecast/AZZ540/observations", OBSERVATIONS),
        ("/zones/forecast/AZZ540/stations", STATIONS),
    ] {
        Mock::given(method("GET"))
            .and(path(expected_path))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/geo+json"))
            .expect(1)
            .mount(&upstream)
            .await;
    }

    let zone = call(
        &server,
        "zones_get",
        json!({
            "zoneType": "forecast",
            "zoneId": "azz540",
            "effective": "2026-08-30T00:00:00Z"
        }),
    )
    .await;
    assert_json_success(&zone, &zone_expected);

    let forecast = call(
        &server,
        "zones_forecast",
        json!({"zoneType": "public", "zoneId": "azz540"}),
    )
    .await;
    assert_json_success(&forecast, &forecast_expected);

    let zones = call(
        &server,
        "zones_list",
        json!({
            "id": ["AZZ540", "AZC013"],
            "area": ["AZ"],
            "region": ["WR"],
            "type": ["forecast", "public"],
            "point": "33.4484,-112.074",
            "includeGeometry": false,
            "limit": 10,
            "effective": "2026-08-30T00:00:00Z"
        }),
    )
    .await;
    assert_json_success(&zones, &zones_expected);

    let zones_of_type = call(
        &server,
        "zones_list_of_type",
        json!({"zoneType": "public", "type": ["fire", "county"], "limit": 1}),
    )
    .await;
    assert_json_success(&zones_of_type, &zones_expected);

    let observations = call(
        &server,
        "zones_observations",
        json!({
            "zoneId": "azz540",
            "start": "2026-08-30T00:00:00Z",
            "end": "2026-08-30T06:00:00Z",
            "limit": 12
        }),
    )
    .await;
    assert_json_success(&observations, &observations_expected);
    assert_eq!(
        observations
            .structured_content
            .as_ref()
            .and_then(|value| value.pointer("/pagination/next")),
        Some(&json!(
            "https://api.weather.gov/stations/ARAU1/observations?cursor=eyJzIjoiMjAyNi0wOS0wMlQwNzoyMDowMCswMDowMCJ9"
        ))
    );

    let stations = call(
        &server,
        "zones_stations",
        json!({"zoneId": "azz540", "limit": 15, "cursor": "abc"}),
    )
    .await;
    assert_json_success(&stations, &stations_expected);
    assert_eq!(
        stations
            .structured_content
            .as_ref()
            .and_then(|value| value.pointer("/pagination/next")),
        Some(&json!(
            "https://api.weather.gov/stations?id%5B0%5D=ARAU1&id%5B1%5D=KDPG&id%5B2%5D=KENV&id%5B3%5D=RSBU1&id%5B4%5D=UT33&id%5B5%5D=UTCUR&id%5B6%5D=UTGRS&cursor=eyJzIjo1MDB9"
        ))
    );

    upstream.verify().await;
    let requests = upstream
        .received_requests()
        .await
        .expect("recorded requests must be readable");
    assert_eq!(requests.len(), 6, "each tool must make exactly one request");
    let contracts = requests
        .iter()
        .map(|request| {
            (
                request.url.path().to_owned(),
                request.url.query().map(str::to_owned),
                request.headers["accept"]
                    .to_str()
                    .expect("Accept must be text")
                    .to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        contracts,
        [
            (
                "/zones/forecast/AZZ540".to_owned(),
                Some("effective=2026-08-30T00%3A00%3A00Z".to_owned()),
                "application/geo+json".to_owned(),
            ),
            (
                "/zones/public/AZZ540/forecast".to_owned(),
                None,
                "application/geo+json".to_owned(),
            ),
            (
                "/zones".to_owned(),
                Some(
                    "id=AZZ540%2CAZC013&area=AZ&region=WR&type=forecast%2Cpublic&point=33.4484%2C-112.074&include_geometry=false&limit=10&effective=2026-08-30T00%3A00%3A00Z"
                        .to_owned(),
                ),
                "application/geo+json".to_owned(),
            ),
            (
                "/zones/public".to_owned(),
                Some("type=fire%2Ccounty&limit=1".to_owned()),
                "application/geo+json".to_owned(),
            ),
            (
                "/zones/forecast/AZZ540/observations".to_owned(),
                Some(
                    "start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T06%3A00%3A00Z&limit=12"
                        .to_owned(),
                ),
                "application/geo+json".to_owned(),
            ),
            (
                "/zones/forecast/AZZ540/stations".to_owned(),
                Some("limit=15&cursor=abc".to_owned()),
                "application/geo+json".to_owned(),
            ),
        ]
    );
}

#[tokio::test]
async fn invalid_typed_arguments_are_rejected_before_http() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::zones_router()).await;

    let invalid_zone_id = call(
        &server,
        "zones_get",
        json!({"zoneType": "forecast", "zoneId": "XXZ040"}),
    )
    .await;
    assert_parameter_failure(&invalid_zone_id);

    let invalid_zone_type = call(
        &server,
        "zones_forecast",
        json!({"zoneType": "city", "zoneId": "AZZ540"}),
    )
    .await;
    assert_parameter_failure(&invalid_zone_type);

    let invalid_timestamp = call(
        &server,
        "zones_observations",
        json!({"zoneId": "AZZ540", "start": "tomorrow"}),
    )
    .await;
    assert_parameter_failure(&invalid_timestamp);
    no_requests(&upstream).await;
}

#[tokio::test]
async fn upstream_failure_is_one_bounded_stable_tool_error() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::zones_router()).await;
    Mock::given(method("GET"))
        .and(path("/zones"))
        .respond_with(ResponseTemplate::new(503).set_body_string("zone-upstream-secret"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "zones_list", json!({})).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    assert!(!text.contains("zone-upstream-secret"));
    let failure: Value = serde_json::from_str(text).expect("tool failure must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    upstream.verify().await;
}
