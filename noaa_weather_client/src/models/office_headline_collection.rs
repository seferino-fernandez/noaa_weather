use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfficeHeadlineCollection {
    #[serde(rename = "@context")]
    pub at_context: Box<models::JsonLdContext>,
    #[serde(rename = "@graph")]
    pub at_graph: Vec<models::OfficeHeadline>,
}

impl OfficeHeadlineCollection {
    pub fn new(
        at_context: models::JsonLdContext,
        at_graph: Vec<models::OfficeHeadline>,
    ) -> OfficeHeadlineCollection {
        OfficeHeadlineCollection {
            at_context: Box::new(at_context),
            at_graph,
        }
    }
}
