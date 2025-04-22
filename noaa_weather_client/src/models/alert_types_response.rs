use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertTypesResponse {
    /// A list of recognized event types
    #[serde(rename = "eventTypes", skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
}

impl AlertTypesResponse {
    pub fn new() -> AlertTypesResponse {
        AlertTypesResponse { event_types: None }
    }
}
