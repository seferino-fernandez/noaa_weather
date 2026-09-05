//! Gridpoint forecast tools.

use noaa_weather_client::GridpointId;
use noaa_weather_client::geo::Feature;
use noaa_weather_client::gridpoints::{Forecast, ForecastQuery};
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GridpointForecastParameters {
    /// NOAA forecast office and grid coordinates as `OFFICE/x,y`.
    gridpoint_id: GridpointId,
    /// Existing forecast options exposed as flat tool arguments.
    #[serde(flatten)]
    query: ForecastQuery,
}

#[rmcp::tool_router(router = gridpoints_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns the multi-day textual forecast for one NOAA grid cell.
    #[rmcp::tool(
        name = "gridpoints_forecast",
        description = "Get the multi-day NOAA textual forecast for a forecast-office grid cell.",
        annotations(
            title = "Get NOAA Gridpoint Forecast",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn gridpoints_forecast(
        &self,
        Parameters(parameters): Parameters<GridpointForecastParameters>,
    ) -> Result<rmcp::Json<Feature<Forecast>>, ToolFailure> {
        let forecast = self
            .client
            .gridpoints()
            .forecast(&parameters.gridpoint_id, &parameters.query)
            .await?;
        Ok(rmcp::Json(forecast))
    }

    /// Returns the hour-by-hour textual forecast for one NOAA grid cell.
    #[rmcp::tool(
        name = "gridpoints_forecast_hourly",
        description = "Get the hour-by-hour NOAA textual forecast for a forecast-office grid cell.",
        annotations(
            title = "Get NOAA Hourly Gridpoint Forecast",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn gridpoints_forecast_hourly(
        &self,
        Parameters(parameters): Parameters<GridpointForecastParameters>,
    ) -> Result<rmcp::Json<Feature<Forecast>>, ToolFailure> {
        let forecast = self
            .client
            .gridpoints()
            .forecast_hourly(&parameters.gridpoint_id, &parameters.query)
            .await?;
        Ok(rmcp::Json(forecast))
    }
}

#[cfg(test)]
mod tests {
    use noaa_weather_client::geo::Feature;
    use noaa_weather_client::gridpoints::Forecast;
    use rmcp::ServerHandler as _;
    use rmcp::model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, NumberOrString, Tool,
    };
    use rmcp::service::RequestContext;
    use serde::Serialize;
    use serde_json::{Value, json};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    use super::super::super::NoaaWeatherServer;
    use super::super::test_support::server;

    const FORECAST: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../noaa-weather-client/tests/fixtures/gridpoints/forecast.json"
    ));
    const HOURLY: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../noaa-weather-client/tests/fixtures/gridpoints/hourly.json"
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
            .expect("success must include structured content");
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
    fn router_lists_two_typed_tools_with_flat_query_schemas() {
        let tools = NoaaWeatherServer::gridpoints_router().list_all();
        let names: Vec<_> = tools.iter().map(|tool| tool.name.as_ref()).collect();
        assert_eq!(names, ["gridpoints_forecast", "gridpoints_forecast_hourly"]);

        for tool in &tools {
            assert_metadata(
                tool,
                match tool.name.as_ref() {
                    "gridpoints_forecast" => "Get NOAA Gridpoint Forecast",
                    "gridpoints_forecast_hourly" => "Get NOAA Hourly Gridpoint Forecast",
                    name => panic!("unexpected gridpoint tool {name}"),
                },
            );
            let properties = tool.input_schema["properties"]
                .as_object()
                .expect("input schema properties must be an object");
            assert!(properties.contains_key("gridpointId"));
            assert!(properties.contains_key("units"));
            assert!(!properties.contains_key("query"));
            assert!(!properties.contains_key("gridpoint_id"));
            assert_eq!(properties.len(), 2);
            assert_eq!(tool.input_schema["required"], json!(["gridpointId"]));
            assert!(tool.output_schema.is_some());
        }
    }

    #[tokio::test]
    async fn gridpoints_forecast_preserves_flat_units_path_media_and_flags() {
        let (upstream, server) = server().await;
        Mock::given(method("GET"))
            .and(path("/gridpoints/TOP/31,80/forecast"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(FORECAST, "application/geo+json"))
            .expect(1)
            .mount(&upstream)
            .await;

        let result = call_tool(
            server,
            CallToolRequestParams::new("gridpoints_forecast").with_arguments(arguments(json!({
                "gridpointId": "TOP/31,80",
                "units": "si"
            }))),
        )
        .await;
        let expected: Feature<Forecast> =
            serde_json::from_str(FORECAST).expect("fixture must decode");
        assert_json_success(&result, &expected);

        let requests = upstream
            .received_requests()
            .await
            .expect("requests must be readable");
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].url.path(), "/gridpoints/TOP/31,80/forecast");
        assert_eq!(requests[0].url.query(), Some("units=si"));
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
        assert_eq!(requests[0].headers["feature-flags"], FORECAST_FLAGS);
    }

    #[tokio::test]
    async fn gridpoints_hourly_omits_absent_units_and_returns_typed_json() {
        let (upstream, server) = server().await;
        Mock::given(method("GET"))
            .and(path("/gridpoints/TOP/31,80/forecast/hourly"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(HOURLY, "application/geo+json"))
            .expect(1)
            .mount(&upstream)
            .await;

        let result = call_tool(
            server,
            CallToolRequestParams::new("gridpoints_forecast_hourly")
                .with_arguments(arguments(json!({ "gridpointId": "TOP/31,80" }))),
        )
        .await;
        let expected: Feature<Forecast> =
            serde_json::from_str(HOURLY).expect("fixture must decode");
        assert_json_success(&result, &expected);

        let requests = upstream
            .received_requests()
            .await
            .expect("requests must be readable");
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].url.path(),
            "/gridpoints/TOP/31,80/forecast/hourly"
        );
        assert_eq!(requests[0].url.query(), None);
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
        assert_eq!(requests[0].headers["feature-flags"], FORECAST_FLAGS);
    }

    #[tokio::test]
    async fn invalid_gridpoint_id_fails_in_rmcp_before_any_request() {
        let (upstream, server) = server().await;
        let result = call_tool(
            server,
            CallToolRequestParams::new("gridpoints_forecast")
                .with_arguments(arguments(json!({ "gridpointId": "TOP/-1,80" }))),
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
}
