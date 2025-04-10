use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertStatus {
    #[serde(rename = "Actual")]
    Actual,
    #[serde(rename = "Exercise")]
    Exercise,
    #[serde(rename = "System")]
    System,
    #[serde(rename = "Test")]
    Test,
    #[serde(rename = "Draft")]
    Draft,
}

impl std::fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Actual => write!(f, "Actual"),
            Self::Exercise => write!(f, "Exercise"),
            Self::System => write!(f, "System"),
            Self::Test => write!(f, "Test"),
            Self::Draft => write!(f, "Draft"),
        }
    }
}
