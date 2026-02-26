use serde::{Deserialize, Serialize};

/// Whether the specific point is on land or marine.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PointType {
    #[serde(rename = "land")]
    Land,
    #[serde(rename = "marine")]
    Marine,
}

impl std::fmt::Display for PointType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Land => write!(f, "land"),
            Self::Marine => write!(f, "marine"),
        }
    }
}
