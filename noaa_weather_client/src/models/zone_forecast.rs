use crate::models;
use serde::{Deserialize, Serialize};

/// ZoneForecast : An object representing a zone area forecast.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneForecast {
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
    /// An API link to the zone this forecast is for.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    /// The time this zone forecast product was published.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// An array of forecast periods.
    #[serde(rename = "periods", skip_serializing_if = "Option::is_none")]
    pub periods: Option<Vec<models::ZoneForecastPeriodsInner>>,
}

impl ZoneForecast {
    /// An object representing a zone area forecast.
    pub fn new() -> ZoneForecast {
        ZoneForecast {
            at_context: None,
            geometry: None,
            zone: None,
            updated: None,
            periods: None,
        }
    }
}
