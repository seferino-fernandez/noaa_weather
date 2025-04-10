use crate::models;
use serde::{Deserialize, Serialize};

/// AlertAtomFeed : An alert feed in Atom format
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertAtomFeed {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "generator", skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::AlertAtomFeedAuthor>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "entry", skip_serializing_if = "Option::is_none")]
    pub entry: Option<Vec<models::AlertAtomEntry>>,
}

impl AlertAtomFeed {
    /// An alert feed in Atom format
    pub fn new() -> AlertAtomFeed {
        AlertAtomFeed {
            id: None,
            generator: None,
            updated: None,
            author: None,
            title: None,
            entry: None,
        }
    }
}
