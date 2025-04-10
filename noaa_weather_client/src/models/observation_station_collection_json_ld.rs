use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationStationCollectionJsonLd {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub at_graph: Option<Vec<models::ObservationStation>>,
    #[serde(
        rename = "observationStations",
        skip_serializing_if = "Option::is_none"
    )]
    pub observation_stations: Option<Vec<String>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::PaginationInfo>>,
}

impl ObservationStationCollectionJsonLd {
    pub fn new() -> ObservationStationCollectionJsonLd {
        ObservationStationCollectionJsonLd {
            at_context: None,
            at_graph: None,
            observation_stations: None,
            pagination: None,
        }
    }
}
