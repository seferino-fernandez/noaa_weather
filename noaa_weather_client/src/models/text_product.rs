use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextProduct {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "wmoCollectiveId", skip_serializing_if = "Option::is_none")]
    pub wmo_collective_id: Option<String>,
    #[serde(rename = "issuingOffice", skip_serializing_if = "Option::is_none")]
    pub issuing_office: Option<String>,
    #[serde(rename = "issuanceTime", skip_serializing_if = "Option::is_none")]
    pub issuance_time: Option<String>,
    #[serde(rename = "productCode", skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    #[serde(rename = "productName", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "productText", skip_serializing_if = "Option::is_none")]
    pub product_text: Option<String>,
}

impl TextProduct {
    pub fn new() -> TextProduct {
        TextProduct {
            at_context: None,
            at_id: None,
            id: None,
            wmo_collective_id: None,
            issuing_office: None,
            issuance_time: None,
            product_code: None,
            product_name: None,
            product_text: None,
        }
    }
}
