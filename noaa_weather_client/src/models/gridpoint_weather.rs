use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointWeather {
    #[serde(rename = "values")]
    pub values: Vec<models::GridpointWeatherValuesInner>,
}

impl GridpointWeather {
    pub fn new(values: Vec<models::GridpointWeatherValuesInner>) -> GridpointWeather {
        GridpointWeather { values }
    }
}
