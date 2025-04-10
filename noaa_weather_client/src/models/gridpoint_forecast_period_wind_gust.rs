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
