use noaa_weather_client::geo::Feature;
use noaa_weather_client::gridpoints::Forecast;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{assert_json_success, assert_metadata, call, result_text, server};

const FORECAST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/gridpoints/forecast.json"
));
const HOURLY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/gridpoints/hourly.json"
));
const FORECAST_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

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
