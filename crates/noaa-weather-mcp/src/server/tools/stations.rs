//! Observation and terminal-aerodrome-forecast tools.

use noaa_weather_client::geo::{Feature, FeatureCollection};
use noaa_weather_client::stations::{
    LatestObservationQuery, Observation, ObservationsQuery, TerminalAerodromeForecast,
    TerminalAerodromeForecastsResponse,
};
use noaa_weather_client::{OffsetDateTime, StationId};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct LatestObservationArguments {
    station_id: StationId,
    #[serde(flatten)]
    query: LatestObservationQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ObservationsArguments {
    station_id: StationId,
    #[serde(flatten)]
    query: ObservationsQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct TafListArguments {
    station_id: StationId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct TafGetArguments {
    station_id: StationId,
    issued: OffsetDateTime,
}

#[rmcp::tool_router(router = stations_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "stations_observation_latest",
        description = "Return the latest surface observation reported by one NOAA station.",
        annotations(
            title = "Latest Station Observation",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_observation_latest(
        &self,
        Parameters(arguments): Parameters<LatestObservationArguments>,
    ) -> Result<Json<Feature<Observation>>, ToolFailure> {
        self.client
            .stations()
            .latest_observation(&arguments.station_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "stations_observations",
        description = "Return one NOAA page of surface observations for a station.",
        annotations(
            title = "Station Observations",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_observations(
        &self,
        Parameters(arguments): Parameters<ObservationsArguments>,
    ) -> Result<Json<FeatureCollection<Observation>>, ToolFailure> {
        self.client
            .stations()
            .observations(&arguments.station_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "stations_taf_list",
        description = "Return metadata for the current Terminal Aerodrome Forecasts at a station.",
        annotations(
            title = "Station TAF List",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_taf_list(
        &self,
        Parameters(arguments): Parameters<TafListArguments>,
    ) -> Result<Json<TerminalAerodromeForecastsResponse>, ToolFailure> {
        self.client
            .stations()
            .tafs(&arguments.station_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "stations_taf_get",
        description = "Return one decoded Terminal Aerodrome Forecast by station and issue time.",
        annotations(title = "Station TAF", read_only_hint = true, open_world_hint = true)
    )]
    async fn stations_taf_get(
        &self,
        Parameters(arguments): Parameters<TafGetArguments>,
    ) -> Result<Json<TerminalAerodromeForecast>, ToolFailure> {
        self.client
            .stations()
            .taf(&arguments.station_id, arguments.issued.timestamp())
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use noaa_weather_client::geo::{Feature, FeatureCollection};
    use noaa_weather_client::stations::{
        Observation, TerminalAerodromeForecast, TerminalAerodromeForecastsResponse,
    };
    use rmcp::RoleServer;
    use rmcp::handler::server::tool::{ToolCallContext, schema_for_input, schema_for_output};
    use rmcp::model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ClientInfo, ContentBlock,
        NumberOrString, Tool,
    };
    use rmcp::service::{RequestContext, serve_directly};
    use serde::Serialize;
    use serde_json::{Value, json};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    use super::*;
    use crate::server::tools::test_support::server;

    const LATEST: &str =
        include_str!("../../../../noaa-weather-client/tests/fixtures/stations/latest.json");
    const OBSERVATIONS: &str =
        include_str!("../../../../noaa-weather-client/tests/fixtures/stations/observations.json");
    const TAFS: &str =
        include_str!("../../../../noaa-weather-client/tests/fixtures/stations/tafs.json");
    const TAF: &str =
        include_str!("../../../../noaa-weather-client/tests/fixtures/stations/taf.xml");

    fn station_tools(server: &NoaaWeatherServer) -> Vec<Tool> {
        server
            .tool_router
            .list_all()
            .into_iter()
            .filter(|tool| tool.name.starts_with("stations_"))
            .collect()
    }

    fn tool<'a>(tools: &'a [Tool], name: &str) -> &'a Tool {
        tools
            .iter()
            .find(|tool| tool.name == name)
            .unwrap_or_else(|| panic!("missing {name} tool"))
    }

    fn property_names(tool: &Tool) -> BTreeSet<&str> {
        tool.input_schema
            .get("properties")
            .and_then(Value::as_object)
            .expect("input schema must contain properties")
            .keys()
            .map(String::as_str)
            .collect()
    }

    fn required_names(tool: &Tool) -> BTreeSet<&str> {
        tool.input_schema
            .get("required")
            .and_then(Value::as_array)
            .expect("input schema must contain required properties")
            .iter()
            .map(|name| name.as_str().expect("required name must be text"))
            .collect()
    }

    async fn call(
        server: &NoaaWeatherServer,
        name: &'static str,
        arguments: Value,
    ) -> CallToolResult {
        let (server_transport, _client_transport) = tokio::io::duplex(4_096);
        let running = serve_directly::<RoleServer, _, _, _, _>(
            server.clone(),
            server_transport,
            Some(ClientInfo::default()),
        );
        let request = CallToolRequestParams::new(name).with_arguments(
            arguments
                .as_object()
                .expect("tool arguments must be an object")
                .clone(),
        );
        let context = ToolCallContext::new(
            running.service(),
            request,
            RequestContext::new(NumberOrString::Number(1), running.peer().clone()),
        );
        let response = running
            .service()
            .tool_router
            .call(context)
            .await
            .unwrap_or_else(|error| panic!("{name} call failed: {error}"));
        running
            .cancel()
            .await
            .expect("test server task must finish cleanly");
        let CallToolResponse::Complete(result) = response else {
            panic!("{name} must return a complete result");
        };
        result
    }

    fn result_text(result: &CallToolResult) -> &str {
        result
            .content
            .first()
            .and_then(ContentBlock::as_text)
            .map(|content| content.text.as_str())
            .expect("tool result must contain JSON text")
    }

    fn assert_success<T: Serialize>(result: &CallToolResult, expected: &T) {
        let expected = serde_json::to_value(expected).expect("fixture value must serialize");
        assert_eq!(result.is_error, Some(false));
        assert_eq!(result.structured_content.as_ref(), Some(&expected));
        assert_eq!(result.content.len(), 1);
        assert_eq!(result_text(result), expected.to_string());
        assert_eq!(
            serde_json::from_str::<Value>(result_text(result))
                .expect("tool text must contain parseable JSON"),
            expected
        );
    }

    fn assert_failure(result: &CallToolResult, code: &str) -> Value {
        assert_eq!(result.is_error, Some(true));
        assert!(result.structured_content.is_none());
        assert_eq!(result.content.len(), 1);
        let text = result_text(result);
        assert!(text.len() <= 4_096);
        let failure: Value = serde_json::from_str(text).expect("tool failure must be JSON text");
        assert_eq!(failure["code"], code);
        failure
    }

    fn assert_annotations(tool: &Tool, title: &str) {
        assert!(
            tool.description
                .as_deref()
                .is_some_and(|description| !description.trim().is_empty())
        );
        let annotations = tool
            .annotations
            .as_ref()
            .expect("station tool must have annotations");
        assert_eq!(annotations.title.as_deref(), Some(title));
        assert_eq!(annotations.read_only_hint, Some(true));
        assert_eq!(annotations.open_world_hint, Some(true));
        assert_eq!(annotations.destructive_hint, None);
        assert_eq!(annotations.idempotent_hint, None);
    }

    #[tokio::test]
    async fn router_inventory_has_flat_inputs_concrete_outputs_and_annotations() {
        let (_upstream, server) = server().await;
        let tools = station_tools(&server);
        assert_eq!(
            tools
                .iter()
                .map(|tool| tool.name.as_ref())
                .collect::<Vec<_>>(),
            [
                "stations_observation_latest",
                "stations_observations",
                "stations_taf_get",
                "stations_taf_list",
            ]
        );

        let latest = tool(&tools, "stations_observation_latest");
        assert_eq!(
            property_names(latest),
            BTreeSet::from(["requireQc", "stationId"])
        );
        assert_eq!(required_names(latest), BTreeSet::from(["stationId"]));
        assert_eq!(
            latest.input_schema,
            schema_for_input::<LatestObservationArguments>().unwrap()
        );
        assert_eq!(
            latest.output_schema.as_ref(),
            Some(&schema_for_output::<Feature<Observation>>())
        );
        assert_annotations(latest, "Latest Station Observation");

        let observations = tool(&tools, "stations_observations");
        assert_eq!(
            property_names(observations),
            BTreeSet::from(["cursor", "end", "limit", "start", "stationId"])
        );
        assert_eq!(required_names(observations), BTreeSet::from(["stationId"]));
        assert_eq!(
            observations.input_schema,
            schema_for_input::<ObservationsArguments>().unwrap()
        );
        assert_eq!(
            observations.output_schema.as_ref(),
            Some(&schema_for_output::<FeatureCollection<Observation>>())
        );
        assert_annotations(observations, "Station Observations");

        let taf_list = tool(&tools, "stations_taf_list");
        assert_eq!(property_names(taf_list), BTreeSet::from(["stationId"]));
        assert_eq!(required_names(taf_list), BTreeSet::from(["stationId"]));
        assert_eq!(
            taf_list.output_schema.as_ref(),
            Some(&schema_for_output::<TerminalAerodromeForecastsResponse>())
        );
        assert_annotations(taf_list, "Station TAF List");

        let taf_get = tool(&tools, "stations_taf_get");
        assert_eq!(
            property_names(taf_get),
            BTreeSet::from(["issued", "stationId"])
        );
        assert_eq!(
            required_names(taf_get),
            BTreeSet::from(["issued", "stationId"])
        );
        assert_eq!(
            taf_get.output_schema.as_ref(),
            Some(&schema_for_output::<TerminalAerodromeForecast>())
        );
        assert_annotations(taf_get, "Station TAF");
    }

    #[tokio::test]
    async fn tools_preserve_typed_results_and_exact_http_contracts() {
        let (upstream, server) = server().await;
        Mock::given(method("GET"))
            .and(path("/stations/KSLC/observations/latest"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(LATEST, "application/geo+json"))
            .expect(1)
            .mount(&upstream)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KSLC/observations"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(OBSERVATIONS, "application/geo+json"),
            )
            .expect(1)
            .mount(&upstream)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/tafs"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(TAFS, "application/ld+json"))
            .expect(1)
            .mount(&upstream)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/tafs/2026-08-30/1729"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(TAF, "application/vnd.wmo.iwxxm+xml"),
            )
            .expect(1)
            .mount(&upstream)
            .await;

        let latest_expected: Feature<Observation> =
            serde_json::from_str(LATEST).expect("latest fixture must decode");
        let observations_expected: FeatureCollection<Observation> =
            serde_json::from_str(OBSERVATIONS).expect("observations fixture must decode");
        let tafs_expected: TerminalAerodromeForecastsResponse =
            serde_json::from_str(TAFS).expect("TAF list fixture must decode");
        let taf_expected =
            TerminalAerodromeForecast::from_iwxxm(TAF.as_bytes()).expect("TAF fixture must decode");

        let latest = call(
            &server,
            "stations_observation_latest",
            json!({ "stationId": "kslc", "requireQc": true }),
        )
        .await;
        assert_success(&latest, &latest_expected);

        let observations = call(
            &server,
            "stations_observations",
            json!({
                "stationId": "kslc",
                "start": "2026-08-30T00:00:00.5Z",
                "end": "2026-08-30T06:00:00Z",
                "limit": 2,
                "cursor": "bmV4dA=="
            }),
        )
        .await;
        assert_success(&observations, &observations_expected);
        assert_eq!(
            observations
                .structured_content
                .as_ref()
                .and_then(|value| value.pointer("/pagination/next")),
            Some(&json!(
                "https://api.weather.gov/stations/KSLC/observations?cursor=eyJzIjoiMjAyNi0wOS0wMlQwNzoxMDowMCswMDowMCJ9"
            ))
        );

        let tafs = call(&server, "stations_taf_list", json!({ "stationId": "kphx" })).await;
        assert_success(&tafs, &tafs_expected);

        let taf = call(
            &server,
            "stations_taf_get",
            json!({
                "stationId": "kphx",
                "issued": "2026-08-30T12:29:45-05:00"
            }),
        )
        .await;
        assert_success(&taf, &taf_expected);
        let taf_json = taf
            .structured_content
            .as_ref()
            .expect("decoded TAF must be structured");
        assert_eq!(
            taf_json["bulletinIdentifier"],
            "A_LTUS45KPSR301700_C_KPSR_20260830173003.xml"
        );
        assert_eq!(taf_json["issuedAt"], "2026-08-30T17:29:00Z");
        assert_eq!(taf_json["aerodrome"]["icaoIdentifier"], "KPHX");
        assert_eq!(taf_json["report"]["kind"], "forecast");
        assert_eq!(
            taf_json.pointer("/report/groups/0/conditions/wind/value/speed/knots"),
            Some(&json!(8.0))
        );

        let requests = upstream
            .received_requests()
            .await
            .expect("recorded requests must be available");
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
                    "/stations/KSLC/observations/latest".to_owned(),
                    Some("require_qc=true".to_owned()),
                    "application/geo+json".to_owned(),
                ),
                (
                    "/stations/KSLC/observations".to_owned(),
                    Some(
                        "start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T06%3A00%3A00Z&limit=2&cursor=bmV4dA%3D%3D"
                            .to_owned(),
                    ),
                    "application/geo+json".to_owned(),
                ),
                (
                    "/stations/KPHX/tafs".to_owned(),
                    None,
                    "application/ld+json".to_owned(),
                ),
                (
                    "/stations/KPHX/tafs/2026-08-30/1729".to_owned(),
                    None,
                    "application/vnd.wmo.iwxxm+xml".to_owned(),
                ),
            ]
        );
    }

    #[tokio::test]
    async fn invalid_parameters_are_rejected_before_an_upstream_request() {
        let (upstream, server) = server().await;
        let invalid_station = call(
            &server,
            "stations_observation_latest",
            json!({ "stationId": "K!" }),
        )
        .await;
        assert_eq!(invalid_station.is_error, Some(true));
        assert!(result_text(&invalid_station).contains("failed to deserialize parameters"));

        let invalid_issued = call(
            &server,
            "stations_taf_get",
            json!({ "stationId": "KPHX", "issued": "tomorrow" }),
        )
        .await;
        assert_eq!(invalid_issued.is_error, Some(true));
        assert!(result_text(&invalid_issued).contains("failed to deserialize parameters"));

        assert!(
            upstream
                .received_requests()
                .await
                .expect("recorded requests must be available")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn upstream_and_taf_decode_failures_are_bounded_json_tool_errors() {
        let (upstream, server) = server().await;
        Mock::given(method("GET"))
            .and(path("/stations/KSLC/observations/latest"))
            .respond_with(ResponseTemplate::new(503).set_body_json(json!({
                "type": "about:blank",
                "title": "Service unavailable",
                "status": 503,
                "detail": "Try again later",
                "instance": "urn:test:unavailable"
            })))
            .expect(1)
            .mount(&upstream)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/tafs/2026-08-30/1729"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw("<broken", "application/vnd.wmo.iwxxm+xml"),
            )
            .expect(1)
            .mount(&upstream)
            .await;

        let upstream_failure = call(
            &server,
            "stations_observation_latest",
            json!({ "stationId": "KSLC" }),
        )
        .await;
        let upstream_failure = assert_failure(&upstream_failure, "upstream_http_error");
        assert_eq!(upstream_failure["status"], 503);

        let taf_failure = call(
            &server,
            "stations_taf_get",
            json!({
                "stationId": "KPHX",
                "issued": "2026-08-30T12:29:45-05:00"
            }),
        )
        .await;
        let taf_failure = assert_failure(&taf_failure, "invalid_taf_response");
        assert_eq!(taf_failure["details"]["kind"], "malformed_xml");
    }
}
