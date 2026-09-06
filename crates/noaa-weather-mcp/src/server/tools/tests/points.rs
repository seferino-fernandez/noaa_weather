use noaa_weather_client::geo::Feature;
use noaa_weather_client::gridpoints::Forecast;
use noaa_weather_client::points::Point;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{assert_json_success, assert_metadata, call, result_text, server};

const POINT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/points/point.json"
));
const FORECAST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/gridpoints/forecast.json"
));
const FORECAST_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

#[test]
fn router_lists_the_two_typed_point_tools() {
    let tools = NoaaWeatherServer::points_router().list_all();
    let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
    assert_eq!(names, ["points_forecast", "points_get"]);

    for tool in &tools {
        assert_metadata(
            tool,
            match tool.name.as_ref() {
                "points_get" => "Get NOAA Point Metadata",
                "points_forecast" => "Get NOAA Forecast for Point",
                name => panic!("unexpected point tool {name}"),
            },
        );
        let properties = tool.input_schema["properties"]
            .as_object()
            .expect("input schema properties must be an object");
        assert!(properties.contains_key("point"));
        assert_eq!(properties.len(), 1);
        assert_eq!(tool.input_schema["required"], json!(["point"]));
    }
}

#[tokio::test]
async fn points_get_returns_the_typed_fixture_as_authoritative_json() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/points/39.7456,-97.0892"))
        .and(header("Accept", "application/geo+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(POINT, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "points_get",
        json!({ "point": "39.7456,-97.0892" }),
    )
    .await;
    let expected: Feature<Point> = serde_json::from_str(POINT).expect("fixture must decode");
    assert_json_success(&result, &expected);

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url.query(), None);
    assert!(!requests[0].headers.contains_key("feature-flags"));
}

#[tokio::test]
async fn points_forecast_preserves_the_two_request_order_and_headers() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/points/39.7456,-97.0892"))
        .and(header("Accept", "application/geo+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(POINT, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;
    Mock::given(method("GET"))
        .and(path("/gridpoints/TOP/32,81/forecast"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(FORECAST, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "points_forecast",
        json!({ "point": "39.7456,-97.0892" }),
    )
    .await;
    let expected: Feature<Forecast> = serde_json::from_str(FORECAST).expect("fixture must decode");
    assert_json_success(&result, &expected);

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].url.path(), "/points/39.7456,-97.0892");
    assert_eq!(requests[0].url.query(), None);
    assert_eq!(requests[0].headers["accept"], "application/geo+json");
    assert!(!requests[0].headers.contains_key("feature-flags"));
    assert_eq!(requests[1].url.path(), "/gridpoints/TOP/32,81/forecast");
    assert_eq!(requests[1].url.query(), None);
    assert_eq!(requests[1].headers["accept"], "application/geo+json");
    assert_eq!(requests[1].headers["feature-flags"], FORECAST_FLAGS);
}

#[tokio::test]
async fn invalid_coordinates_fail_in_rmcp_before_any_request() {
    let (upstream, server) = server().await;
    let result = call(&server, "points_get", json!({ "point": "91,0" })).await;

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
async fn upstream_failure_is_projected_as_json_tool_error() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/points/39.7456,-97.0892"))
        .respond_with(ResponseTemplate::new(503).set_body_string("upstream body is private"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "points_get",
        json!({ "point": "39.7456,-97.0892" }),
    )
    .await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    let error: Value = serde_json::from_str(text).expect("failure text must be JSON");
    assert_eq!(error["code"], "upstream_http_error");
    assert_eq!(error["status"], 503);
    assert!(!text.contains("upstream body is private"));
}
