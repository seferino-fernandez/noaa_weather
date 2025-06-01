use serde::{Deserialize, Serialize};
use std::option::Option;

use super::JsonLdContext;

#[derive(Serialize, Deserialize)]
pub struct TerminalAerodromeForecastsResponse {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<JsonLdContext>>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub graph: Option<Vec<TerminalAerodromeForecastMetadata>>,
}

#[derive(Serialize, Deserialize)]
pub struct TerminalAerodromeForecastMetadata {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "issueTime", skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "geometry", skip_serializing_if = "Option::is_none")]
    pub geometry: Option<String>,
}
