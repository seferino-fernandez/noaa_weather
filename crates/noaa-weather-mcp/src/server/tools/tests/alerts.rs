use std::collections::BTreeSet;

use noaa_weather_client::alerts::Alert;
use noaa_weather_client::geo::{Feature, FeatureCollection};
use rmcp::model::CallToolResult;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::super::test_support::{assert_json_success, assert_metadata, call, result_text, server};
use super::NoaaWeatherServer;

const ALERTS_LIST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/alerts/list.json"
));
const ALERT_SINGLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/alerts/single.json"
));

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
            .expect("WireMock requests must be readable")
            .is_empty()
    );
}

#[test]
fn router_inventory_publishes_exact_names_schemas_and_annotations() {
    let tools = NoaaWeatherServer::alerts_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        ["alerts_active", "alerts_get"]
    );

    for tool in &tools {
        assert!(
            tool.description
                .as_deref()
                .is_some_and(|description| description.len() > 20)
        );
        assert_metadata(
            tool,
            match tool.name.as_ref() {
                "alerts_active" => "Get Active NOAA Weather Alerts",
                "alerts_get" => "Get NOAA Weather Alert",
                name => panic!("unexpected alert tool {name}"),
            },
        );
    }

    let active = tools
        .iter()
        .find(|tool| tool.name == "alerts_active")
        .expect("active alerts tool must be registered");
    let properties = active.input_schema["properties"]
        .as_object()
        .expect("active input schema must expose object properties");
    let actual = properties
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = [
        "area",
        "certainty",
        "code",
        "event",
        "messageType",
        "point",
        "region",
        "regionType",
        "severity",
        "status",
        "urgency",
        "zone",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert!(active.input_schema.get("query").is_none());

    let get = tools
        .iter()
        .find(|tool| tool.name == "alerts_get")
        .expect("get alert tool must be registered");
    assert!(get.input_schema["properties"].get("alertId").is_some());
}

#[tokio::test]
async fn active_routes_flat_filters_and_returns_exactly_one_typed_page() {
    let expected: FeatureCollection<Alert> =
        serde_json::from_str(ALERTS_LIST).expect("captured alert list must decode");
    let expected_json = serde_json::to_value(&expected).expect("typed alert list must serialize");
    let expected_pagination = expected_json["pagination"].clone();
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/alerts/active"))
        .and(query_param("status", "actual,test"))
        .and(query_param("message_type", "Alert,Cancel"))
        .and(query_param("event", "Flood Watch,Wind/Warning"))
        .and(query_param("region_type", "marine"))
        .and(header("Accept", "application/geo+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(ALERTS_LIST, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "alerts_active",
        json!({
            "status": ["Actual", "Test"],
            "messageType": ["Alert", "Cancel"],
            "event": ["Flood Watch", "Wind/Warning"],
            "regionType": "marine"
        }),
    )
    .await;

    assert_json_success(&result, &expected);
    assert_eq!(
        result
            .structured_content
            .as_ref()
            .expect("success must be structured")["pagination"],
        expected_pagination,
    );
    upstream.verify().await;
    let requests = upstream
        .received_requests()
        .await
        .expect("WireMock requests must be readable");
    assert_eq!(requests.len(), 1, "the tool must not follow pagination");
    assert_eq!(
        requests[0].url.query(),
        Some(
            "status=actual%2Ctest&message_type=Alert%2CCancel&event=Flood+Watch%2CWind%2FWarning&region_type=marine"
        )
    );
}

#[tokio::test]
async fn get_routes_alert_id_and_returns_the_typed_feature() {
    let expected: Feature<Alert> =
        serde_json::from_str(ALERT_SINGLE).expect("captured alert must decode");
    let alert_id = expected.properties.id.as_str().to_owned();
    let expected_path = format!("/alerts/{alert_id}");
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path(expected_path))
        .and(header("Accept", "application/geo+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(ALERT_SINGLE, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "alerts_get", json!({"alertId": alert_id})).await;

    assert_json_success(&result, &expected);
    upstream.verify().await;
    let requests = upstream
        .received_requests()
        .await
        .expect("WireMock requests must be readable");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url.query(), None);
}

#[tokio::test]
async fn invalid_alert_id_is_rejected_by_parameter_decoding_before_http() {
    let (upstream, server) = server().await;
    let result = call(&server, "alerts_get", json!({"alertId": ""})).await;

    assert_parameter_failure(&result);
    no_requests(&upstream).await;
}

#[tokio::test]
async fn invalid_active_filter_is_rejected_by_parameter_decoding_before_http() {
    let (upstream, server) = server().await;
    let result = call(
        &server,
        "alerts_active",
        json!({"regionType": "not-a-region"}),
    )
    .await;

    assert_parameter_failure(&result);
    no_requests(&upstream).await;
}

#[tokio::test]
async fn upstream_failure_uses_the_bounded_stable_tool_error() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/alerts/active"))
        .respond_with(ResponseTemplate::new(503).set_body_string("upstream-body-secret"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "alerts_active", json!({})).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    assert!(!text.contains("upstream-body-secret"));
    let failure: Value = serde_json::from_str(text).expect("tool failure text must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    upstream.verify().await;
}
