use crate::models;
use serde::{Deserialize, Serialize};

/// GeoJsonFeatureCollection : A GeoJSON feature collection. Please refer to IETF RFC 7946 for information on the GeoJSON format.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoJsonFeatureCollection {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "features")]
    pub features: Vec<models::GeoJsonFeature>,
}

impl GeoJsonFeatureCollection {
    /// A GeoJSON feature collection. Please refer to IETF RFC 7946 for information on the GeoJSON format.
    pub fn new(r#type: Type, features: Vec<models::GeoJsonFeature>) -> GeoJsonFeatureCollection {
        GeoJsonFeatureCollection {
            at_context: None,
            r#type,
            features,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FeatureCollection")]
    FeatureCollection,
}

impl Default for Type {
    fn default() -> Type {
        Self::FeatureCollection
    }
}
