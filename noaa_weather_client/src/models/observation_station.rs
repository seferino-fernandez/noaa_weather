use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationStation {
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
    #[serde(rename = "elevation", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "stationIdentifier", skip_serializing_if = "Option::is_none")]
    pub station_identifier: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// A link to the NWS public forecast zone containing this station.
    #[serde(rename = "forecast", skip_serializing_if = "Option::is_none")]
    pub forecast: Option<String>,
    /// A link to the NWS county zone containing this station.
    #[serde(rename = "county", skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// A link to the NWS fire weather forecast zone containing this station.
    #[serde(rename = "fireWeatherZone", skip_serializing_if = "Option::is_none")]
    pub fire_weather_zone: Option<String>,
}

impl ObservationStation {
    pub fn new() -> ObservationStation {
        ObservationStation {
            at_context: None,
            geometry: None,
            at_id: None,
            at_type: None,
            elevation: None,
            station_identifier: None,
            name: None,
            time_zone: None,
            forecast: None,
            county: None,
            fire_weather_zone: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "wx:ObservationStation")]
    WxColonObservationStation,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::WxColonObservationStation
    }
}
