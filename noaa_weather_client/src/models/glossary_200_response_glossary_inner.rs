use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Glossary200ResponseGlossaryInner {
    /// The term being defined
    #[serde(rename = "term", skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// A definition for the term
    #[serde(rename = "definition", skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}

impl Glossary200ResponseGlossaryInner {
    pub fn new() -> Glossary200ResponseGlossaryInner {
        Glossary200ResponseGlossaryInner {
            term: None,
            definition: None,
        }
    }
}
