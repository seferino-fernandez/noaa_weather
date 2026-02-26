use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationCollectionGeoJson {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "features")]
    pub features: Vec<models::ObservationGeoJson>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<models::PaginationInfo>,
}

impl ObservationCollectionGeoJson {
    pub fn new(
        r#type: Type,
        features: Vec<models::ObservationGeoJson>,
    ) -> ObservationCollectionGeoJson {
        ObservationCollectionGeoJson {
            at_context: None,
            r#type,
            features,
            pagination: None,
        }
    }
}
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum Type {
    #[serde(rename = "FeatureCollection")]
    #[default]
    FeatureCollection,
}
