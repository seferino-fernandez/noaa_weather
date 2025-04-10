use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelativeLocationJsonLd {
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "bearing", skip_serializing_if = "Option::is_none")]
    pub bearing: Option<Box<models::QuantitativeValue>>,
    /// A geometry represented in Well-Known Text (WKT) format.
    #[serde(rename = "geometry", deserialize_with = "Option::deserialize")]
    pub geometry: Option<String>,
}

impl RelativeLocationJsonLd {
    pub fn new(geometry: Option<String>) -> RelativeLocationJsonLd {
        RelativeLocationJsonLd {
            city: None,
            state: None,
            distance: None,
            bearing: None,
            geometry,
        }
    }
}
