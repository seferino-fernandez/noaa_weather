use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationCollectionGeoJsonAllOfFeatures {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::Observation>>,
}

impl ObservationCollectionGeoJsonAllOfFeatures {
    pub fn new() -> ObservationCollectionGeoJsonAllOfFeatures {
        ObservationCollectionGeoJsonAllOfFeatures { properties: None }
    }
}
