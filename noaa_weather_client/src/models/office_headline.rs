use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeHeadline {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "office", skip_serializing_if = "Option::is_none")]
    pub office: Option<String>,
    #[serde(rename = "important", skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    #[serde(rename = "issuanceTime", skip_serializing_if = "Option::is_none")]
    pub issuance_time: Option<String>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(
        rename = "summary",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub summary: Option<Option<String>>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl OfficeHeadline {
    pub fn new() -> OfficeHeadline {
        OfficeHeadline {
            at_context: None,
            at_id: None,
            id: None,
            office: None,
            important: None,
            issuance_time: None,
            link: None,
            name: None,
            title: None,
            summary: None,
            content: None,
        }
    }
}
