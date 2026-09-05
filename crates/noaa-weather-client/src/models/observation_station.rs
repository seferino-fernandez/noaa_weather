//! Observation-station metadata returned by station-listing endpoints.
//!
//! # Requiredness
//!
//! A live census of 500 stations on 2026-09-04 found every key below on
//! every record except `county` and `fireWeatherZone`, each of which was
//! absent once. A state-filtered Arizona response also contained station A4837
//! without any forecast-zone links, so `forecast` is optional as well. Empty
//! provider strings are real values and remain distinct from absent fields.
//! `distance` and `bearing` occur only when a gridpoint ranks its nearby
//! stations.

use jiff::tz::TimeZone;
use serde::{Deserialize, Serialize};

use super::Quantity;
use crate::StationId;

/// Metadata for one surface observation station.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ObservationStation {
    /// Canonical API URL for the station.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// JSON-LD resource type (`wx:ObservationStation`).
    #[serde(rename = "@type")]
    pub at_type: ObservationType,
    /// Station elevation.
    pub elevation: Quantity,
    /// ICAO, FAA, or provider station identifier.
    pub station_identifier: StationId,
    /// Human-readable station name.
    pub name: String,
    /// IANA time zone observed by this station.
    #[serde(with = "jiff::fmt::serde::tz::required")]
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    pub time_zone: TimeZone,
    /// Link to the NWS public forecast zone containing this station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forecast: Option<String>,
    /// Link to the NWS county zone containing this station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// Link to the NWS fire-weather zone containing this station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fire_weather_zone: Option<String>,
    /// Primary data provider.
    pub provider: String,
    /// Provider subdivision, or an empty string when there is none.
    pub sub_provider: String,
    /// Distance from the gridpoint that requested this station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<Quantity>,
    /// Bearing from the gridpoint that requested this station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bearing: Option<Quantity>,
}

/// JSON-LD type shared by station metadata and station observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum ObservationType {
    /// A weather observation station resource.
    #[serde(rename = "wx:ObservationStation")]
    Station,
}
