use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoJsonMultiLineString {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "coordinates")]
    pub coordinates: Vec<Vec<Vec<f64>>>,
    /// A GeoJSON bounding box. Please refer to IETF RFC 7946 for information on the GeoJSON format.
    #[serde(rename = "bbox", skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,
}

impl GeoJsonMultiLineString {
    pub fn new(r#type: Type, coordinates: Vec<Vec<Vec<f64>>>) -> GeoJsonMultiLineString {
        GeoJsonMultiLineString {
            r#type,
            coordinates,
            bbox: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MultiLineString")]
    MultiLineString,
}

impl Default for Type {
    fn default() -> Type {
        Self::MultiLineString
    }
}
