//! NOAA office tools.

#![cfg_attr(
    not(test),
    allow(dead_code, reason = "the family router is composed by integration")
)]

use noaa_weather_client::OfficeId;
use noaa_weather_client::offices::{
    Office, OfficeBriefingResponse, OfficeHeadline, OfficeHeadlineCollection,
    OfficeWeatherStoryCollection,
};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::super::result_limit::{self, BinaryContent};
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct OfficeArguments {
    /// Forecast office or regional or national headquarters identifier.
    office_id: OfficeId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct OfficeHeadlineArguments {
    /// Forecast office or regional or national headquarters identifier.
    office_id: OfficeId,
    /// Opaque server-issued headline identifier.
    headline_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct OfficeBriefingDocumentArguments {
    /// Forecast office or regional or national headquarters identifier.
    office_id: OfficeId,
    /// Opaque server-issued briefing document identifier.
    briefing_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct OfficeWeatherStoryImageArguments {
    /// Forecast office or regional or national headquarters identifier.
    office_id: OfficeId,
    /// Opaque server-issued weather-story image identifier.
    image_id: String,
}

#[rmcp::tool_router(router = offices_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "offices_get",
        description = "Return metadata for one NOAA forecast office or regional or national headquarters.",
        annotations(
            title = "Get NOAA Office",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_get(
        &self,
        Parameters(arguments): Parameters<OfficeArguments>,
    ) -> Result<Json<Office>, ToolFailure> {
        self.client
            .offices()
            .get(&arguments.office_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "offices_headlines",
        description = "Return recent news headlines published by one NOAA office.",
        annotations(
            title = "List NOAA Office Headlines",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_headlines(
        &self,
        Parameters(arguments): Parameters<OfficeArguments>,
    ) -> Result<Json<OfficeHeadlineCollection>, ToolFailure> {
        self.client
            .offices()
            .headlines(&arguments.office_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "offices_headline_get",
        description = "Return one NOAA office headline by its server-issued identifier.",
        annotations(
            title = "Get NOAA Office Headline",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_headline_get(
        &self,
        Parameters(arguments): Parameters<OfficeHeadlineArguments>,
    ) -> Result<Json<OfficeHeadline>, ToolFailure> {
        self.client
            .offices()
            .headline(&arguments.office_id, &arguments.headline_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "offices_briefing",
        description = "Return active briefing metadata for one NOAA office, including a null briefing when none is active.",
        annotations(
            title = "Get NOAA Office Briefing",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_briefing(
        &self,
        Parameters(arguments): Parameters<OfficeArguments>,
    ) -> Result<Json<OfficeBriefingResponse>, ToolFailure> {
        self.client
            .offices()
            .briefing(&arguments.office_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "offices_latest_briefing_document",
        description = "Download the latest NOAA office briefing as a base64-encoded PDF resource.",
        annotations(
            title = "Download Latest NOAA Office Briefing",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_latest_briefing_document(
        &self,
        Parameters(arguments): Parameters<OfficeArguments>,
    ) -> Result<CallToolResult, ToolFailure> {
        self.client
            .offices()
            .latest_briefing_document(&arguments.office_id)
            .await
            .map(|payload| {
                result_limit::binary(payload, BinaryContent::Resource, self.max_response_bytes)
            })
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "offices_briefing_document",
        description = "Download one NOAA office briefing by its server-issued identifier as a base64-encoded PDF resource.",
        annotations(
            title = "Download NOAA Office Briefing",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_briefing_document(
        &self,
        Parameters(arguments): Parameters<OfficeBriefingDocumentArguments>,
    ) -> Result<CallToolResult, ToolFailure> {
        self.client
            .offices()
            .briefing_document(&arguments.office_id, &arguments.briefing_id)
            .await
            .map(|payload| {
                result_limit::binary(payload, BinaryContent::Resource, self.max_response_bytes)
            })
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "offices_weather_stories",
        description = "Return active weather-story metadata for one NOAA office.",
        annotations(
            title = "List NOAA Office Weather Stories",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_weather_stories(
        &self,
        Parameters(arguments): Parameters<OfficeArguments>,
    ) -> Result<Json<OfficeWeatherStoryCollection>, ToolFailure> {
        self.client
            .offices()
            .weather_stories(&arguments.office_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "offices_weather_story_image",
        description = "Download one NOAA office weather-story image as base64-encoded MCP image content.",
        annotations(
            title = "Download NOAA Office Weather Story Image",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn offices_weather_story_image(
        &self,
        Parameters(arguments): Parameters<OfficeWeatherStoryImageArguments>,
    ) -> Result<CallToolResult, ToolFailure> {
        self.client
            .offices()
            .weather_story_image(&arguments.office_id, &arguments.image_id)
            .await
            .map(|payload| {
                result_limit::binary(payload, BinaryContent::Image, self.max_response_bytes)
            })
            .map_err(ToolFailure::from)
    }
}
