use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MetarSkyCoverage {
    #[serde(rename = "OVC")]
    Ovc,
    #[serde(rename = "BKN")]
    Bkn,
    #[serde(rename = "SCT")]
    Sct,
    #[serde(rename = "FEW")]
    Few,
    #[serde(rename = "SKC")]
    Skc,
    #[serde(rename = "CLR")]
    Clr,
    #[serde(rename = "VV")]
    Vv,
}

impl std::fmt::Display for MetarSkyCoverage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ovc => write!(f, "OVC"),
            Self::Bkn => write!(f, "BKN"),
            Self::Sct => write!(f, "SCT"),
            Self::Few => write!(f, "FEW"),
            Self::Skc => write!(f, "SKC"),
            Self::Clr => write!(f, "CLR"),
            Self::Vv => write!(f, "VV"),
        }
    }
}

impl Default for MetarSkyCoverage {
    fn default() -> MetarSkyCoverage {
        Self::Ovc
    }
}
