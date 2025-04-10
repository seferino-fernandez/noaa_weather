use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertAtomEntryAuthor {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AlertAtomEntryAuthor {
    pub fn new() -> AlertAtomEntryAuthor {
        AlertAtomEntryAuthor { name: None }
    }
}
