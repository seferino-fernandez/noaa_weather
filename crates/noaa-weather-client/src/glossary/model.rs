//! NWS glossary terms returned by `/glossary`.
//!
//! # Requiredness
//!
//! A complete live census on 2026-09-04 found 3,183 entries. Every entry had
//! exactly `term` and `definition`; both keys were present, non-null, and
//! non-empty. The response's empty JSON-LD context is vocabulary metadata and
//! is intentionally not modeled.

use serde::{Deserialize, Serialize};

/// A collection of glossary definitions returned by the NWS API.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct GlossaryResponse {
    /// Glossary terms in the order returned by the service.
    #[serde(default)]
    pub glossary: Vec<GlossaryTerm>,
}

/// A glossary term and its definition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct GlossaryTerm {
    /// Term being defined. Present and non-empty in all 3,183 entries in the
    /// complete 2026-09-04 census.
    pub term: String,
    /// Definition of the term. Present and non-empty in all 3,183 entries in
    /// the complete 2026-09-04 census.
    pub definition: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../tests/fixtures/glossary/terms.json");

    #[test]
    fn captured_terms_have_required_text() {
        let response: GlossaryResponse = serde_json::from_str(FIXTURE).unwrap();

        assert!(!response.glossary.is_empty());
        assert!(
            response.glossary.iter().all(|entry| {
                !entry.term.trim().is_empty() && !entry.definition.trim().is_empty()
            })
        );
    }

    #[test]
    fn missing_term_or_definition_is_rejected() {
        for entry in [
            r#"{"definition":"Ice crystals that fall from a cloud."}"#,
            r#"{"term":"Diamond Dust"}"#,
        ] {
            assert!(serde_json::from_str::<GlossaryTerm>(entry).is_err());
        }
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn public_glossary_models_have_schemas() {
        let response = schemars::schema_for!(GlossaryResponse);
        let term = schemars::schema_for!(GlossaryTerm);
        let response = response.as_value();
        let term = term.as_value();

        assert_eq!(response["properties"]["glossary"]["type"], "array");
        assert_eq!(term["properties"]["term"]["type"], "string");
        assert_eq!(term["properties"]["definition"]["type"], "string");
        assert!(term["required"].as_array().is_some_and(|required| {
            required.contains(&serde_json::json!("term"))
                && required.contains(&serde_json::json!("definition"))
        }));
    }
}
