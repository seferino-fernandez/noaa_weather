use serde::{Deserialize, Serialize};

use super::{JsonLdContext, RadarQueue};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarQueuesResponse {
    #[serde(rename = "@context")]
    pub at_context: Box<JsonLdContext>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub radar_queues: Option<Vec<RadarQueue>>,
}
