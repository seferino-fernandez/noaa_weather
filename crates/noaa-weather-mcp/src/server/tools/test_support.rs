//! Hermetic server construction for tool-family tests.

use std::num::NonZeroUsize;

use noaa_weather_client::{Client, RetryPolicy};
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, NumberOrString, Tool,
};
use rmcp::service::{RequestContext, serve_directly};
use rmcp::{RoleServer, ServerHandler as _};
use serde::Serialize;
use serde_json::Value;
use wiremock::MockServer;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

/// Starts a mock upstream and builds the real client/server adapter against it.
pub(in crate::server) async fn server_with_limit(
    max_response_bytes: NonZeroUsize,
) -> (MockServer, NoaaWeatherServer) {
    let upstream = MockServer::start().await;
    let client = Client::builder("noaa-weather-mcp-test/0.0")
        .base_url(upstream.uri())
        .retry(RetryPolicy::none())
        .build()
        .expect("wiremock URI and test user agent must be valid");
    let server = NoaaWeatherServer::with_client(client, max_response_bytes);
    (upstream, server)
}

/// Starts a mock upstream with the production one-mebibyte result cap.
pub(in crate::server) async fn server() -> (MockServer, NoaaWeatherServer) {
    server_with_limit(NonZeroUsize::new(1_048_576).expect("one MiB is nonzero")).await
}

/// Calls one tool through the server handler while hiding rmcp transport setup.
pub(in crate::server) async fn call(
    server: &NoaaWeatherServer,
    name: &'static str,
    arguments: Value,
) -> CallToolResult {
    let (server_transport, _client_transport) = tokio::io::duplex(1_048_576);
    let running = serve_directly::<RoleServer, _, _, _, _>(server.clone(), server_transport, None);
    let request = CallToolRequestParams::new(name).with_arguments(
        arguments
            .as_object()
            .expect("tool arguments must be an object")
            .clone(),
    );
    let context = RequestContext::new(NumberOrString::Number(1), running.peer().clone());
    let response = running
        .service()
        .call_tool(request, context)
        .await
        .unwrap_or_else(|error| panic!("{name} call must complete: {error}"));
    running
        .cancel()
        .await
        .expect("test server must cancel cleanly");
    let CallToolResponse::Complete(result) = response else {
        panic!("{name} must return a complete result");
    };
    result
}

/// Returns the single JSON text block carried by a tool result.
pub(in crate::server) fn result_text(result: &CallToolResult) -> &str {
    result
        .content
        .first()
        .and_then(ContentBlock::as_text)
        .map(|content| content.text.as_str())
        .expect("tool result must contain text")
}

/// Asserts the metadata shared by every read-only NOAA tool.
pub(in crate::server) fn assert_metadata(tool: &Tool, expected_title: &str) {
    assert!(
        tool.description
            .as_deref()
            .is_some_and(|description| !description.trim().is_empty())
    );
    assert!(tool.input_schema.get("properties").is_some());
    assert!(tool.output_schema.is_some());
    let annotations = tool.annotations.as_ref().expect("annotations must exist");
    assert_eq!(annotations.title.as_deref(), Some(expected_title));
    assert_eq!(annotations.read_only_hint, Some(true));
    assert_eq!(annotations.open_world_hint, Some(true));
    assert_eq!(annotations.destructive_hint, None);
    assert_eq!(annotations.idempotent_hint, None);
}

/// Asserts typed structured JSON and the matching JSON text fallback.
pub(in crate::server) fn assert_json_success<T: Serialize>(result: &CallToolResult, expected: &T) {
    let expected = serde_json::to_value(expected).expect("typed fixture must serialize");
    let structured = result
        .structured_content
        .as_ref()
        .unwrap_or_else(|| panic!("success must include structured content: {result:#?}"));
    assert_eq!(result.is_error, Some(false));
    assert_eq!(structured, &expected);
    assert_eq!(result.content.len(), 1);
    let text = result_text(result);
    assert_eq!(text, structured.to_string());
    assert_eq!(
        serde_json::from_str::<Value>(text).expect("tool text must contain parseable JSON"),
        expected
    );
    assert!(!text.contains("```"));
}

/// Returns the exact caller-visible JSON text and parsed value for a client error.
pub(in crate::server) fn projected_failure(
    error: noaa_weather_client::Error,
) -> (String, serde_json::Value) {
    let failure = ToolFailure::from(error);
    let text = failure.json().to_owned();
    let value = serde_json::from_str(&text).expect("tool failure must be valid JSON");
    (text, value)
}
