use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveAlertsCountResponse {
    /// The total number of active alerts
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// The total number of active alerts affecting land zones
    #[serde(rename = "land", skip_serializing_if = "Option::is_none")]
    pub land: Option<i32>,
    /// The total number of active alerts affecting marine zones
    #[serde(rename = "marine", skip_serializing_if = "Option::is_none")]
    pub marine: Option<i32>,
    /// Active alerts by marine region
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<std::collections::HashMap<String, i32>>,
    /// Active alerts by area (state/territory)
    #[serde(rename = "areas", skip_serializing_if = "Option::is_none")]
    pub areas: Option<std::collections::HashMap<String, i32>>,
    /// Active alerts by NWS public zone or county code
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<std::collections::HashMap<String, i32>>,
}

impl ActiveAlertsCountResponse {
    pub fn new() -> ActiveAlertsCountResponse {
        ActiveAlertsCountResponse {
            total: None,
            land: None,
            marine: None,
            regions: None,
            areas: None,
            zones: None,
        }
    }
}
