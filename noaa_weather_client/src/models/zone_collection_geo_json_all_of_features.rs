use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneCollectionGeoJsonAllOfFeatures {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::Zone>>,
}

impl ZoneCollectionGeoJsonAllOfFeatures {
    pub fn new() -> ZoneCollectionGeoJsonAllOfFeatures {
        ZoneCollectionGeoJsonAllOfFeatures { properties: None }
    }
}
