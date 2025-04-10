use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneCollectionGeoJson {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "features")]
    pub features: Vec<models::ZoneCollectionGeoJsonAllOfFeatures>,
}

impl ZoneCollectionGeoJson {
    pub fn new(
        r#type: Type,
        features: Vec<models::ZoneCollectionGeoJsonAllOfFeatures>,
    ) -> ZoneCollectionGeoJson {
        ZoneCollectionGeoJson {
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
