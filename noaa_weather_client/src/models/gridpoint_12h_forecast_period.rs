use std::fmt::Display;
use std::str::FromStr;

use crate::models;
use serde::{Deserialize, Serialize};
use serde_with::{NoneAsEmptyString, serde_as};

#[serde_as]
/// Gridpoint12hForecastPeriod : An object containing forecast information for a 12-hour time period.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gridpoint12hForecastPeriod {
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
    pub temperature: Option<Box<models::QuantitativeValue>>,
    /// If not null, indicates a non-diurnal temperature trend for the period (either rising temperature overnight, or falling temperature during the day)
    #[serde_as(as = "Option<NoneAsEmptyString>")]
    pub temperature_trend: Option<Option<TemperatureTrend>>,
    #[serde(
        rename = "probabilityOfPrecipitation",
        skip_serializing_if = "Option::is_none"
    )]
    pub probability_of_precipitation: Option<Box<models::QuantitativeValue>>,
    #[serde(rename = "windSpeed", skip_serializing_if = "Option::is_none")]
    pub wind_speed: Option<Box<models::QuantitativeValue>>,
    #[serde(
        rename = "windGust",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub wind_gust: Option<Option<Box<models::QuantitativeValue>>>,
    /// The prevailing direction of the wind for the period, using a 16-point compass.
    #[serde_as(as = "Option<NoneAsEmptyString>")]
    #[serde(rename(deserialize = "windDirection"))]
    pub wind_direction: Option<Option<WindDirection>>,
    /// A brief textual forecast summary for the period.
    #[serde(rename = "shortForecast", skip_serializing_if = "Option::is_none")]
    pub short_forecast: Option<String>,
    /// A detailed textual forecast for the period.
    #[serde(rename = "detailedForecast", skip_serializing_if = "Option::is_none")]
    pub detailed_forecast: Option<String>,
}

impl Gridpoint12hForecastPeriod {
    /// An object containing forecast information for a 12-hour time period.
    pub fn new() -> Gridpoint12hForecastPeriod {
        Gridpoint12hForecastPeriod {
            number: None,
            name: None,
            start_time: None,
            end_time: None,
            is_daytime: None,
            temperature: None,
            temperature_trend: None,
            probability_of_precipitation: None,
            wind_speed: None,
            wind_gust: None,
            wind_direction: None,
            short_forecast: None,
            detailed_forecast: None,
        }
    }
}
/// If not null, indicates a non-diurnal temperature trend for the period (either rising temperature overnight, or falling temperature during the day)
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum TemperatureTrend {
    #[serde(rename = "rising")]
    #[default]
    Rising,
    #[serde(rename = "falling")]
    Falling,
}

impl Display for TemperatureTrend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl FromStr for TemperatureTrend {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lower_string = string.to_lowercase();
        match lower_string.as_str() {
            "rising" => Ok(TemperatureTrend::Rising),
            "falling" => Ok(TemperatureTrend::Falling),
            _ => Err(format!("Invalid temperature trend: {string}")),
        }
    }
}

/// The prevailing direction of the wind for the period, using a 16-point compass.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
#[derive(Default)]
pub enum WindDirection {
    #[serde(rename = "N")]
    #[default]
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

impl Display for WindDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{self:?}").to_uppercase())
    }
}

impl FromStr for WindDirection {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lower_string = string.to_lowercase();
        match lower_string.as_str() {
            "n" => Ok(WindDirection::N),
            "nne" => Ok(WindDirection::Nne),
            "ne" => Ok(WindDirection::Ne),
            "ene" => Ok(WindDirection::Ene),
            "e" => Ok(WindDirection::E),
            "ese" => Ok(WindDirection::Ese),
            "se" => Ok(WindDirection::Se),
            "sse" => Ok(WindDirection::Sse),
            "s" => Ok(WindDirection::S),
            "ssw" => Ok(WindDirection::Ssw),
            "sw" => Ok(WindDirection::Sw),
            "wsw" => Ok(WindDirection::Wsw),
            "w" => Ok(WindDirection::W),
            "wnw" => Ok(WindDirection::Wnw),
            "nw" => Ok(WindDirection::Nw),
            "nnw" => Ok(WindDirection::Nnw),
            _ => Err(format!("Invalid wind direction: {string}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Gridpoint12hForecastPeriod;

    #[test]
    fn quantitative_forecast_ignores_removed_temperature_unit_and_icon() {
        let period: Gridpoint12hForecastPeriod = serde_json::from_str(
            r#"{
                "temperature":{"value":72,"unitCode":"wmoUnit:degF"},
                "temperatureUnit":"F",
                "windSpeed":{"value":10,"unitCode":"wmoUnit:mi_h-1"},
                "windGust":{"value":18,"unitCode":"wmoUnit:mi_h-1"},
                "icon":"https://example.test/legacy.png"
            }"#,
        )
        .unwrap();

        assert_eq!(period.temperature.as_ref().unwrap().value, Some(Some(72.0)));
        assert_eq!(
            period.wind_speed.as_ref().unwrap().unit_code.as_deref(),
            Some("wmoUnit:mi_h-1")
        );
        assert_eq!(
            period.wind_gust.as_ref().unwrap().as_ref().unwrap().value,
            Some(Some(18.0))
        );
        let serialized = serde_json::to_value(period).unwrap();
        assert!(serialized.get("temperatureUnit").is_none());
        assert!(serialized.get("icon").is_none());
    }
}
