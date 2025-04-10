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
/// For values in observation records, the quality control flag from the MADIS system. The definitions of these flags can be found at https://madis.ncep.noaa.gov/madis_sfc_qc_notes.shtml
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QualityControl {
    #[serde(rename = "Z")]
    Z,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "S")]
    S,
    #[serde(rename = "V")]
    V,
    #[serde(rename = "X")]
    X,
    #[serde(rename = "Q")]
    Q,
    #[serde(rename = "G")]
    G,
    #[serde(rename = "B")]
    B,
    #[serde(rename = "T")]
    T,
}

impl Default for QualityControl {
    fn default() -> QualityControl {
        Self::Z
    }
}
