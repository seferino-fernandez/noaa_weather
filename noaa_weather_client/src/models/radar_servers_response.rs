use serde::{Deserialize, Serialize};

use super::{JsonLdContext, RadarServer};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServersResponse {
    #[serde(rename = "@context")]
    pub at_context: Box<JsonLdContext>,
    #[serde(rename = "@graph", skip_serializing_if = "Option::is_none")]
    pub radar_servers: Option<Vec<RadarServer>>,
}
