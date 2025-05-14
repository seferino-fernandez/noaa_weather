use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAddress {
    #[serde(alias = "addressLocality", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(alias = "addressRegion", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(alias = "streetAddress", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(alias = "postalCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

impl OfficeAddress {
    pub fn new() -> OfficeAddress {
        OfficeAddress {
            city: None,
            state: None,
            street_address: None,
            zip_code: None,
        }
    }
}
