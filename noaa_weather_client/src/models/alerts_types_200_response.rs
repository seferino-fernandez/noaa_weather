use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsTypes200Response {
    /// A list of recognized event types
    #[serde(rename = "eventTypes", skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
}

impl AlertsTypes200Response {
    pub fn new() -> AlertsTypes200Response {
        AlertsTypes200Response { event_types: None }
    }
}
