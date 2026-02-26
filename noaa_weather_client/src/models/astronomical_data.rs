use serde::{Deserialize, Serialize};

/// Sunrise, sunset, and twilight information for a location.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AstronomicalData {
    #[serde(rename = "sunrise", skip_serializing_if = "Option::is_none")]
    pub sunrise: Option<String>,
    #[serde(rename = "sunset", skip_serializing_if = "Option::is_none")]
    pub sunset: Option<String>,
    #[serde(rename = "transit", skip_serializing_if = "Option::is_none")]
    pub transit: Option<String>,
    #[serde(rename = "civilTwilightBegin", skip_serializing_if = "Option::is_none")]
    pub civil_twilight_begin: Option<String>,
    #[serde(rename = "civilTwilightEnd", skip_serializing_if = "Option::is_none")]
    pub civil_twilight_end: Option<String>,
    #[serde(
        rename = "nauticalTwilightBegin",
        skip_serializing_if = "Option::is_none"
    )]
    pub nautical_twilight_begin: Option<String>,
    #[serde(
        rename = "nauticalTwilightEnd",
        skip_serializing_if = "Option::is_none"
    )]
    pub nautical_twilight_end: Option<String>,
    #[serde(
        rename = "astronomicalTwilightBegin",
        skip_serializing_if = "Option::is_none"
    )]
    pub astronomical_twilight_begin: Option<String>,
    #[serde(
        rename = "astronomicalTwilightEnd",
        skip_serializing_if = "Option::is_none"
    )]
    pub astronomical_twilight_end: Option<String>,
}
