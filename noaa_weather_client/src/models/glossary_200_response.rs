use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Glossary200Response {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    /// A list of glossary terms
    #[serde(rename = "glossary", skip_serializing_if = "Option::is_none")]
    pub glossary: Option<Vec<models::Glossary200ResponseGlossaryInner>>,
}

impl Glossary200Response {
    pub fn new() -> Glossary200Response {
        Glossary200Response {
            at_context: None,
            glossary: None,
        }
    }
}
