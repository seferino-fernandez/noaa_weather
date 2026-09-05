use serde::{Deserialize, Serialize};

/// Identifies an NWS regional headquarters office.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum NwsRegionalHqid {
    /// The `ARH` identifier for Alaska Region Headquarters in Anchorage, Alaska.
    #[serde(rename = "ARH")]
    #[default]
    Arh,
    /// The `CRH` identifier for Central Region Headquarters in Kansas City, Missouri.
    #[serde(rename = "CRH")]
    Crh,
    /// The `ERH` identifier for Eastern Region Headquarters in Bohemia, New York.
    #[serde(rename = "ERH")]
    Erh,
    /// The `PRH` identifier for Pacific Region Headquarters in Honolulu, Hawaii.
    #[serde(rename = "PRH")]
    Prh,
    /// The `SRH` identifier for Southern Region Headquarters in Fort Worth, Texas.
    #[serde(rename = "SRH")]
    Srh,
    /// The `WRH` identifier for Western Region Headquarters in Salt Lake City, Utah.
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
