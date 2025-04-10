use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    /// A geometry represented in Well-Known Text (WKT) format.
    #[serde(
        rename = "geometry",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub geometry: Option<Option<String>>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    #[serde(rename = "cwa", skip_serializing_if = "Option::is_none")]
    pub cwa: Option<models::NwsForecastOfficeId>,
    #[serde(rename = "forecastOffice", skip_serializing_if = "Option::is_none")]
    pub forecast_office: Option<String>,
    #[serde(rename = "gridId", skip_serializing_if = "Option::is_none")]
    pub grid_id: Option<models::NwsForecastOfficeId>,
    #[serde(rename = "gridX", skip_serializing_if = "Option::is_none")]
    pub grid_x: Option<i32>,
    #[serde(rename = "gridY", skip_serializing_if = "Option::is_none")]
    pub grid_y: Option<i32>,
    #[serde(rename = "forecast", skip_serializing_if = "Option::is_none")]
    pub forecast: Option<String>,
    #[serde(rename = "forecastHourly", skip_serializing_if = "Option::is_none")]
    pub forecast_hourly: Option<String>,
    #[serde(rename = "forecastGridData", skip_serializing_if = "Option::is_none")]
    pub forecast_grid_data: Option<String>,
    #[serde(
        rename = "observationStations",
        skip_serializing_if = "Option::is_none"
    )]
    pub observation_stations: Option<String>,
    #[serde(rename = "relativeLocation", skip_serializing_if = "Option::is_none")]
    pub relative_location: Option<Box<models::PointRelativeLocation>>,
    #[serde(rename = "forecastZone", skip_serializing_if = "Option::is_none")]
    pub forecast_zone: Option<String>,
    #[serde(rename = "county", skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    #[serde(rename = "fireWeatherZone", skip_serializing_if = "Option::is_none")]
    pub fire_weather_zone: Option<String>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename = "radarStation", skip_serializing_if = "Option::is_none")]
    pub radar_station: Option<String>,
}

impl Point {
    pub fn new() -> Point {
        Point {
            at_context: None,
            geometry: None,
            at_id: None,
            at_type: None,
            cwa: None,
            forecast_office: None,
            grid_id: None,
            grid_x: None,
            grid_y: None,
            forecast: None,
            forecast_hourly: None,
            forecast_grid_data: None,
            observation_stations: None,
            relative_location: None,
            forecast_zone: None,
            county: None,
            fire_weather_zone: None,
            time_zone: None,
            radar_station: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "wx:Point")]
    WxColonPoint,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::WxColonPoint
    }
}
