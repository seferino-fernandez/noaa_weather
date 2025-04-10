use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// MarineRegionCode : Marine region code. These are groups of marine areas combined.
///     * AL: Alaska waters (PK)
///     * AT: Atlantic Ocean (AM, AN)
///     * GL: Great Lakes (LC, LE, LH, LM, LO, LS, SL)
///     * GM: Gulf of Mexico (GM)
///     * PA: Eastern Pacific Ocean and U.S. West Coast (PZ)
///     * PI: Central and Western Pacific (PH, PM, PS)
///
/// Marine region code. These are groups of marine areas combined. * AL: Alaska waters (PK) * AT: Atlantic Ocean (AM, AN) * GL: Great Lakes (LC, LE, LH, LM, LO, LS, SL) * GM: Gulf of Mexico (GM) * PA: Eastern Pacific Ocean and U.S. West Coast (PZ) * PI: Central and Western Pacific (PH, PM, PS)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MarineRegionCode {
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PI")]
    Pi,
}

impl std::fmt::Display for MarineRegionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Al => write!(f, "AL"),
            Self::At => write!(f, "AT"),
            Self::Gl => write!(f, "GL"),
            Self::Gm => write!(f, "GM"),
            Self::Pa => write!(f, "PA"),
            Self::Pi => write!(f, "PI"),
        }
    }
}

impl Default for MarineRegionCode {
    fn default() -> MarineRegionCode {
        Self::Al
    }
}

impl FromStr for MarineRegionCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AL" => Ok(MarineRegionCode::Al),
            "AT" => Ok(MarineRegionCode::At),
            "GL" => Ok(MarineRegionCode::Gl),
            "GM" => Ok(MarineRegionCode::Gm),
            "PA" => Ok(MarineRegionCode::Pa),
            "PI" => Ok(MarineRegionCode::Pi),
            _ => Err(format!("Invalid marine region code: {}", s)),
        }
    }
}
