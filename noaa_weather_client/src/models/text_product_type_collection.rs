use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextProductTypeCollection {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub at_graph: Option<Vec<models::TextProductTypeCollectionGraphInner>>,
}

impl TextProductTypeCollection {
    pub fn new() -> TextProductTypeCollection {
        TextProductTypeCollection {
            at_context: None,
            at_graph: None,
        }
    }
}
