//! Curated NOAA Weather Radio transmitter metadata and SSML broadcasts.
//!
//! # Requiredness
//!
//! A live audit on 2026-09-04 inspected 500 records from `/radio`, all 128
//! records from `/zones/county/AZC013/radio`, and the direct `/radio/KEC94`
//! response. Every transmitter carried the same ten non-null data fields.
//! The catalog response carried pagination while the county response did not,
//! so pagination remains optional. JSON-LD context is vocabulary metadata and
//! is deliberately not part of these curated models.
//!
//! Broadcast endpoints return SSML rather than JSON. Their public tree keeps
//! the spoken text, `<say-as>` pronunciation hints, and metadata marks while
//! [`Sentence::full_text`] provides the semantic transcript used by human
//! summaries.

use serde::{Deserialize, Serialize};
use url::Url;

use crate::geo::Pagination;
use crate::ids::{CallSign, Cursor, ZoneId};
use crate::models::StateTerritoryCode;

/// Metadata for one NOAA Weather Radio transmitter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadioTransmitter {
    /// Canonical transmitter URL.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// JSON-LD resource type. NOAA currently sends `wx:Transmitter`.
    #[serde(rename = "@type")]
    pub at_type: String,
    /// Identifier of the transmitter data set.
    pub set_id: String,
    /// Transmitter call sign.
    pub call_sign: CallSign,
    /// Transmitter frequency exactly as supplied by NOAA, such as `162.550`.
    #[serde(rename = "transmitterFrequency")]
    pub frequency: String,
    /// Transmitter site name.
    pub site_name: String,
    /// Transmitter site city or locality.
    #[serde(rename = "siteCity")]
    pub city: String,
    /// State or territory containing the transmitter site.
    #[serde(rename = "siteState")]
    pub state: StateTerritoryCode,
    /// Six-digit Specific Area Message Encoding codes covered by the
    /// transmitter, preserving NOAA's order and representation.
    pub same_codes: Vec<String>,
    /// County zones covered by the transmitter, preserving NOAA's order.
    pub counties: Vec<ZoneId>,
}

/// A paginated JSON-LD collection of NOAA Weather Radio transmitters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadioTransmitterCollection {
    /// Transmitters in NOAA's response order.
    #[serde(rename = "@graph", default)]
    pub transmitters: Vec<RadioTransmitter>,
    /// Link to the next page. County-specific responses omit it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

impl RadioTransmitterCollection {
    /// Returns how many transmitters this page holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.transmitters.len()
    }

    /// Returns whether this page holds no transmitters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.transmitters.is_empty()
    }

    /// Iterates over transmitters in NOAA's response order.
    pub fn iter(&self) -> impl Iterator<Item = &RadioTransmitter> {
        self.transmitters.iter()
    }

    /// Returns the cursor from the next-page link, if NOAA supplied a valid
    /// one.
    #[must_use]
    pub fn next_cursor(&self) -> Option<Cursor> {
        let next = Url::parse(&self.pagination.as_ref()?.next).ok()?;
        let (_, cursor) = next.query_pairs().find(|(name, _)| name == "cursor")?;
        cursor.parse().ok()
    }
}

/// A NOAA Weather Radio broadcast in SSML format.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename = "speak")]
#[non_exhaustive]
pub struct RadioBroadcast {
    /// SSML specification version, currently `1.1`.
    #[serde(rename = "@version")]
    pub version: String,
    /// Broadcast language, currently `en-US`.
    #[serde(rename = "@xml:lang")]
    pub lang: String,
    /// Spoken paragraphs in broadcast order.
    #[serde(rename = "p", default)]
    pub paragraphs: Vec<Paragraph>,
}

impl RadioBroadcast {
    /// Decodes the SSML document returned by a radio broadcast endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error when `source` is not well-formed SSML or omits a
    /// required broadcast attribute.
    pub fn from_ssml(source: &str) -> Result<Self, quick_xml::DeError> {
        quick_xml::de::from_str(source)
    }
}

/// A paragraph within an SSML broadcast.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct Paragraph {
    /// Sentences within this paragraph.
    #[serde(rename = "s", default)]
    pub sentences: Vec<Sentence>,
    /// Metadata marks embedded in this paragraph.
    #[serde(rename = "mark", default)]
    pub marks: Vec<BroadcastMark>,
}

/// A sentence within an SSML broadcast.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct Sentence {
    /// Ordered plain-text and pronunciation-aware fragments.
    #[serde(rename = "$value", default)]
    pub content: Vec<SentenceContent>,
}

impl Sentence {
    /// Returns the sentence's spoken text, discarding only pronunciation
    /// instructions rather than their content.
    #[must_use]
    pub fn full_text(&self) -> String {
        self.content
            .iter()
            .map(|part| match part {
                SentenceContent::Text(text) => text.as_str(),
                SentenceContent::SayAs(say_as) => say_as.text.as_str(),
            })
            .collect()
    }
}

/// One ordered fragment within a sentence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub enum SentenceContent {
    /// Plain spoken text.
    #[serde(rename = "$text")]
    Text(String),
    /// Text with an SSML pronunciation instruction.
    #[serde(rename = "say-as")]
    SayAs(SayAs),
}

/// An SSML pronunciation instruction.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct SayAs {
    /// How a speech synthesizer should interpret the text.
    #[serde(rename = "@interpret-as")]
    pub interpret_as: String,
    /// Text to speak.
    #[serde(rename = "$text")]
    pub text: String,
}

/// An SSML metadata mark carried between spoken fragments.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct BroadcastMark {
    /// NOAA's opaque metadata payload. It currently resembles a Python
    /// dictionary rather than JSON, so it is preserved as text.
    #[serde(rename = "@name")]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRANSMITTERS: &str = include_str!("../../tests/fixtures/radio/transmitters.json");
    const TRANSMITTER: &str = include_str!("../../tests/fixtures/radio/transmitter.json");
    const COUNTY: &str = include_str!("../../tests/fixtures/radio/county.json");
    const BROADCAST: &str = include_str!("../../tests/fixtures/radio/broadcast.xml");
    const POINT: &str = include_str!("../../tests/fixtures/radio/point.xml");

    #[test]
    fn transmitter_fixtures_decode_to_typed_values() {
        let transmitter: RadioTransmitter = serde_json::from_str(TRANSMITTER).unwrap();
        assert_eq!(transmitter.call_sign.as_str(), "KEC94");
        assert_eq!(transmitter.state.to_string(), "AZ");
        assert_eq!(transmitter.counties[0].to_string(), "AZC013");
        assert_eq!(transmitter.frequency, "162.550");

        let page: RadioTransmitterCollection = serde_json::from_str(TRANSMITTERS).unwrap();
        assert_eq!(page.len(), 5);
        assert_eq!(page.next_cursor().unwrap().as_str(), "eyJpIjo1MDB9");

        let county: RadioTransmitterCollection = serde_json::from_str(COUNTY).unwrap();
        assert_eq!(county.len(), 5);
        assert!(county.pagination.is_none());
        assert!(county.next_cursor().is_none());
    }

    #[test]
    fn missing_required_transmitter_metadata_is_rejected() {
        let source: serde_json::Value = serde_json::from_str(TRANSMITTER).unwrap();
        for key in [
            "@id",
            "@type",
            "setId",
            "callSign",
            "transmitterFrequency",
            "siteName",
            "siteCity",
            "siteState",
            "sameCodes",
            "counties",
        ] {
            let mut without = source.clone();
            without.as_object_mut().unwrap().remove(key);
            assert!(
                serde_json::from_value::<RadioTransmitter>(without).is_err(),
                "{key} unexpectedly optional"
            );
        }
    }

    #[test]
    fn broadcasts_preserve_spoken_text_pronunciation_and_marks() {
        let broadcast = RadioBroadcast::from_ssml(BROADCAST).unwrap();
        assert_eq!(broadcast.version, "1.1");
        assert_eq!(broadcast.lang, "en-US");
        assert!(broadcast.paragraphs.len() > 2);
        assert!(
            broadcast.paragraphs[0].sentences[0]
                .full_text()
                .contains("KEC94")
        );
        assert_eq!(broadcast.paragraphs[0].marks.len(), 4);

        let point = RadioBroadcast::from_ssml(POINT).unwrap();
        assert!(point.paragraphs.len() > 2);
        assert!(
            point.paragraphs[0].sentences[0]
                .full_text()
                .contains("point forecast")
        );
    }

    #[test]
    fn missing_required_ssml_attributes_are_rejected() {
        assert!(RadioBroadcast::from_ssml("<speak xml:lang=\"en-US\"/>").is_err());
        assert!(RadioBroadcast::from_ssml("<speak version=\"1.1\"/>").is_err());
        assert!(quick_xml::de::from_str::<SayAs>("<say-as>KEC94</say-as>").is_err());
        assert!(quick_xml::de::from_str::<BroadcastMark>("<mark/>").is_err());
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn every_public_radio_model_has_a_schema() {
        let transmitter = schemars::schema_for!(RadioTransmitter);
        let collection = schemars::schema_for!(RadioTransmitterCollection);
        let broadcast = schemars::schema_for!(RadioBroadcast);
        let paragraph = schemars::schema_for!(Paragraph);
        let sentence = schemars::schema_for!(Sentence);
        let content = schemars::schema_for!(SentenceContent);
        let say_as = schemars::schema_for!(SayAs);
        let mark = schemars::schema_for!(BroadcastMark);

        let required = transmitter.as_value()["required"].as_array().unwrap();
        for key in [
            "@id",
            "@type",
            "setId",
            "callSign",
            "transmitterFrequency",
            "siteName",
            "siteCity",
            "siteState",
            "sameCodes",
            "counties",
        ] {
            assert!(required.iter().any(|value| value == key), "{key} optional");
        }
        assert_eq!(
            collection.as_value()["properties"]["@graph"]["type"],
            "array"
        );
        assert_eq!(broadcast.as_value()["properties"]["p"]["type"], "array");
        for schema in [paragraph, sentence, content, say_as, mark] {
            assert!(schema.as_value().is_object());
        }
    }
}
