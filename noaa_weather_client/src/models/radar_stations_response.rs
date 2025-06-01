use serde::{Deserialize, Serialize};

use super::{JsonLdContext, RadarStationFeature};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarStationsResponse {
    /// JSON-LD context, can be a string, an object, or an array of strings and objects.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<JsonLdContext>>,
    /// The type of the GeoJSON object, typically "FeatureCollection".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// An array of Feature objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<RadarStationFeature>>,
}
