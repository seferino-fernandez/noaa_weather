use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAddress {
    pub city: Option<String>,
    pub state: Option<String>,
    pub street: Option<String>,
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

impl OfficeAddress {
    pub fn new() -> OfficeAddress {
        OfficeAddress {
            city: None,
            state: None,
            street: None,
            zip_code: None,
        }
    }
}
