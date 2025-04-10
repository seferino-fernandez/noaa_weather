use crate::models;
use serde::{Deserialize, Serialize};

/// GridpointForecastPeriod : An object containing forecast information for a specific time period (generally 12-hour or 1-hour).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointForecastPeriod {
    /// Sequential period number.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
    /// A textual identifier for the period. This value will not be present for hourly forecasts.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The starting time that this forecast period is valid for.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The ending time that this forecast period is valid for.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Indicates whether this period is daytime or nighttime.
    #[serde(rename = "isDaytime", skip_serializing_if = "Option::is_none")]
    pub is_daytime: Option<bool>,
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Box<models::GridpointForecastPeriodTemperature>>,
    /// The unit of the temperature value (Fahrenheit or Celsius). This property is deprecated. Future versions will indicate the unit within the quantitative value object for the temperature property. To make use of the future standard format now, set the \"forecast_temperature_qv\" feature flag on the request.
    #[serde(rename = "temperatureUnit", skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<TemperatureUnit>,
    /// If not null, indicates a non-diurnal temperature trend for the period (either rising temperature overnight, or falling temperature during the day)
    #[serde(
        rename = "temperatureTrend",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub temperature_trend: Option<Option<TemperatureTrend>>,
    #[serde(
        rename = "probabilityOfPrecipitation",
        skip_serializing_if = "Option::is_none"
    )]
    pub probability_of_precipitation: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "dewpoint", skip_serializing_if = "Option::is_none")]
    pub dewpoint: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "relativeHumidity", skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "windSpeed", skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<Box<models::GridpointForecastPeriodWindSpeed>>,
    #[serde(
        rename = "windGust",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub wind_gust: Option<Option<Box<models::GridpointForecastPeriodWindGust>>>,
    /// The prevailing direction of the wind for the period, using a 16-point compass.
    #[serde(rename = "windDirection", skip_serializing_if = "Option::is_none")]
    pub wind_direction: Option<WindDirection>,
    /// A link to an icon representing the forecast summary.
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// A brief textual forecast summary for the period.
    #[serde(rename = "shortForecast", skip_serializing_if = "Option::is_none")]
    pub short_forecast: Option<String>,
    /// A detailed textual forecast for the period.
    #[serde(rename = "detailedForecast", skip_serializing_if = "Option::is_none")]
    pub detailed_forecast: Option<String>,
}

impl GridpointForecastPeriod {
    /// An object containing forecast information for a specific time period (generally 12-hour or 1-hour).
    pub fn new() -> GridpointForecastPeriod {
        GridpointForecastPeriod {
            number: None,
            name: None,
            start_time: None,
            end_time: None,
            is_daytime: None,
            temperature: None,
            temperature_unit: None,
            temperature_trend: None,
            probability_of_precipitation: None,
            dewpoint: None,
            relative_humidity: None,
            wind_speed: None,
            wind_gust: None,
            wind_direction: None,
            icon: None,
            short_forecast: None,
            detailed_forecast: None,
        }
    }
}
/// The unit of the temperature value (Fahrenheit or Celsius). This property is deprecated. Future versions will indicate the unit within the quantitative value object for the temperature property. To make use of the future standard format now, set the \"forecast_temperature_qv\" feature flag on the request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemperatureUnit {
    #[serde(rename = "F")]
    F,
    #[serde(rename = "C")]
    C,
}

impl Default for TemperatureUnit {
    fn default() -> TemperatureUnit {
        Self::F
    }
}
/// If not null, indicates a non-diurnal temperature trend for the period (either rising temperature overnight, or falling temperature during the day)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemperatureTrend {
    #[serde(rename = "rising")]
    Rising,
    #[serde(rename = "falling")]
    Falling,
}

impl Default for TemperatureTrend {
    fn default() -> TemperatureTrend {
        Self::Rising
    }
}
/// The prevailing direction of the wind for the period, using a 16-point compass.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WindDirection {
    #[serde(rename = "N")]
    N,
    #[serde(rename = "NNE")]
    Nne,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "ENE")]
    Ene,
    #[serde(rename = "E")]
    E,
    #[serde(rename = "ESE")]
    Ese,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SSE")]
    Sse,
    #[serde(rename = "S")]
    S,
    #[serde(rename = "SSW")]
    Ssw,
    #[serde(rename = "SW")]
    Sw,
    #[serde(rename = "WSW")]
    Wsw,
    #[serde(rename = "W")]
    W,
    #[serde(rename = "WNW")]
    Wnw,
    #[serde(rename = "NW")]
    Nw,
    #[serde(rename = "NNW")]
    Nnw,
}

impl Default for WindDirection {
    fn default() -> WindDirection {
        Self::N
    }
}
