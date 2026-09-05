use std::fmt::{self, Display};
use std::str::FromStr;

use crate::geo::{MarineAreaCode, StateTerritoryCode};
use serde::{Deserialize, Serialize};

/// Identifies either a state or territory, or an NWS marine forecast area.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AreaCode {
    /// Contains a [`StateTerritoryCode`] for a state or territory.
    StateTerritoryCode(StateTerritoryCode),
    /// Contains a [`MarineAreaCode`] for an NWS marine forecast area.
    MarineAreaCode(MarineAreaCode),
}

impl Display for AreaCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AreaCode::StateTerritoryCode(code) => write!(f, "{code}"),
            AreaCode::MarineAreaCode(code) => write!(f, "{code}"),
        }
    }
}

impl FromStr for AreaCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        StateTerritoryCode::from_str(s)
            .map(AreaCode::StateTerritoryCode)
            .or_else(|_| MarineAreaCode::from_str(s).map(AreaCode::MarineAreaCode))
    }
}
