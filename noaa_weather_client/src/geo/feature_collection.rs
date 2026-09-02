use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct as _};
use url::Url;

use super::Feature;
use crate::ids::Cursor;
use crate::time::OffsetDateTime;

/// A GeoJSON `FeatureCollection`: NOAA's envelope around a list of resources.
///
/// Every list operation that returns GeoJSON (`/alerts`, `/stations`,
/// `/zones`, `/aviation/sigmets`, ...) produces a `FeatureCollection<T>` of
/// [`Feature<T>`]. NOAA decorates some collections with extra members, all
/// optional here: `/alerts` sends `title` and `updated`; paged operations
/// send `pagination`. The `observationStations` list that station
/// collections carry duplicates `features[].id` and is not kept.
///
/// # Wire shape
///
/// Serialized as `{"type": "FeatureCollection", "features": [...]}` followed
/// by `title`, `updated`, and `pagination` when present. On input the `type`
/// member is not checked and unknown members (`@context`,
/// `observationStations`) are ignored.
///
/// # Paging
///
/// [`FeatureCollection::next_cursor`] extracts the opaque [`Cursor`] from
/// `pagination.next` so it can be placed in the next request's `cursor`
/// field. Only `/alerts`, `/stations`, and `/stations/{id}/observations`
/// page correctly on NOAA's side; see the handle docs for the others.
///
/// ```
/// use noaa_weather_client::geo::FeatureCollection;
///
/// #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
/// struct Name { name: String }
///
/// let raw = r#"{
///   "@context": [],
///   "type": "FeatureCollection",
///   "features": [
///     {"id": "https://api.weather.gov/stations/KSLC", "type": "Feature", "geometry": null,
///      "properties": {"name": "Salt Lake City"}}
///   ],
///   "observationStations": ["https://api.weather.gov/stations/KSLC"],
///   "pagination": {"next": "https://api.weather.gov/stations?limit=1&cursor=eyJzIjoxfQ%3D%3D"}
/// }"#;
/// let page: FeatureCollection<Name> = serde_json::from_str(raw).unwrap();
/// assert_eq!(page.len(), 1);
/// assert_eq!(page.iter().next().unwrap().name, "Salt Lake City");
/// assert_eq!(page.next_cursor().unwrap().as_str(), "eyJzIjoxfQ==");
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FeatureCollection<T> {
    /// The resources on this page, in NOAA's order.
    pub features: Vec<Feature<T>>,
    /// A title for the collection; `/alerts` sends one.
    #[serde(default)]
    pub title: Option<String>,
    /// When the collection last changed; `/alerts` sends one, in UTC
    /// (`+00:00`).
    #[serde(default)]
    pub updated: Option<OffsetDateTime>,
    /// The link to the next page, when NOAA offered one.
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

/// Links for retrieving more of a paged collection.
///
/// `next` is kept as the raw URL NOAA sent so JSON output reproduces it;
/// use [`FeatureCollection::next_cursor`] to obtain the cursor it carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Pagination {
    /// The full URL of the next page.
    pub next: String,
}

impl<T> FeatureCollection<T> {
    /// Returns how many features this page holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Returns whether this page holds no features.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }

    /// Iterates over the features in order.
    pub fn iter(&self) -> impl Iterator<Item = &Feature<T>> {
        self.features.iter()
    }

    /// Returns the cursor for the next page, taken from the `cursor` query
    /// parameter of `pagination.next`.
    ///
    /// Returns `None` when there is no pagination, the link is not a URL,
    /// it has no `cursor` parameter, or the value is not a valid [`Cursor`].
    /// The value is percent-decoded before validation, so NOAA's `%3D`
    /// padding becomes `=`.
    #[must_use]
    pub fn next_cursor(&self) -> Option<Cursor> {
        let next = Url::parse(&self.pagination.as_ref()?.next).ok()?;
        let (_, cursor) = next.query_pairs().find(|(name, _)| name == "cursor")?;
        cursor.parse().ok()
    }
}

impl<T> IntoIterator for FeatureCollection<T> {
    type Item = Feature<T>;
    type IntoIter = std::vec::IntoIter<Feature<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a FeatureCollection<T> {
    type Item = &'a Feature<T>;
    type IntoIter = std::slice::Iter<'a, Feature<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.iter()
    }
}

impl<T: Serialize> Serialize for FeatureCollection<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let fields = 2
            + usize::from(self.title.is_some())
            + usize::from(self.updated.is_some())
            + usize::from(self.pagination.is_some());
        let mut state = serializer.serialize_struct("FeatureCollection", fields)?;
        state.serialize_field("type", "FeatureCollection")?;
        state.serialize_field("features", &self.features)?;
        if let Some(title) = &self.title {
            state.serialize_field("title", title)?;
        }
        if let Some(updated) = &self.updated {
            state.serialize_field("updated", updated)?;
        }
        if let Some(pagination) = &self.pagination {
            state.serialize_field("pagination", pagination)?;
        }
        state.end()
    }
}

#[cfg(feature = "schemars")]
impl<T: schemars::JsonSchema> schemars::JsonSchema for FeatureCollection<T> {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        format!("FeatureCollection_{}", T::schema_name()).into()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        format!("{}::FeatureCollection<{}>", module_path!(), T::schema_id()).into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "description": "GeoJSON FeatureCollection: NOAA's envelope around a list of resources.",
            "properties": {
                "type": {"type": "string", "const": "FeatureCollection"},
                "features": {"type": "array", "items": generator.subschema_for::<Feature<T>>()},
                "title": {"type": "string"},
                "updated": generator.subschema_for::<OffsetDateTime>(),
                "pagination": generator.subschema_for::<Pagination>(),
            },
            "required": ["type", "features"],
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Props {
        name: String,
    }

    fn feature(name: &str) -> Feature<Props> {
        Feature {
            id: Some(format!("https://api.weather.gov/stations/{name}")),
            geometry: None,
            properties: Props {
                name: name.to_owned(),
            },
        }
    }

    fn page(next: Option<&str>) -> FeatureCollection<Props> {
        FeatureCollection {
            features: vec![feature("KSLC"), feature("KDEN")],
            title: None,
            updated: None,
            pagination: next.map(|next| Pagination {
                next: next.to_owned(),
            }),
        }
    }

    #[test]
    fn deserializes_noaa_alert_collection_members_and_ignores_the_rest() {
        let collection: FeatureCollection<Props> = serde_json::from_value(json!({
            "@context": ["https://geojson.org/geojson-ld/geojson-context.jsonld"],
            "type": "FeatureCollection",
            "features": [{"id": "https://api.weather.gov/alerts/urn:oid:1", "type": "Feature",
                          "geometry": null, "properties": {"name": "a"}}],
            "title": "Current watches, warnings, and advisories",
            "updated": "2026-09-02T02:05:00+00:00",
            "observationStations": ["https://api.weather.gov/stations/KSLC"],
            "pagination": {"next": "https://api.weather.gov/alerts?limit=1&cursor=eyJ0IjoxfQ%3D%3D"}
        }))
        .unwrap();
        assert_eq!(collection.len(), 1);
        assert!(!collection.is_empty());
        assert_eq!(
            collection.title.as_deref(),
            Some("Current watches, warnings, and advisories")
        );
        assert_eq!(
            collection.updated.unwrap().to_string(),
            "2026-09-02T02:05:00+00:00"
        );
        assert_eq!(collection.next_cursor().unwrap().as_str(), "eyJ0IjoxfQ==");
        assert_eq!(collection.iter().next().unwrap().name, "a");
    }

    #[test]
    fn bare_collections_default_every_optional_member() {
        let collection: FeatureCollection<Props> =
            serde_json::from_value(json!({"type": "FeatureCollection", "features": []})).unwrap();
        assert!(collection.is_empty());
        assert_eq!(collection.title, None);
        assert_eq!(collection.updated, None);
        assert_eq!(collection.pagination, None);
        assert_eq!(collection.next_cursor(), None);
        assert!(
            serde_json::from_value::<FeatureCollection<Props>>(
                json!({"type": "FeatureCollection"})
            )
            .is_err()
        );
    }

    #[test]
    fn serializes_type_then_features_then_present_extras() {
        let bare = page(None);
        assert_eq!(
            serde_json::to_string(&bare).unwrap(),
            r#"{"type":"FeatureCollection","features":[{"type":"Feature","id":"https://api.weather.gov/stations/KSLC","geometry":null,"properties":{"name":"KSLC"}},{"type":"Feature","id":"https://api.weather.gov/stations/KDEN","geometry":null,"properties":{"name":"KDEN"}}]}"#
        );
        let mut decorated = page(Some("https://api.weather.gov/stations?cursor=abc"));
        decorated.title = Some("Stations".to_owned());
        decorated.updated = Some("2026-09-02T02:05:00Z".parse().unwrap());
        // `serde_json::Value` objects sort their keys, so member order is
        // asserted on the emitted text.
        let text = serde_json::to_string(&decorated).unwrap();
        assert!(text.starts_with(r#"{"type":"FeatureCollection","features":["#));
        assert!(text.ends_with(
            r#"],"title":"Stations","updated":"2026-09-02T02:05:00+00:00","pagination":{"next":"https://api.weather.gov/stations?cursor=abc"}}"#
        ));
        let value: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(value["updated"], "2026-09-02T02:05:00+00:00");
        assert_eq!(
            value["pagination"],
            json!({"next": "https://api.weather.gov/stations?cursor=abc"})
        );
        let reparsed: FeatureCollection<Props> = serde_json::from_value(value).unwrap();
        assert_eq!(reparsed, decorated);
    }

    #[test]
    fn iteration_yields_features_in_order_by_reference_and_by_value() {
        let collection = page(None);
        let borrowed: Vec<&str> = (&collection)
            .into_iter()
            .map(|feature| feature.name.as_str())
            .collect();
        assert_eq!(borrowed, ["KSLC", "KDEN"]);
        let owned: Vec<String> = collection
            .into_iter()
            .map(|feature| feature.properties.name)
            .collect();
        assert_eq!(owned, ["KSLC", "KDEN"]);
    }

    #[test]
    fn next_cursor_decodes_percent_encoding_and_rejects_bad_links() {
        let good = page(Some(
            "https://api.weather.gov/alerts?limit=2&cursor=eyJ0IjoxNzU2Nzc0NzAwfQ%3D%3D",
        ));
        assert_eq!(
            good.next_cursor().unwrap().as_str(),
            "eyJ0IjoxNzU2Nzc0NzAwfQ=="
        );
        let observations = page(Some(
            "https://api.weather.gov/stations/KSLC/observations?cursor=eyJzIjoiMjAyNi0wOS0wMlQwMjowNTowMCswMDowMCJ9",
        ));
        assert_eq!(
            observations.next_cursor().unwrap().as_str(),
            "eyJzIjoiMjAyNi0wOS0wMlQwMjowNTowMCswMDowMCJ9"
        );
        for next in [
            "not a url",
            "https://api.weather.gov/alerts?limit=2",
            "https://api.weather.gov/alerts?cursor=",
            "https://api.weather.gov/alerts?cursor=has%20space",
            "https://api.weather.gov/alerts?cursor=bad!chars",
        ] {
            assert_eq!(page(Some(next)).next_cursor(), None, "{next}");
        }
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_describes_the_envelope_members() {
        #[derive(schemars::JsonSchema)]
        #[allow(dead_code)]
        struct Props {
            name: String,
        }

        let schema = schemars::schema_for!(FeatureCollection<Props>);
        let value = schema.as_value();
        assert_eq!(
            value["properties"]["type"]["const"], "FeatureCollection",
            "{value}"
        );
        assert_eq!(value["properties"]["features"]["type"], "array", "{value}");
        assert!(value["properties"].get("pagination").is_some(), "{value}");
    }
}
