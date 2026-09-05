use serde::{Deserialize, Serialize};

/// Classifies cloud cover in a METAR sky-condition group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum MetarSkyCoverage {
    /// Reports an overcast layer covering the entire sky (`OVC`).
    #[serde(rename = "OVC")]
    Ovc,
    /// Reports a broken layer covering five-eighths through seven-eighths of the sky (`BKN`).
    #[serde(rename = "BKN")]
    Bkn,
    /// Reports a scattered layer covering three-eighths through four-eighths of the sky (`SCT`).
    #[serde(rename = "SCT")]
    Sct,
    /// Reports a few clouds covering one-eighth through two-eighths of the sky (`FEW`).
    #[serde(rename = "FEW")]
    Few,
    /// Reports an observer-confirmed clear sky (`SKC`).
    #[serde(rename = "SKC")]
    Skc,
    /// Reports that an automated station detected no clouds at or below 12,000 feet (`CLR`).
    #[serde(rename = "CLR")]
    Clr,
    /// Reports vertical visibility into a surface-based obscuration (`VV`).
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
