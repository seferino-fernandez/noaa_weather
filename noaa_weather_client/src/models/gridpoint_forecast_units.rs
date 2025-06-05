use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// GridpointForecastUnits : Denotes the units used in the textual portions of the forecast.
/// The International System of Units (`si`) and the United States Customary System (`us`) are the two systems of units.
/// The default units are `us`.
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

impl FromStr for GridpointForecastUnits {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "us" => Ok(Self::Us),
            "si" => Ok(Self::Si),
            _ => Err(format!("Invalid gridpoint forecast units: {string}")),
        }
    }
}
