use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconsSummary200Response {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "icons")]
    pub icons: std::collections::HashMap<String, models::IconsSummary200ResponseIconsValue>,
}

impl IconsSummary200Response {
    pub fn new(
        icons: std::collections::HashMap<String, models::IconsSummary200ResponseIconsValue>,
    ) -> IconsSummary200Response {
        IconsSummary200Response {
            at_context: None,
            icons,
        }
    }
}
