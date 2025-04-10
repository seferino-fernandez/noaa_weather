use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertReferencesInner {
    /// An API link to the prior alert.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// The identifier of the alert message.
    #[serde(rename = "identifier", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The sender of the prior alert.
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// The time the prior alert was sent.
    #[serde(rename = "sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
}

impl AlertReferencesInner {
    pub fn new() -> AlertReferencesInner {
        AlertReferencesInner {
            at_id: None,
            identifier: None,
            sender: None,
            sent: None,
        }
    }
}
