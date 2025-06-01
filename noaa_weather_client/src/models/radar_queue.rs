use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarQueue {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "arrivalTime", skip_serializing_if = "Option::is_none")]
    pub arrival_time: Option<String>,
    #[serde(rename = "creationTime", skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "feed", skip_serializing_if = "Option::is_none")]
    pub feed: Option<String>,
    #[serde(rename = "resolutionVersion", skip_serializing_if = "Option::is_none")]
    pub resolution_version: Option<i32>,
    #[serde(rename = "sequenceNumber", skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
