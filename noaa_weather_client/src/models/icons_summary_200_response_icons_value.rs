use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconsSummary200ResponseIconsValue {
    #[serde(rename = "description")]
    pub description: String,
}

impl IconsSummary200ResponseIconsValue {
    pub fn new(description: String) -> IconsSummary200ResponseIconsValue {
        IconsSummary200ResponseIconsValue { description }
    }
}
