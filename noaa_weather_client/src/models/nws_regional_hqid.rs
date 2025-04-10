use serde::{Deserialize, Serialize};

/// NwsRegionalHqid : Three-letter identifier for a NWS Regional HQ.
/// Three-letter identifier for a NWS Regional HQ.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NwsRegionalHqid {
    #[serde(rename = "ARH")]
    Arh,
    #[serde(rename = "CRH")]
    Crh,
    #[serde(rename = "ERH")]
    Erh,
    #[serde(rename = "PRH")]
    Prh,
    #[serde(rename = "SRH")]
    Srh,
    #[serde(rename = "WRH")]
    Wrh,
}

impl std::fmt::Display for NwsRegionalHqid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Arh => write!(f, "ARH"),
            Self::Crh => write!(f, "CRH"),
            Self::Erh => write!(f, "ERH"),
            Self::Prh => write!(f, "PRH"),
            Self::Srh => write!(f, "SRH"),
            Self::Wrh => write!(f, "WRH"),
        }
    }
}

impl Default for NwsRegionalHqid {
    fn default() -> NwsRegionalHqid {
        Self::Arh
    }
}
