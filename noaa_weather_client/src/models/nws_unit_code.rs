//! Units with the namespace "nwsUnit" are currently custom and do not align to any standard.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NwsUnitCode {
    #[serde(rename = "nwsUnit:s")]
    Second,
    #[serde(rename = "nwsUnit:ns")]
    Nanosecond,
    #[serde(rename = "nwsUnit:MHz")]
    Megahertz,
    #[serde(rename = "nwsUnit:dBZ")]
    DecibelZ,
    #[serde(rename = "nwsUnit:dB")]
    Decibel,
}

impl NwsUnitCode {
    /// Returns the original `skos:prefLabel` for the unit.
    pub fn pref_label(&self) -> &'static str {
        match self {
            NwsUnitCode::Second => "second",
            NwsUnitCode::Nanosecond => "nanosecond",
            NwsUnitCode::Megahertz => "megahertz",
            NwsUnitCode::DecibelZ => "decibelZ",
            NwsUnitCode::Decibel => "decibel",
        }
    }

    /// Returns the `skos:notation` for the unit (e.g., 'degC', 'm/s').
    pub fn notation(&self) -> &'static str {
        match self {
            NwsUnitCode::Second => "s",
            NwsUnitCode::Nanosecond => "ns",
            NwsUnitCode::Megahertz => "MHz",
            NwsUnitCode::DecibelZ => "dBz",
            NwsUnitCode::Decibel => "dB",
        }
    }

    /// Returns the `skos:altLabel` for the unit.
    pub fn alt_label(&self) -> &'static str {
        match self {
            NwsUnitCode::Second => "s",
            NwsUnitCode::Nanosecond => "ns",
            NwsUnitCode::Megahertz => "MHz",
            NwsUnitCode::DecibelZ => "dBz",
            NwsUnitCode::Decibel => "dB",
        }
    }
}
