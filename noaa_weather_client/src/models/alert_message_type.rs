use serde::{Deserialize, Serialize};

/// Type of the alert message
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertMessageType {
    #[serde(rename = "Alert")]
    Alert,
    #[serde(rename = "Update")]
    Update,
    #[serde(rename = "Cancel")]
    Cancel,
    #[serde(rename = "Ack")]
    Ack,
    #[serde(rename = "Error")]
    Error,
}

impl std::fmt::Display for AlertMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Alert => write!(f, "Alert"),
            Self::Update => write!(f, "Update"),
            Self::Cancel => write!(f, "Cancel"),
            Self::Ack => write!(f, "Ack"),
            Self::Error => write!(f, "Error"),
        }
    }
}
