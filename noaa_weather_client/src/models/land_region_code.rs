use serde::{Deserialize, Serialize};

/// LandRegionCode : Land region code. These correspond to the six NWS regional headquarters: * AR: Alaska Region * CR: Central Region * ER: Eastern Region * PR: Pacific Region * SR: Southern Region * WR: Western Region
/// Land region code. These correspond to the six NWS regional headquarters: * AR: Alaska Region * CR: Central Region * ER: Eastern Region * PR: Pacific Region * SR: Southern Region * WR: Western Region
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LandRegionCode {
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "WR")]
    Wr,
}

impl std::fmt::Display for LandRegionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ar => write!(f, "AR"),
            Self::Cr => write!(f, "CR"),
            Self::Er => write!(f, "ER"),
            Self::Pr => write!(f, "PR"),
            Self::Sr => write!(f, "SR"),
            Self::Wr => write!(f, "WR"),
        }
    }
}

impl Default for LandRegionCode {
    fn default() -> LandRegionCode {
        Self::Ar
    }
}
