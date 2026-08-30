use serde::{Deserialize, Serialize};

use super::JsonLdContext;

/// A collection of glossary definitions returned by the NWS API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GlossaryResponse {
    /// JSON-LD context supplied with the response.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<JsonLdContext>>,
    /// Glossary terms in the order returned by the service.
    #[serde(default)]
    pub glossary: Vec<GlossaryTerm>,
}

/// A glossary term and its definition.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GlossaryTerm {
    /// Term being defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// Definition of the term.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}
