use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelativeLocation {
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "bearing", skip_serializing_if = "Option::is_none")]
    pub bearing: Option<Box<models::QuantitativeValue>>,
}

impl RelativeLocation {
    pub fn new() -> RelativeLocation {
        RelativeLocation {
            city: None,
            state: None,
            distance: None,
            bearing: None,
        }
    }
}
