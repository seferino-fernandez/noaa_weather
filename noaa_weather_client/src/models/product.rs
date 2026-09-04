//! Curated models for NWS text products and their catalogs.
//!
//! # Requiredness
//!
//! A live census on 2026-09-04 inspected 500 records from `/products`, all
//! 4,612 current Area Forecast Discussions from `/products/types/AFD`, and
//! 34 records from `/products/types/AFD/locations/LWX`. Every catalog record
//! carried the same seven non-null metadata fields. The 338 entries from
//! `/products/types` likewise always carried a code and name. Full-product
//! responses add `productText`; because NOAA reuses the same product shape
//! for catalog entries that omit it, that field remains optional.
//!
//! Product-location keys are deliberately strings rather than [`OfficeId`]:
//! `/products/locations` currently includes one- and two-character legacy
//! identifiers as well as office-like codes.
//! JSON-LD context is vocabulary metadata rather than product data and is not
//! part of these curated models.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ids::{OfficeId, ProductId, ProductTypeCode};
use crate::time::OffsetDateTime;

/// Metadata for one NWS text product, with the full text when requested from
/// a single-product endpoint.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TextProduct {
    /// The product's canonical API URL.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// Server-issued product identifier.
    pub id: ProductId,
    /// WMO abbreviated heading identifier.
    pub wmo_collective_id: String,
    /// Office that issued the product.
    pub issuing_office: OfficeId,
    /// When the product was issued.
    pub issuance_time: OffsetDateTime,
    /// Product type code, such as `AFD` or `RR3`.
    pub product_code: ProductTypeCode,
    /// Human-readable product type name.
    pub product_name: String,
    /// Full raw product text. Catalog endpoints omit this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_text: Option<String>,
}

/// A JSON-LD catalog of text products.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct TextProductCollection {
    /// Products in NOAA's response order.
    #[serde(rename = "@graph", default)]
    pub at_graph: Vec<TextProduct>,
}

/// Product issuance locations keyed by NOAA's location identifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct TextProductLocationCollection {
    /// Location identifier to display name. Legacy identifiers often have no
    /// corresponding name.
    #[serde(default)]
    pub locations: BTreeMap<String, Option<String>>,
}

/// One entry in the NWS text-product type catalog.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TextProductType {
    /// Product type code.
    pub product_code: ProductTypeCode,
    /// Human-readable product type name.
    pub product_name: String,
}

/// A JSON-LD catalog of text-product types.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct TextProductTypeCollection {
    /// Product types sorted as NOAA returned them.
    #[serde(rename = "@graph", default)]
    pub at_graph: Vec<TextProductType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRODUCT: &str = include_str!("../../tests/fixtures/products/product.json");
    const PRODUCTS: &str = include_str!("../../tests/fixtures/products/list.json");
    const LOCATIONS: &str = include_str!("../../tests/fixtures/products/locations.json");
    const TYPES: &str = include_str!("../../tests/fixtures/products/types.json");

    #[test]
    fn fixtures_decode_to_typed_product_values() {
        let product: TextProduct = serde_json::from_str(PRODUCT).unwrap();
        assert_eq!(product.id.as_str(), "dcfd78a0-561b-423e-9fd6-889455b8c535");
        assert_eq!(product.issuing_office.as_str(), "KTBW");
        assert_eq!(product.product_code.as_str(), "ABV");
        assert_eq!(
            product.issuance_time.to_string(),
            "2026-09-28T00:00:00+00:00"
        );
        assert!(
            product
                .product_text
                .as_deref()
                .is_some_and(|text| !text.is_empty())
        );

        let products: TextProductCollection = serde_json::from_str(PRODUCTS).unwrap();
        assert_eq!(products.at_graph.len(), 5);
        assert!(
            products
                .at_graph
                .iter()
                .all(|product| product.product_text.is_none())
        );
    }

    #[test]
    fn location_catalog_preserves_legacy_ids_and_null_names() {
        let locations: TextProductLocationCollection = serde_json::from_str(LOCATIONS).unwrap();
        assert_eq!(locations.locations.get("0"), Some(&None));
        assert_eq!(
            locations.locations.get("ABQ").and_then(Option::as_deref),
            Some("Albuquerque, NM")
        );
    }

    #[test]
    fn product_type_catalog_uses_validated_codes() {
        let types: TextProductTypeCollection = serde_json::from_str(TYPES).unwrap();
        assert_eq!(types.at_graph[0].product_code.as_str(), "ABV");
        assert_eq!(
            types.at_graph[0].product_name,
            "Rawinsonde Data Above 100 Millibars"
        );
    }

    #[test]
    fn required_product_metadata_cannot_disappear_silently() {
        let mut product: serde_json::Value = serde_json::from_str(PRODUCT).unwrap();
        product.as_object_mut().unwrap().remove("issuanceTime");
        assert!(serde_json::from_value::<TextProduct>(product).is_err());

        let mut product_type: serde_json::Value =
            serde_json::from_str(r#"{"productCode":"AFD","productName":"Discussion"}"#).unwrap();
        product_type.as_object_mut().unwrap().remove("productName");
        assert!(serde_json::from_value::<TextProductType>(product_type).is_err());
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn every_product_model_publishes_a_required_schema() {
        let product = schemars::schema_for!(TextProduct);
        let collection = schemars::schema_for!(TextProductCollection);
        let locations = schemars::schema_for!(TextProductLocationCollection);
        let product_type = schemars::schema_for!(TextProductType);
        let types = schemars::schema_for!(TextProductTypeCollection);

        let required = product.as_value()["required"].as_array().unwrap();
        for key in [
            "@id",
            "id",
            "wmoCollectiveId",
            "issuingOffice",
            "issuanceTime",
            "productCode",
            "productName",
        ] {
            assert!(
                required.iter().any(|value| value == key),
                "{key} is optional"
            );
        }
        assert_eq!(
            collection.as_value()["properties"]["@graph"]["type"],
            "array"
        );
        assert_eq!(
            locations.as_value()["properties"]["locations"]["type"],
            "object"
        );
        let type_required = product_type.as_value()["required"].as_array().unwrap();
        for key in ["productCode", "productName"] {
            assert!(
                type_required.iter().any(|value| value == key),
                "{key} is optional"
            );
        }
        assert_eq!(types.as_value()["properties"]["@graph"]["type"], "array");
    }
}
