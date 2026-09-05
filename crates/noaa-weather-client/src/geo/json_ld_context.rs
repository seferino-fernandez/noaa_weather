use serde::{Deserialize, Serialize};

/// Represents any JSON-LD context shape emitted by NOAA.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonLdContext {
    /// Contains a context URI or compact context name.
    String(String),
    /// Contains an ordered collection of context entries.
    Array(Vec<JsonLdContextElement>),
    /// Contains an inline context definition.
    Object(Box<JsonLdContextObject>),
}

/// Represents one entry in a JSON-LD context array.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonLdContextElement {
    /// Contains a context URI or compact context name.
    String(String),
    /// Contains an inline context definition.
    Object(Box<JsonLdContextObject>),
}

/// Defines JSON-LD prefixes and NOAA term-expansion rules inline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonLdContextObject {
    /// Specifies the JSON-LD processing version.
    #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Specifies the default vocabulary URI used to expand unprefixed terms.
    #[serde(rename = "@vocab", skip_serializing_if = "Option::is_none")]
    pub vocab: Option<String>,
    /// Binds the `wx` prefix to the NOAA weather vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx: Option<String>,

    /// Maps the `city` term to its address-locality property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Binds the `geo` prefix to the GeoSPARQL vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    /// Binds the `s` prefix to the Schema.org vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    /// Maps the `state` term to its address-region property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Binds the `unit` prefix to the WMO common-unit vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// URI for WMO unit definitions.
    #[serde(rename = "wmoUnit", skip_serializing_if = "Option::is_none")]
    pub wmo_unit: Option<String>,
    /// URI for NWS unit definitions.
    #[serde(rename = "nwsUnit", skip_serializing_if = "Option::is_none")]
    pub nws_unit: Option<String>,

    /// Defines the JSON-LD type of the `bearing` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearing: Option<TypeDefinition>,
    /// Defines the JSON-LD type of the `county` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<TypeDefinition>,
    /// Defines the identifier and type used by the `distance` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<QuantitativeValueDefinition>,
    /// Defines the JSON-LD type of the `forecastGridData` property.
    #[serde(rename = "forecastGridData", skip_serializing_if = "Option::is_none")]
    pub forecast_grid_data: Option<TypeDefinition>,
    /// Defines the JSON-LD type of the `forecastOffice` property.
    #[serde(rename = "forecastOffice", skip_serializing_if = "Option::is_none")]
    pub forecast_office: Option<TypeDefinition>,
    /// Defines the identifier and type used by the `geometry` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeometryDefinition>,
    /// Defines the JSON-LD type of the `publicZone` property.
    #[serde(rename = "publicZone", skip_serializing_if = "Option::is_none")]
    pub public_zone: Option<TypeDefinition>,
    /// Defines the identifier and type used by the `unitCode` property.
    #[serde(rename = "unitCode", skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<TypeIdDefinition>,
    /// Defines the identifier used by the `value` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueDefinition>,
}

/// Defines a JSON-LD term by its `@type` mapping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeDefinition {
    /// Identifies the JSON-LD type associated with the term.
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Defines a JSON-LD term by its `@id` and `@type` mappings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeIdDefinition {
    /// Identifies the vocabulary term or property.
    #[serde(rename = "@id")]
    pub id: String,
    /// Identifies the JSON-LD type associated with the term.
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Defines the JSON-LD identifier and type of a GeoJSON geometry value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeometryDefinition {
    /// Identifies the vocabulary property represented by the geometry.
    #[serde(rename = "@id")]
    pub id: String,
    /// Identifies the JSON-LD type of the geometry value.
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Defines the JSON-LD identifier and type of a quantitative value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuantitativeValueDefinition {
    /// Identifies the vocabulary property represented by the quantity.
    #[serde(rename = "@id")]
    pub id: String,
    /// Identifies the JSON-LD type of the quantity.
    #[serde(rename = "@type")]
    pub type_: String,
}

/// Defines a JSON-LD value property by its vocabulary identifier.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueDefinition {
    /// Identifies the vocabulary property represented by the value.
    #[serde(rename = "@id")]
    pub id: String,
}

impl Default for JsonLdContext {
    fn default() -> Self {
        JsonLdContext::Array(Vec::new())
    }
}
