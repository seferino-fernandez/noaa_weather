use std::collections::BTreeSet;

use noaa_weather_client::radio::{RadioBroadcast, RadioTransmitter, RadioTransmitterCollection};
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

const POINT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radio/point.xml"
));
const BROADCAST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radio/broadcast.xml"
));
const TRANSMITTERS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radio/transmitters.json"
));
const TRANSMITTER: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radio/transmitter.json"
));
const COUNTY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/radio/county.json"
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
fn router_publishes_exact_flat_typed_radio_contracts() {
    let tools = NoaaWeatherServer::radio_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "radio_broadcast",
            "radio_broadcast_for_point",
            "radio_transmitter_get",
            "radio_transmitters",
            "radio_transmitters_for_county",
        ]
    );

    assert_contract::<RadioBroadcast>(
        &tools,
        "radio_broadcast_for_point",
        &["point"],
        &["point"],
        "Get Radio Broadcast for Point",
    );
    assert_contract::<RadioBroadcast>(
        &tools,
        "radio_broadcast",
        &["callSign"],
        &["callSign"],
        "Get Radio Broadcast",
    );
    assert_contract::<RadioTransmitterCollection>(
        &tools,
        "radio_transmitters",
        &["cursor"],
        &[],
        "List Radio Transmitters",
    );
    assert_contract::<RadioTransmitter>(
        &tools,
        "radio_transmitter_get",
        &["callSign"],
        &["callSign"],
        "Get Radio Transmitter",
    );
    assert_contract::<RadioTransmitterCollection>(
        &tools,
        "radio_transmitters_for_county",
        &["zoneId"],
        &["zoneId"],
        "List Radio Transmitters for County",
    );
}

#[tokio::test]
async fn every_radio_tool_preserves_route_media_and_typed_json_parity() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::radio_router()).await;
    mount(
        &upstream,
        "/points/33.4484,-112.074/radio",
        POINT,
        "application/ssml+xml",
    )
    .await;
    mount(
        &upstream,
        "/radio/KEC94/broadcast",
        BROADCAST,
        "application/ssml+xml",
    )
    .await;
    mount(&upstream, "/radio", TRANSMITTERS, "application/ld+json").await;
    mount(
        &upstream,
        "/radio/KEC94",
        TRANSMITTER,
        "application/ld+json",
    )
    .await;
    mount(
        &upstream,
        "/zones/county/AZC013/radio",
        COUNTY,
        "application/ld+json",
    )
    .await;

    let result = call(
        &server,
        "radio_broadcast_for_point",
        json!({ "point": "33.4484,-112.074" }),
    )
    .await;
    assert_json_success(
        &result,
        &RadioBroadcast::from_ssml(POINT).expect("point fixture must decode semantically"),
    );

    let result = call(&server, "radio_broadcast", json!({ "callSign": "kec94" })).await;
    assert_json_success(
        &result,
        &RadioBroadcast::from_ssml(BROADCAST).expect("broadcast fixture must decode semantically"),
    );

    let result = call(
        &server,
        "radio_transmitters",
        json!({ "cursor": "opaque+/=_-" }),
    )
    .await;
    let transmitters: RadioTransmitterCollection =
        serde_json::from_str(TRANSMITTERS).expect("transmitters fixture must decode");
    assert_json_success(&result, &transmitters);
    assert_eq!(
        result
            .structured_content
            .as_ref()
            .and_then(|value| value.pointer("/pagination/next")),
        Some(&json!("https://api.weather.gov/radio?cursor=eyJpIjo1MDB9"))
    );

    let result = call(
        &server,
        "radio_transmitter_get",
        json!({ "callSign": "kec94" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<RadioTransmitter>(TRANSMITTER).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "radio_transmitters_for_county",
        json!({ "zoneId": "azc013" }),
    )
    .await;
    let county: RadioTransmitterCollection =
        serde_json::from_str(COUNTY).expect("county fixture must decode");
    assert!(county.pagination.is_none());
    assert_json_success(&result, &county);

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 5);
    assert_eq!(
        requests
            .iter()
            .map(|request| request.url.path())
            .collect::<Vec<_>>(),
        [
            "/points/33.4484,-112.074/radio",
            "/radio/KEC94/broadcast",
            "/radio",
            "/radio/KEC94",
            "/zones/county/AZC013/radio",
        ]
    );
    assert_eq!(requests[0].url.query(), None);
    assert_eq!(requests[1].url.query(), None);
    assert_eq!(requests[2].url.query(), Some("cursor=opaque%2B%2F%3D_-"));
    assert!(
        requests[3..]
            .iter()
            .all(|request| request.url.query().is_none())
    );
}

#[tokio::test]
async fn invalid_radio_call_sign_is_rejected_before_http() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::radio_router()).await;
    let result = call(&server, "radio_broadcast", json!({ "callSign": "WX" })).await;

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
async fn malformed_ssml_becomes_a_bounded_json_xml_error() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::radio_router()).await;
    Mock::given(method("GET"))
        .and(path("/radio/KEC94/broadcast"))
        .and(header("Accept", "application/ssml+xml"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw("<speak>sensitive malformed source", "application/ssml+xml"),
        )
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "radio_broadcast", json!({ "callSign": "KEC94" })).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    let failure: Value = serde_json::from_str(text).expect("failure must be JSON");
    assert_eq!(failure["code"], "invalid_xml_response");
    assert!(!text.contains("sensitive malformed source"));
}
