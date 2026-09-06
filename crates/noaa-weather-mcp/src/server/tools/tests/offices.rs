use std::collections::BTreeSet;
use std::num::NonZeroUsize;

use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use noaa_weather_client::offices::{
    Office, OfficeBriefingResponse, OfficeHeadline, OfficeHeadlineCollection,
    OfficeWeatherStoryCollection,
};
use rmcp::handler::server::tool::schema_for_output;
use rmcp::model::{CallToolResult, ContentBlock, ResourceContents, Tool};
use schemars::JsonSchema;
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::super::super::NoaaWeatherServer;
use super::super::test_support::{
    assert_json_success, assert_metadata, call, result_text, server_with_limit, server_with_router,
};

const OFFICE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/offices/office.json"
));
const HEADLINES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/offices/headlines.json"
));
const HEADLINE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/offices/headline.json"
));
const BRIEFING: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/offices/briefing.json"
));
const WEATHER_STORIES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/offices/weather_stories.json"
));

fn tool<'a>(tools: &'a [Tool], name: &str) -> &'a Tool {
    tools
        .iter()
        .find(|tool| tool.name == name)
        .unwrap_or_else(|| panic!("missing {name} tool"))
}

fn names(value: Option<&Value>) -> BTreeSet<&str> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .map(|name| name.as_str().expect("schema name must be text"))
        .collect()
}

fn assert_contract<T: JsonSchema + 'static>(
    tool: &Tool,
    title: &str,
    properties: &[&str],
    required: &[&str],
) {
    assert_metadata(tool, title);
    let actual_properties = tool.input_schema["properties"]
        .as_object()
        .expect("input properties must be an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_properties, properties.iter().copied().collect());
    assert_eq!(
        names(tool.input_schema.get("required")),
        required.iter().copied().collect()
    );
    assert!(!actual_properties.contains("query"));
    assert_eq!(tool.output_schema.as_ref(), Some(&schema_for_output::<T>()));
}

fn assert_binary_contract(tool: &Tool, title: &str, properties: &[&str], required: &[&str]) {
    assert_metadata(tool, title);
    let actual_properties = tool.input_schema["properties"]
        .as_object()
        .expect("input properties must be an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_properties, properties.iter().copied().collect());
    assert_eq!(
        names(tool.input_schema.get("required")),
        required.iter().copied().collect()
    );
    assert!(tool.output_schema.is_none());
}

fn assert_parameter_failure(result: &CallToolResult) {
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    assert!(result_text(result).starts_with("failed to deserialize parameters:"));
}

async fn no_requests(upstream: &MockServer) {
    assert!(
        upstream
            .received_requests()
            .await
            .expect("recorded requests must be readable")
            .is_empty()
    );
}

#[tokio::test]
async fn router_inventory_has_exact_flat_schemas_outputs_and_annotations() {
    let (_upstream, server) = server_with_router(NoaaWeatherServer::offices_router()).await;
    let tools = server.tool_router.list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "offices_briefing",
            "offices_briefing_document",
            "offices_get",
            "offices_headline_get",
            "offices_headlines",
            "offices_latest_briefing_document",
            "offices_weather_stories",
            "offices_weather_story_image",
        ]
    );

    assert_contract::<OfficeBriefingResponse>(
        tool(&tools, "offices_briefing"),
        "Get NOAA Office Briefing",
        &["officeId"],
        &["officeId"],
    );
    assert_contract::<Office>(
        tool(&tools, "offices_get"),
        "Get NOAA Office",
        &["officeId"],
        &["officeId"],
    );
    assert_contract::<OfficeHeadline>(
        tool(&tools, "offices_headline_get"),
        "Get NOAA Office Headline",
        &["headlineId", "officeId"],
        &["headlineId", "officeId"],
    );
    assert_contract::<OfficeHeadlineCollection>(
        tool(&tools, "offices_headlines"),
        "List NOAA Office Headlines",
        &["officeId"],
        &["officeId"],
    );
    assert_contract::<OfficeWeatherStoryCollection>(
        tool(&tools, "offices_weather_stories"),
        "List NOAA Office Weather Stories",
        &["officeId"],
        &["officeId"],
    );
    assert_binary_contract(
        tool(&tools, "offices_briefing_document"),
        "Download NOAA Office Briefing",
        &["briefingId", "officeId"],
        &["briefingId", "officeId"],
    );
    assert_binary_contract(
        tool(&tools, "offices_latest_briefing_document"),
        "Download Latest NOAA Office Briefing",
        &["officeId"],
        &["officeId"],
    );
    assert_binary_contract(
        tool(&tools, "offices_weather_story_image"),
        "Download NOAA Office Weather Story Image",
        &["imageId", "officeId"],
        &["imageId", "officeId"],
    );
}

#[tokio::test]
async fn binary_tools_return_native_mcp_content_and_exact_http_contract() {
    const PDF: &[u8] = b"%PDF-1.7\nbriefing";
    const IMAGE: &[u8] = b"\x89PNG\r\n\x1a\nweather-story";
    let (upstream, server) = server_with_router(NoaaWeatherServer::offices_router()).await;

    for (expected_path, accept, content_type, body) in [
        (
            "/offices/PSR/briefing/download/latest",
            "application/pdf",
            "application/pdf; version=1.7",
            PDF,
        ),
        (
            "/offices/PSR/briefing/download/brief%20%2F%25%3F",
            "application/pdf",
            "application/pdf",
            PDF,
        ),
        (
            "/offices/PSR/weatherstories/download/story%20%2F%25%3F",
            "image/*",
            "image/png",
            IMAGE,
        ),
    ] {
        Mock::given(method("GET"))
            .and(path(expected_path))
            .and(header("Accept", accept))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(body)
                    .insert_header("Content-Type", content_type),
            )
            .expect(1)
            .mount(&upstream)
            .await;
    }

    for (name, arguments, expected_mime_type) in [
        (
            "offices_latest_briefing_document",
            json!({"officeId": "psr"}),
            "application/pdf; version=1.7",
        ),
        (
            "offices_briefing_document",
            json!({"officeId": "psr", "briefingId": "brief /%?"}),
            "application/pdf",
        ),
    ] {
        let result = call(&server, name, arguments).await;
        assert_eq!(result.is_error, Some(false));
        assert!(result.structured_content.is_none());
        assert_eq!(result.content.len(), 1);
        let ContentBlock::Resource(resource) = &result.content[0] else {
            panic!("{name} must return an embedded resource");
        };
        let ResourceContents::BlobResourceContents {
            uri,
            mime_type,
            blob,
            ..
        } = &resource.resource
        else {
            panic!("{name} must return a blob resource");
        };
        assert!(uri.starts_with(&upstream.uri()));
        assert!(uri.contains("/offices/PSR/briefing/download/"));
        assert_eq!(mime_type.as_deref(), Some(expected_mime_type));
        assert_eq!(BASE64_STANDARD.decode(blob).unwrap(), PDF);
    }

    let image = call(
        &server,
        "offices_weather_story_image",
        json!({"officeId": "psr", "imageId": "story /%?"}),
    )
    .await;
    assert_eq!(image.is_error, Some(false));
    assert!(image.structured_content.is_none());
    assert_eq!(image.content.len(), 1);
    let ContentBlock::Image(image) = &image.content[0] else {
        panic!("weather story must return image content");
    };
    assert_eq!(image.mime_type, "image/png");
    assert_eq!(BASE64_STANDARD.decode(&image.data).unwrap(), IMAGE);

    upstream.verify().await;
    assert_eq!(
        upstream
            .received_requests()
            .await
            .expect("recorded requests must be readable")
            .len(),
        3,
        "each binary tool must make exactly one request"
    );
}

#[tokio::test]
async fn binary_tool_rejects_raw_payload_over_the_configured_limit_before_encoding() {
    const IMAGE: &[u8] = b"12345";
    let (upstream, server) = server_with_limit(NonZeroUsize::new(IMAGE.len() - 1).unwrap()).await;
    Mock::given(method("GET"))
        .and(path("/offices/PSR/weatherstories/download/story-1"))
        .and(header("Accept", "image/*"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(IMAGE, "image/jpeg"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "offices_weather_story_image",
        json!({"officeId": "PSR", "imageId": "story-1"}),
    )
    .await;

    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let failure: Value =
        serde_json::from_str(result_text(&result)).expect("limit failure must be JSON");
    assert_eq!(failure["code"], "response_too_large");
    assert_eq!(failure["bytes"], IMAGE.len());
    assert_eq!(failure["limit"], IMAGE.len() - 1);
    assert!(failure["message"].as_str().unwrap().contains("binary"));
    upstream.verify().await;
}

#[tokio::test]
async fn binary_tool_media_failure_uses_the_bounded_client_error_contract() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::offices_router()).await;
    Mock::given(method("GET"))
        .and(path("/offices/PSR/weatherstories/download/story-1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not an image", "text/plain"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "offices_weather_story_image",
        json!({"officeId": "PSR", "imageId": "story-1"}),
    )
    .await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let failure: Value =
        serde_json::from_str(result_text(&result)).expect("tool failure must be JSON");
    assert_eq!(failure["code"], "protocol_error");
    assert_eq!(
        failure["details"]["protocolSubtype"],
        "incompatible_content_type"
    );
    upstream.verify().await;
}

#[tokio::test]
async fn tools_preserve_every_typed_result_and_exact_http_contract() {
    let office_expected: Office = serde_json::from_str(OFFICE).expect("office fixture must decode");
    let headlines_expected: OfficeHeadlineCollection =
        serde_json::from_str(HEADLINES).expect("headlines fixture must decode");
    let headline_expected: OfficeHeadline =
        serde_json::from_str(HEADLINE).expect("headline fixture must decode");
    let briefing_expected: OfficeBriefingResponse =
        serde_json::from_str(BRIEFING).expect("briefing fixture must decode");
    let stories_expected: OfficeWeatherStoryCollection =
        serde_json::from_str(WEATHER_STORIES).expect("weather stories fixture must decode");
    let (upstream, server) = server_with_router(NoaaWeatherServer::offices_router()).await;

    for (expected_path, body) in [
        ("/offices/PSR", OFFICE),
        ("/offices/PSR/headlines", HEADLINES),
        ("/offices/PSR/headlines/headline%20%2F%25%3F", HEADLINE),
        ("/offices/PSR/briefing", BRIEFING),
        ("/offices/PSR/weatherstories", WEATHER_STORIES),
    ] {
        Mock::given(method("GET"))
            .and(path(expected_path))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/ld+json"))
            .expect(1)
            .mount(&upstream)
            .await;
    }

    let office = call(&server, "offices_get", json!({"officeId": "psr"})).await;
    assert_json_success(&office, &office_expected);

    let headlines = call(&server, "offices_headlines", json!({"officeId": "psr"})).await;
    assert_json_success(&headlines, &headlines_expected);

    let headline = call(
        &server,
        "offices_headline_get",
        json!({"officeId": "psr", "headlineId": "headline /%?"}),
    )
    .await;
    assert_json_success(&headline, &headline_expected);

    let briefing = call(&server, "offices_briefing", json!({"officeId": "psr"})).await;
    assert_json_success(&briefing, &briefing_expected);
    assert_eq!(
        briefing
            .structured_content
            .as_ref()
            .and_then(|value| value.get("briefing")),
        Some(&Value::Null)
    );

    let stories = call(
        &server,
        "offices_weather_stories",
        json!({"officeId": "psr"}),
    )
    .await;
    assert_json_success(&stories, &stories_expected);

    upstream.verify().await;
    let requests = upstream
        .received_requests()
        .await
        .expect("recorded requests must be readable");
    assert_eq!(requests.len(), 5, "each tool must make exactly one request");
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
                "/offices/PSR".to_owned(),
                None,
                "application/ld+json".to_owned(),
            ),
            (
                "/offices/PSR/headlines".to_owned(),
                None,
                "application/ld+json".to_owned(),
            ),
            (
                "/offices/PSR/headlines/headline%20%2F%25%3F".to_owned(),
                None,
                "application/ld+json".to_owned(),
            ),
            (
                "/offices/PSR/briefing".to_owned(),
                None,
                "application/ld+json".to_owned(),
            ),
            (
                "/offices/PSR/weatherstories".to_owned(),
                None,
                "application/ld+json".to_owned(),
            ),
        ]
    );
}

#[tokio::test]
async fn invalid_typed_argument_is_rejected_before_http() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::offices_router()).await;
    let result = call(&server, "offices_get", json!({"officeId": "P!"})).await;

    assert_parameter_failure(&result);
    no_requests(&upstream).await;
}

#[tokio::test]
async fn upstream_failure_is_one_bounded_stable_tool_error() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::offices_router()).await;
    Mock::given(method("GET"))
        .and(path("/offices/PSR"))
        .respond_with(ResponseTemplate::new(503).set_body_string("office-upstream-secret"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(&server, "offices_get", json!({"officeId": "PSR"})).await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    assert!(!text.contains("office-upstream-secret"));
    let failure: Value = serde_json::from_str(text).expect("tool failure must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    upstream.verify().await;
}
