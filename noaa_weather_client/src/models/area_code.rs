use std::fmt::{self, Display};

use crate::models;
use serde::{Deserialize, Serialize};

/// AreaCode : State/territory codes and marine area codes
/// State/territory codes and marine area codes
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AreaCode {
    StateTerritoryCode(models::StateTerritoryCode),
    MarineAreaCode(models::MarineAreaCode),
}

impl Display for AreaCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AreaCode::StateTerritoryCode(code) => write!(f, "{code}"),
            AreaCode::MarineAreaCode(code) => write!(f, "{code}"),
        }
    }
}
