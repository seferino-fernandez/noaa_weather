use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertCollectionGeoJsonAllOfFeatures {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::Alert>>,
}

impl AlertCollectionGeoJsonAllOfFeatures {
    pub fn new() -> AlertCollectionGeoJsonAllOfFeatures {
        AlertCollectionGeoJsonAllOfFeatures { properties: None }
    }
}
