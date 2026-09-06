use std::collections::BTreeSet;

use noaa_weather_client::aviation::{CenterWeatherAdvisory, CwsuOffice, Sigmet};
use noaa_weather_client::geo::{Feature, FeatureCollection};
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

const CWSU: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/aviation/cwsu.json"
));
const CWAS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/aviation/cwas.json"
));
const CWA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/aviation/cwa.json"
));
const SIGMETS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/aviation/sigmets.json"
));
const SIGMET: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/aviation/sigmet.json"
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
fn router_publishes_exact_flat_typed_aviation_contracts() {
    let tools = NoaaWeatherServer::aviation_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "aviation_cwa_get",
            "aviation_cwas",
            "aviation_cwsu_get",
            "aviation_sigmet_get",
            "aviation_sigmets",
            "aviation_sigmets_for_atsu",
            "aviation_sigmets_for_atsu_on_date",
        ]
    );

    assert_contract::<CwsuOffice>(
        &tools,
        "aviation_cwsu_get",
        &["cwsuId"],
        &["cwsuId"],
        "Get Center Weather Service Unit",
    );
    assert_contract::<FeatureCollection<CenterWeatherAdvisory>>(
        &tools,
        "aviation_cwas",
        &["cwsuId"],
        &["cwsuId"],
        "List Center Weather Advisories",
    );
    assert_contract::<Feature<CenterWeatherAdvisory>>(
        &tools,
        "aviation_cwa_get",
        &["cwsuId", "date", "sequence"],
        &["cwsuId", "date", "sequence"],
        "Get Center Weather Advisory",
    );
    assert_contract::<FeatureCollection<Sigmet>>(
        &tools,
        "aviation_sigmets",
        &["start", "end", "date", "atsu", "sequence"],
        &[],
        "Search SIGMETs and AIRMETs",
    );
    assert_contract::<FeatureCollection<Sigmet>>(
        &tools,
        "aviation_sigmets_for_atsu",
        &["atsuId"],
        &["atsuId"],
        "List SIGMETs for ATSU",
    );
    assert_contract::<FeatureCollection<Sigmet>>(
        &tools,
        "aviation_sigmets_for_atsu_on_date",
        &["atsuId", "date"],
        &["atsuId", "date"],
        "List SIGMETs for ATSU on Date",
    );
    assert_contract::<Feature<Sigmet>>(
        &tools,
        "aviation_sigmet_get",
        &["atsuId", "issued"],
        &["atsuId", "issued"],
        "Get SIGMET or AIRMET",
    );

    for name in ["aviation_cwa_get", "aviation_sigmets_for_atsu_on_date"] {
        let date = &tool(&tools, name).input_schema["properties"]["date"];
        assert_eq!(date["type"], "string");
        assert_eq!(date["format"], "date");
    }
    let sequence = &tool(&tools, "aviation_cwa_get").input_schema["properties"]["sequence"];
    assert_eq!(sequence["type"], "integer");
    assert_eq!(sequence["minimum"], 100);
}

#[tokio::test]
async fn every_aviation_tool_preserves_route_media_and_typed_json_parity() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::aviation_router()).await;
    mount(
        &upstream,
        "/aviation/cwsus/ZAB",
        CWSU,
        "application/ld+json",
    )
    .await;
    mount(
        &upstream,
        "/aviation/cwsus/ZAB/cwas",
        CWAS,
        "application/geo+json",
    )
    .await;
    mount(
        &upstream,
        "/aviation/cwsus/ZAB/cwas/2026-09-02/100",
        CWA,
        "application/geo+json",
    )
    .await;
    mount(
        &upstream,
        "/aviation/sigmets",
        SIGMETS,
        "application/geo+json",
    )
    .await;
    mount(
        &upstream,
        "/aviation/sigmets/KKCI",
        SIGMETS,
        "application/geo+json",
    )
    .await;
    mount(
        &upstream,
        "/aviation/sigmets/KKCI/2026-09-02",
        SIGMETS,
        "application/geo+json",
    )
    .await;
    mount(
        &upstream,
        "/aviation/sigmets/KKCI/2026-09-02/0655",
        SIGMET,
        "application/geo+json",
    )
    .await;

    let result = call(&server, "aviation_cwsu_get", json!({ "cwsuId": "zab" })).await;
    assert_json_success(
        &result,
        &serde_json::from_str::<CwsuOffice>(CWSU).expect("fixture must decode"),
    );

    let result = call(&server, "aviation_cwas", json!({ "cwsuId": "zab" })).await;
    assert_json_success(
        &result,
        &serde_json::from_str::<FeatureCollection<CenterWeatherAdvisory>>(CWAS)
            .expect("fixture must decode"),
    );

    let result = call(
        &server,
        "aviation_cwa_get",
        json!({ "cwsuId": "zab", "date": "2026-09-02", "sequence": 100 }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<Feature<CenterWeatherAdvisory>>(CWA).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "aviation_sigmets",
        json!({
            "start": "2026-09-02T00:00:00Z",
            "end": "2026-09-02T12:00:00Z",
            "date": "2026-09-02",
            "atsu": "kkci",
            "sequence": "19W"
        }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<FeatureCollection<Sigmet>>(SIGMETS).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "aviation_sigmets_for_atsu",
        json!({ "atsuId": "kkci" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<FeatureCollection<Sigmet>>(SIGMETS).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "aviation_sigmets_for_atsu_on_date",
        json!({ "atsuId": "kkci", "date": "2026-09-02" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<FeatureCollection<Sigmet>>(SIGMETS).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "aviation_sigmet_get",
        json!({ "atsuId": "kkci", "issued": "2026-09-01T23:55:59-07:00" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<Feature<Sigmet>>(SIGMET).expect("fixture must decode"),
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
            "/aviation/cwsus/ZAB",
            "/aviation/cwsus/ZAB/cwas",
            "/aviation/cwsus/ZAB/cwas/2026-09-02/100",
            "/aviation/sigmets",
            "/aviation/sigmets/KKCI",
            "/aviation/sigmets/KKCI/2026-09-02",
            "/aviation/sigmets/KKCI/2026-09-02/0655",
        ]
    );
    assert_eq!(
        requests[3].url.query(),
        Some(
            "start=2026-09-02T00%3A00%3A00Z&end=2026-09-02T12%3A00%3A00Z&date=2026-09-02\
             &atsu=KKCI&sequence=19W"
        )
    );
    assert!(
        requests[..3]
            .iter()
            .chain(&requests[4..])
            .all(|request| request.url.query().is_none())
    );
}

#[tokio::test]
async fn cwa_sequence_below_one_hundred_is_rejected_before_http() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::aviation_router()).await;
    let result = call(
        &server,
        "aviation_cwa_get",
        json!({ "cwsuId": "ZAB", "date": "2026-09-02", "sequence": 99 }),
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
async fn aviation_upstream_failure_is_bounded_json_without_body_leakage() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::aviation_router()).await;
    Mock::given(method("GET"))
        .and(path("/aviation/cwsus/ZAB"))
        .respond_with(ResponseTemplate::new(503).set_body_string("private aviation body"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "aviation_cwsu_get", json!({ "cwsuId": "ZAB" })).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    let failure: Value = serde_json::from_str(text).expect("failure must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    assert!(!text.contains("private aviation body"));
}
