//! Metadata returned by `/stations/{stationId}/tafs`.

use serde::{Deserialize, Serialize};

use crate::StationId;
use crate::time::OffsetDateTime;

/// Current Terminal Aerodrome Forecast metadata for one station.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalAerodromeForecastsResponse {
    /// Forecast metadata in NOAA's response order.
    #[serde(rename = "@graph")]
    pub forecasts: Vec<TerminalAerodromeForecastMetadata>,
}

/// Addressing and validity metadata for one TAF document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalAerodromeForecastMetadata {
    /// Canonical URL for the TAF document.
    pub id: String,
    /// Time NOAA issued the document.
    pub issue_time: OffsetDateTime,
    /// Station the TAF describes.
    pub location: StationId,
    /// Beginning of the forecast period.
    pub start: OffsetDateTime,
    /// End of the forecast period.
    pub end: OffsetDateTime,
    /// Aerodrome point in NOAA's WKT representation.
    pub geometry: String,
}
