use serde::{Deserialize, Serialize};

/// GridpointForecastUnits : Denotes the units used in the textual portions of the forecast.
/// Denotes the units used in the textual portions of the forecast.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GridpointForecastUnits {
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "si")]
    Si,
}

impl std::fmt::Display for GridpointForecastUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Us => write!(f, "us"),
            Self::Si => write!(f, "si"),
        }
    }
}

impl Default for GridpointForecastUnits {
    fn default() -> GridpointForecastUnits {
        Self::Us
    }
}
