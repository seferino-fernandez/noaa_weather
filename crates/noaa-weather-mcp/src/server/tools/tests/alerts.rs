use std::collections::BTreeSet;

use noaa_weather_client::alerts::{ActiveAlertCounts, Alert, AlertEventTypes};
use noaa_weather_client::geo::{Feature, FeatureCollection};
use rmcp::handler::server::tool::schema_for_output;
use rmcp::model::CallToolResult;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::super::test_support::{
    assert_json_success, assert_metadata, call, result_text, server, server_with_router,
};
use super::NoaaWeatherServer;

const ALERTS_LIST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/alerts/list.json"
));
const ALERT_SINGLE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/alerts/single.json"
));
const ALERTS_COUNT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/alerts/count.json"
));
const ALERT_TYPES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/alerts/types.json"
));

fn remaining_tool<'a>(tools: &'a [rmcp::model::Tool], name: &str) -> &'a rmcp::model::Tool {
    tools
        .iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("missing {name} tool"))
}

fn schema_names<'a>(tool: &'a rmcp::model::Tool, member: &str) -> BTreeSet<&'a str> {
    tool.input_schema
        .get(member)
        .and_then(Value::as_object)
        .map(|members| members.keys().map(String::as_str).collect())
        .or_else(|| {
            tool.input_schema
                .get(member)
                .and_then(Value::as_array)
                .map(|members| {
                    members
                        .iter()
                        .map(|name| name.as_str().expect("schema name must be text"))
                        .collect()
                })
        })
        .unwrap_or_default()
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

#[test]
fn remaining_router_publishes_exact_flat_typed_contracts() {
    let tools = NoaaWeatherServer::alerts_remaining_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "alerts_active_count",
            "alerts_active_for_area",
            "alerts_active_for_marine_region",
            "alerts_active_for_zone",
            "alerts_search",
            "alerts_types",
        ]
    );

    let contracts = [
        (
            "alerts_active_count",
            "Count Active NOAA Weather Alerts",
            "Return counts of currently active NOAA weather alerts by land, marine region, area, and zone.",
            BTreeSet::new(),
            BTreeSet::new(),
            schema_for_output::<ActiveAlertCounts>(),
        ),
        (
            "alerts_active_for_area",
            "Get Active NOAA Alerts for Area",
            "Return active NOAA weather alerts for one state, territory, or marine area code.",
            BTreeSet::from(["area"]),
            BTreeSet::from(["area"]),
            schema_for_output::<FeatureCollection<Alert>>(),
        ),
        (
            "alerts_active_for_marine_region",
            "Get Active NOAA Alerts for Marine Region",
            "Return active NOAA weather alerts for one NOAA marine region code.",
            BTreeSet::from(["region"]),
            BTreeSet::from(["region"]),
            schema_for_output::<FeatureCollection<Alert>>(),
        ),
        (
            "alerts_active_for_zone",
            "Get Active NOAA Alerts for Zone",
            "Return active NOAA weather alerts for one NOAA public zone or county identifier.",
            BTreeSet::from(["zoneId"]),
            BTreeSet::from(["zoneId"]),
            schema_for_output::<FeatureCollection<Alert>>(),
        ),
        (
            "alerts_search",
            "Search NOAA Weather Alerts",
            "Return one NOAA page of weather alerts, including past alerts, matching optional time, status, message, location, severity, and pagination filters.",
            BTreeSet::from([
                "area",
                "certainty",
                "code",
                "cursor",
                "end",
                "event",
                "limit",
                "messageType",
                "point",
                "region",
                "regionType",
                "severity",
                "start",
                "status",
                "urgency",
                "zone",
            ]),
            BTreeSet::new(),
            schema_for_output::<FeatureCollection<Alert>>(),
        ),
        (
            "alerts_types",
            "List NOAA Weather Alert Types",
            "Return the weather-alert event types recognized by NOAA's alert system.",
            BTreeSet::new(),
            BTreeSet::new(),
            schema_for_output::<AlertEventTypes>(),
        ),
    ];

    for (name, title, description, properties, required, output) in contracts {
        let tool = remaining_tool(&tools, name);
        assert_eq!(tool.description.as_deref(), Some(description), "{name}");
        assert_metadata(tool, title);
        assert_eq!(schema_names(tool, "properties"), properties, "{name}");
        assert_eq!(schema_names(tool, "required"), required, "{name}");
        assert_eq!(tool.output_schema.as_ref(), Some(&output), "{name}");
    }
}

#[tokio::test]
async fn remaining_tools_preserve_routes_media_one_page_and_typed_json() {
    let expected_alerts: FeatureCollection<Alert> =
        serde_json::from_str(ALERTS_LIST).expect("captured alert list must decode");
    let expected_count: ActiveAlertCounts =
        serde_json::from_str(ALERTS_COUNT).expect("captured alert counts must decode");
    let expected_types: AlertEventTypes =
        serde_json::from_str(ALERT_TYPES).expect("captured alert types must decode");
    let expected_pagination = serde_json::to_value(&expected_alerts)
        .expect("typed alert list must serialize")["pagination"]
        .clone();
    let (upstream, server) = server_with_router(NoaaWeatherServer::alerts_remaining_router()).await;

    for route in [
        "/alerts/active/area/CA",
        "/alerts/active/region/GM",
        "/alerts/active/zone/CAZ043",
    ] {
        Mock::given(method("GET"))
            .and(path(route))
            .and(header("Accept", "application/geo+json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(ALERTS_LIST, "application/geo+json"),
            )
            .expect(1)
            .mount(&upstream)
            .await;
    }
    Mock::given(method("GET"))
        .and(path("/alerts/active/count"))
        .and(header("Accept", "application/ld+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(ALERTS_COUNT, "application/ld+json"))
        .expect(1)
        .mount(&upstream)
        .await;
    Mock::given(method("GET"))
        .and(path("/alerts"))
        .and(header("Accept", "application/geo+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(ALERTS_LIST, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;
    Mock::given(method("GET"))
        .and(path("/alerts/types"))
        .and(header("Accept", "application/ld+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(ALERT_TYPES, "application/ld+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let area = call(&server, "alerts_active_for_area", json!({ "area": "CA" })).await;
    let region = call(
        &server,
        "alerts_active_for_marine_region",
        json!({ "region": "GM" }),
    )
    .await;
    let zone = call(
        &server,
        "alerts_active_for_zone",
        json!({ "zoneId": "caz043" }),
    )
    .await;
    let count = call(&server, "alerts_active_count", json!({})).await;
    let search = call(
        &server,
        "alerts_search",
        json!({
            "start": "2026-08-30T00:00:00.123456789Z",
            "end": "2026-08-30T06:30:00-05:00",
            "event": ["Tornado Warning"],
            "limit": 25,
            "cursor": "bmV4dA=="
        }),
    )
    .await;
    let types = call(&server, "alerts_types", json!({})).await;

    for result in [&area, &region, &zone, &search] {
        assert_json_success(result, &expected_alerts);
        assert_eq!(
            result
                .structured_content
                .as_ref()
                .expect("collection must be structured")["pagination"],
            expected_pagination,
        );
    }
    assert_json_success(&count, &expected_count);
    assert_json_success(&types, &expected_types);

    upstream.verify().await;
    let requests = upstream
        .received_requests()
        .await
        .expect("WireMock requests must be readable");
    assert_eq!(requests.len(), 6, "every collection must remain one page");
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
                "/alerts/active/area/CA".to_owned(),
                None,
                "application/geo+json".to_owned(),
            ),
            (
                "/alerts/active/region/GM".to_owned(),
                None,
                "application/geo+json".to_owned(),
            ),
            (
                "/alerts/active/zone/CAZ043".to_owned(),
                None,
                "application/geo+json".to_owned(),
            ),
            (
                "/alerts/active/count".to_owned(),
                None,
                "application/ld+json".to_owned(),
            ),
            (
                "/alerts".to_owned(),
                Some(
                    "start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T11%3A30%3A00Z&event=Tornado+Warning&limit=25&cursor=bmV4dA%3D%3D"
                        .to_owned(),
                ),
                "application/geo+json".to_owned(),
            ),
            (
                "/alerts/types".to_owned(),
                None,
                "application/ld+json".to_owned(),
            ),
        ]
    );
}

#[tokio::test]
async fn remaining_router_rejects_invalid_marine_region_before_http() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::alerts_remaining_router()).await;
    let result = call(
        &server,
        "alerts_active_for_marine_region",
        json!({ "region": "XX" }),
    )
    .await;

    assert_parameter_failure(&result);
    no_requests(&upstream).await;
}

#[tokio::test]
async fn remaining_router_projects_one_bounded_upstream_error() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::alerts_remaining_router()).await;
    Mock::given(method("GET"))
        .and(path("/alerts/active/count"))
        .respond_with(ResponseTemplate::new(503).set_body_string("remaining-alert-secret"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "alerts_active_count", json!({})).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    assert!(!text.contains("remaining-alert-secret"));
    let failure: Value = serde_json::from_str(text).expect("tool failure text must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    upstream.verify().await;
}
