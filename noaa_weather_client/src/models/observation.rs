//! Surface observations returned by the `/stations` and `/zones` families.
//!
//! # Requiredness
//!
//! A live probe on 2026-09-04 sampled the latest observation from KPHX,
//! KSLC, KJFK, PADQ, and PHNL. Every sample carried the same 26 core keys;
//! `precipitationLastHour` and `precipitationLast6Hours` were the only keys
//! that varied by station. Those two fields therefore remain optional. A
//! `icon` is always keyed but is null on non-airport observations. A
//! measurement whose sensor has no reading is still present as a [`Quantity`]
//! with a null value, preserving absence of a reading separately from absence
//! of the field.

use serde::{Deserialize, Serialize};

use super::{MetarPhenomenon, MetarSkyCoverage, ObservationType, Quantity};
use crate::StationId;
use crate::time::OffsetDateTime;

/// One decoded surface-weather observation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Observation {
    /// Canonical URL for this observation.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// JSON-LD resource type (`wx:ObservationStation`).
    #[serde(rename = "@type")]
    pub at_type: ObservationType,
    /// Elevation of the observing station.
    pub elevation: Quantity,
    /// Canonical URL for the observing station.
    pub station: String,
    /// Identifier of the observing station.
    pub station_id: StationId,
    /// Human-readable station name.
    pub station_name: String,
    /// Time at which the observation was made.
    pub timestamp: OffsetDateTime,
    /// Original METAR message, when the provider supplied one.
    pub raw_message: String,
    /// Short plain-language description of the conditions.
    pub text_description: String,
    /// URL for an icon representing the conditions, when NOAA has one.
    pub icon: Option<String>,
    /// Decoded present-weather phenomena.
    pub present_weather: Vec<MetarPhenomenon>,
    /// Air temperature.
    pub temperature: Quantity,
    /// Dewpoint temperature.
    pub dewpoint: Quantity,
    /// Direction from which the wind blows.
    pub wind_direction: Quantity,
    /// Sustained wind speed.
    pub wind_speed: Quantity,
    /// Wind gust speed.
    pub wind_gust: Quantity,
    /// Station barometric pressure.
    pub barometric_pressure: Quantity,
    /// Pressure reduced to sea level.
    pub sea_level_pressure: Quantity,
    /// Horizontal visibility.
    pub visibility: Quantity,
    /// Maximum temperature during the preceding 24 hours.
    pub max_temperature_last24_hours: Quantity,
    /// Minimum temperature during the preceding 24 hours.
    pub min_temperature_last24_hours: Quantity,
    /// Precipitation during the preceding hour, when reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precipitation_last_hour: Option<Quantity>,
    /// Precipitation during the preceding three hours.
    pub precipitation_last3_hours: Quantity,
    /// Precipitation during the preceding six hours, when reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precipitation_last6_hours: Option<Quantity>,
    /// Relative humidity.
    pub relative_humidity: Quantity,
    /// Wind-chill temperature.
    pub wind_chill: Quantity,
    /// Heat-index temperature.
    pub heat_index: Quantity,
    /// Observed cloud layers in ascending order.
    pub cloud_layers: Vec<ObservationCloudLayer>,
}

/// One observed cloud layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ObservationCloudLayer {
    /// Height of the cloud base.
    pub base: Quantity,
    /// Fraction of the sky covered, as a METAR code.
    pub amount: MetarSkyCoverage,
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::Observation;

    const LATEST: &str = include_str!("../../tests/fixtures/stations/latest.json");

    #[test]
    fn full_observation_round_trips_to_the_same_keys() {
        let raw: Value = serde_json::from_str(LATEST).unwrap();
        let observation: crate::Feature<Observation> = serde_json::from_value(raw.clone()).unwrap();
        let round_tripped = serde_json::to_value(observation).unwrap();
        assert_eq!(
            round_tripped["properties"]["icon"],
            raw["properties"]["icon"]
        );
        assert_eq!(
            round_tripped["properties"]
                .as_object()
                .unwrap()
                .keys()
                .collect::<Vec<_>>(),
            raw["properties"]
                .as_object()
                .unwrap()
                .keys()
                .collect::<Vec<_>>()
        );
    }
}
