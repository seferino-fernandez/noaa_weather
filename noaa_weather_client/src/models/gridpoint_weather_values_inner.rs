use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointWeatherValuesInner {
    #[serde(rename = "validTime")]
    pub valid_time: Box<models::Iso8601Interval>,
    #[serde(rename = "value")]
    pub value: Vec<models::GridpointWeatherValuesInnerValueInner>,
}

impl GridpointWeatherValuesInner {
    pub fn new(
        valid_time: models::Iso8601Interval,
        value: Vec<models::GridpointWeatherValuesInnerValueInner>,
    ) -> GridpointWeatherValuesInner {
        GridpointWeatherValuesInner {
            valid_time: Box::new(valid_time),
            value,
        }
    }
}
