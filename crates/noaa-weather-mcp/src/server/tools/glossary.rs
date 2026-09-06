//! Weather glossary tools.

use noaa_weather_client::glossary::GlossaryResponse;
use rmcp::Json;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[rmcp::tool_router(router = glossary_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "glossary_terms",
        description = "Return every NWS weather glossary term and its definition.",
        annotations(
            title = "List NWS Glossary Terms",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn glossary_terms(&self) -> Result<Json<GlossaryResponse>, ToolFailure> {
        self.client
            .glossary()
            .terms()
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}
