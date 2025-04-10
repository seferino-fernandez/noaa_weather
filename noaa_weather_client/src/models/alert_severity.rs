use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Severity of the alert
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertSeverity {
    #[serde(rename = "Extreme")]
    Extreme,
    #[serde(rename = "Severe")]
    Severe,
    #[serde(rename = "Moderate")]
    Moderate,
    #[serde(rename = "Minor")]
    Minor,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Extreme => write!(f, "Extreme"),
            Self::Severe => write!(f, "Severe"),
            Self::Moderate => write!(f, "Moderate"),
            Self::Minor => write!(f, "Minor"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for AlertSeverity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Extreme" => Ok(AlertSeverity::Extreme),
            "Severe" => Ok(AlertSeverity::Severe),
            "Moderate" => Ok(AlertSeverity::Moderate),
            "Minor" => Ok(AlertSeverity::Minor),
            "Unknown" => Ok(AlertSeverity::Unknown),
            _ => Err(format!("Invalid alert severity: {}", s)),
        }
    }
}
