use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertCollection {
    /// A title describing the alert collection
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The last time a change occurred to this collection
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<models::PaginationInfo>>,
}

impl AlertCollection {
    pub fn new() -> AlertCollection {
        AlertCollection {
            title: None,
            updated: None,
            pagination: None,
        }
    }
}
