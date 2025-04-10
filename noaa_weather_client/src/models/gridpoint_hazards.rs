use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointHazards {
    #[serde(rename = "values")]
    pub values: Vec<models::GridpointHazardsValuesInner>,
}

impl GridpointHazards {
    pub fn new(values: Vec<models::GridpointHazardsValuesInner>) -> GridpointHazards {
        GridpointHazards { values }
    }
}
