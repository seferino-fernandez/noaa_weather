use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertAtomFeedAuthor {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AlertAtomFeedAuthor {
    pub fn new() -> AlertAtomFeedAuthor {
        AlertAtomFeedAuthor { name: None }
    }
}
