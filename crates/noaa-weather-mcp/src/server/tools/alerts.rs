//! Alert tools.

use noaa_weather_client::AlertId;
use noaa_weather_client::alerts::{ActiveAlertsQuery, Alert};
use noaa_weather_client::geo::{Feature, FeatureCollection};
use rmcp::handler::server::wrapper::Parameters;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GetAlertParams {
    alert_id: AlertId,
}

#[rmcp::tool_router(router = alerts_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns one page of currently active NOAA weather alerts matching the supplied filters.
    #[rmcp::tool(
        name = "alerts_active",
        description = "Return one NOAA page of currently active weather alerts matching optional status, message, event, location, urgency, severity, and certainty filters.",
        annotations(
            title = "Get Active NOAA Weather Alerts",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_active(
        &self,
        Parameters(query): Parameters<ActiveAlertsQuery>,
    ) -> Result<rmcp::Json<FeatureCollection<Alert>>, ToolFailure> {
        let alerts = self.client.alerts().active(&query).await?;
        Ok(rmcp::Json(alerts))
    }

    /// Returns one NOAA weather alert by its canonical alert identifier.
    #[rmcp::tool(
        name = "alerts_get",
        description = "Return one NOAA weather alert by its canonical alert identifier.",
        annotations(
            title = "Get NOAA Weather Alert",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_get(
        &self,
        Parameters(GetAlertParams { alert_id }): Parameters<GetAlertParams>,
    ) -> Result<rmcp::Json<Feature<Alert>>, ToolFailure> {
        let alert = self.client.alerts().get(&alert_id).await?;
        Ok(rmcp::Json(alert))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use noaa_weather_client::alerts::Alert;
    use noaa_weather_client::geo::{Feature, FeatureCollection};
    use rmcp::ServiceExt as _;
    use serde_json::{Value, json};
    use tokio::io::{
        AsyncBufReadExt as _, AsyncWriteExt as _, BufReader, DuplexStream, ReadHalf, WriteHalf,
    };
    use wiremock::matchers::{header, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_support;
    use super::NoaaWeatherServer;

    const ALERTS_LIST: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../noaa-weather-client/tests/fixtures/alerts/list.json"
    ));
    const ALERT_SINGLE: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../noaa-weather-client/tests/fixtures/alerts/single.json"
    ));

    async fn send_json(writer: &mut WriteHalf<DuplexStream>, message: &Value) {
        let mut encoded = serde_json::to_vec(message).expect("MCP test message must serialize");
        encoded.push(b'\n');
        writer
            .write_all(&encoded)
            .await
            .expect("MCP test message must be written");
    }

    async fn receive_json(reader: &mut BufReader<ReadHalf<DuplexStream>>) -> Value {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .await
            .expect("MCP response must be readable");
        assert!(!line.is_empty(), "MCP server closed before responding");
        serde_json::from_str(&line).expect("MCP response must be JSON")
    }

    async fn call_tool(server: NoaaWeatherServer, name: &str, arguments: Value) -> Value {
        let (server_transport, client_transport) = tokio::io::duplex(128 * 1_024);
        let server_task = tokio::spawn(async move {
            let service = server
                .serve(server_transport)
                .await
                .expect("test MCP server must initialize");
            service
                .waiting()
                .await
                .expect("test MCP server must shut down cleanly")
        });
        let (client_read, mut client_write) = tokio::io::split(client_transport);
        let mut client_read = BufReader::new(client_read);

        send_json(
            &mut client_write,
            &json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2025-11-25",
                    "capabilities": {},
                    "clientInfo": {"name": "alerts-test", "version": "0.0.0"}
                }
            }),
        )
        .await;
        let initialized = receive_json(&mut client_read).await;
        assert_eq!(initialized["id"], 1);
        assert!(initialized.get("result").is_some(), "{initialized}");

        send_json(
            &mut client_write,
            &json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        )
        .await;
        send_json(
            &mut client_write,
            &json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/call",
                "params": {"name": name, "arguments": arguments}
            }),
        )
        .await;
        let response = receive_json(&mut client_read).await;
        assert_eq!(response["id"], 2);

        drop(client_write);
        drop(client_read);
        server_task.await.expect("test MCP server task must join");
        response
    }

    fn result(response: &Value) -> &Value {
        response
            .get("result")
            .unwrap_or_else(|| panic!("expected tool result, got {response}"))
    }

    fn assert_structured_success(response: &Value, expected: &Value) {
        let result = result(response);
        assert_eq!(result["isError"], false);
        assert_eq!(result["structuredContent"], *expected);
        let content = result["content"]
            .as_array()
            .expect("tool content must be an array");
        assert_eq!(content.len(), 1);
        assert_eq!(content[0]["type"], "text");
        let text = content[0]["text"]
            .as_str()
            .expect("tool fallback must be text");
        assert_eq!(
            text,
            serde_json::to_string(expected).expect("expected response must serialize")
        );
        assert_eq!(
            serde_json::from_str::<Value>(text).expect("tool fallback must be JSON"),
            *expected
        );
    }

    fn assert_parameter_failure(response: &Value) {
        let result = result(response);
        assert_eq!(result["isError"], true);
        assert!(result.get("structuredContent").is_none());
        let content = result["content"]
            .as_array()
            .expect("parameter failure content must be an array");
        assert_eq!(content.len(), 1);
        assert!(
            content[0]["text"]
                .as_str()
                .is_some_and(|text| text.starts_with("failed to deserialize parameters:"))
        );
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
            assert!(tool.output_schema.is_some());
            let annotations = tool
                .annotations
                .as_ref()
                .expect("alert tools must publish annotations");
            assert!(
                annotations
                    .title
                    .as_deref()
                    .is_some_and(|title| !title.is_empty())
            );
            assert_eq!(annotations.read_only_hint, Some(true));
            assert_eq!(annotations.open_world_hint, Some(true));
            assert_eq!(annotations.destructive_hint, None);
            assert_eq!(annotations.idempotent_hint, None);
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
        let expected_json =
            serde_json::to_value(&expected).expect("typed alert list must serialize");
        let expected_pagination = expected_json["pagination"].clone();
        let (upstream, server) = test_support::server().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active"))
            .and(query_param("status", "actual,test"))
            .and(query_param("message_type", "Alert,Cancel"))
            .and(query_param("event", "Flood Watch,Wind/Warning"))
            .and(query_param("region_type", "marine"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(ALERTS_LIST, "application/geo+json"),
            )
            .expect(1)
            .mount(&upstream)
            .await;

        let response = call_tool(
            server,
            "alerts_active",
            json!({
                "status": ["Actual", "Test"],
                "messageType": ["Alert", "Cancel"],
                "event": ["Flood Watch", "Wind/Warning"],
                "regionType": "marine"
            }),
        )
        .await;

        assert_structured_success(&response, &expected_json);
        assert_eq!(
            result(&response)["structuredContent"]["pagination"],
            expected_pagination
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
        let expected_json = serde_json::to_value(&expected).expect("typed alert must serialize");
        let alert_id = expected.properties.id.as_str().to_owned();
        let expected_path = format!("/alerts/{alert_id}");
        let (upstream, server) = test_support::server().await;
        Mock::given(method("GET"))
            .and(path(expected_path))
            .and(header("Accept", "application/geo+json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(ALERT_SINGLE, "application/geo+json"),
            )
            .expect(1)
            .mount(&upstream)
            .await;

        let response = call_tool(server, "alerts_get", json!({"alertId": alert_id})).await;

        assert_structured_success(&response, &expected_json);
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
        let (upstream, server) = test_support::server().await;
        let response = call_tool(server, "alerts_get", json!({"alertId": ""})).await;

        assert_parameter_failure(&response);
        no_requests(&upstream).await;
    }

    #[tokio::test]
    async fn invalid_active_filter_is_rejected_by_parameter_decoding_before_http() {
        let (upstream, server) = test_support::server().await;
        let response = call_tool(
            server,
            "alerts_active",
            json!({"regionType": "not-a-region"}),
        )
        .await;

        assert_parameter_failure(&response);
        no_requests(&upstream).await;
    }

    #[tokio::test]
    async fn upstream_failure_uses_the_bounded_stable_tool_error() {
        let (upstream, server) = test_support::server().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active"))
            .respond_with(ResponseTemplate::new(503).set_body_string("upstream-body-secret"))
            .expect(1)
            .mount(&upstream)
            .await;

        let response = call_tool(server, "alerts_active", json!({})).await;
        let result = result(&response);
        assert_eq!(result["isError"], true);
        assert!(result.get("structuredContent").is_none());
        let content = result["content"]
            .as_array()
            .expect("tool failure content must be an array");
        assert_eq!(content.len(), 1);
        assert_eq!(content[0]["type"], "text");
        let text = content[0]["text"]
            .as_str()
            .expect("tool failure must contain text");
        assert!(text.len() <= 4_096);
        assert!(!text.contains("upstream-body-secret"));
        let failure: Value = serde_json::from_str(text).expect("tool failure text must be JSON");
        assert_eq!(failure["code"], "upstream_http_error");
        assert_eq!(failure["status"], 503);
        upstream.verify().await;
    }
}
