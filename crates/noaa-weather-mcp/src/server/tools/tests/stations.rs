use std::collections::BTreeSet;

use noaa_weather_client::geo::{Feature, FeatureCollection};
use noaa_weather_client::stations::{
    Observation, ObservationStation, TerminalAerodromeForecast, TerminalAerodromeForecastsResponse,
};
use rmcp::handler::server::tool::schema_for_output;
use rmcp::model::{CallToolResult, Tool};
use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{
    assert_json_success, assert_metadata, call, result_text, server, server_with_router,
};

const LATEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/stations/latest.json"
));
const OBSERVATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/stations/observations.json"
));
const TAFS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/stations/tafs.json"
));
const TAF: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/stations/taf.xml"
));
const STATION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/stations/single.json"
));
const STATIONS_LIST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/stations/list.json"
));

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

#[tokio::test]
async fn router_inventory_has_flat_inputs_concrete_outputs_and_annotations() {
    let (_upstream, server) = server_with_router(NoaaWeatherServer::stations_router()).await;
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
        latest.output_schema.as_ref(),
        Some(&schema_for_output::<Feature<Observation>>())
    );
    assert_metadata(latest, "Latest Station Observation");

    let observations = tool(&tools, "stations_observations");
    assert_eq!(
        property_names(observations),
        BTreeSet::from(["cursor", "end", "limit", "start", "stationId"])
    );
    assert_eq!(required_names(observations), BTreeSet::from(["stationId"]));
    assert_eq!(
        observations.output_schema.as_ref(),
        Some(&schema_for_output::<FeatureCollection<Observation>>())
    );
    assert_metadata(observations, "Station Observations");

    let taf_list = tool(&tools, "stations_taf_list");
    assert_eq!(property_names(taf_list), BTreeSet::from(["stationId"]));
    assert_eq!(required_names(taf_list), BTreeSet::from(["stationId"]));
    assert_eq!(
        taf_list.output_schema.as_ref(),
        Some(&schema_for_output::<TerminalAerodromeForecastsResponse>())
    );
    assert_metadata(taf_list, "Station TAF List");

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
    assert_metadata(taf_get, "Station TAF");
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
        .respond_with(ResponseTemplate::new(200).set_body_raw(OBSERVATIONS, "application/geo+json"))
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
        .respond_with(ResponseTemplate::new(200).set_body_raw(TAF, "application/vnd.wmo.iwxxm+xml"))
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
    assert_json_success(&latest, &latest_expected);

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
    assert_json_success(&observations, &observations_expected);
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
    assert_json_success(&tafs, &tafs_expected);

    let taf = call(
        &server,
        "stations_taf_get",
        json!({
            "stationId": "kphx",
            "issued": "2026-08-30T12:29:45-05:00"
        }),
    )
    .await;
    assert_json_success(&taf, &taf_expected);
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

#[test]
fn remaining_router_has_exact_flat_typed_contracts() {
    let tools = NoaaWeatherServer::stations_remaining_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        ["stations_get", "stations_list", "stations_observation_at"]
    );

    let get = tool(&tools, "stations_get");
    assert_eq!(
        get.description.as_deref(),
        Some("Return NOAA metadata for one surface observation station.")
    );
    assert_eq!(property_names(get), BTreeSet::from(["stationId"]));
    assert_eq!(required_names(get), BTreeSet::from(["stationId"]));
    assert_eq!(
        get.output_schema.as_ref(),
        Some(&schema_for_output::<Feature<ObservationStation>>())
    );
    assert_metadata(get, "Get NOAA Observation Station");

    let list = tool(&tools, "stations_list");
    assert_eq!(
        list.description.as_deref(),
        Some(
            "Return one NOAA page of observation stations matching optional identifiers, areas, and pagination fields."
        )
    );
    assert_eq!(
        property_names(list),
        BTreeSet::from(["cursor", "id", "limit", "state"])
    );
    assert!(
        list.input_schema
            .get("required")
            .and_then(Value::as_array)
            .is_none_or(Vec::is_empty)
    );
    assert_eq!(
        list.output_schema.as_ref(),
        Some(&schema_for_output::<FeatureCollection<ObservationStation>>())
    );
    assert_metadata(list, "List NOAA Observation Stations");

    let observation = tool(&tools, "stations_observation_at");
    assert_eq!(
        observation.description.as_deref(),
        Some("Return the NOAA surface observation recorded by one station at an exact instant.")
    );
    assert_eq!(
        property_names(observation),
        BTreeSet::from(["stationId", "time"])
    );
    assert_eq!(
        required_names(observation),
        BTreeSet::from(["stationId", "time"])
    );
    assert_eq!(
        observation.output_schema.as_ref(),
        Some(&schema_for_output::<Feature<Observation>>())
    );
    assert_metadata(observation, "Get Station Observation at Time");
}

#[tokio::test]
async fn remaining_tools_preserve_routes_media_one_page_and_typed_json() {
    let expected_station: Feature<ObservationStation> =
        serde_json::from_str(STATION).expect("captured station must decode");
    let expected_stations: FeatureCollection<ObservationStation> =
        serde_json::from_str(STATIONS_LIST).expect("captured station list must decode");
    let expected_observation: Feature<Observation> =
        serde_json::from_str(LATEST).expect("captured observation must decode");
    let expected_pagination = serde_json::to_value(&expected_stations)
        .expect("typed station list must serialize")["pagination"]
        .clone();
    let (upstream, server) =
        server_with_router(NoaaWeatherServer::stations_remaining_router()).await;
    Mock::given(method("GET"))
        .and(path("/stations/KSLC"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(STATION, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;
    Mock::given(method("GET"))
        .and(path("/stations"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(STATIONS_LIST, "application/geo+json"),
        )
        .expect(1)
        .mount(&upstream)
        .await;
    Mock::given(method("GET"))
        .and(path("/stations/KPHX/observations/2026-08-30T12:34:56Z"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(LATEST, "application/geo+json"))
        .expect(1)
        .mount(&upstream)
        .await;

    let station = call(&server, "stations_get", json!({ "stationId": "kslc" })).await;
    let stations = call(
        &server,
        "stations_list",
        json!({
            "id": ["KPHX", "kiwa"],
            "state": ["AZ", "CA"],
            "limit": 20,
            "cursor": "next-page"
        }),
    )
    .await;
    let observation = call(
        &server,
        "stations_observation_at",
        json!({
            "stationId": "KPHX",
            "time": "2026-08-30T07:34:56.789-05:00"
        }),
    )
    .await;

    assert_json_success(&station, &expected_station);
    assert_json_success(&stations, &expected_stations);
    assert_json_success(&observation, &expected_observation);
    assert_eq!(
        stations
            .structured_content
            .as_ref()
            .expect("station list must be structured")["pagination"],
        expected_pagination,
    );
    upstream.verify().await;
    let requests = upstream
        .received_requests()
        .await
        .expect("recorded requests must be available");
    assert_eq!(requests.len(), 3, "station listing must remain one page");
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
                "/stations/KSLC".to_owned(),
                None,
                "application/geo+json".to_owned(),
            ),
            (
                "/stations".to_owned(),
                Some("id=KPHX%2CKIWA&state=AZ%2CCA&limit=20&cursor=next-page".to_owned()),
                "application/geo+json".to_owned(),
            ),
            (
                "/stations/KPHX/observations/2026-08-30T12:34:56Z".to_owned(),
                None,
                "application/geo+json".to_owned(),
            ),
        ]
    );
}

#[tokio::test]
async fn remaining_router_rejects_invalid_observation_time_before_http() {
    let (upstream, server) =
        server_with_router(NoaaWeatherServer::stations_remaining_router()).await;
    let result = call(
        &server,
        "stations_observation_at",
        json!({ "stationId": "KPHX", "time": "tomorrow" }),
    )
    .await;

    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert!(result_text(&result).contains("failed to deserialize parameters"));
    assert!(
        upstream
            .received_requests()
            .await
            .expect("recorded requests must be available")
            .is_empty()
    );
}

#[tokio::test]
async fn remaining_router_projects_one_bounded_upstream_error() {
    let (upstream, server) =
        server_with_router(NoaaWeatherServer::stations_remaining_router()).await;
    Mock::given(method("GET"))
        .and(path("/stations/KSLC"))
        .respond_with(ResponseTemplate::new(503).set_body_string("remaining-station-secret"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "stations_get", json!({ "stationId": "KSLC" })).await;
    let failure = assert_failure(&result, "upstream_http_error");
    assert_eq!(failure["status"], 503);
    assert!(!result_text(&result).contains("remaining-station-secret"));
    upstream.verify().await;
}
