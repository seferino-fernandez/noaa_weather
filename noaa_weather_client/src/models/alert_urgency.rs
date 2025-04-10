use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertUrgency {
    #[serde(rename = "Immediate")]
    Immediate,
    #[serde(rename = "Expected")]
    Expected,
    #[serde(rename = "Future")]
    Future,
    #[serde(rename = "Past")]
    Past,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl std::fmt::Display for AlertUrgency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Immediate => write!(f, "Immediate"),
            Self::Expected => write!(f, "Expected"),
            Self::Future => write!(f, "Future"),
            Self::Past => write!(f, "Past"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for AlertUrgency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Immediate" => Ok(AlertUrgency::Immediate),
            "Expected" => Ok(AlertUrgency::Expected),
            "Future" => Ok(AlertUrgency::Future),
            "Past" => Ok(AlertUrgency::Past),
            "Unknown" => Ok(AlertUrgency::Unknown),
            _ => Err(format!("Invalid alert urgency: {}", s)),
        }
    }
}
