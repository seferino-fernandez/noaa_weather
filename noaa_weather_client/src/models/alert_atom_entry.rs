use crate::models;
use serde::{Deserialize, Serialize};

/// AlertAtomEntry : An alert entry in an Atom feed
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertAtomEntry {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "published", skip_serializing_if = "Option::is_none")]
    pub published: Option<String>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::AlertAtomEntryAuthor>>,
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename = "sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
    #[serde(rename = "effective", skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "msgType", skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "urgency", skip_serializing_if = "Option::is_none")]
    pub urgency: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "certainty", skip_serializing_if = "Option::is_none")]
    pub certainty: Option<String>,
    #[serde(rename = "areaDesc", skip_serializing_if = "Option::is_none")]
    pub area_desc: Option<String>,
    #[serde(rename = "polygon", skip_serializing_if = "Option::is_none")]
    pub polygon: Option<String>,
    #[serde(rename = "geocode", skip_serializing_if = "Option::is_none")]
    pub geocode: Option<Vec<models::AlertXmlParameter>>,
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<Vec<models::AlertXmlParameter>>,
}

impl AlertAtomEntry {
    /// An alert entry in an Atom feed
    pub fn new() -> AlertAtomEntry {
        AlertAtomEntry {
            id: None,
            updated: None,
            published: None,
            author: None,
            summary: None,
            event: None,
            sent: None,
            effective: None,
            expires: None,
            status: None,
            msg_type: None,
            category: None,
            urgency: None,
            severity: None,
            certainty: None,
            area_desc: None,
            polygon: None,
            geocode: None,
            parameter: None,
        }
    }
}
