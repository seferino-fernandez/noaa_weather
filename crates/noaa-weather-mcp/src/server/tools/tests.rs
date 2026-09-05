use std::num::NonZeroUsize;

use noaa_weather_client::stations::TerminalAerodromeForecast;
use noaa_weather_client::{
    CallSign, Client, Error as ClientError, InvalidValue, RetryPolicy, ValueKind,
};
use rmcp::ServerHandler as _;
use rmcp::handler::server::tool::IntoCallToolResult as _;
use rmcp::model::{CallToolResponse, CallToolResult, ContentBlock};
use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use super::super::result_limit;
use super::test_support::{projected_failure, server, server_with_limit};

fn limit(bytes: usize) -> NonZeroUsize {
    NonZeroUsize::new(bytes).expect("test limit must be nonzero")
}

fn successful(value: Value) -> CallToolResponse {
    let text = serde_json::to_string(&value).expect("JSON value must serialize");
    let mut result = CallToolResult::success(vec![ContentBlock::text(text)]);
    result.structured_content = Some(value);
    result.into()
}

fn complete(response: &CallToolResponse) -> &CallToolResult {
    let CallToolResponse::Complete(result) = response else {
        panic!("expected a complete tool response");
    };
    result
}

fn text(result: &CallToolResult) -> &str {
    result
        .content
        .first()
        .and_then(ContentBlock::as_text)
        .map(|content| content.text.as_str())
        .expect("tool result must contain text")
}

#[test]
fn server_advertises_tools_capability_with_an_empty_foundation_inventory() {
    let server = super::super::NoaaWeatherServer::new(limit(1_048_576))
        .expect("the built-in NOAA client configuration must be valid");
    let info = server.get_info();

    assert_eq!(info.server_info.name, "noaa_weather_mcp");
    assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION"));
    assert!(info.capabilities.tools.is_some());
    assert!(server.tool_router.list_all().is_empty());
    assert!(
        info.instructions
            .as_deref()
            .is_some_and(|instructions| instructions.contains("structured JSON"))
    );
}

#[tokio::test]
async fn test_support_injects_a_real_client_with_retries_disabled() {
    let (upstream, server) = server().await;
    assert_eq!(
        server.client.base_url().as_str(),
        format!("{}/", upstream.uri())
    );
}

#[test]
fn structured_result_below_the_limit_passes_unchanged() {
    let response = successful(json!({ "forecast": "clear" }));
    let expected = response.clone();
    let actual = result_limit::apply(response, limit(1_024));
    assert_eq!(complete(&actual), complete(&expected));
}

#[test]
fn structured_result_exactly_at_the_limit_passes_unchanged() {
    let value = json!({ "forecast": "clear" });
    let measured = serde_json::to_vec(&value)
        .expect("JSON value must serialize")
        .len();
    let response = successful(value);
    let expected = response.clone();
    let actual = result_limit::apply(response, limit(measured));
    assert_eq!(complete(&actual), complete(&expected));
}

#[test]
fn structured_result_over_the_limit_becomes_a_bounded_tool_error() {
    let value = json!({ "forecast": "clear" });
    let measured = serde_json::to_vec(&value)
        .expect("JSON value must serialize")
        .len();
    let actual = result_limit::apply(successful(value), limit(measured - 1));
    let result = complete(&actual);
    let error: Value = serde_json::from_str(text(result)).expect("limit error must be JSON");

    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(error["code"], "response_too_large");
    assert_eq!(error["bytes"], measured);
    assert_eq!(error["limit"], measured - 1);
    assert!(
        error["hint"]
            .as_str()
            .is_some_and(|hint| hint.contains("page size"))
    );
}

#[test]
fn structured_result_limit_counts_multibyte_utf8_bytes() {
    let value = json!({ "weather": "☃" });
    let encoded = serde_json::to_vec(&value).expect("JSON value must serialize");
    assert!(encoded.len() > value.to_string().chars().count());

    let actual = result_limit::apply(successful(value), limit(encoded.len() - 1));
    let error: Value = serde_json::from_str(text(complete(&actual))).expect("error must be JSON");
    assert_eq!(error["bytes"], encoded.len());
}

#[test]
fn existing_tool_errors_bypass_the_success_limiter() {
    let mut existing = CallToolResult::error(vec![ContentBlock::text("already failed")]);
    existing.structured_content = Some(json!({ "large": "xxxxxxxx" }));
    let response: CallToolResponse = existing.clone().into();
    let actual = result_limit::apply(response, limit(1));
    assert_eq!(complete(&actual), &existing);
}

#[test]
fn invalid_input_projection_preserves_bounded_structured_details() {
    let input = format!("{}☃", "x".repeat(700));
    let invalid = InvalidValue::new(ValueKind::StationId, input, "must be a station identifier");
    let (encoded, failure) = projected_failure(ClientError::from(invalid));

    assert_eq!(failure["code"], "invalid_input");
    assert_eq!(failure["retryable"], false);
    assert_eq!(failure["attempts"], 1);
    assert_eq!(failure["details"]["kind"], "station id");
    assert!(failure["details"]["input"].as_str().unwrap().len() <= 512);
    assert_eq!(failure["details"]["reason"], "must be a station identifier");
    assert!(encoded.len() <= 4_096);
}

#[test]
fn tool_failure_converts_through_rmcp_to_one_json_text_error() {
    let invalid = InvalidValue::new(ValueKind::StationId, "bad!", "must be a station identifier");
    let failure = super::error::ToolFailure::from(ClientError::from(invalid));
    let response = Result::<(), super::error::ToolFailure>::Err(failure)
        .into_call_tool_result()
        .expect("a tool failure must remain caller-visible");
    let result = complete(&response);

    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let parsed: Value =
        serde_json::from_str(text(result)).expect("tool failure text must be valid JSON");
    assert_eq!(parsed["code"], "invalid_input");
}

#[test]
fn json_and_taf_projections_keep_truthful_safe_subtypes() {
    let json_error = serde_json::from_str::<Value>("{").expect_err("fixture must be invalid");
    let (_, json_failure) = projected_failure(ClientError::from(json_error));
    assert_eq!(json_failure["code"], "invalid_json_response");
    assert_eq!(json_failure["details"]["category"], "eof");
    assert_eq!(
        json_failure["message"],
        "NOAA returned a response that could not be decoded as the endpoint's expected JSON shape."
    );

    let data_error = serde_json::from_str::<Vec<String>>(r#"{"forecast":"clear"}"#)
        .expect_err("valid JSON with the wrong shape must fail");
    let (_, data_failure) = projected_failure(ClientError::from(data_error));
    assert_eq!(data_failure["details"]["category"], "data");
    assert_eq!(data_failure["message"], json_failure["message"]);

    let taf_error = TerminalAerodromeForecast::from_iwxxm(b"<broken")
        .expect_err("fixture must be invalid IWXXM");
    let (_, taf_failure) = projected_failure(ClientError::from(taf_error));
    assert_eq!(taf_failure["code"], "invalid_taf_response");
    assert_eq!(taf_failure["details"]["kind"], "malformed_xml");
    assert_eq!(taf_failure["details"]["path"], "TAF");
}

#[tokio::test]
async fn response_projection_preserves_problem_metadata_retry_delay_and_bounds() {
    let (upstream, server) = server_with_limit(limit(1_048_576)).await;
    Mock::given(method("GET"))
        .and(path("/glossary"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "2")
                .insert_header("X-Correlation-Id", "header-correlation")
                .insert_header("X-Request-Id", "request-42")
                .set_body_json(json!({
                    "type": "https://api.weather.gov/problems/rate-limited",
                    "title": "Slow down",
                    "status": 429,
                    "detail": "Try again later",
                    "instance": "urn:problem:42",
                    "correlationId": "problem-correlation",
                })),
        )
        .expect(1)
        .mount(&upstream)
        .await;

    let error = server
        .client
        .glossary()
        .terms()
        .await
        .expect_err("429 must fail");
    let (encoded, failure) = projected_failure(error);
    assert_eq!(failure["code"], "rate_limited");
    assert_eq!(failure["status"], 429);
    assert_eq!(failure["retryable"], true);
    assert_eq!(failure["attempts"], 1);
    assert_eq!(
        failure["retryAfter"],
        json!({ "seconds": 2, "nanoseconds": 0 })
    );
    assert_eq!(failure["correlationId"], "header-correlation");
    assert_eq!(failure["requestId"], "request-42");
    assert_eq!(failure["details"]["problem"]["title"], "Slow down");
    assert!(!encoded.contains(&upstream.uri()));
}

#[tokio::test]
async fn oversized_problem_projection_falls_back_below_four_kib_without_body_leakage() {
    let (upstream, server) = server().await;
    let secret = "sensitive-upstream-body-marker";
    Mock::given(method("GET"))
        .and(path("/glossary"))
        .respond_with(ResponseTemplate::new(503).set_body_json(json!({
            "type": format!("urn:{}", "\\".repeat(1_000)),
            "title": "\\".repeat(1_000),
            "status": 503,
            "detail": format!("{}{}", secret, "\\".repeat(3_000)),
            "instance": format!("urn:{}", "\\".repeat(1_000)),
            "correlationId": "\\".repeat(1_000),
        })))
        .expect(1)
        .mount(&upstream)
        .await;

    let error = server
        .client
        .glossary()
        .terms()
        .await
        .expect_err("503 must fail");
    let (encoded, failure) = projected_failure(error);
    assert!(encoded.len() <= 4_096);
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    assert_eq!(failure["retryable"], true);
    assert!(failure.get("details").is_none());
    assert!(!encoded.contains(secret));
    assert!(!encoded.contains(&upstream.uri()));
}

#[tokio::test]
async fn protocol_projection_names_the_subtype_without_exposing_the_final_url() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/glossary"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(r#"{"glossary":[]}"#, "application/json"),
        )
        .expect(1)
        .mount(&upstream)
        .await;

    let error = server
        .client
        .glossary()
        .terms()
        .await
        .expect_err("wrong media type must fail");
    let (encoded, failure) = projected_failure(error);
    assert_eq!(failure["code"], "protocol_error");
    assert_eq!(
        failure["details"]["protocolSubtype"],
        "incompatible_content_type"
    );
    assert_eq!(failure["details"]["expected"], "application/ld+json");
    assert_eq!(failure["details"]["actual"], "application/json");
    assert!(!encoded.contains(&upstream.uri()));
}

#[tokio::test]
async fn not_found_projection_does_not_expose_an_unparsed_response_body() {
    let (upstream, server) = server().await;
    let secret = "sensitive-unparsed-response-body";
    Mock::given(method("GET"))
        .and(path("/glossary"))
        .respond_with(ResponseTemplate::new(404).set_body_string(secret))
        .expect(1)
        .mount(&upstream)
        .await;

    let error = server
        .client
        .glossary()
        .terms()
        .await
        .expect_err("404 must fail");
    let (encoded, failure) = projected_failure(error);
    assert_eq!(failure["code"], "not_found");
    assert_eq!(failure["status"], 404);
    assert!(!encoded.contains(secret));
    assert!(!encoded.contains(&upstream.uri()));
}

#[tokio::test]
async fn invalid_xml_projection_uses_the_stable_code_without_decoder_text() {
    let (upstream, server) = server().await;
    Mock::given(method("GET"))
        .and(path("/radio/KEC94/broadcast"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw("<speak>sensitive malformed source", "application/ssml+xml"),
        )
        .expect(1)
        .mount(&upstream)
        .await;
    let call_sign: CallSign = "KEC94".parse().expect("fixture call sign must be valid");

    let error = server
        .client
        .radio()
        .broadcast(&call_sign)
        .await
        .expect_err("malformed SSML must fail");
    let (encoded, failure) = projected_failure(error);
    assert_eq!(failure["code"], "invalid_xml_response");
    assert!(!encoded.contains("sensitive malformed source"));
}

#[tokio::test]
async fn transport_projection_is_generic_and_does_not_expose_reqwest_text() {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("an unused local port must be available");
    let address = format!(
        "http://{}",
        listener.local_addr().expect("listener has an address")
    );
    drop(listener);
    let client = Client::builder("noaa-weather-mcp-test/0.0")
        .base_url(&address)
        .retry(RetryPolicy::none())
        .build()
        .expect("local test URL must be valid");
    let server = super::super::NoaaWeatherServer::with_client(client, limit(1_048_576));

    let error = server
        .client
        .glossary()
        .terms()
        .await
        .expect_err("stopped mock server must refuse the request");
    let (encoded, failure) = projected_failure(error);
    assert_eq!(failure["code"], "transport_error");
    assert_eq!(failure["details"]["transportSubtype"], "connect");
    assert!(!encoded.contains(&address));
}
