//! Shared NOAA client, tool routing, and result policy.

mod result_limit;
mod tools;

use std::num::NonZeroUsize;

use noaa_weather_client::Client;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::tool::ToolCallContext;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, Implementation, ServerCapabilities, ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler};

/// Owns the shared NOAA client, composed tool router, and response limit.
#[derive(Clone, Debug)]
pub struct NoaaWeatherServer {
    client: Client,
    tool_router: ToolRouter<Self>,
    max_response_bytes: NonZeroUsize,
}

impl NoaaWeatherServer {
    /// Builds the tool server around the production NOAA client.
    pub fn new(max_response_bytes: NonZeroUsize) -> anyhow::Result<Self> {
        let user_agent = concat!(
            "noaa-weather-mcp/",
            env!("CARGO_PKG_VERSION"),
            " (+https://github.com/seferino-fernandez/noaa_weather)"
        );
        let client = Client::builder(user_agent).build()?;
        Ok(Self::with_client(client, max_response_bytes))
    }

    fn with_client(client: Client, max_response_bytes: NonZeroUsize) -> Self {
        Self {
            client,
            tool_router: tools::router(),
            max_response_bytes,
        }
    }
}

#[rmcp::tool_handler(router = self.tool_router)]
impl ServerHandler for NoaaWeatherServer {
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let context = ToolCallContext::new(self, request, context);
        let response = self.tool_router.call(context).await?;
        Ok(result_limit::apply(response, self.max_response_bytes))
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Read NOAA weather.gov data through typed tools. Successful calls return authoritative structured JSON and an identical JSON text representation; tool failures return bounded JSON text with stable error codes."
                    .to_owned(),
            )
    }
}
