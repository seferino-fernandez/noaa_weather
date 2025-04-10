use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonLdContext {
    String(String),
    Array(Vec<JsonLdContextElement>),
    Object(Box<JsonLdContextObject>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonLdContextElement {
    String(String),
    Object(Box<JsonLdContextObject>),
}

/// Represents a complex JSON-LD context object with type definitions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonLdContextObject {
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "@vocab", skip_serializing_if = "Option::is_none")]
    pub vocab: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<TypeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<TypeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<QuantitativeValueDefinition>,
    #[serde(rename = "forecastGridData", skip_serializing_if = "Option::is_none")]
    pub forecast_grid_data: Option<TypeDefinition>,
    #[serde(rename = "forecastOffice", skip_serializing_if = "Option::is_none")]
    pub forecast_office: Option<TypeDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeometryDefinition>,
    #[serde(rename = "publicZone", skip_serializing_if = "Option::is_none")]
    pub public_zone: Option<TypeDefinition>,
    #[serde(rename = "unitCode", skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<TypeIdDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueDefinition>,
}

/// Represents a simple type definition with just @type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeDefinition {
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Represents a definition with both @id and @type
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeIdDefinition {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Represents the geometry definition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeometryDefinition {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Represents a quantitative value definition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuantitativeValueDefinition {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Represents a simple value definition
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueDefinition {
    #[serde(rename = "@id")]
    pub id: String,
}

impl Default for JsonLdContext {
    fn default() -> Self {
        JsonLdContext::Array(Vec::new())
    }
}
