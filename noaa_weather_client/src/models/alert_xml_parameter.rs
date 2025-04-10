use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertXmlParameter {
    #[serde(rename = "valueName", skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl AlertXmlParameter {
    pub fn new() -> AlertXmlParameter {
        AlertXmlParameter {
            value_name: None,
            value: None,
        }
    }
}
