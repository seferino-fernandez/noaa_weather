use std::fmt::{self, Display};
use std::str::FromStr;

use crate::geo::{LandRegionCode, MarineRegionCode};
use serde::{Deserialize, Serialize};

/// Identifies either an NWS land region or a grouped marine region.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegionCode {
    /// Contains a [`LandRegionCode`] for one of the six NWS regions.
    Land(LandRegionCode),
    /// Contains a [`MarineRegionCode`] grouping related marine forecast areas.
    Marine(MarineRegionCode),
}

impl Display for RegionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionCode::Land(code) => write!(f, "{code}"),
            RegionCode::Marine(code) => write!(f, "{code}"),
        }
    }
}

impl FromStr for RegionCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LandRegionCode::from_str(s)
            .map(RegionCode::Land)
            .or_else(|_| MarineRegionCode::from_str(s).map(RegionCode::Marine))
            .map_err(|_| format!("Invalid region code: {s}"))
    }
}
