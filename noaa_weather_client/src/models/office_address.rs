use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeAddress {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    #[serde(rename = "streetAddress", skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(rename = "addressLocality", skip_serializing_if = "Option::is_none")]
    pub address_locality: Option<String>,
    #[serde(rename = "addressRegion", skip_serializing_if = "Option::is_none")]
    pub address_region: Option<String>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl OfficeAddress {
    pub fn new() -> OfficeAddress {
        OfficeAddress {
            at_type: None,
            street_address: None,
            address_locality: None,
            address_region: None,
            postal_code: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "PostalAddress")]
    PostalAddress,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::PostalAddress
    }
}
