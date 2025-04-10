use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Observation {
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
    #[serde(rename = "station", skip_serializing_if = "Option::is_none")]
    pub station: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "rawMessage", skip_serializing_if = "Option::is_none")]
    pub raw_message: Option<String>,
    #[serde(rename = "textDescription", skip_serializing_if = "Option::is_none")]
    pub text_description: Option<String>,
    #[serde(
        rename = "icon",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon: Option<Option<String>>,
    #[serde(rename = "presentWeather", skip_serializing_if = "Option::is_none")]
    pub present_weather: Option<Vec<models::MetarPhenomenon>>,
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "dewpoint", skip_serializing_if = "Option::is_none")]
    pub dewpoint: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "windDirection", skip_serializing_if = "Option::is_none")]
    pub wind_direction: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "windSpeed", skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "windGust", skip_serializing_if = "Option::is_none")]
    pub wind_gust: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "barometricPressure", skip_serializing_if = "Option::is_none")]
    pub barometric_pressure: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "seaLevelPressure", skip_serializing_if = "Option::is_none")]
    pub sea_level_pressure: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Box<models::QuantitativeValue>>,
    #[serde(
        rename = "maxTemperatureLast24Hours",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_temperature_last24_hours: Option<Box<models::QuantitativeValue>>,
    #[serde(
        rename = "minTemperatureLast24Hours",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_temperature_last24_hours: Option<Box<models::QuantitativeValue>>,
    #[serde(
        rename = "precipitationLastHour",
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation_last_hour: Option<Box<models::QuantitativeValue>>,
    #[serde(
        rename = "precipitationLast3Hours",
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation_last3_hours: Option<Box<models::QuantitativeValue>>,
    #[serde(
        rename = "precipitationLast6Hours",
        skip_serializing_if = "Option::is_none"
    )]
    pub precipitation_last6_hours: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "relativeHumidity", skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "windChill", skip_serializing_if = "Option::is_none")]
    pub wind_chill: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "heatIndex", skip_serializing_if = "Option::is_none")]
    pub heat_index: Option<Box<models::QuantitativeValue>>,
    #[serde(
        rename = "cloudLayers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_layers: Option<Option<Vec<models::ObservationCloudLayersInner>>>,
}

impl Observation {
    pub fn new() -> Observation {
        Observation {
            at_context: None,
            geometry: None,
            at_id: None,
            at_type: None,
            elevation: None,
            station: None,
            timestamp: None,
            raw_message: None,
            text_description: None,
            icon: None,
            present_weather: None,
            temperature: None,
            dewpoint: None,
            wind_direction: None,
            wind_speed: None,
            wind_gust: None,
            barometric_pressure: None,
            sea_level_pressure: None,
            visibility: None,
            max_temperature_last24_hours: None,
            min_temperature_last24_hours: None,
            precipitation_last_hour: None,
            precipitation_last3_hours: None,
            precipitation_last6_hours: None,
            relative_humidity: None,
            wind_chill: None,
            heat_index: None,
            cloud_layers: None,
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
