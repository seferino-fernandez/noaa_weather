//! Product summaries against every captured response shape.

use noaa_weather_client::models::{
    TextProduct, TextProductCollection, TextProductLocationCollection, TextProductTypeCollection,
};
use noaa_weather_summary::{Section, Summarize, SummaryOptions, coverage_gaps};

const PRODUCT: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/products/product.json");
const PRODUCTS: &str = include_str!("../../noaa_weather_client/tests/fixtures/products/list.json");
const LOCATIONS: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/products/locations.json");
const TYPES: &str = include_str!("../../noaa_weather_client/tests/fixtures/products/types.json");

fn decode<T: serde::de::DeserializeOwned>(source: &str) -> T {
    serde_json::from_str(source).expect("product fixture decodes")
}

#[test]
fn product_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<TextProduct>(PRODUCT).summarize(&SummaryOptions::default())
    );
}

#[test]
fn product_collection_summary_snapshot() {
    let mut products = decode::<TextProductCollection>(PRODUCTS);
    products.at_graph.truncate(2);
    insta::assert_yaml_snapshot!(products.summarize(&SummaryOptions::default()));
}

#[test]
fn location_collection_summary_snapshot() {
    let mut locations = decode::<TextProductLocationCollection>(LOCATIONS);
    locations
        .locations
        .retain(|id, _| matches!(id.as_str(), "0" | "ABQ" | "PSR"));
    insta::assert_yaml_snapshot!(locations.summarize(&SummaryOptions::default()));
}

#[test]
fn type_collection_summary_snapshot() {
    let mut types = decode::<TextProductTypeCollection>(TYPES);
    types.at_graph.truncate(3);
    insta::assert_yaml_snapshot!(types.summarize(&SummaryOptions::default()));
}

#[test]
fn every_product_endpoint_fixture_has_complete_summary_coverage() {
    for source in [
        include_str!("../../noaa_weather_client/tests/fixtures/products/list.json"),
        include_str!("../../noaa_weather_client/tests/fixtures/products/type.json"),
        include_str!("../../noaa_weather_client/tests/fixtures/products/type_location.json"),
    ] {
        assert_no_gaps(&decode::<TextProductCollection>(source));
    }
    for source in [
        include_str!("../../noaa_weather_client/tests/fixtures/products/product.json"),
        include_str!("../../noaa_weather_client/tests/fixtures/products/latest.json"),
    ] {
        assert_no_gaps(&decode::<TextProduct>(source));
    }
    for source in [
        include_str!("../../noaa_weather_client/tests/fixtures/products/locations.json"),
        include_str!("../../noaa_weather_client/tests/fixtures/products/type_locations.json"),
    ] {
        assert_no_gaps(&decode::<TextProductLocationCollection>(source));
    }
    for source in [
        include_str!("../../noaa_weather_client/tests/fixtures/products/types.json"),
        include_str!("../../noaa_weather_client/tests/fixtures/products/location_types.json"),
    ] {
        assert_no_gaps(&decode::<TextProductTypeCollection>(source));
    }
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn empty_catalogs_explain_why_there_are_no_rows() {
    let empty_products: TextProductCollection = decode(r#"{"@graph":[]}"#);
    let empty_locations: TextProductLocationCollection = decode(r#"{"locations":{}}"#);
    let empty_types: TextProductTypeCollection = decode(r#"{"@graph":[]}"#);

    for summary in [
        empty_products.summarize(&SummaryOptions::default()),
        empty_locations.summarize(&SummaryOptions::default()),
        empty_types.summarize(&SummaryOptions::default()),
    ] {
        assert!(
            summary
                .sections
                .iter()
                .any(|section| matches!(section, Section::Empty { .. }))
        );
    }
}

#[test]
fn missing_product_text_is_reported_as_empty_content() {
    let mut product = decode::<TextProduct>(PRODUCT);
    product.product_text = None;
    let summary = product.summarize(&SummaryOptions::default());
    assert!(summary.sections.iter().any(|section| matches!(
        section,
        Section::Empty {
            key: Some("productText"),
            ..
        }
    )));
}
