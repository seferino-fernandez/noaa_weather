use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use super::Coordinates;
use crate::ids::InvalidValue;

/// One GeoJSON position: a `[longitude, latitude]` pair in decimal degrees.
///
/// GeoJSON (RFC 7946) orders positions longitude first, the opposite of
/// NOAA's `latitude,longitude` request form held by [`Coordinates`]. A
/// position is the raw wire value: it is not range-checked or rounded, so
/// it reproduces NOAA's geometry exactly. Convert to [`Coordinates`] with
/// `TryFrom` when a position is needed as a request value.
///
/// On the wire a position is a JSON array. A third element (elevation) is
/// accepted on input and dropped; NOAA does not send one.
///
/// ```
/// use noaa_weather_client::{Coordinates, geo::Position};
///
/// let position: Position = serde_json::from_str("[-97.0892, 39.7456]").unwrap();
/// assert_eq!(position.lon(), -97.0892);
/// assert_eq!(position.lat(), 39.7456);
/// assert_eq!(serde_json::to_string(&position).unwrap(), "[-97.0892,39.7456]");
///
/// let coordinates = Coordinates::try_from(position)?;
/// assert_eq!(coordinates.to_string(), "39.7456,-97.0892");
/// # Ok::<(), noaa_weather_client::InvalidValue>(())
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Position([f64; 2]);

impl Position {
    /// Creates a position from a longitude and a latitude, in that order.
    #[must_use]
    pub const fn new(lon: f64, lat: f64) -> Self {
        Self([lon, lat])
    }

    /// Returns the longitude in decimal degrees.
    #[must_use]
    pub const fn lon(&self) -> f64 {
        self.0[0]
    }

    /// Returns the latitude in decimal degrees.
    #[must_use]
    pub const fn lat(&self) -> f64 {
        self.0[1]
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Position({}, {})", self.lon(), self.lat())
    }
}

impl TryFrom<Position> for Coordinates {
    type Error = InvalidValue;

    fn try_from(position: Position) -> Result<Self, Self::Error> {
        Self::new(position.lat(), position.lon())
    }
}

impl Serialize for Position {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(
                    "a GeoJSON position: [longitude, latitude] with an optional elevation",
                )
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Position, A::Error> {
                let lon: f64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let lat: f64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let _elevation: Option<f64> = seq.next_element()?;
                if seq.next_element::<de::IgnoredAny>()?.is_some() {
                    return Err(de::Error::invalid_length(4, &self));
                }
                Ok(Position::new(lon, lat))
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for Position {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Position")
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(concat!(module_path!(), "::Position"))
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "array",
            "description": "GeoJSON position as [longitude, latitude] in decimal degrees; an optional third element (elevation) is accepted and ignored.",
            "items": {"type": "number"},
            "minItems": 2,
            "maxItems": 3,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors_follow_lon_lat_order() {
        let position = Position::new(-97.0892, 39.7456);
        assert_eq!(position.lon(), -97.0892);
        assert_eq!(position.lat(), 39.7456);
        assert_eq!(format!("{position:?}"), "Position(-97.0892, 39.7456)");
    }

    #[test]
    fn serde_uses_a_two_element_array_and_accepts_elevation() {
        let position = Position::new(-97.0892, 39.7456);
        assert_eq!(
            serde_json::to_string(&position).unwrap(),
            "[-97.0892,39.7456]"
        );
        assert_eq!(
            serde_json::from_str::<Position>("[-97.0892, 39.7456]").unwrap(),
            position
        );
        assert_eq!(
            serde_json::from_str::<Position>("[-97.0892, 39.7456, 1500.0]").unwrap(),
            position
        );
        assert_eq!(
            serde_json::from_str::<Position>("[-97, 39]").unwrap(),
            Position::new(-97.0, 39.0)
        );
    }

    #[test]
    fn serde_rejects_other_shapes() {
        for input in [
            "[]",
            "[1.0]",
            "[1.0, 2.0, 3.0, 4.0]",
            "[\"a\", \"b\"]",
            "{\"lon\": 1.0, \"lat\": 2.0}",
            "\"1,2\"",
            "null",
        ] {
            assert!(
                serde_json::from_str::<Position>(input).is_err(),
                "{input} should be rejected"
            );
        }
    }

    #[test]
    fn converts_to_validated_coordinates() {
        let coordinates = Coordinates::try_from(Position::new(-97.08919, 39.74561)).unwrap();
        assert_eq!(coordinates.to_string(), "39.7456,-97.0892");
        let error = Coordinates::try_from(Position::new(-200.0, 0.0)).unwrap_err();
        assert_eq!(error.kind(), crate::ValueKind::Coordinates);
        assert!(Coordinates::try_from(Position::new(0.0, 91.0)).is_err());
    }

    #[test]
    fn positions_do_not_validate_or_round() {
        let raw = Position::new(-200.123456, 95.0);
        assert_eq!(raw.lon(), -200.123456);
        assert_eq!(raw.lat(), 95.0);
        let json = serde_json::to_string(&raw).unwrap();
        assert_eq!(serde_json::from_str::<Position>(&json).unwrap(), raw);
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_is_a_bounded_number_array() {
        let schema = schemars::schema_for!(Position);
        let value = schema.as_value();
        assert_eq!(value["type"], "array");
        assert_eq!(value["items"]["type"], "number");
        assert_eq!(value["minItems"], 2);
        assert_eq!(value["maxItems"], 3);
    }
}
