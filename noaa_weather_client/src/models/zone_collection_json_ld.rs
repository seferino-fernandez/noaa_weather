use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneCollectionJsonLd {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub at_graph: Option<Vec<models::Zone>>,
}

impl ZoneCollectionJsonLd {
    pub fn new() -> Self {
        Self {
            at_context: None,
            at_graph: None,
        }
    }
}
