use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CenterWeatherAdvisory {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "issueTime", skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<String>,
    #[serde(rename = "cwsu", skip_serializing_if = "Option::is_none")]
    pub cwsu: Option<models::NwsCenterWeatherServiceUnitId>,
    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "observedProperty", skip_serializing_if = "Option::is_none")]
    pub observed_property: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CenterWeatherAdvisory {
    pub fn new() -> CenterWeatherAdvisory {
        CenterWeatherAdvisory {
            id: None,
            issue_time: None,
            cwsu: None,
            sequence: None,
            start: None,
            end: None,
            observed_property: None,
            text: None,
        }
    }
}
