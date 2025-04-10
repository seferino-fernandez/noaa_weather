use crate::models;
use serde::{Deserialize, Serialize};

/// GridpointForecast : A multi-day forecast for a 2.5km grid square.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointForecast {
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
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<models::GridpointForecastUnits>,
    /// The internal generator class used to create the forecast text (used for NWS debugging).
    #[serde(rename = "forecastGenerator", skip_serializing_if = "Option::is_none")]
    pub forecast_generator: Option<String>,
    /// The time this forecast data was generated.
    #[serde(rename = "generatedAt", skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
    /// The last update time of the data this forecast was generated from.
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(rename = "validTimes", skip_serializing_if = "Option::is_none")]
    pub valid_times: Option<Box<models::Iso8601Interval>>,
    #[serde(rename = "elevation", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<Box<models::QuantitativeValue>>,
    /// An array of forecast periods.
    #[serde(rename = "periods", skip_serializing_if = "Option::is_none")]
    pub periods: Option<Vec<models::GridpointForecastPeriod>>,
}

impl GridpointForecast {
    /// A multi-day forecast for a 2.5km grid square.
    pub fn new() -> GridpointForecast {
        GridpointForecast {
            at_context: None,
            geometry: None,
            units: None,
            forecast_generator: None,
            generated_at: None,
            update_time: None,
            valid_times: None,
            elevation: None,
            periods: None,
        }
    }
}
