//! Checks that a [`Summarize`] impl accounts for every NOAA property.

use std::collections::BTreeSet;

use serde_json::Value as Json;

use crate::{Summarize, SummaryOptions};

/// NOAA property keys of `value` that the summary neither shows nor lists in
/// [`Summarize::OMITTED`], sorted.
///
/// Keys are gathered from the top-level JSON object, from `properties` when it
/// is an object (a GeoJSON `Feature`) and from every `features[].properties`
/// object (a `FeatureCollection`). An empty result means every property is
/// either rendered or deliberately omitted with a reason.
///
/// The summary is taken under [`SummaryOptions::default`] and there is no
/// argument for anything else: which keys an impl accounts for is a property
/// of the impl, not of the unit system it was asked to speak.
///
/// # Panics
///
/// Panics if `value` cannot be serialized to JSON, which no NOAA model does.
pub fn coverage_gaps<T: Summarize>(value: &T) -> Vec<String> {
    let json = serde_json::to_value(value).expect("summarized values serialize to JSON");
    let shown = value.summarize(&SummaryOptions::default()).keys();
    let omitted: BTreeSet<&str> = T::OMITTED.iter().map(|(key, _)| *key).collect();

    property_keys(&json)
        .into_iter()
        .filter(|key| !shown.contains(key.as_str()) && !omitted.contains(key.as_str()))
        .collect()
}

fn property_keys(json: &Json) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    let Some(object) = json.as_object() else {
        return keys;
    };
    keys.extend(object.keys().cloned());
    if let Some(properties) = object.get("properties").and_then(Json::as_object) {
        keys.extend(properties.keys().cloned());
    }
    if let Some(features) = object.get("features").and_then(Json::as_array) {
        for feature in features.iter().filter_map(Json::as_object) {
            // A feature's own envelope keys (`id`, `type`, `geometry`) count
            // too, so a collection cannot silently drop geometry; its
            // `properties` object is expanded rather than listed.
            keys.extend(feature.keys().filter(|key| *key != "properties").cloned());
            if let Some(properties) = feature.get("properties").and_then(Json::as_object) {
                keys.extend(properties.keys().cloned());
            }
        }
    }
    keys
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::*;
    use crate::{Fact, Section, Summary, Value};

    #[derive(Serialize)]
    struct Sample {
        shown: u32,
        omitted: u32,
        uncovered: u32,
    }

    impl Summarize for Sample {
        fn summarize(&self, _options: &SummaryOptions) -> Summary {
            Summary::new("Sample").push(Section::Facts {
                heading: None,
                facts: vec![Fact::new(
                    "Shown",
                    Some("shown"),
                    Value::count(u64::from(self.shown)),
                )],
            })
        }

        const OMITTED: &'static [(&'static str, &'static str)] =
            &[("omitted", "left out on purpose for this test")];
    }

    #[test]
    fn reports_only_keys_neither_shown_nor_omitted() {
        let sample = Sample {
            shown: 1,
            omitted: 2,
            uncovered: 3,
        };
        assert_eq!(coverage_gaps(&sample), vec!["uncovered".to_owned()]);
    }

    #[derive(Serialize)]
    struct Collection {
        features: Vec<Json>,
    }

    impl Summarize for Collection {
        fn summarize(&self, _options: &SummaryOptions) -> Summary {
            Summary::new("Collection").push(Section::Table {
                heading: None,
                columns: vec![crate::Column::new("Event", Some("event"))],
                rows: Vec::new(),
            })
        }

        const OMITTED: &'static [(&'static str, &'static str)] = &[("features", "the rows")];
    }

    #[test]
    fn walks_feature_collection_properties() {
        let collection = Collection {
            features: vec![
                serde_json::json!({ "properties": { "event": "Flood", "sent": "x" } }),
                serde_json::json!({ "properties": { "event": "Wind", "web": "y" } }),
            ],
        };
        assert_eq!(
            coverage_gaps(&collection),
            vec!["sent".to_owned(), "web".to_owned()]
        );
    }

    #[derive(Serialize)]
    struct Feature {
        properties: Json,
    }

    impl Summarize for Feature {
        fn summarize(&self, _options: &SummaryOptions) -> Summary {
            Summary::new("Feature")
        }

        const OMITTED: &'static [(&'static str, &'static str)] = &[("properties", "the body")];
    }

    #[test]
    fn walks_feature_properties() {
        let feature = Feature {
            properties: serde_json::json!({ "id": "a" }),
        };
        assert_eq!(coverage_gaps(&feature), vec!["id".to_owned()]);
    }
}
