use serde::{Deserialize, Serialize};

use super::{JsonLdContext, RadarStationAlarm};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarStationAlarmsResponse {
    #[serde(rename = "@context")]
    pub at_context: Box<JsonLdContext>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub radar_station_alarms: Option<Vec<RadarStationAlarm>>,
}
