use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// A Local Data Manager host serving one of NOAA's radar distribution queues.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum RadarQueueHost {
    /// The queue host represented as `tds` by the API.
    #[serde(rename = "tds")]
    Tds,
    /// The queue host represented as `rds` by the API.
    #[serde(rename = "rds")]
    Rds,
}

impl std::fmt::Display for RadarQueueHost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tds => write!(f, "tds"),
            Self::Rds => write!(f, "rds"),
        }
    }
}

impl FromStr for RadarQueueHost {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tds" => Ok(RadarQueueHost::Tds),
            "rds" => Ok(RadarQueueHost::Rds),
            _ => Err(format!("Invalid radar queue host: {s}")),
        }
    }
}
