use noaa_weather_client::glossary::GlossaryResponse;
use rmcp::handler::server::tool::schema_for_output;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{
    assert_json_success, assert_metadata, call, result_text, server_with_router,
};

const TERMS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/glossary/terms.json"
));

#[test]
fn router_publishes_the_exact_typed_glossary_contract() {
    let tools = NoaaWeatherServer::glossary_router().list_all();
    assert_eq!(tools.len(), 1);
    let tool = &tools[0];
    assert_eq!(tool.name, "glossary_terms");
    assert_eq!(
        tool.input_schema["properties"],
        json!({}),
        "no-argument tool must still expose an object schema"
    );
    assert!(
        tool.input_schema
            .get("required")
            .is_none_or(|required| required == &json!([]))
    );
    assert_eq!(
        tool.output_schema.as_ref(),
        Some(&schema_for_output::<GlossaryResponse>())
    );
    assert_metadata(tool, "List NWS Glossary Terms");
}

#[tokio::test]
async fn glossary_terms_preserves_route_media_and_typed_json_parity() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::glossary_router()).await;
    Mock::given(method("GET"))
        .and(path("/glossary"))
        .and(header("Accept", "application/ld+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(TERMS, "application/ld+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "glossary_terms", json!({})).await;
    let expected: GlossaryResponse = serde_json::from_str(TERMS).expect("fixture must decode");
    assert_json_success(&result, &expected);

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url.path(), "/glossary");
    assert_eq!(requests[0].url.query(), None);
}

#[tokio::test]
async fn glossary_upstream_failure_is_bounded_json_without_body_leakage() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::glossary_router()).await;
    Mock::given(method("GET"))
        .and(path("/glossary"))
        .respond_with(ResponseTemplate::new(503).set_body_string("private glossary response body"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "glossary_terms", json!({})).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    let failure: Value = serde_json::from_str(text).expect("failure must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    assert!(!text.contains("private glossary response body"));
}
