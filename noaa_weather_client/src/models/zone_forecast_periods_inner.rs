use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneForecastPeriodsInner {
    /// A sequential identifier number.
    #[serde(rename = "number")]
    pub number: i32,
    /// A textual description of the period.
    #[serde(rename = "name")]
    pub name: String,
    /// A detailed textual forecast for the period.
    #[serde(rename = "detailedForecast")]
    pub detailed_forecast: String,
}

impl ZoneForecastPeriodsInner {
    pub fn new(number: i32, name: String, detailed_forecast: String) -> ZoneForecastPeriodsInner {
        ZoneForecastPeriodsInner {
            number,
            name,
            detailed_forecast,
        }
    }
}
