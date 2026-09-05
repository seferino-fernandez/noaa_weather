//! Curated models for NWS office metadata and published documents.
//!
//! # Requiredness
//!
//! A live audit on 2026-09-04 inspected metadata for eight forecast offices,
//! one regional headquarters, and the national office. Their organization and
//! postal-address fields shared one non-null shape. Forecast-office
//! responsibility arrays are absent from headquarters, `parentOrganization`
//! is absent from the national office, and its `nwsRegion` is null, so those
//! fields remain optional.
//!
//! The same audit inspected 16 headlines from nine offices, 30 weather stories
//! from eleven offices, and active briefings from AKQ, LWX, and TOP. Headlines
//! shared one keyset with only `summary` nullable; stories and briefings shared
//! fully non-null keysets. JSON-LD context is vocabulary metadata and is
//! deliberately excluded from these curated models.

use serde::{Deserialize, Serialize};

use super::StateTerritoryCode;
use crate::ids::{OfficeId, StationId, ZoneId};
use crate::time::OffsetDateTime;

/// Metadata for one NWS forecast office or headquarters.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Office {
    /// Canonical office API URL.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// Fixed JSON-LD resource type.
    #[serde(rename = "@type")]
    pub at_type: OfficeResourceType,
    /// Office identifier.
    pub id: OfficeId,
    /// Human-readable office name.
    pub name: String,
    /// Mailing address.
    pub address: OfficeAddress,
    /// Public telephone number. NOAA may send an empty string.
    #[serde(rename = "telephone")]
    pub phone_number: String,
    /// Public fax number. NOAA commonly sends an empty string.
    pub fax_number: String,
    /// Public contact email address.
    pub email: String,
    /// Public weather.gov website.
    #[serde(rename = "sameAs")]
    pub website_url: String,
    /// NWS region code. The national office sends `null`.
    pub nws_region: Option<String>,
    /// Canonical API URL of the parent organization. The national office
    /// omits this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_organization: Option<String>,
    /// County-zone URLs for which this forecast office is responsible.
    /// Headquarters omit this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_counties: Option<Vec<String>>,
    /// Forecast-zone URLs for which this office is responsible. Headquarters
    /// omit this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_forecast_zones: Option<Vec<String>>,
    /// Fire-zone URLs for which this office is responsible. Headquarters omit
    /// this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_fire_zones: Option<Vec<String>>,
    /// Observation-station URLs approved by this forecast office.
    /// Headquarters omit this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_observation_stations: Option<Vec<String>>,
}

impl Office {
    /// Returns the parent office identifier from its URL, when present and valid.
    #[must_use]
    pub fn parent_office_id(&self) -> Option<OfficeId> {
        last_segment(self.parent_organization.as_deref()?)?
            .parse()
            .ok()
    }

    /// Iterates over valid county identifiers in the responsibility URLs.
    pub fn responsible_county_ids(&self) -> impl Iterator<Item = ZoneId> + '_ {
        ids_from_urls(self.responsible_counties.as_deref().unwrap_or_default())
    }

    /// Iterates over valid forecast-zone identifiers in the responsibility URLs.
    pub fn responsible_forecast_zone_ids(&self) -> impl Iterator<Item = ZoneId> + '_ {
        ids_from_urls(
            self.responsible_forecast_zones
                .as_deref()
                .unwrap_or_default(),
        )
    }

    /// Iterates over valid fire-zone identifiers in the responsibility URLs.
    pub fn responsible_fire_zone_ids(&self) -> impl Iterator<Item = ZoneId> + '_ {
        ids_from_urls(self.responsible_fire_zones.as_deref().unwrap_or_default())
    }

    /// Iterates over valid station identifiers in the approved-station URLs.
    pub fn approved_observation_station_ids(&self) -> impl Iterator<Item = StationId> + '_ {
        ids_from_urls(
            self.approved_observation_stations
                .as_deref()
                .unwrap_or_default(),
        )
    }
}

/// Postal address for an NWS office.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OfficeAddress {
    /// Fixed JSON-LD resource type.
    #[serde(rename = "@type")]
    pub at_type: PostalAddressResourceType,
    /// Street address, possibly containing multiple lines.
    pub street_address: String,
    /// City or locality.
    #[serde(rename = "addressLocality")]
    pub city: String,
    /// State or territory.
    #[serde(rename = "addressRegion")]
    pub state: StateTerritoryCode,
    /// Postal code.
    #[serde(rename = "postalCode")]
    pub postal_code: String,
}

/// Fixed JSON-LD type attached to an office.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum OfficeResourceType {
    /// A government organization.
    GovernmentOrganization,
}

/// Fixed JSON-LD type attached to an office address.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum PostalAddressResourceType {
    /// A postal address.
    PostalAddress,
}

/// One news headline published by an NWS office.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OfficeHeadline {
    /// Canonical headline API URL.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// Server-issued headline identifier.
    pub id: String,
    /// Canonical URL of the publishing office.
    pub office: String,
    /// Whether the office marked the headline important.
    pub important: bool,
    /// When the headline was issued.
    pub issuance_time: OffsetDateTime,
    /// Public destination for the headline.
    pub link: String,
    /// Stable publication name.
    pub name: String,
    /// Human-readable title.
    pub title: String,
    /// Short summary, when the office supplied one.
    pub summary: Option<String>,
    /// Full HTML content.
    pub content: String,
}

impl OfficeHeadline {
    /// Returns the publishing office identifier from its URL.
    #[must_use]
    pub fn office_id(&self) -> Option<OfficeId> {
        last_segment(&self.office)?.parse().ok()
    }
}

/// A JSON-LD collection of office headlines.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct OfficeHeadlineCollection {
    /// Headlines in service order.
    #[serde(rename = "@graph", default)]
    pub at_graph: Vec<OfficeHeadline>,
}

/// Metadata for an active office briefing.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NwsConnectDocumentMetadata {
    /// Server-issued document identifier.
    pub id: String,
    /// Publishing office identifier.
    pub office_id: OfficeId,
    /// Time when the document becomes active.
    pub start_time: OffsetDateTime,
    /// Time when the document becomes inactive.
    pub end_time: OffsetDateTime,
    /// Time when the document was last updated.
    pub update_time: OffsetDateTime,
    /// Short document title.
    pub title: String,
    /// Longer document description or caption.
    pub description: String,
    /// Whether the office marked the document as a priority.
    pub priority: bool,
    /// URL from which the document can be downloaded.
    pub download: String,
}

/// Metadata for an active office briefing.
pub type OfficeBriefing = NwsConnectDocumentMetadata;

/// Active briefing response. `briefing` is null when the office has no active document.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct OfficeBriefingResponse {
    /// Active briefing metadata, or `None` when nothing is published.
    pub briefing: Option<OfficeBriefing>,
}

/// Metadata for an NWS office weather-story image.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OfficeWeatherStory {
    /// Publishing office identifier.
    pub office_id: OfficeId,
    /// Time when the story becomes active.
    pub start_time: OffsetDateTime,
    /// Time when the story becomes inactive.
    pub end_time: OffsetDateTime,
    /// Time when the story was last updated.
    pub update_time: OffsetDateTime,
    /// Short story title.
    pub title: String,
    /// Longer story description or caption.
    pub description: String,
    /// Alternative text for the image.
    pub alt_text: String,
    /// Whether the office marked the story as a priority.
    pub priority: bool,
    /// Display order supplied by the office; zero is valid.
    pub order: u32,
    /// URL from which the image can be downloaded.
    pub download: String,
}

/// A collection of active office weather stories.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct OfficeWeatherStoryCollection {
    /// Stories in service order.
    #[serde(default)]
    pub stories: Vec<OfficeWeatherStory>,
}

fn last_segment(url: &str) -> Option<&str> {
    url.trim_end_matches('/')
        .rsplit('/')
        .find(|segment| !segment.is_empty())
}

fn ids_from_urls<'a, T>(urls: &'a [String]) -> impl Iterator<Item = T> + 'a
where
    T: std::str::FromStr + 'a,
{
    urls.iter()
        .filter_map(|url| last_segment(url)?.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    const OFFICE: &str = include_str!("../../tests/fixtures/offices/office.json");
    const HEADLINE: &str = include_str!("../../tests/fixtures/offices/headline.json");
    const HEADLINES: &str = include_str!("../../tests/fixtures/offices/headlines.json");
    const BRIEFING: &str = include_str!("../../tests/fixtures/offices/briefing.json");
    const STORIES: &str = include_str!("../../tests/fixtures/offices/weather_stories.json");

    #[test]
    fn fixtures_decode_to_typed_office_values() {
        let office: Office = serde_json::from_str(OFFICE).unwrap();
        assert_eq!(office.id.as_str(), "PSR");
        assert_eq!(office.address.state.to_string(), "AZ");
        assert_eq!(office.parent_office_id().unwrap().as_str(), "WRH");
        assert_eq!(
            office.responsible_county_ids().next().unwrap().as_str(),
            "AZC007"
        );
        assert_eq!(
            office
                .approved_observation_station_ids()
                .next()
                .unwrap()
                .as_str(),
            "BTTC1"
        );

        let headline: OfficeHeadline = serde_json::from_str(HEADLINE).unwrap();
        assert_eq!(headline.office_id().unwrap().as_str(), "PSR");
        assert_eq!(headline.summary, None);
        assert_eq!(
            headline.issuance_time.to_string(),
            "2026-09-01T09:27:00+00:00"
        );

        let headlines: OfficeHeadlineCollection = serde_json::from_str(HEADLINES).unwrap();
        assert_eq!(headlines.at_graph.len(), 2);

        let briefing: OfficeBriefingResponse = serde_json::from_str(BRIEFING).unwrap();
        assert_eq!(briefing.briefing, None);

        let stories: OfficeWeatherStoryCollection = serde_json::from_str(STORIES).unwrap();
        assert_eq!(stories.stories.len(), 4);
        assert_eq!(stories.stories[0].office_id.as_str(), "PSR");
        assert_eq!(stories.stories[0].order, 0);
    }

    #[test]
    fn missing_required_metadata_is_rejected() {
        let mut office: serde_json::Value = serde_json::from_str(OFFICE).unwrap();
        office.as_object_mut().unwrap().remove("name");
        assert!(serde_json::from_value::<Office>(office).is_err());

        let mut headline: serde_json::Value = serde_json::from_str(HEADLINE).unwrap();
        headline.as_object_mut().unwrap().remove("issuanceTime");
        assert!(serde_json::from_value::<OfficeHeadline>(headline).is_err());

        let mut story: serde_json::Value = serde_json::from_str(STORIES).unwrap();
        story["stories"][0].as_object_mut().unwrap().remove("title");
        assert!(serde_json::from_value::<OfficeWeatherStoryCollection>(story).is_err());

        let briefing = serde_json::json!({"briefing": {"id": "brief-1"}});
        assert!(serde_json::from_value::<OfficeBriefingResponse>(briefing).is_err());
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn office_models_publish_required_schemas() {
        for schema in [
            schemars::schema_for!(Office),
            schemars::schema_for!(OfficeHeadline),
            schemars::schema_for!(NwsConnectDocumentMetadata),
            schemars::schema_for!(OfficeWeatherStory),
        ] {
            assert!(schema.as_value()["required"].as_array().is_some());
        }
        assert_eq!(
            schemars::schema_for!(OfficeHeadlineCollection).as_value()["properties"]["@graph"]["type"],
            "array"
        );
        assert_eq!(
            schemars::schema_for!(OfficeWeatherStoryCollection).as_value()["properties"]["stories"]
                ["type"],
            "array"
        );
    }
}
