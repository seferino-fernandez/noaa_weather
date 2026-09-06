use std::collections::BTreeSet;

use noaa_weather_client::products::{
    TextProduct, TextProductCollection, TextProductLocationCollection, TextProductTypeCollection,
};
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

const LIST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/list.json"
));
const PRODUCT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/product.json"
));
const LOCATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/locations.json"
));
const TYPES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/types.json"
));
const LOCATION_TYPES: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/location_types.json"
));
const TYPE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/type.json"
));
const TYPE_LOCATION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/type_location.json"
));
const TYPE_LOCATIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/type_locations.json"
));
const LATEST: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../noaa-weather-client/tests/fixtures/products/latest.json"
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

fn properties(tool: &Tool) -> BTreeSet<&str> {
    tool.input_schema["properties"]
        .as_object()
        .expect("properties must be an object")
        .keys()
        .map(String::as_str)
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
    assert_eq!(
        names(tool.input_schema.get("required")),
        expected_required.iter().copied().collect()
    );
    assert_eq!(tool.output_schema.as_ref(), Some(&schema_for_output::<T>()));
    assert_metadata(tool, title);
}

async fn mount(upstream: &MockServer, route: &str, fixture: &'static str) {
    Mock::given(method("GET"))
        .and(path(route))
        .and(header("Accept", "application/ld+json"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(fixture, "application/ld+json"))
        .expect(1)
        .mount(upstream)
        .await;
}

#[test]
fn router_publishes_exact_flat_typed_product_contracts() {
    let tools = NoaaWeatherServer::products_router().list_all();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_ref())
            .collect::<Vec<_>>(),
        [
            "products_by_type",
            "products_by_type_and_location",
            "products_get",
            "products_latest",
            "products_locations",
            "products_locations_for_type",
            "products_search",
            "products_types",
            "products_types_for_location",
        ]
    );

    assert_contract::<TextProductCollection>(
        &tools,
        "products_search",
        &[
            "locationIds",
            "start",
            "end",
            "officeIds",
            "wmoIds",
            "productTypeCodes",
            "limit",
        ],
        &[],
        "Search NWS Text Products",
    );
    assert_contract::<TextProduct>(
        &tools,
        "products_get",
        &["productId"],
        &["productId"],
        "Get NWS Text Product",
    );
    assert_contract::<TextProductLocationCollection>(
        &tools,
        "products_locations",
        &[],
        &[],
        "List NWS Product Locations",
    );
    assert_contract::<TextProductTypeCollection>(
        &tools,
        "products_types",
        &[],
        &[],
        "List NWS Product Types",
    );
    assert_contract::<TextProductTypeCollection>(
        &tools,
        "products_types_for_location",
        &["locationId"],
        &["locationId"],
        "List Product Types for Location",
    );
    assert_contract::<TextProductCollection>(
        &tools,
        "products_by_type",
        &["productTypeCode"],
        &["productTypeCode"],
        "List NWS Products by Type",
    );
    assert_contract::<TextProductCollection>(
        &tools,
        "products_by_type_and_location",
        &["productTypeCode", "locationId"],
        &["productTypeCode", "locationId"],
        "List Products by Type and Location",
    );
    assert_contract::<TextProductLocationCollection>(
        &tools,
        "products_locations_for_type",
        &["productTypeCode"],
        &["productTypeCode"],
        "List Locations for Product Type",
    );
    assert_contract::<TextProduct>(
        &tools,
        "products_latest",
        &["productTypeCode", "locationId"],
        &["productTypeCode", "locationId"],
        "Get Latest NWS Text Product",
    );
}

#[tokio::test]
async fn every_product_tool_preserves_route_media_and_typed_json_parity() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::products_router()).await;
    mount(&upstream, "/products", LIST).await;
    mount(
        &upstream,
        "/products/dcfd78a0-561b-423e-9fd6-889455b8c535",
        PRODUCT,
    )
    .await;
    mount(&upstream, "/products/locations", LOCATIONS).await;
    mount(&upstream, "/products/types", TYPES).await;
    mount(&upstream, "/products/locations/LWX/types", LOCATION_TYPES).await;
    mount(&upstream, "/products/types/AFD", TYPE).await;
    mount(
        &upstream,
        "/products/types/AFD/locations/LWX",
        TYPE_LOCATION,
    )
    .await;
    mount(&upstream, "/products/types/AFD/locations", TYPE_LOCATIONS).await;
    mount(
        &upstream,
        "/products/types/AFD/locations/PSR/latest",
        LATEST,
    )
    .await;

    let result = call(
        &server,
        "products_search",
        json!({
            "locationIds": ["lwx", "PQR"],
            "start": "2026-08-30T00:00:00Z",
            "end": "2026-08-30T12:00:00Z",
            "officeIds": ["LWX"],
            "wmoIds": ["TTAA 00", "TT/BB%"],
            "productTypeCodes": ["afd", "HWO"],
            "limit": 5
        }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProductCollection>(LIST).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "products_get",
        json!({ "productId": "dcfd78a0-561b-423e-9fd6-889455b8c535" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProduct>(PRODUCT).expect("fixture must decode"),
    );

    let result = call(&server, "products_locations", json!({})).await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProductLocationCollection>(LOCATIONS)
            .expect("fixture must decode"),
    );

    let result = call(&server, "products_types", json!({})).await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProductTypeCollection>(TYPES).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "products_types_for_location",
        json!({ "locationId": "lwx" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProductTypeCollection>(LOCATION_TYPES)
            .expect("fixture must decode"),
    );

    let result = call(
        &server,
        "products_by_type",
        json!({ "productTypeCode": "afd" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProductCollection>(TYPE).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "products_by_type_and_location",
        json!({ "productTypeCode": "afd", "locationId": "lwx" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProductCollection>(TYPE_LOCATION).expect("fixture must decode"),
    );

    let result = call(
        &server,
        "products_locations_for_type",
        json!({ "productTypeCode": "afd" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProductLocationCollection>(TYPE_LOCATIONS)
            .expect("fixture must decode"),
    );

    let result = call(
        &server,
        "products_latest",
        json!({ "productTypeCode": "afd", "locationId": "psr" }),
    )
    .await;
    assert_json_success(
        &result,
        &serde_json::from_str::<TextProduct>(LATEST).expect("fixture must decode"),
    );

    let requests = upstream
        .received_requests()
        .await
        .expect("requests must be readable");
    assert_eq!(requests.len(), 9);
    assert_eq!(
        requests
            .iter()
            .map(|request| request.url.path())
            .collect::<Vec<_>>(),
        [
            "/products",
            "/products/dcfd78a0-561b-423e-9fd6-889455b8c535",
            "/products/locations",
            "/products/types",
            "/products/locations/LWX/types",
            "/products/types/AFD",
            "/products/types/AFD/locations/LWX",
            "/products/types/AFD/locations",
            "/products/types/AFD/locations/PSR/latest",
        ]
    );
    assert_eq!(
        requests[0].url.query(),
        Some(
            "location=LWX%2CPQR&start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T12%3A00%3A00Z\
             &office=LWX&wmoid=TTAA+00%2CTT%2FBB%25&type=AFD%2CHWO&limit=5"
        )
    );
    assert!(
        requests[1..]
            .iter()
            .all(|request| request.url.query().is_none())
    );
}

#[tokio::test]
async fn invalid_product_id_is_rejected_before_http() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::products_router()).await;
    let result = call(&server, "products_get", json!({ "productId": "abc_def" })).await;

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
async fn product_upstream_failure_is_bounded_json_without_body_leakage() {
    let (upstream, server) = server_with_router(NoaaWeatherServer::products_router()).await;
    Mock::given(method("GET"))
        .and(path("/products/known-product"))
        .respond_with(ResponseTemplate::new(503).set_body_string("private product response body"))
        .expect(1)
        .mount(&upstream)
        .await;

    let result = call(
        &server,
        "products_get",
        json!({ "productId": "known-product" }),
    )
    .await;
    assert_eq!(result.is_error, Some(true));
    assert!(result.structured_content.is_none());
    assert_eq!(result.content.len(), 1);
    let text = result_text(&result);
    assert!(text.len() <= 4_096);
    let failure: Value = serde_json::from_str(text).expect("failure must be JSON");
    assert_eq!(failure["code"], "upstream_http_error");
    assert_eq!(failure["status"], 503);
    assert!(!text.contains("private product response body"));
}
