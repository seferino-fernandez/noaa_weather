use crate::models;
use serde::{Deserialize, Serialize};

/// GeoJsonGeometry : A GeoJSON geometry object. Please refer to IETF RFC 7946 for information on the GeoJSON format.
/// A GeoJSON geometry object. Please refer to IETF RFC 7946 for information on the GeoJSON format.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeoJsonGeometry {
    GeoJsonPoint(Box<models::GeoJsonPoint>),
    GeoJsonLineString(Box<models::GeoJsonLineString>),
    GeoJsonPolygon(Box<models::GeoJsonPolygon>),
    GeoJsonMultiPoint(Box<models::GeoJsonMultiPoint>),
    GeoJsonMultiLineString(Box<models::GeoJsonMultiLineString>),
    GeoJsonMultiPolygon(Box<models::GeoJsonMultiPolygon>),
}

impl Default for GeoJsonGeometry {
    fn default() -> Self {
        Self::GeoJsonPoint(Default::default())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Point")]
    Point,
    #[serde(rename = "LineString")]
    LineString,
    #[serde(rename = "Polygon")]
    Polygon,
    #[serde(rename = "MultiPoint")]
    MultiPoint,
    #[serde(rename = "MultiLineString")]
    MultiLineString,
    #[serde(rename = "MultiPolygon")]
    MultiPolygon,
}

impl Default for Type {
    fn default() -> Type {
        Self::Point
    }
}
