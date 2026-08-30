use serde::{Deserialize, Deserializer, Serialize, de::Error as _};

use super::JsonLdContext;

/// Metadata shared by NWS Connect briefing and weather-story documents.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NwsConnectDocumentMetadata {
    /// Document identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Forecast office identifier observed in live responses.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<String>,
    /// Time when the document becomes active.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Time when the document becomes inactive.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Time when the document was last updated.
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// Short document title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Longer document description or caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the document should be emphasized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<bool>,
    /// URL from which the document content can be downloaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
}

/// Metadata for an active office briefing.
pub type OfficeBriefing = NwsConnectDocumentMetadata;

/// Active briefing response, tolerant of direct and wrapped API payloads.
#[derive(Clone, Default, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OfficeBriefingResponse {
    /// JSON-LD context supplied by wrapped live responses.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<JsonLdContext>>,
    /// Active briefing metadata, including an explicit `null` response as `None`.
    pub briefing: Option<OfficeBriefing>,
}

impl<'de> Deserialize<'de> for OfficeBriefingResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(rename = "@context")]
            context: Option<Box<JsonLdContext>>,
            briefing: Option<OfficeBriefing>,
        }

        let value = serde_json::Value::deserialize(deserializer)?;
        if value
            .as_object()
            .is_some_and(|object| object.contains_key("briefing"))
        {
            let Wrapper { context, briefing } =
                serde_json::from_value(value).map_err(D::Error::custom)?;
            Ok(Self { context, briefing })
        } else {
            let briefing = serde_json::from_value(value).map_err(D::Error::custom)?;
            Ok(Self {
                context: None,
                briefing: Some(briefing),
            })
        }
    }
}

/// Metadata for an NWS office weather-story image.
#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OfficeWeatherStory {
    /// Document identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Forecast office identifier observed in live responses.
    #[serde(rename = "officeId", skip_serializing_if = "Option::is_none")]
    pub office_id: Option<String>,
    /// Time when the story becomes active.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// Time when the story becomes inactive.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Time when the story was last updated.
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// Short story title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Longer story description or caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the story should be emphasized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<bool>,
    /// URL from which the image can be downloaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<String>,
    /// Alternative text for the image.
    #[serde(rename = "altText", skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    /// Display order supplied by the service; live data may use zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

/// A weather-story collection, tolerant of bare arrays and wrapped live payloads.
#[derive(Clone, Default, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct OfficeWeatherStoryCollection {
    /// JSON-LD context supplied by wrapped responses.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<JsonLdContext>>,
    /// Stories in service order.
    pub stories: Vec<OfficeWeatherStory>,
}

impl<'de> Deserialize<'de> for OfficeWeatherStoryCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Wrapped {
                #[serde(rename = "@context")]
                context: Option<Box<JsonLdContext>>,
                #[serde(alias = "@graph", alias = "weatherStories")]
                stories: Vec<OfficeWeatherStory>,
            },
            Bare(Vec<OfficeWeatherStory>),
        }

        Ok(match Wire::deserialize(deserializer)? {
            Wire::Wrapped { context, stories } => Self { context, stories },
            Wire::Bare(stories) => Self {
                context: None,
                stories,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{OfficeBriefingResponse, OfficeWeatherStoryCollection};

    #[test]
    fn briefing_accepts_direct_wrapped_and_null_shapes() {
        let direct: OfficeBriefingResponse =
            serde_json::from_str(r#"{"id":"brief-1","officeId":"PSR","title":"Monsoon outlook"}"#)
                .unwrap();
        assert_eq!(direct.briefing.unwrap().office_id.as_deref(), Some("PSR"));

        let wrapped: OfficeBriefingResponse = serde_json::from_str(
            r#"{"@context":{"@version":"1.1"},"briefing":{"download":"https://example.test/brief.pdf"}}"#,
        )
        .unwrap();
        assert!(wrapped.context.is_some());
        assert_eq!(
            wrapped.briefing.unwrap().download.as_deref(),
            Some("https://example.test/brief.pdf")
        );

        let null: OfficeBriefingResponse = serde_json::from_str(r#"{"briefing":null}"#).unwrap();
        assert_eq!(null.briefing, None);
    }

    #[test]
    fn stories_accept_bare_and_wrapped_tolerant_shapes() {
        let bare: OfficeWeatherStoryCollection = serde_json::from_str(
            r#"[{"title":"Heat","order":0},{"title":null,"description":null}]"#,
        )
        .unwrap();
        assert_eq!(bare.stories.len(), 2);
        assert_eq!(bare.stories[0].order, Some(0));
        assert_eq!(bare.stories[1].title, None);

        let wrapped: OfficeWeatherStoryCollection = serde_json::from_str(
            r#"{"@context":{"@version":"1.1"},"stories":[{"officeId":"PSR","unknown":true}]}"#,
        )
        .unwrap();
        assert!(wrapped.context.is_some());
        assert_eq!(wrapped.stories[0].office_id.as_deref(), Some("PSR"));
        assert_eq!(wrapped.stories[0].alt_text, None);
    }
}
