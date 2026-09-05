use serde::{Deserialize, Serialize};

/// Identifies the NWS national headquarters office.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum NwsNationalHqid {
    /// The `NWS` identifier for National Weather Service headquarters.
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
