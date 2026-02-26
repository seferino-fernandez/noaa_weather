use serde::{Deserialize, Serialize};

/// NOAA Weather Radio metadata for a point.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoaaWeatherRadio {
    /// Transmitter callsign.
    #[serde(
        rename = "transmitter",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter: Option<Option<String>>,
    /// The SAME code of this point's county.
    #[serde(rename = "sameCode", skip_serializing_if = "Option::is_none")]
    pub same_code: Option<String>,
    /// A link to the area NWR broadcast from this transmitter.
    #[serde(
        rename = "areaBroadcast",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub area_broadcast: Option<Option<String>>,
    /// A link to the local NWR broadcast for this point.
    #[serde(
        rename = "pointBroadcast",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub point_broadcast: Option<Option<String>>,
}
