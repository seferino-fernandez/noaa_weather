//! Point metadata and point-to-forecast tools.

use noaa_weather_client::Coordinates;
use noaa_weather_client::geo::Feature;
use noaa_weather_client::gridpoints::Forecast;
use noaa_weather_client::points::Point;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct PointParameters {
    /// Latitude and longitude as `latitude,longitude` in decimal degrees.
    point: Coordinates,
}

#[rmcp::tool_router(router = points_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns NOAA metadata for the forecast grid and zones covering a point.
    #[rmcp::tool(
        name = "points_get",
        description = "Get NOAA metadata for the forecast grid and zones covering a latitude/longitude point.",
        annotations(
            title = "Get NOAA Point Metadata",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn points_get(
        &self,
        Parameters(parameters): Parameters<PointParameters>,
    ) -> Result<rmcp::Json<Feature<Point>>, ToolFailure> {
        let point = self.client.points().get(parameters.point).await?;
        Ok(rmcp::Json(point))
    }

    /// Resolves a point to its NOAA grid cell and returns the multi-day forecast.
    #[rmcp::tool(
        name = "points_forecast",
        description = "Resolve a latitude/longitude point to its NOAA grid cell and get the multi-day textual forecast.",
        annotations(
            title = "Get NOAA Forecast for Point",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn points_forecast(
        &self,
        Parameters(parameters): Parameters<PointParameters>,
    ) -> Result<rmcp::Json<Feature<Forecast>>, ToolFailure> {
        let forecast = self.client.points().forecast_for(parameters.point).await?;
        Ok(rmcp::Json(forecast))
    }
}

#[cfg(test)]
mod tests {
    use noaa_weather_client::geo::Feature;
    use noaa_weather_client::gridpoints::Forecast;
    use noaa_weather_client::points::Point;
    use rmcp::ServerHandler as _;
    use rmcp::model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, NumberOrString, Tool,
    };
    use rmcp::service::RequestContext;
    use serde::Serialize;
    use serde_json::{Value, json};
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    use super::super::super::NoaaWeatherServer;
    use super::super::test_support::server;

    const POINT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../noaa-weather-client/tests/fixtures/points/point.json"
    ));
    const FORECAST: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../noaa-weather-client/tests/fixtures/gridpoints/forecast.json"
    ));
    const FORECAST_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

    async fn call_tool(
        server: NoaaWeatherServer,
        request: CallToolRequestParams,
    ) -> CallToolResult {
        let (server_transport, _client_transport) = tokio::io::duplex(1_048_576);
        let running = rmcp::service::serve_directly::<rmcp::RoleServer, _, _, _, _>(
            server,
            server_transport,
            None,
        );
        let context = RequestContext::new(NumberOrString::Number(1), running.peer().clone());
        let response = running
            .service()
            .call_tool(request, context)
            .await
            .expect("tool call must complete");
        running.cancel().await.expect("server must cancel cleanly");
        let CallToolResponse::Complete(result) = response else {
            panic!("tool call must return a complete result");
        };
        result
    }

    fn arguments(value: Value) -> serde_json::Map<String, Value> {
        value
            .as_object()
            .expect("arguments must be an object")
            .clone()
    }

    fn assert_metadata(tool: &Tool, expected_title: &str) {
        assert!(
            tool.description
                .as_deref()
                .is_some_and(|text| !text.is_empty())
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

    fn assert_json_success<T: Serialize>(result: &CallToolResult, expected: &T) {
        let expected = serde_json::to_value(expected).expect("typed fixture must serialize");
        let structured = result
            .structured_content
            .as_ref()
            .unwrap_or_else(|| panic!("success must include structured content: {result:#?}"));
        assert_eq!(result.is_error, Some(false));
        assert_eq!(structured, &expected);
        assert_eq!(result.content.len(), 1);
        let text = result.content[0]
            .as_text()
            .map(|content| content.text.as_str())
            .expect("success must contain JSON text");
        assert_eq!(text, structured.to_string());
        assert_eq!(
            serde_json::from_str::<Value>(text).expect("text must be JSON"),
            expected
        );
        assert!(!text.contains("```"));
    }

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

        let result = call_tool(
            server,
            CallToolRequestParams::new("points_get")
                .with_arguments(arguments(json!({ "point": "39.7456,-97.0892" }))),
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

        let result = call_tool(
            server,
            CallToolRequestParams::new("points_forecast")
                .with_arguments(arguments(json!({ "point": "39.7456,-97.0892" }))),
        )
        .await;
        let expected: Feature<Forecast> =
            serde_json::from_str(FORECAST).expect("fixture must decode");
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
        let result = call_tool(
            server,
            CallToolRequestParams::new("points_get")
                .with_arguments(arguments(json!({ "point": "91,0" }))),
        )
        .await;

        assert_eq!(result.is_error, Some(true));
        assert!(result.structured_content.is_none());
        assert!(
            result.content[0]
                .as_text()
                .is_some_and(|content| content.text.contains("failed to deserialize parameters"))
        );
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

        let result = call_tool(
            server,
            CallToolRequestParams::new("points_get")
                .with_arguments(arguments(json!({ "point": "39.7456,-97.0892" }))),
        )
        .await;
        assert_eq!(result.is_error, Some(true));
        assert!(result.structured_content.is_none());
        assert_eq!(result.content.len(), 1);
        let text = result.content[0]
            .as_text()
            .map(|content| content.text.as_str())
            .expect("failure must be JSON text");
        let error: Value = serde_json::from_str(text).expect("failure text must be JSON");
        assert_eq!(error["code"], "upstream_http_error");
        assert_eq!(error["status"], 503);
        assert!(!text.contains("upstream body is private"));
    }
}
