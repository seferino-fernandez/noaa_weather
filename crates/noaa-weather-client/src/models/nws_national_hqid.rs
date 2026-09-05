use serde::{Deserialize, Serialize};

/// NwsNationalHqid : Three-letter identifier for NWS National HQ.
/// Three-letter identifier for NWS National HQ.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum NwsNationalHqid {
    #[serde(rename = "NWS")]
    #[default]
    Nws,
}

impl std::fmt::Display for NwsNationalHqid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Nws => write!(f, "NWS"),
        }
    }
}
