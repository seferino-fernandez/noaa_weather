use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointQuantitativeValueLayerValuesInner {
    #[serde(rename = "validTime")]
    pub valid_time: Box<models::Iso8601Interval>,
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<f64>,
}

impl GridpointQuantitativeValueLayerValuesInner {
    pub fn new(
        valid_time: models::Iso8601Interval,
        value: Option<f64>,
    ) -> GridpointQuantitativeValueLayerValuesInner {
        GridpointQuantitativeValueLayerValuesInner {
            valid_time: Box::new(valid_time),
            value,
        }
    }
}
