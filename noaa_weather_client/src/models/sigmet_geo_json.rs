use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SigmetGeoJson {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "geometry", deserialize_with = "Option::deserialize")]
    pub geometry: Option<Box<models::GeoJsonGeometry>>,
    #[serde(rename = "properties")]
    pub properties: Box<models::Sigmet>,
}

impl SigmetGeoJson {
    pub fn new(
        r#type: Type,
        geometry: Option<models::GeoJsonGeometry>,
        properties: models::Sigmet,
    ) -> SigmetGeoJson {
        SigmetGeoJson {
            at_context: None,
            id: None,
            r#type,
            geometry: geometry.map(Box::new),
            properties: Box::new(properties),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Feature")]
    Feature,
}

impl Default for Type {
    fn default() -> Type {
        Self::Feature
    }
}
