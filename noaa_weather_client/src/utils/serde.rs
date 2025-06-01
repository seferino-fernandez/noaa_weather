use serde::de::{MapAccess, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

/// Deserializes a field that can be a JSON map, an empty JSON array, or JSON null
/// into an `Option<HashMap<K, V>>`.
///
/// # Behavior:
/// - A JSON map (e.g., `{"key": true}`) becomes `Some(HashMap<...>)`.
/// - An empty JSON array (e.g., `[]`) becomes `Some(HashMap::new())`. This handles cases
///   where an API might use an empty array to represent an empty map.
/// - JSON `null` becomes `None`.
/// - If the field itself is missing in the JSON structure, Serde's default handling for
///   `Option<T>` will also result in `None`.
pub fn deserialize_optional_map_or_empty_array<'de, D, K, V>(
    deserializer: D,
) -> Result<Option<HashMap<K, V>>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Eq + std::hash::Hash,
    V: Deserialize<'de>,
{
    struct OptionalMapOrEmptyArrayVisitor<Key, Value> {
        marker: PhantomData<Option<HashMap<Key, Value>>>,
    }

    impl<'de, K, V> Visitor<'de> for OptionalMapOrEmptyArrayVisitor<K, V>
    where
        K: Deserialize<'de> + Eq + std::hash::Hash,
        V: Deserialize<'de>,
    {
        type Value = Option<HashMap<K, V>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map, an empty array, or null")
        }

        /// Handles the case where the JSON value is an object (e.g., `{"id1": true, "id2": false}`).
        fn visit_map<M>(self, mut map_access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = HashMap::with_capacity(map_access.size_hint().unwrap_or(0));
            while let Some((key, value)) = map_access.next_entry()? {
                map.insert(key, value);
            }
            Ok(Some(map))
        }

        /// Handles the case where the JSON value is a sequence (e.g., `[]`).
        /// We specifically expect an *empty* sequence to be treated as an empty map.
        fn visit_seq<S>(self, mut seq_access: S) -> Result<Self::Value, S::Error>
        where
            S: SeqAccess<'de>,
        {
            // Attempt to deserialize the first element using `serde_json::Value`
            // as a generic placeholder. We only care if the sequence is empty or not.
            if seq_access.next_element::<serde_json::Value>()?.is_some() {
                // If `next_element` returns `Ok(Some(_))`, the sequence is not empty.
                // This is an unexpected type in this context, as we are looking for
                // an empty array to substitute for a map, not a populated array.
                return Err(serde::de::Error::invalid_type(
                    Unexpected::Seq,
                    &"an empty array (when a map or empty array is expected for this field)",
                ));
            }
            // If `next_element()` returned `Ok(None)`, the sequence is empty.
            // Interpret this empty array as an empty map, wrapped in `Some`.
            Ok(Some(HashMap::new()))
        }

        /// Handles the case where the JSON value is `null`.
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            // `null` in JSON should deserialize to `None` for an `Option` field.
            Ok(None)
        }
    }

    // `deserialize_any` allows the Visitor to inspect the actual JSON type (map, sequence, null, etc.)
    // and decide how to handle it, providing the needed flexibility.
    deserializer.deserialize_any(OptionalMapOrEmptyArrayVisitor {
        marker: PhantomData,
    })
}
