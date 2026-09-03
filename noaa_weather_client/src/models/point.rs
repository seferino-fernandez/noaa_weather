//! Models for the `/points` family: [`Point`], [`RelativeLocation`], and
//! [`PointType`].
//!
//! [`Point`] is the `properties` object of `/points/{latitude},{longitude}`.
//! It is the entry point of nearly every forecast workflow: it names the
//! forecast office and grid cell covering a coordinate, the zones it falls
//! in, and the URLs of the endpoints that describe it.
//!
//! # Requiredness
//!
//! Nothing here is `Option` except `astronomicalData` and `nwr`. A live
//! probe of 63 points across 62 forecast offices (CONUS, Alaska, Hawaii,
//! Puerto Rico, and Guam) found every other key present and non-null on
//! every sample, so a missing one is a decode error rather than a silent
//! `None`. `astronomicalData` and `nwr` are equally always present but are
//! not part of what a point *is*, so they stay `Option` and are skipped
//! when absent.
//!
//! `/points` answers only over land — it returns 404 over open water — so
//! every sampled `type` was `land`. [`PointType`] keeps the marine variant
//! the API specification defines.
//!
//! # Typed fields
//!
//! `gridId`, `cwa`, `gridX`, and `gridY` are typed, so a point converts
//! straight into a [`GridpointId`](crate::ids::GridpointId) with
//! `GridpointId::try_from(&point)`. `timeZone` is a real
//! [`jiff::tz::TimeZone`], resolved against the system zone database, which
//! makes rendering a forecast in the point's own local time a lookup rather
//! than a string match.
//!
//! The remaining fields are URLs into other NOAA families and stay
//! `String`. Each has an accessor ([`Point::forecast_zone_id`],
//! [`Point::county_id`], [`Point::fire_weather_zone_id`],
//! [`Point::radar_station_id`], [`Point::forecast_office_id`]) that returns
//! the typed identifier in the URL's last segment, or `None` when the
//! segment is not one.

use jiff::tz::TimeZone;
use serde::{Deserialize, Serialize};

use super::{AstronomicalData, NoaaWeatherRadio, NwsForecastOfficeId, Quantity};
use crate::geo::Feature;
use crate::ids::{OfficeId, RadarStationId, ZoneId};

/// Metadata for one latitude/longitude point.
///
/// ```
/// use noaa_weather_client::GridpointId;
/// use noaa_weather_client::models::{Point, PointType};
///
/// let point: Point = serde_json::from_str(r#"{
///   "@id": "https://api.weather.gov/points/39.7456,-97.0892",
///   "@type": "wx:Point",
///   "cwa": "TOP",
///   "type": "land",
///   "forecastOffice": "https://api.weather.gov/offices/TOP",
///   "gridId": "TOP", "gridX": 32, "gridY": 81,
///   "forecast": "https://api.weather.gov/gridpoints/TOP/32,81/forecast",
///   "forecastHourly": "https://api.weather.gov/gridpoints/TOP/32,81/forecast/hourly",
///   "forecastGridData": "https://api.weather.gov/gridpoints/TOP/32,81",
///   "observationStations": "https://api.weather.gov/gridpoints/TOP/32,81/stations",
///   "relativeLocation": {"type": "Feature", "geometry": null, "properties": {
///     "city": "Linn", "state": "KS",
///     "distance": {"unitCode": "wmoUnit:m", "value": 6745.3279758024},
///     "bearing": {"unitCode": "wmoUnit:degree_(angle)", "value": 358}
///   }},
///   "forecastZone": "https://api.weather.gov/zones/forecast/KSZ009",
///   "county": "https://api.weather.gov/zones/county/KSC201",
///   "fireWeatherZone": "https://api.weather.gov/zones/fire/KSZ009",
///   "timeZone": "America/Chicago",
///   "radarStation": "KTWX"
/// }"#).unwrap();
///
/// assert_eq!(point.point_type, PointType::Land);
/// assert_eq!(point.relative_location.city, "Linn");
/// assert_eq!(point.time_zone.iana_name(), Some("America/Chicago"));
/// assert_eq!(GridpointId::try_from(&point).unwrap().to_string(), "TOP/32,81");
/// assert_eq!(point.forecast_zone_id().unwrap().as_str(), "KSZ009");
/// assert_eq!(point.radar_station_id().unwrap().as_str(), "KTWX");
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Point {
    /// The canonical API URL for this point.
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// The JSON-LD type assigned to this point (`wx:Point`).
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// The county warning area: the office responsible for this point.
    pub cwa: NwsForecastOfficeId,
    /// The API URL of the responsible forecast office. See
    /// [`Point::forecast_office_id`].
    pub forecast_office: String,
    /// The forecast office whose grid covers this point.
    pub grid_id: NwsForecastOfficeId,
    /// The column of the 2.5 km forecast grid covering this point.
    pub grid_x: u32,
    /// The row of the 2.5 km forecast grid covering this point.
    pub grid_y: u32,
    /// The API URL of the multi-day textual forecast for this point.
    pub forecast: String,
    /// The API URL of the hourly textual forecast for this point.
    pub forecast_hourly: String,
    /// The API URL of the raw forecast layers for this point.
    pub forecast_grid_data: String,
    /// The API URL listing observation stations usable for this point.
    pub observation_stations: String,
    /// The nearest named place, with its distance and bearing.
    pub relative_location: Feature<RelativeLocation>,
    /// The API URL of the NWS public forecast zone containing this point.
    /// See [`Point::forecast_zone_id`].
    pub forecast_zone: String,
    /// The API URL of the NWS county zone containing this point. See
    /// [`Point::county_id`].
    pub county: String,
    /// The API URL of the NWS fire weather zone containing this point. See
    /// [`Point::fire_weather_zone_id`].
    pub fire_weather_zone: String,
    /// The IANA time zone this point observes, resolved against the system
    /// time zone database.
    #[serde(with = "jiff::fmt::serde::tz::required")]
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    pub time_zone: TimeZone,
    /// The radar station covering this point, such as `KTWX`. See
    /// [`Point::radar_station_id`].
    pub radar_station: String,
    /// Whether this point is on land or at sea.
    #[serde(rename = "type")]
    pub point_type: PointType,
    /// Sunrise, sunset, and twilight times for this point.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub astronomical_data: Option<AstronomicalData>,
    /// NOAA Weather Radio coverage for this point.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nwr: Option<NoaaWeatherRadio>,
}

/// Returns the last non-empty path segment of `url`.
fn last_segment(url: &str) -> Option<&str> {
    url.trim_end_matches('/')
        .rsplit('/')
        .find(|segment| !segment.is_empty())
}

impl Point {
    /// Returns the office named by [`Point::forecast_office`].
    #[must_use]
    pub fn forecast_office_id(&self) -> Option<OfficeId> {
        last_segment(&self.forecast_office)?.parse().ok()
    }

    /// Returns the zone named by [`Point::forecast_zone`].
    #[must_use]
    pub fn forecast_zone_id(&self) -> Option<ZoneId> {
        last_segment(&self.forecast_zone)?.parse().ok()
    }

    /// Returns the county zone named by [`Point::county`].
    #[must_use]
    pub fn county_id(&self) -> Option<ZoneId> {
        last_segment(&self.county)?.parse().ok()
    }

    /// Returns the fire weather zone named by [`Point::fire_weather_zone`].
    #[must_use]
    pub fn fire_weather_zone_id(&self) -> Option<ZoneId> {
        last_segment(&self.fire_weather_zone)?.parse().ok()
    }

    /// Returns the radar station named by [`Point::radar_station`].
    #[must_use]
    pub fn radar_station_id(&self) -> Option<RadarStationId> {
        self.radar_station.parse().ok()
    }
}

/// The nearest named place to a point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RelativeLocation {
    /// The name of the place, such as `Linn`.
    pub city: String,
    /// The two-letter state or territory code of the place.
    pub state: String,
    /// How far the point is from the place.
    pub distance: Quantity,
    /// The compass bearing from the place to the point.
    pub bearing: Quantity,
}

/// Whether a point is on land or at sea.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum PointType {
    /// A point over land.
    #[serde(rename = "land")]
    Land,
    /// A point over water.
    #[serde(rename = "marine")]
    Marine,
}

impl std::fmt::Display for PointType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Land => write!(f, "land"),
            Self::Marine => write!(f, "marine"),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    const POINT: &str = r#"{
  "@id": "https://api.weather.gov/points/39.7456,-97.0892",
  "@type": "wx:Point",
  "cwa": "TOP",
  "type": "land",
  "forecastOffice": "https://api.weather.gov/offices/TOP",
  "gridId": "TOP",
  "gridX": 32,
  "gridY": 81,
  "forecast": "https://api.weather.gov/gridpoints/TOP/32,81/forecast",
  "forecastHourly": "https://api.weather.gov/gridpoints/TOP/32,81/forecast/hourly",
  "forecastGridData": "https://api.weather.gov/gridpoints/TOP/32,81",
  "observationStations": "https://api.weather.gov/gridpoints/TOP/32,81/stations",
  "relativeLocation": {
    "type": "Feature",
    "geometry": {"type": "Point", "coordinates": [-97.0867936, 39.6792898]},
    "properties": {
      "city": "Linn",
      "state": "KS",
      "distance": {"unitCode": "wmoUnit:m", "value": 6745.3279758024},
      "bearing": {"unitCode": "wmoUnit:degree_(angle)", "value": 358}
    }
  },
  "forecastZone": "https://api.weather.gov/zones/forecast/KSZ009",
  "county": "https://api.weather.gov/zones/county/KSC201",
  "fireWeatherZone": "https://api.weather.gov/zones/fire/KSZ009",
  "timeZone": "America/Chicago",
  "radarStation": "KTWX",
  "nwr": {
    "transmitter": "KZZ67",
    "sameCode": "020201",
    "areaBroadcast": "https://api.weather.gov/radio/KZZ67/broadcast",
    "pointBroadcast": "https://api.weather.gov/points/39.7456,-97.0892/radio"
  }
}"#;

    fn point() -> Point {
        serde_json::from_str(POINT).unwrap()
    }

    /// Rewrites every JSON number as a float, the one normalization a
    /// point round trip performs: measured values are `f64`, so NOAA's
    /// `"value": 358` comes back as `358.0`.
    fn as_floats(value: &Value) -> Value {
        match value {
            Value::Number(number) => serde_json::Number::from_f64(number.as_f64().unwrap())
                .map_or(Value::Null, Value::Number),
            Value::Array(items) => Value::Array(items.iter().map(as_floats).collect()),
            Value::Object(members) => Value::Object(
                members
                    .iter()
                    .map(|(key, child)| (key.clone(), as_floats(child)))
                    .collect(),
            ),
            other => other.clone(),
        }
    }

    #[test]
    fn a_full_point_round_trips_to_the_same_json() {
        let original: Value = serde_json::from_str(POINT).unwrap();
        let round_tripped = serde_json::to_value(point()).unwrap();
        assert_eq!(as_floats(&round_tripped), as_floats(&original));
        assert_eq!(
            round_tripped["relativeLocation"]["properties"]["bearing"]["value"],
            serde_json::json!(358.0),
            "whole measured values are written as floats"
        );
    }

    #[test]
    fn typed_fields_carry_noaa_values() {
        let point = point();
        assert_eq!(point.cwa, NwsForecastOfficeId::Top);
        assert_eq!(point.grid_id, NwsForecastOfficeId::Top);
        assert_eq!((point.grid_x, point.grid_y), (32, 81));
        assert_eq!(point.point_type, PointType::Land);
        assert_eq!(point.time_zone.iana_name(), Some("America/Chicago"));
        assert_eq!(point.relative_location.state, "KS");
        assert_eq!(
            point.relative_location.distance.value,
            Some(6745.3279758024)
        );
        assert_eq!(
            point.relative_location.bearing.unit.code(),
            "wmoUnit:degree_(angle)"
        );
        assert_eq!(point.astronomical_data, None);
        assert!(point.nwr.is_some());
    }

    #[test]
    fn url_accessors_return_the_identifier_in_the_last_segment() {
        let point = point();
        assert_eq!(point.forecast_office_id().unwrap().as_str(), "TOP");
        assert_eq!(point.forecast_zone_id().unwrap().as_str(), "KSZ009");
        assert_eq!(point.county_id().unwrap().as_str(), "KSC201");
        assert_eq!(point.fire_weather_zone_id().unwrap().as_str(), "KSZ009");
        assert_eq!(point.radar_station_id().unwrap().as_str(), "KTWX");
    }

    #[test]
    fn url_accessors_are_none_when_the_last_segment_is_not_an_identifier() {
        let mut point = point();
        point.forecast_zone = "https://api.weather.gov/zones/forecast/".to_owned();
        point.county = String::new();
        point.radar_station = "not a station".to_owned();
        assert_eq!(point.forecast_zone_id(), None);
        assert_eq!(point.county_id(), None);
        assert_eq!(point.radar_station_id(), None);
    }

    #[test]
    fn absent_astronomical_data_and_radio_are_skipped() {
        let point = point();
        let value = serde_json::to_value(&point).unwrap();
        assert!(!value.as_object().unwrap().contains_key("astronomicalData"));
        assert!(value.as_object().unwrap().contains_key("nwr"));
    }

    #[test]
    fn a_missing_required_key_is_a_decode_error() {
        for key in ["cwa", "gridId", "gridX", "gridY", "timeZone", "type"] {
            let mut value: Value = serde_json::from_str(POINT).unwrap();
            value.as_object_mut().unwrap().remove(key);
            assert!(
                serde_json::from_value::<Point>(value).is_err(),
                "{key} should be required"
            );
        }
    }
}
