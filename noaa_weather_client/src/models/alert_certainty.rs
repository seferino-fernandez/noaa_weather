use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Certainty of the alert
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertCertainty {
    #[serde(rename = "Observed")]
    Observed,
    #[serde(rename = "Likely")]
    Likely,
    #[serde(rename = "Possible")]
    Possible,
    #[serde(rename = "Unlikely")]
    Unlikely,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl std::fmt::Display for AlertCertainty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Observed => write!(f, "Observed"),
            Self::Likely => write!(f, "Likely"),
            Self::Possible => write!(f, "Possible"),
            Self::Unlikely => write!(f, "Unlikely"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Default for AlertCertainty {
    fn default() -> AlertCertainty {
        Self::Observed
    }
}

impl FromStr for AlertCertainty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Observed" => Ok(AlertCertainty::Observed),
            "Likely" => Ok(AlertCertainty::Likely),
            "Possible" => Ok(AlertCertainty::Possible),
            "Unlikely" => Ok(AlertCertainty::Unlikely),
            "Unknown" => Ok(AlertCertainty::Unknown),
            _ => Err(format!("Invalid alert certainty: {}", s)),
        }
    }
}
