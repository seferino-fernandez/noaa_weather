use std::collections::BTreeSet;

use noaa_weather_client::geo::{Feature, FeatureCollection};
use noaa_weather_client::gridpoints::{Forecast, Gridpoint};
use noaa_weather_client::stations::ObservationStation;
use rmcp::handler::server::tool::schema_for_output;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{
    assert_json_success, assert_metadata, call, result_text, server, server_with_router,
};

const FORECAST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/gridpoints/forecast.json"
));
const HOURLY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/gridpoints/hourly.json"
));
const GRIDPOINT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/gridpoints/gridpoint.json"
));
const STATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/gridpoints/stations.json"
));
const FORECAST_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

fn names<'a>(tool: &'a rmcp::model::Tool, member: &str) -> BTreeSet<&'a str> {
    tool.input_schema
        .get(member)
        .and_then(|value| match value {
            Value::Object(members) => Some(members.keys().map(String::as_str).collect()),
            Value::Array(members) => Some(
                members
                    .iter()
                    .map(|name| name.as_str().expect("schema name must be text"))
                    .collect(),
            ),
            _ => None,
        })
        .unwrap_or_default()
}

#[test]
fn router_lists_two_typed_tools_with_flat_query_schemas() {
    let tools = NoaaWeatherServer::gridpoints_router().list_all();
    let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
    assert_eq!(names, ["gridpoints_forecast", "gridpoints_forecast_hourly"]);

    for tool in &tools {
        assert_metadata(
            tool,
            match tool.name.as_ref() {
                "gridpoints_forecast" => "Get NOAA Gridpoint Forecast",
                "gridpoints_forecast_hourly" => "Get NOAA Hourly Gridpoint Forecast",
                name => panic!("unexpected gridpoint tool {name}"),
            },
        );
        let properties = tool.input_schema["properties"]
            .as_object()
            .expect("input schema properties must be an object");
        assert!(properties.contains_key("gridpointId"));
        assert!(properties.contains_key("units"));
        assert!(!properties.contains_key("query"));
        assert!(!properties.contains_key("gridpoint_id"));
        assert_eq!(properties.len(), 2);
        assert_eq!(tool.input_schema["required"], json!(["gridpointId"]));
        assert!(tool.output_schema.is_some());
    }
}

#[tokio::test]
async fn gridpoints_forecast_preserves_flat_units_path_media_and_flags() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/gridpoints/TOP/31,80/forecast"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(FORECAST, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "gridpoints_forecast",
        json!({
            "gridpointId": "TOP/31,80",
            "units": "si"
        }),
    )
    .await;
    let expected: Feature<Forecast> = serde_json::from_str(FORECAST).expect("fixture must decode");
    assert_json_success(&result, &expected);

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url.path(), "/gridpoints/TOP/31,80/forecast");
    assert_eq!(requests[0].url.query(), Some("units=si"));
    assert_eq!(requests[0].headers["accept"], "application/geo+json");
    assert_eq!(requests[0].headers["feature-flags"], FORECAST_FLAGS);
}

#[tokio::test]
async fn gridpoints_hourly_omits_absent_units_and_returns_typed_json() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/gridpoints/TOP/31,80/forecast/hourly"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(HOURLY, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "gridpoints_forecast_hourly",
        json!({ "gridpointId": "TOP/31,80" }),
    )
    .await;
    let expected: Feature<Forecast> = serde_json::from_str(HOURLY).expect("fixture must decode");
    assert_json_success(&result, &expected);

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests[0].url.path(),
        "/gridpoints/TOP/31,80/forecast/hourly"
    );
    assert_eq!(requests[0].url.query(), None);
    assert_eq!(requests[0].headers["accept"], "application/geo+json");
    assert_eq!(requests[0].headers["feature-flags"], FORECAST_FLAGS);
}

#[tokio::test]
async fn invalid_gridpoint_id_fails_in_rmcp_before_any_request() {
    let (upstream, server) = server().await;
    let result = call(
        &server,
        "gridpoints_forecast",
        json!({ "gridpointId": "TOP/-1,80" }),
    )
    .await;

    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert!(result_text(&result).contains("failed to deserialize parameters"));
    assert!(
        upstream
            .received_requests()
            .await
            .expect("requests must be readable")
            .is_empty()
    );
}

#[test]
fn remaining_router_lists_exact_typed_tools_without_cursor() {
    let tools = NoaaWeatherServer::gridpoints_remaining_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        ["gridpoints_get", "gridpoints_stations"]
    );

    let get = tools
        .iter()
        .find(|tool| tool.name == "gridpoints_get")
        .expect("gridpoints_get must be registered");
    assert_eq!(
        get.description.as_deref(),
        Some("Return NOAA's raw numerical forecast layers for one forecast-office grid cell.")
    );
    assert_metadata(get, "Get NOAA Gridpoint Data");
    assert_eq!(names(get, "properties"), BTreeSet::from(["gridpointId"]));
    assert_eq!(names(get, "required"), BTreeSet::from(["gridpointId"]));
    assert_eq!(
        get.output_schema.as_ref(),
        Some(&schema_for_output::<Feature<Gridpoint>>())
    );

    let stations = tools
        .iter()
        .find(|tool| tool.name == "gridpoints_stations")
        .expect("gridpoints_stations must be registered");
    assert_eq!(
        stations.description.as_deref(),
        Some(
            "Return one NOAA response containing observation stations usable for a forecast-office grid cell."
        )
    );
    assert_metadata(stations, "Get NOAA Gridpoint Observation Stations");
    assert_eq!(
        names(stations, "properties"),
        BTreeSet::from(["gridpointId", "limit"])
    );
    assert_eq!(names(stations, "required"), BTreeSet::from(["gridpointId"]));
    assert!(
        !stations.input_schema["properties"]
            .as_object()
            .expect("properties must be an object")
            .contains_key("cursor")
    );
    assert_eq!(
        stations.output_schema.as_ref(),
        Some(&schema_for_output::<FeatureCollection<ObservationStation>>())
    );
}

#[tokio::test]
async fn remaining_tools_preserve_routes_media_one_response_and_typed_json() {
    let expected_gridpoint: Feature<Gridpoint> =
        serde_json::from_str(GRIDPOINT).expect("captured gridpoint must decode");
    let expected_stations: FeatureCollection<ObservationStation> =
        serde_json::from_str(STATIONS).expect("captured station collection must decode");
    let expected_pagination = serde_json::to_value(&expected_stations)
        .expect("typed stations must serialize")["pagination"]
        .clone();
    let (upstream, server) =
        server_with_router(NoaaWeatherServer::gridpoints_remaining_router()).await;
    Mock::given(method("GET"))
        .and(path("/gridpoints/TOP/31,80"))
        .and(header("Accept", "application/geo+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(GRIDPOINT, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;
    Mock::given(method("GET"))
        .and(path("/gridpoints/TOP/31,80/stations"))
        .and(header("Accept", "application/geo+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(STATIONS, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let gridpoint = call(
        &server,
        "gridpoints_get",
        json!({ "gridpointId": "TOP/31,80" }),
    )
    .await;
    let stations = call(
        &server,
        "gridpoints_stations",
        json!({ "gridpointId": "TOP/31,80", "limit": 25 }),
    )
    .await;

    assert_json_success(&gridpoint, &expected_gridpoint);
    assert_json_success(&stations, &expected_stations);
    assert_eq!(
        stations
            .structured_content
            .as_ref()
            .expect("collection must be structured")["pagination"],
        expected_pagination,
    );
    upstream.verify().await;
    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(
        requests.len(),
        2,
        "the known-bad next link must not be followed"
    );
    assert_eq!(requests[0].url.path(), "/gridpoints/TOP/31,80");
    assert_eq!(requests[0].url.query(), None);
    assert_eq!(requests[0].headers["accept"], "application/geo+json");
    assert!(!requests[0].headers.contains_key("feature-flags"));
    assert_eq!(requests[1].url.path(), "/gridpoints/TOP/31,80/stations");
    assert_eq!(requests[1].url.query(), Some("limit=25"));
    assert_eq!(requests[1].headers["accept"], "application/geo+json");
    assert!(!requests[1].headers.contains_key("feature-flags"));
}

#[tokio::test]
async fn remaining_router_rejects_invalid_gridpoint_before_http() {
    let (upstream, server) =
        server_with_router(NoaaWeatherServer::gridpoints_remaining_router()).await;
    let result = call(
        &server,
        "gridpoints_get",
        json!({ "gridpointId": "TOP/-1,80" }),
    )
    .await;

    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert!(result_text(&result).contains("failed to deserialize parameters"));
    assert!(
        upstream
            .received_requests()
            .await
            .expect("requests must be readable")
            .is_empty()
    );
}

#[tokio::test]
async fn remaining_router_projects_one_bounded_upstream_error() {
    let (upstream, server) =
        server_with_router(NoaaWeatherServer::gridpoints_remaining_router()).await;
    Mock::given(method("GET"))
        .and(path("/gridpoints/TOP/31,80"))
        .respond_with(ResponseTemplate::new(503).set_body_string("remaining-gridpoint-secret"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "gridpoints_get",
        json!({ "gridpointId": "TOP/31,80" }),
    )
    .await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    assert!(!text.contains("remaining-gridpoint-secret"));
    let failure: Value = serde_json::from_str(text).expect("tool failure text must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    upstream.verify().await;
}
