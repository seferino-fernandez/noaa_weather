use serde::{Deserialize, Serialize};

/// AlertGeocode : Lists of codes for NWS public zones and counties affected by the alert.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertGeocode {
    /// A list of NWS public zone or county identifiers.
    #[serde(rename = "UGC", skip_serializing_if = "Option::is_none")]
    pub ugc: Option<Vec<String>>,
    /// A list of SAME (Specific Area Message Encoding) codes for affected counties.
    #[serde(rename = "SAME", skip_serializing_if = "Option::is_none")]
    pub same: Option<Vec<String>>,
}

impl AlertGeocode {
    /// Lists of codes for NWS public zones and counties affected by the alert.
    pub fn new() -> AlertGeocode {
        AlertGeocode {
            ugc: None,
            same: None,
        }
    }
}
