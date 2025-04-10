use crate::models;
use serde::{Deserialize, Serialize};

/// Gridpoint : Raw forecast data for a 2.5km grid square. This is a list of all potential data layers that may appear. Some layers may not be present in all areas. * temperature * dewpoint * maxTemperature * minTemperature * relativeHumidity * apparentTemperature * heatIndex * windChill * wetBulbGlobeTemperature * skyCover * windDirection * windSpeed * windGust * weather * hazards: Watch and advisory products in effect * probabilityOfPrecipitation * quantitativePrecipitation * iceAccumulation * snowfallAmount * snowLevel * ceilingHeight * visibility * transportWindSpeed * transportWindDirection * mixingHeight * hainesIndex * lightningActivityLevel * twentyFootWindSpeed * twentyFootWindDirection * waveHeight * wavePeriod * waveDirection * primarySwellHeight * primarySwellDirection * secondarySwellHeight * secondarySwellDirection * wavePeriod2 * windWaveHeight * dispersionIndex * pressure: Barometric pressure * probabilityOfTropicalStormWinds * probabilityOfHurricaneWinds * potentialOf15mphWinds * potentialOf25mphWinds * potentialOf35mphWinds * potentialOf45mphWinds * potentialOf20mphWindGusts * potentialOf30mphWindGusts * potentialOf40mphWindGusts * potentialOf50mphWindGusts * potentialOf60mphWindGusts * grasslandFireDangerIndex * probabilityOfThunder * davisStabilityIndex * atmosphericDispersionIndex * lowVisibilityOccurrenceRiskIndex * stability * redFlagThreatIndex
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gridpoint {
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
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(rename = "validTimes", skip_serializing_if = "Option::is_none")]
    pub valid_times: Option<Box<models::Iso8601Interval>>,
    #[serde(rename = "elevation", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "forecastOffice", skip_serializing_if = "Option::is_none")]
    pub forecast_office: Option<String>,
    #[serde(rename = "gridId", skip_serializing_if = "Option::is_none")]
    pub grid_id: Option<String>,
    #[serde(rename = "gridX", skip_serializing_if = "Option::is_none")]
    pub grid_x: Option<i32>,
    #[serde(rename = "gridY", skip_serializing_if = "Option::is_none")]
    pub grid_y: Option<i32>,
    #[serde(rename = "weather", skip_serializing_if = "Option::is_none")]
    pub weather: Option<Box<models::GridpointWeather>>,
    #[serde(rename = "hazards", skip_serializing_if = "Option::is_none")]
    pub hazards: Option<Box<models::GridpointHazards>>,
}

impl Gridpoint {
    /// Raw forecast data for a 2.5km grid square. This is a list of all potential data layers that may appear. Some layers may not be present in all areas. * temperature * dewpoint * maxTemperature * minTemperature * relativeHumidity * apparentTemperature * heatIndex * windChill * wetBulbGlobeTemperature * skyCover * windDirection * windSpeed * windGust * weather * hazards: Watch and advisory products in effect * probabilityOfPrecipitation * quantitativePrecipitation * iceAccumulation * snowfallAmount * snowLevel * ceilingHeight * visibility * transportWindSpeed * transportWindDirection * mixingHeight * hainesIndex * lightningActivityLevel * twentyFootWindSpeed * twentyFootWindDirection * waveHeight * wavePeriod * waveDirection * primarySwellHeight * primarySwellDirection * secondarySwellHeight * secondarySwellDirection * wavePeriod2 * windWaveHeight * dispersionIndex * pressure: Barometric pressure * probabilityOfTropicalStormWinds * probabilityOfHurricaneWinds * potentialOf15mphWinds * potentialOf25mphWinds * potentialOf35mphWinds * potentialOf45mphWinds * potentialOf20mphWindGusts * potentialOf30mphWindGusts * potentialOf40mphWindGusts * potentialOf50mphWindGusts * potentialOf60mphWindGusts * grasslandFireDangerIndex * probabilityOfThunder * davisStabilityIndex * atmosphericDispersionIndex * lowVisibilityOccurrenceRiskIndex * stability * redFlagThreatIndex
    pub fn new() -> Gridpoint {
        Gridpoint {
            at_context: None,
            geometry: None,
            at_id: None,
            at_type: None,
            update_time: None,
            valid_times: None,
            elevation: None,
            forecast_office: None,
            grid_id: None,
            grid_x: None,
            grid_y: None,
            weather: None,
            hazards: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "wx:Gridpoint")]
    WxColonGridpoint,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::WxColonGridpoint
    }
}
