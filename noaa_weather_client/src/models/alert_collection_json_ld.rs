use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertCollectionJsonLd {
    /// A title describing the alert collection
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The last time a change occurred to this collection
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::PaginationInfo>>,
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub at_graph: Option<Vec<models::Alert>>,
}

impl AlertCollectionJsonLd {
    pub fn new() -> AlertCollectionJsonLd {
        AlertCollectionJsonLd {
            title: None,
            updated: None,
            pagination: None,
            at_context: None,
            at_graph: None,
        }
    }
}
