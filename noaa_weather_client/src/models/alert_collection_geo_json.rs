use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertCollectionGeoJson {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "features")]
    pub features: Vec<models::AlertCollectionGeoJsonAllOfFeatures>,
    /// A title describing the alert collection
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The last time a change occurred to this collection
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::PaginationInfo>>,
}

impl AlertCollectionGeoJson {
    pub fn new(
        r#type: Type,
        features: Vec<models::AlertCollectionGeoJsonAllOfFeatures>,
    ) -> AlertCollectionGeoJson {
        AlertCollectionGeoJson {
            at_context: None,
            r#type,
            features,
            title: None,
            updated: None,
            pagination: None,
        }
    }
}
/// Type of the alert collection
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
