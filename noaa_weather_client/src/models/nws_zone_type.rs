use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// The type of NWS zone.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NwsZoneType {
    #[serde(rename = "land")]
    Land,
    #[serde(rename = "marine")]
    Marine,
    #[serde(rename = "forecast")]
    Forecast,
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "coastal")]
    Coastal,
    #[serde(rename = "offshore")]
    Offshore,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "county")]
    County,
}

impl std::fmt::Display for NwsZoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Land => write!(f, "land"),
            Self::Marine => write!(f, "marine"),
            Self::Forecast => write!(f, "forecast"),
            Self::Public => write!(f, "public"),
            Self::Coastal => write!(f, "coastal"),
            Self::Offshore => write!(f, "offshore"),
            Self::Fire => write!(f, "fire"),
            Self::County => write!(f, "county"),
        }
    }
}

impl FromStr for NwsZoneType {
    type Err = String;

    /// Convert a string to a NwsZoneType.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "land" => Ok(NwsZoneType::Land),
            "marine" => Ok(NwsZoneType::Marine),
            "forecast" => Ok(NwsZoneType::Forecast),
            "public" => Ok(NwsZoneType::Public),
            "coastal" => Ok(NwsZoneType::Coastal),
            "offshore" => Ok(NwsZoneType::Offshore),
            "fire" => Ok(NwsZoneType::Fire),
            "county" => Ok(NwsZoneType::County),
            _ => Err(format!("Invalid NWS zone type: {}", s)),
        }
    }
}
