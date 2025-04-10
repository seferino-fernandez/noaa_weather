use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextProductLocationCollection {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "locations", skip_serializing_if = "Option::is_none")]
    pub locations: Option<std::collections::HashMap<String, String>>,
}

impl TextProductLocationCollection {
    pub fn new() -> TextProductLocationCollection {
        TextProductLocationCollection {
            at_context: None,
            locations: None,
        }
    }
}
