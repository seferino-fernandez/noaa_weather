//! Units in NOAA's custom `nwsUnit` namespace.

use serde::{Deserialize, Serialize};

/// Identifies a unit in NOAA's custom `nwsUnit` namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NwsUnitCode {
    /// Measures a duration in seconds (`nwsUnit:s`).
    #[serde(rename = "nwsUnit:s")]
    Second,
    /// Measures a duration in nanoseconds (`nwsUnit:ns`).
    #[serde(rename = "nwsUnit:ns")]
    Nanosecond,
    /// Measures a frequency in megahertz (`nwsUnit:MHz`).
    #[serde(rename = "nwsUnit:MHz")]
    Megahertz,
    /// Measures radar reflectivity in decibels relative to Z (`nwsUnit:dBZ`).
    #[serde(rename = "nwsUnit:dBZ")]
    DecibelZ,
    /// Measures a logarithmic ratio in decibels (`nwsUnit:dB`).
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

    /// Returns the `unitCode` string NOAA writes on the wire, such as
    /// `nwsUnit:s`.
    #[must_use]
    pub fn unit_code(&self) -> &'static str {
        match self {
            NwsUnitCode::Second => "nwsUnit:s",
            NwsUnitCode::Nanosecond => "nwsUnit:ns",
            NwsUnitCode::Megahertz => "nwsUnit:MHz",
            NwsUnitCode::DecibelZ => "nwsUnit:dBZ",
            NwsUnitCode::Decibel => "nwsUnit:dB",
        }
    }
}
