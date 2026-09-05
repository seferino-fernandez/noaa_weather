use std::ops::Deref;

use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct as _};

use super::Geometry;

/// One GeoJSON `Feature`: NOAA's envelope around a single resource.
///
/// Every single-resource GeoJSON operation (`/points/{point}`,
/// `/alerts/{id}`, `/stations/{id}`, `/zones/{type}/{id}`, ...) returns a
/// `Feature<T>` whose `properties` is the resource model from
/// [`crate::models`]. `Feature<T>` dereferences to `T`, so
/// `alert.event` reads the property directly.
///
/// # Field name clash
///
/// [`Feature::id`] is the GeoJSON feature id: the NOAA self-link URL such as
/// `https://api.weather.gov/alerts/urn:oid:2.49...`. Several properties
/// models also have an `id` field with a different meaning; for alerts
/// `feature.properties.id` is the bare URN, for zones it is the zone URL.
/// Because `Deref` never shadows a struct's own fields, `feature.id` always
/// names the envelope id; write `feature.properties.id` for the property.
///
/// # Wire shape
///
/// Serialized as `{"type": "Feature", "id": ..., "geometry": ..., "properties": ...}`.
/// `id` is omitted when absent (Center Weather Advisories have none) and
/// `geometry` is written as `null` when absent, so the output is valid
/// GeoJSON exactly like NOAA's. On input the `type` member is not checked
/// and unknown members, including NOAA's `@context` JSON-LD vocabulary, are
/// ignored; `@context` is the one part of the response this crate drops.
///
/// ```
/// use noaa_weather_client::geo::{Feature, Geometry, Position};
///
/// #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
/// struct Name { name: String }
///
/// let raw = r#"{
///   "@context": ["https://geojson.org/geojson-ld/geojson-context.jsonld"],
///   "id": "https://api.weather.gov/stations/KSLC",
///   "type": "Feature",
///   "geometry": {"type": "Point", "coordinates": [-111.97, 40.77]},
///   "properties": {"name": "Salt Lake City"}
/// }"#;
/// let station: Feature<Name> = serde_json::from_str(raw).unwrap();
/// assert_eq!(station.id.as_deref(), Some("https://api.weather.gov/stations/KSLC"));
/// assert_eq!(station.geometry, Some(Geometry::Point(Position::new(-111.97, 40.77))));
/// assert_eq!(station.name, "Salt Lake City");
///
/// let unlocated = Feature { id: None, geometry: None, properties: Name { name: "x".into() } };
/// assert_eq!(
///     serde_json::to_string(&unlocated).unwrap(),
///     r#"{"type":"Feature","geometry":null,"properties":{"name":"x"}}"#
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct Feature<T> {
    /// The GeoJSON feature id: NOAA's self-link URL for the resource. Absent
    /// on Center Weather Advisories.
    #[serde(default)]
    pub id: Option<String>,
    /// The feature's geometry, or `None` when NOAA sent `null` (most alerts).
    #[serde(default)]
    pub geometry: Option<Geometry>,
    /// The resource itself.
    pub properties: T,
}

impl<T> Deref for Feature<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.properties
    }
}

impl<T: Serialize> Serialize for Feature<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let fields = 3 + usize::from(self.id.is_some());
        let mut state = serializer.serialize_struct("Feature", fields)?;
        state.serialize_field("type", "Feature")?;
        if let Some(id) = &self.id {
            state.serialize_field("id", id)?;
        }
        state.serialize_field("geometry", &self.geometry)?;
        state.serialize_field("properties", &self.properties)?;
        state.end()
    }
}

#[cfg(feature = "schemars")]
impl<T: schemars::JsonSchema> schemars::JsonSchema for Feature<T> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("Feature_{}", T::schema_name()).into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!("{}::Feature<{}>", module_path!(), T::schema_id()).into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "description": "GeoJSON Feature: NOAA's envelope around one resource.",
            "properties": {
                "type": {"type": "string", "const": "Feature"},
                "id": {"type": "string", "description": "NOAA self-link URL for the resource."},
                "geometry": generator.subschema_for::<Option<Geometry>>(),
                "properties": generator.subschema_for::<T>(),
            },
            "required": ["type", "geometry", "properties"],
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::geo::Position;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Props {
        id: Option<String>,
        event: Option<String>,
    }

    fn props() -> Props {
        Props {
            id: Some("urn:oid:1".to_owned()),
            event: Some("Flood Watch".to_owned()),
        }
    }

    #[test]
    fn deserializes_noaa_shape_and_ignores_context_and_type() {
        let feature: Feature<Props> = serde_json::from_value(json!({
            "@context": ["https://geojson.org/geojson-ld/geojson-context.jsonld", {"@version": "1.1"}],
            "id": "https://api.weather.gov/alerts/urn:oid:1",
            "type": "Anything",
            "geometry": null,
            "properties": {"id": "urn:oid:1", "event": "Flood Watch"}
        }))
        .unwrap();
        assert_eq!(
            feature,
            Feature {
                id: Some("https://api.weather.gov/alerts/urn:oid:1".to_owned()),
                geometry: None,
                properties: props(),
            }
        );
    }

    #[test]
    fn missing_id_and_geometry_default_to_none() {
        let feature: Feature<Props> = serde_json::from_value(
            json!({"type": "Feature", "properties": {"id": null, "event": null}}),
        )
        .unwrap();
        assert_eq!(feature.id, None);
        assert_eq!(feature.geometry, None);
        assert!(serde_json::from_value::<Feature<Props>>(json!({"type": "Feature"})).is_err());
    }

    #[test]
    fn serializes_type_first_geometry_null_and_skips_missing_id() {
        let feature = Feature {
            id: None,
            geometry: None,
            properties: props(),
        };
        assert_eq!(
            serde_json::to_string(&feature).unwrap(),
            r#"{"type":"Feature","geometry":null,"properties":{"id":"urn:oid:1","event":"Flood Watch"}}"#
        );
        let located = Feature {
            id: Some("https://api.weather.gov/stations/KSLC".to_owned()),
            geometry: Some(Geometry::Point(Position::new(-111.97, 40.77))),
            properties: props(),
        };
        assert_eq!(
            serde_json::to_string(&located).unwrap(),
            r#"{"type":"Feature","id":"https://api.weather.gov/stations/KSLC","geometry":{"type":"Point","coordinates":[-111.97,40.77]},"properties":{"id":"urn:oid:1","event":"Flood Watch"}}"#
        );
        let reparsed: Feature<Props> =
            serde_json::from_str(&serde_json::to_string(&located).unwrap()).unwrap();
        assert_eq!(reparsed, located);
    }

    #[test]
    fn deref_reads_properties_while_own_fields_win() {
        let feature = Feature {
            id: Some("https://api.weather.gov/alerts/urn:oid:1".to_owned()),
            geometry: None,
            properties: props(),
        };
        assert_eq!(feature.event.as_deref(), Some("Flood Watch"));
        assert_eq!(
            feature.id.as_deref(),
            Some("https://api.weather.gov/alerts/urn:oid:1")
        );
        assert_eq!(feature.properties.id.as_deref(), Some("urn:oid:1"));
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_names_the_envelope_and_embeds_the_properties() {
        #[derive(schemars::JsonSchema)]
        #[allow(dead_code)]
        struct Props {
            event: String,
        }

        let schema = schemars::schema_for!(Feature<Props>);
        let value = schema.as_value();
        assert_eq!(value["properties"]["type"]["const"], "Feature", "{value}");
        assert!(value["properties"]["properties"].is_object(), "{value}");
        assert!(value["properties"].get("geometry").is_some(), "{value}");
    }
}
