use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct as _};

use super::Position;

/// A GeoJSON geometry object (RFC 7946, section 3.1).
///
/// Every variant serializes as `{"type": "<Variant>", "coordinates": ...}`
/// except [`Geometry::GeometryCollection`], which uses `geometries` as the
/// standard requires. Positions are `[longitude, latitude]` arrays; see
/// [`Position`]. A `bbox` member on input is ignored.
///
/// NOAA sends `Point` for stations and gridpoints, `Polygon` and
/// `MultiPolygon` for zones and alert areas, and `null` (no geometry) for
/// most alerts, which is why [`Feature::geometry`](super::Feature::geometry)
/// is optional.
///
/// ```
/// use noaa_weather_client::geo::{Geometry, Position};
///
/// let geometry: Geometry =
///     serde_json::from_str(r#"{"type":"Point","coordinates":[-97.0892,39.7456]}"#).unwrap();
/// assert_eq!(geometry, Geometry::Point(Position::new(-97.0892, 39.7456)));
/// assert_eq!(
///     serde_json::to_string(&geometry).unwrap(),
///     r#"{"type":"Point","coordinates":[-97.0892,39.7456]}"#
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Geometry {
    /// A single position.
    Point(Position),
    /// Two or more positions forming a line.
    LineString(Vec<Position>),
    /// One outer ring followed by any holes; each ring is closed.
    Polygon(Vec<Vec<Position>>),
    /// A set of positions.
    MultiPoint(Vec<Position>),
    /// A set of lines.
    MultiLineString(Vec<Vec<Position>>),
    /// A set of polygons.
    MultiPolygon(Vec<Vec<Vec<Position>>>),
    /// A heterogeneous set of geometries, serialized under `geometries`.
    GeometryCollection(Vec<Geometry>),
}

impl Geometry {
    /// Returns the GeoJSON `type` member for this geometry.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Point(_) => "Point",
            Self::LineString(_) => "LineString",
            Self::Polygon(_) => "Polygon",
            Self::MultiPoint(_) => "MultiPoint",
            Self::MultiLineString(_) => "MultiLineString",
            Self::MultiPolygon(_) => "MultiPolygon",
            Self::GeometryCollection(_) => "GeometryCollection",
        }
    }
}

impl Serialize for Geometry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Geometry", 2)?;
        state.serialize_field("type", self.type_name())?;
        match self {
            Self::Point(coordinates) => state.serialize_field("coordinates", coordinates)?,
            Self::LineString(coordinates) | Self::MultiPoint(coordinates) => {
                state.serialize_field("coordinates", coordinates)?;
            }
            Self::Polygon(coordinates) | Self::MultiLineString(coordinates) => {
                state.serialize_field("coordinates", coordinates)?;
            }
            Self::MultiPolygon(coordinates) => state.serialize_field("coordinates", coordinates)?,
            Self::GeometryCollection(geometries) => {
                state.serialize_field("geometries", geometries)?;
            }
        }
        state.end()
    }
}

/// The wire shape: an internally tagged object whose payload key depends on
/// the variant. Struct variants ignore unknown members such as `bbox`.
#[derive(Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "type")]
enum Wire {
    Point {
        coordinates: Position,
    },
    LineString {
        coordinates: Vec<Position>,
    },
    Polygon {
        coordinates: Vec<Vec<Position>>,
    },
    MultiPoint {
        coordinates: Vec<Position>,
    },
    MultiLineString {
        coordinates: Vec<Vec<Position>>,
    },
    MultiPolygon {
        coordinates: Vec<Vec<Vec<Position>>>,
    },
    GeometryCollection {
        geometries: Vec<Geometry>,
    },
}

impl From<Wire> for Geometry {
    fn from(wire: Wire) -> Self {
        match wire {
            Wire::Point { coordinates } => Self::Point(coordinates),
            Wire::LineString { coordinates } => Self::LineString(coordinates),
            Wire::Polygon { coordinates } => Self::Polygon(coordinates),
            Wire::MultiPoint { coordinates } => Self::MultiPoint(coordinates),
            Wire::MultiLineString { coordinates } => Self::MultiLineString(coordinates),
            Wire::MultiPolygon { coordinates } => Self::MultiPolygon(coordinates),
            Wire::GeometryCollection { geometries } => Self::GeometryCollection(geometries),
        }
    }
}

impl<'de> Deserialize<'de> for Geometry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Wire::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Geometry {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Geometry")
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(concat!(module_path!(), "::Geometry"))
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = <Wire as schemars::JsonSchema>::json_schema(generator);
        schema.insert(
            "description".to_owned(),
            "GeoJSON geometry object (RFC 7946): type plus coordinates, or geometries for a GeometryCollection.".into(),
        );
        schema
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn position(lon: f64, lat: f64) -> Position {
        Position::new(lon, lat)
    }

    #[test]
    fn all_seven_shapes_round_trip_with_their_wire_keys() {
        let ring = vec![
            position(-1.0, 1.0),
            position(1.0, 1.0),
            position(1.0, -1.0),
            position(-1.0, 1.0),
        ];
        let cases = [
            (
                Geometry::Point(position(-97.0892, 39.7456)),
                json!({"type": "Point", "coordinates": [-97.0892, 39.7456]}),
            ),
            (
                Geometry::LineString(vec![position(0.0, 0.0), position(1.0, 1.0)]),
                json!({"type": "LineString", "coordinates": [[0.0, 0.0], [1.0, 1.0]]}),
            ),
            (
                Geometry::Polygon(vec![ring.clone()]),
                json!({"type": "Polygon", "coordinates": [[[-1.0, 1.0], [1.0, 1.0], [1.0, -1.0], [-1.0, 1.0]]]}),
            ),
            (
                Geometry::MultiPoint(vec![position(0.0, 0.0), position(1.0, 1.0)]),
                json!({"type": "MultiPoint", "coordinates": [[0.0, 0.0], [1.0, 1.0]]}),
            ),
            (
                Geometry::MultiLineString(vec![vec![position(0.0, 0.0), position(1.0, 1.0)]]),
                json!({"type": "MultiLineString", "coordinates": [[[0.0, 0.0], [1.0, 1.0]]]}),
            ),
            (
                Geometry::MultiPolygon(vec![vec![ring.clone()]]),
                json!({"type": "MultiPolygon", "coordinates": [[[[-1.0, 1.0], [1.0, 1.0], [1.0, -1.0], [-1.0, 1.0]]]]}),
            ),
            (
                Geometry::GeometryCollection(vec![
                    Geometry::Point(position(0.0, 0.0)),
                    Geometry::LineString(vec![position(0.0, 0.0), position(1.0, 1.0)]),
                ]),
                json!({"type": "GeometryCollection", "geometries": [
                    {"type": "Point", "coordinates": [0.0, 0.0]},
                    {"type": "LineString", "coordinates": [[0.0, 0.0], [1.0, 1.0]]},
                ]}),
            ),
        ];
        for (geometry, wire) in cases {
            assert_eq!(
                serde_json::to_value(&geometry).unwrap(),
                wire,
                "{geometry:?}"
            );
            let parsed: Geometry = serde_json::from_value(wire.clone()).unwrap();
            assert_eq!(parsed, geometry, "{wire}");
            assert_eq!(
                wire["type"].as_str().unwrap(),
                geometry.type_name(),
                "{geometry:?}"
            );
        }
    }

    #[test]
    fn type_key_is_emitted_first_and_bbox_is_ignored_on_input() {
        let point = Geometry::Point(position(-97.0892, 39.7456));
        assert_eq!(
            serde_json::to_string(&point).unwrap(),
            r#"{"type":"Point","coordinates":[-97.0892,39.7456]}"#
        );
        let with_bbox: Geometry = serde_json::from_str(
            r#"{"type":"Point","bbox":[-97.1,39.7,-97.0,39.8],"coordinates":[-97.0892,39.7456]}"#,
        )
        .unwrap();
        assert_eq!(with_bbox, point);
    }

    #[test]
    fn rejects_unknown_types_and_mismatched_payloads() {
        for input in [
            r#"{"type":"Circle","coordinates":[0.0,0.0]}"#,
            r#"{"coordinates":[0.0,0.0]}"#,
            r#"{"type":"Point","coordinates":[[0.0,0.0]]}"#,
            r#"{"type":"Point","geometries":[]}"#,
            r#"{"type":"GeometryCollection","coordinates":[0.0,0.0]}"#,
            "null",
        ] {
            assert!(
                serde_json::from_str::<Geometry>(input).is_err(),
                "{input} should be rejected"
            );
        }
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_is_one_object_per_variant_with_a_type_tag() {
        let schema = schemars::schema_for!(Geometry);
        let value = schema.as_value();
        let variants = value["oneOf"].as_array().expect("oneOf variants");
        assert_eq!(variants.len(), 7, "{value}");
        let collection = variants
            .iter()
            .find(|variant| variant["properties"]["type"]["const"] == "GeometryCollection")
            .expect("GeometryCollection variant");
        assert!(collection["properties"].get("geometries").is_some());
        assert!(collection["properties"].get("coordinates").is_none());
    }
}
