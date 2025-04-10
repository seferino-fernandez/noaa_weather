use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertJsonLd {
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub at_graph: Option<Vec<models::Alert>>,
}

impl AlertJsonLd {
    pub fn new() -> AlertJsonLd {
        AlertJsonLd { at_graph: None }
    }
}
