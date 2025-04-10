use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationStationCollectionGeoJsonAllOfFeatures {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::ObservationStation>>,
}

impl ObservationStationCollectionGeoJsonAllOfFeatures {
    pub fn new() -> ObservationStationCollectionGeoJsonAllOfFeatures {
        ObservationStationCollectionGeoJsonAllOfFeatures { properties: None }
    }
}
