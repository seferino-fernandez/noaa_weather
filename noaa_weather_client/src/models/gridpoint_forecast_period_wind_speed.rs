use crate::models;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// GridpointForecastPeriodWindSpeed : Wind speed for the period. This property as an string value is deprecated. Future versions will express this value as a quantitative value object. To make use of the future standard format now, set the \"forecast_wind_speed_qv\" feature flag on the request.
/// Wind speed for the period. This property as an string value is deprecated. Future versions will express this value as a quantitative value object. To make use of the future standard format now, set the \"forecast_wind_speed_qv\" feature flag on the request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridpointForecastPeriodWindSpeed {
    QuantitativeValue(Box<models::QuantitativeValue>),
    String(String),
}

impl Display for GridpointForecastPeriodWindSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridpointForecastPeriodWindSpeed::QuantitativeValue(qv) => {
                write!(f, "{:?}", qv.value)
            }
            GridpointForecastPeriodWindSpeed::String(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl Default for GridpointForecastPeriodWindSpeed {
    fn default() -> Self {
        Self::QuantitativeValue(Default::default())
    }
}
