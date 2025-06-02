use crate::models;
use serde::{Deserialize, Serialize};

/// GridpointForecastPeriodWindGust : Peak wind gust for the period. This property as an string value is deprecated. Future versions will express this value as a quantitative value object. To make use of the future standard format now, set the \"forecast_wind_speed_qv\" feature flag on the request.
/// Peak wind gust for the period. This property as an string value is deprecated. Future versions will express this value as a quantitative value object. To make use of the future standard format now, set the \"forecast_wind_speed_qv\" feature flag on the request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridpointForecastPeriodWindGust {
    QuantitativeValue(Box<models::QuantitativeValue>),
    String(String),
}

impl Default for GridpointForecastPeriodWindGust {
    fn default() -> Self {
        Self::QuantitativeValue(Default::default())
    }
}
