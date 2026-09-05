//! Hermetic server construction for tool-family tests.

use std::num::NonZeroUsize;

use noaa_weather_client::{Client, RetryPolicy};
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

/// Returns the exact caller-visible JSON text and parsed value for a client error.
pub(in crate::server) fn projected_failure(
    error: noaa_weather_client::Error,
) -> (String, serde_json::Value) {
    let failure = ToolFailure::from(error);
    let text = failure.json().to_owned();
    let value = serde_json::from_str(&text).expect("tool failure must be valid JSON");
    (text, value)
}
