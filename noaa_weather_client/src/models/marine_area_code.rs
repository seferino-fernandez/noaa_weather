use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Marine area code as defined in NWS Directive 10-302:
/// * AM: Western North Atlantic Ocean and along U.S. East Coast south of Currituck Beach Light NC following the coastline into Gulf of Mexico to Ocean Reef FL including the Caribbean
/// * AN: Western North Atlantic Ocean and along U.S. East Coast from Canadian border south to Currituck Beach Light NC
/// * GM: Gulf of Mexico and along the U.S. Gulf Coast from the Mexican border to Ocean Reef FL
/// * LC: Lake St. Clair
/// * LE: Lake Erie
/// * LH: Lake Huron
/// * LM: Lake Michigan
/// * LO: Lake Ontario
/// * LS: Lake Superior
/// * PH: Central Pacific Ocean including Hawaiian waters
/// * PK: North Pacific Ocean near Alaska and along Alaska coastline including the Bering Sea and the Gulf of Alaska
/// * PM: Western Pacific Ocean including Mariana Island waters
/// * PS: South Central Pacific Ocean including American Samoa waters
/// * PZ: Eastern North Pacific Ocean and along U.S. West Coast from Canadian border to Mexican border
/// * SL: St. Lawrence River above St. Regis
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MarineAreaCode {
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AN")]
    An,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LE")]
    Le,
    #[serde(rename = "LH")]
    Lh,
    #[serde(rename = "LM")]
    Lm,
    #[serde(rename = "LO")]
    Lo,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PZ")]
    Pz,
    #[serde(rename = "SL")]
    Sl,
}

impl std::fmt::Display for MarineAreaCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Am => write!(f, "AM"),
            Self::An => write!(f, "AN"),
            Self::Gm => write!(f, "GM"),
            Self::Lc => write!(f, "LC"),
            Self::Le => write!(f, "LE"),
            Self::Lh => write!(f, "LH"),
            Self::Lm => write!(f, "LM"),
            Self::Lo => write!(f, "LO"),
            Self::Ls => write!(f, "LS"),
            Self::Ph => write!(f, "PH"),
            Self::Pk => write!(f, "PK"),
            Self::Pm => write!(f, "PM"),
            Self::Ps => write!(f, "PS"),
            Self::Pz => write!(f, "PZ"),
            Self::Sl => write!(f, "SL"),
        }
    }
}

impl FromStr for MarineAreaCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AM" => Ok(MarineAreaCode::Am),
            "AN" => Ok(MarineAreaCode::An),
            "GM" => Ok(MarineAreaCode::Gm),
            "LC" => Ok(MarineAreaCode::Lc),
            "LE" => Ok(MarineAreaCode::Le),
            "LH" => Ok(MarineAreaCode::Lh),
            "LM" => Ok(MarineAreaCode::Lm),
            "LO" => Ok(MarineAreaCode::Lo),
            "LS" => Ok(MarineAreaCode::Ls),
            "PH" => Ok(MarineAreaCode::Ph),
            "PK" => Ok(MarineAreaCode::Pk),
            "PM" => Ok(MarineAreaCode::Pm),
            "PS" => Ok(MarineAreaCode::Ps),
            "PZ" => Ok(MarineAreaCode::Pz),
            "SL" => Ok(MarineAreaCode::Sl),
            _ => Err(format!("Invalid marine area code: {}", s)),
        }
    }
}
