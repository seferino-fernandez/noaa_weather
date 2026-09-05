//! Curated models for zone metadata and text forecasts.
//!
//! # Requiredness
//!
//! A live census on 2026-09-04 sampled 500 records from each of `/zones`,
//! `/zones/land`, `/zones/marine`, `/zones/forecast`, `/zones/public`,
//! `/zones/coastal`, `/zones/fire`, and `/zones/county`, plus all 130
//! offshore zones. Every field was present in every one of the 4,130
//! records. `state` was null on marine, coastal, and offshore records, and
//! `radarStation` was null on most records; those two fields remain nullable.
//! The `cwa` and `forecastOffices` arrays are deprecated in NOAA's schema but
//! still occurred on every sampled record, so they remain part of the model.

use std::fmt;
use std::str::FromStr;

use jiff::tz::TimeZone;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::StateTerritoryCode;
use crate::ids::{OfficeId, StationId, ZoneId};
use crate::time::OffsetDateTime;

/// Metadata for one NWS forecast, fire, county, or marine zone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Zone {
    /// The zone's canonical API URL.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// The fixed JSON-LD type for a zone.
    #[serde(rename = "@type")]
    pub at_type: ZoneResourceType,
    /// The zone's UGC identifier.
    pub id: ZoneId,
    /// The kind of zone this record describes.
    #[serde(rename = "type")]
    pub zone_type: ZoneType,
    /// Human-readable zone name.
    pub name: String,
    /// When this zone definition took effect.
    pub effective_date: OffsetDateTime,
    /// When this zone definition expires.
    pub expiration_date: OffsetDateTime,
    /// State or territory containing the zone. Marine zones have no state.
    pub state: Option<ZoneState>,
    /// Canonical URL of the primary forecast office.
    pub forecast_office: String,
    /// Forecast-office identifiers responsible for the zone.
    ///
    /// NOAA marks this property deprecated, but still sends it.
    #[serde(default)]
    pub cwa: Vec<OfficeId>,
    /// Forecast-office URLs responsible for the zone.
    ///
    /// NOAA marks this property deprecated, but still sends it.
    #[serde(default)]
    pub forecast_offices: Vec<String>,
    /// Office identifier used by the forecast grid.
    pub grid_identifier: OfficeId,
    /// AWIPS location identifier for the zone.
    pub awips_location_identifier: OfficeId,
    /// IANA time zones observed within the zone.
    #[serde(default, with = "time_zones")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub time_zone: Vec<TimeZone>,
    /// URLs of observation stations associated with the zone.
    #[serde(default)]
    pub observation_stations: Vec<String>,
    /// Three-letter radar site identifier, when the zone has one.
    pub radar_station: Option<String>,
}

impl Zone {
    /// Returns the office identifier in [`Zone::forecast_office`].
    #[must_use]
    pub fn forecast_office_id(&self) -> Option<OfficeId> {
        last_segment(&self.forecast_office)?.parse().ok()
    }

    /// Returns the typed station identifiers present in the station URLs.
    pub fn observation_station_ids(&self) -> impl Iterator<Item = StationId> + '_ {
        self.observation_stations
            .iter()
            .filter_map(|url| last_segment(url)?.parse().ok())
    }

    /// Returns the typed office identifiers present in the deprecated office
    /// URL array.
    pub fn forecast_office_ids(&self) -> impl Iterator<Item = OfficeId> + '_ {
        self.forecast_offices
            .iter()
            .filter_map(|url| last_segment(url)?.parse().ok())
    }
}

/// A text forecast for one zone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ZoneForecast {
    /// API URL of the zone this forecast describes.
    pub zone: String,
    /// When NOAA published this forecast.
    pub updated: OffsetDateTime,
    /// Forecast periods in display order.
    #[serde(default)]
    pub periods: Vec<ZoneForecastPeriod>,
}

impl ZoneForecast {
    /// Returns the zone identifier in [`ZoneForecast::zone`].
    #[must_use]
    pub fn zone_id(&self) -> Option<ZoneId> {
        last_segment(&self.zone)?.parse().ok()
    }
}

/// One named day or night in a zone text forecast.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ZoneForecastPeriod {
    /// Sequential period number.
    pub number: u32,
    /// Human-readable period name, such as `Today` or `Tonight`.
    pub name: String,
    /// Detailed text forecast for the period.
    pub detailed_forecast: String,
}

/// The type of NWS zone.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[serde(rename_all = "lowercase")]
pub enum ZoneType {
    /// All land zones.
    Land,
    /// All marine zones.
    Marine,
    /// Zones accepted by the forecast-zone endpoint.
    Forecast,
    /// Public forecast zone.
    Public,
    /// Coastal marine zone.
    Coastal,
    /// Offshore marine zone.
    Offshore,
    /// Fire-weather zone.
    Fire,
    /// County zone.
    County,
}

impl fmt::Display for ZoneType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Land => "land",
            Self::Marine => "marine",
            Self::Forecast => "forecast",
            Self::Public => "public",
            Self::Coastal => "coastal",
            Self::Offshore => "offshore",
            Self::Fire => "fire",
            Self::County => "county",
        })
    }
}

impl FromStr for ZoneType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "land" => Ok(Self::Land),
            "marine" => Ok(Self::Marine),
            "forecast" => Ok(Self::Forecast),
            "public" => Ok(Self::Public),
            "coastal" => Ok(Self::Coastal),
            "offshore" => Ok(Self::Offshore),
            "fire" => Ok(Self::Fire),
            "county" => Ok(Self::County),
            _ => Err(format!("invalid NWS zone type: {input}")),
        }
    }
}

/// State value attached to a zone.
///
/// Known codes decode to [`StateTerritoryCode`]. The string variant preserves
/// NOAA's documented empty-string value and any future code without turning a
/// response into a decode failure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ZoneState {
    /// A known state or territory code.
    StateTerritoryCode(StateTerritoryCode),
    /// An empty or not-yet-known state value.
    Other(String),
}

impl fmt::Display for ZoneState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StateTerritoryCode(code) => code.fmt(formatter),
            Self::Other(code) => formatter.write_str(code),
        }
    }
}

/// Fixed JSON-LD resource type attached to zone properties.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum ZoneResourceType {
    /// A NOAA zone resource.
    #[serde(rename = "wx:Zone")]
    Zone,
}

fn last_segment(url: &str) -> Option<&str> {
    url.trim_end_matches('/')
        .rsplit('/')
        .find(|segment| !segment.is_empty())
}

mod time_zones {
    use super::*;

    pub(super) fn serialize<S>(zones: &[TimeZone], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let names = zones
            .iter()
            .map(|zone| {
                zone.iana_name().ok_or_else(|| {
                    serde::ser::Error::custom("zone metadata requires named IANA time zones")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        names.serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<TimeZone>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<String>::deserialize(deserializer)?
            .into_iter()
            .map(|name| TimeZone::get(&name).map_err(serde::de::Error::custom))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZONE: &str = include_str!("../../tests/fixtures/zones/single.json");
    const FORECAST: &str = include_str!("../../tests/fixtures/zones/forecast.json");

    #[test]
    fn fixture_decodes_to_typed_values_and_preserves_live_office_fields() {
        let feature: crate::Feature<Zone> = serde_json::from_str(ZONE).unwrap();
        let zone = feature.properties;

        assert_eq!(zone.id.as_str(), "UTZ101");
        assert_eq!(zone.zone_type, ZoneType::Public);
        assert_eq!(zone.effective_date.to_string(), "2026-04-16T18:00:00+00:00");
        assert_eq!(zone.state.as_ref().unwrap().to_string(), "UT");
        assert_eq!(zone.time_zone[0].iana_name(), Some("America/Denver"));
        assert_eq!(zone.forecast_office_id().unwrap().as_str(), "SLC");
        assert_eq!(
            zone.observation_station_ids().next().unwrap().as_str(),
            "ARAU1"
        );
        assert_eq!(zone.cwa[0].as_str(), "SLC");
        assert_eq!(zone.forecast_office_ids().next().unwrap().as_str(), "SLC");

        let serialized = serde_json::to_value(zone).unwrap();
        assert_eq!(serialized["cwa"], serde_json::json!(["SLC"]));
        assert_eq!(
            serialized["forecastOffices"],
            serde_json::json!(["https://api.weather.gov/offices/SLC"])
        );
    }

    #[test]
    fn forecast_fixture_has_a_typed_update_and_zone_id() {
        let feature: crate::Feature<ZoneForecast> = serde_json::from_str(FORECAST).unwrap();
        let forecast = feature.properties;

        assert_eq!(forecast.zone_id().unwrap().as_str(), "UTZ101");
        assert_eq!(forecast.updated.to_string(), "2026-09-02T12:44:00-06:00");
        assert_eq!(forecast.periods.len(), 10);
        assert_eq!(forecast.periods[0].number, 1);
        assert_eq!(forecast.periods[0].name, "Today");
    }

    #[test]
    fn zone_type_parses_case_insensitively_and_round_trips() {
        let zone_type: ZoneType = "Coastal".parse().unwrap();
        assert_eq!(zone_type, ZoneType::Coastal);
        assert_eq!(zone_type.to_string(), "coastal");
        assert_eq!(serde_json::to_string(&zone_type).unwrap(), "\"coastal\"");
        assert!("weather".parse::<ZoneType>().is_err());
    }

    #[test]
    fn unknown_and_empty_state_strings_are_preserved() {
        let unknown: ZoneState = serde_json::from_str("\"ZZ\"").unwrap();
        let empty: ZoneState = serde_json::from_str("\"\"").unwrap();
        assert_eq!(unknown, ZoneState::Other("ZZ".to_owned()));
        assert_eq!(empty, ZoneState::Other(String::new()));
        assert_eq!(serde_json::to_string(&unknown).unwrap(), "\"ZZ\"");
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn every_zone_model_publishes_a_schema() {
        let zone = schemars::schema_for!(Zone);
        let forecast = schemars::schema_for!(ZoneForecast);
        assert_eq!(zone.as_value()["properties"]["id"]["type"], "string");
        assert_eq!(
            forecast.as_value()["properties"]["periods"]["type"],
            "array"
        );
    }
}
