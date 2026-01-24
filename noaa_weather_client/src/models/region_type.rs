use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Region type.
///  - Land: Land
///  - Marine: Marine
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum RegionType {
    #[serde(rename = "Land")]
    #[default]
    Land,
    #[serde(rename = "Marine")]
    Marine,
}

impl std::fmt::Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Land => write!(f, "land"),
            Self::Marine => write!(f, "marine"),
        }
    }
}


impl FromStr for RegionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "land" => Ok(RegionType::Land),
            "marine" => Ok(RegionType::Marine),
            _ => Err(format!("Invalid region type: {s}")),
        }
    }
}
