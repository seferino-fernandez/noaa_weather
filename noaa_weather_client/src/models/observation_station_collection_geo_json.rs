use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationStationCollectionGeoJson {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "features")]
    pub features: Vec<models::ObservationStationGeoJson>,
    #[serde(
        rename = "observationStations",
        skip_serializing_if = "Option::is_none"
    )]
    pub observation_stations: Option<Vec<String>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::PaginationInfo>>,
}

impl ObservationStationCollectionGeoJson {
    pub fn new(
        r#type: Type,
        features: Vec<models::ObservationStationGeoJson>,
    ) -> ObservationStationCollectionGeoJson {
        ObservationStationCollectionGeoJson {
            at_context: None,
            r#type,
            features,
            observation_stations: None,
            pagination: None,
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
