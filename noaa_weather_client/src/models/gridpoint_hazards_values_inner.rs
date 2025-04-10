use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointHazardsValuesInner {
    #[serde(rename = "validTime")]
    pub valid_time: Box<models::Iso8601Interval>,
    #[serde(rename = "value")]
    pub value: Vec<models::GridpointHazardsValuesInnerValueInner>,
}

impl GridpointHazardsValuesInner {
    pub fn new(
        valid_time: models::Iso8601Interval,
        value: Vec<models::GridpointHazardsValuesInnerValueInner>,
    ) -> GridpointHazardsValuesInner {
        GridpointHazardsValuesInner {
            valid_time: Box::new(valid_time),
            value,
        }
    }
}
