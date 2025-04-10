use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CenterWeatherAdvisoryCollectionGeoJsonAllOfFeatures {
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Box<models::CenterWeatherAdvisory>>,
}

impl CenterWeatherAdvisoryCollectionGeoJsonAllOfFeatures {
    pub fn new() -> CenterWeatherAdvisoryCollectionGeoJsonAllOfFeatures {
        CenterWeatherAdvisoryCollectionGeoJsonAllOfFeatures { properties: None }
    }
}
