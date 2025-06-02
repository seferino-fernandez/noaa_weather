use crate::models;
use serde::{Deserialize, Serialize};

/// GridpointForecastPeriodTemperature : High/low temperature for the period, depending on whether the period is day or night. This property as an integer value is deprecated. Future versions will express this value as a quantitative value object. To make use of the future standard format now, set the \"forecast_temperature_qv\" feature flag on the request.
/// High/low temperature for the period, depending on whether the period is day or night. This property as an integer value is deprecated. Future versions will express this value as a quantitative value object. To make use of the future standard format now, set the \"forecast_temperature_qv\" feature flag on the request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridpointForecastPeriodTemperature {
    QuantitativeValue(Box<models::QuantitativeValue>),
    Integer(i32),
}

impl Default for GridpointForecastPeriodTemperature {
    fn default() -> Self {
        Self::QuantitativeValue(Default::default())
    }
}
