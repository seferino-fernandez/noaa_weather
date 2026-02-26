use serde::{Deserialize, Serialize};

/// A NOAA Weather Radio broadcast in SSML (Speech Synthesis Markup Language) format.
///
/// Represents the root `<speak>` element of the SSML document returned by
/// the `/radio/{callSign}/broadcast` and `/points/{point}/radio` endpoints.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "speak")]
pub struct RadioBroadcast {
    /// SSML specification version (e.g., "1.1").
    #[serde(rename = "@version")]
    pub version: String,

    /// Language of the broadcast (e.g., "en-US").
    #[serde(rename = "@xml:lang")]
    pub lang: String,

    /// Broadcast paragraphs containing sentences and metadata marks.
    #[serde(rename = "p", default)]
    pub paragraphs: Vec<Paragraph>,
}

/// A paragraph within the broadcast, corresponding to a `<p>` SSML element.
///
/// Contains one or more sentences and optional metadata marks.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paragraph {
    /// Sentences within this paragraph.
    #[serde(rename = "s", default)]
    pub sentences: Vec<Sentence>,

    /// Metadata marks embedded in this paragraph (e.g., station ID, frequency).
    #[serde(rename = "mark", default)]
    pub marks: Vec<BroadcastMark>,
}

/// A sentence within a broadcast paragraph, corresponding to an `<s>` SSML element.
///
/// Sentences may contain plain text, `<say-as>` pronunciation hints, or a mix of both.
/// The `content` field preserves the ordering of text and inline elements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sentence {
    /// Ordered sequence of text fragments and inline SSML elements.
    #[serde(rename = "$value", default)]
    pub content: Vec<SentenceContent>,
}

impl Sentence {
    /// Returns the full text of the sentence, concatenating all text fragments
    /// and the text content of any `<say-as>` elements.
    pub fn full_text(&self) -> String {
        self.content
            .iter()
            .map(|part| match part {
                SentenceContent::Text(t) => t.as_str(),
                SentenceContent::SayAs(sa) => sa.text.as_str(),
            })
            .collect()
    }
}

/// A content fragment within a [`Sentence`].
///
/// Represents either a plain text node or an inline `<say-as>` element
/// that provides pronunciation guidance for text-to-speech engines.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SentenceContent {
    /// A plain text fragment.
    #[serde(rename = "$text")]
    Text(String),

    /// A `<say-as>` element providing pronunciation interpretation.
    #[serde(rename = "say-as")]
    SayAs(SayAs),
}

/// An SSML `<say-as>` element that tells a speech synthesizer how to interpret text.
///
/// For example, `<say-as interpret-as="characters">KEC94</say-as>` instructs the
/// synthesizer to spell out each character individually.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SayAs {
    /// Interpretation hint (e.g., "characters", "date", "telephone").
    #[serde(rename = "@interpret-as")]
    pub interpret_as: String,

    /// The text content to be interpreted.
    #[serde(rename = "$text")]
    pub text: String,
}

/// An SSML `<mark>` element carrying broadcast metadata as a JSON-like string.
///
/// The `name` attribute typically contains a Python-style dictionary string
/// with keys such as `requesterSameCode`, `transmitter`, `stationId`, and
/// `sameLocations`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BroadcastMark {
    /// Metadata payload as a string (often a Python-style dict literal).
    #[serde(rename = "@name")]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_radio_broadcast() {
        let xml = include_str!("../../../notes/radio-KEC94-broadcast-response.xml");
        let broadcast: RadioBroadcast = quick_xml::de::from_str(xml).unwrap();

        assert_eq!(broadcast.version, "1.1");
        assert_eq!(broadcast.lang, "en-US");
        assert!(!broadcast.paragraphs.is_empty());

        let first = &broadcast.paragraphs[0];
        assert!(!first.sentences.is_empty());
        assert!(!first.marks.is_empty());
        assert_eq!(first.marks.len(), 4);
        assert_eq!(first.marks[0].name, "{'requesterSameCode': '04013'}");
        assert_eq!(first.marks[2].name, "{'stationId':  'KEC94'}");
    }

    #[test]
    fn test_sentence_with_say_as_mixed_content() {
        let xml = include_str!("../../../notes/radio-KEC94-broadcast-response.xml");
        let broadcast: RadioBroadcast = quick_xml::de::from_str(xml).unwrap();

        let first_sentence = &broadcast.paragraphs[0].sentences[0];
        let full = first_sentence.full_text();
        assert!(full.contains("KEC94"));
        assert!(full.contains("NOAA weather radio"));

        let has_say_as = first_sentence.content.iter().any(|c| {
            matches!(c, SentenceContent::SayAs(sa) if sa.interpret_as == "characters" && sa.text == "KEC94")
        });
        assert!(has_say_as);
    }

    #[test]
    fn test_plain_text_sentence() {
        let xml = include_str!("../../../notes/radio-KEC94-broadcast-response.xml");
        let broadcast: RadioBroadcast = quick_xml::de::from_str(xml).unwrap();

        let second_para = &broadcast.paragraphs[1];
        assert_eq!(second_para.sentences.len(), 1);
        assert_eq!(
            second_para.sentences[0].full_text(),
            "Here is your NOAA Weather Radio area forecast."
        );
    }

    #[test]
    fn test_paragraph_count() {
        let xml = include_str!("../../../notes/radio-KEC94-broadcast-response.xml");
        let broadcast: RadioBroadcast = quick_xml::de::from_str(xml).unwrap();

        // The sample has many paragraphs: intro + roundup header + station reports + forecast sections
        assert!(broadcast.paragraphs.len() > 10);
    }

    #[test]
    fn test_multi_sentence_paragraph() {
        let xml = include_str!("../../../notes/radio-KEC94-broadcast-response.xml");
        let broadcast: RadioBroadcast = quick_xml::de::from_str(xml).unwrap();

        // Paragraph 3 (index 3) is Phoenix with multiple sentences
        let phoenix_para = &broadcast.paragraphs[3];
        assert!(phoenix_para.sentences.len() >= 3);
        assert!(phoenix_para.sentences[0].full_text().contains("Phoenix"));
    }
}
