//! The initial NOAA Weather MCP server handler.

use noaa_weather_client::Client;
use noaa_weather_summary::SummaryOptions;
use rmcp::ServerHandler;
use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};

/// Owns the shared client and summary policy that future tool families use.
#[derive(Clone, Debug)]
pub struct NoaaWeatherServer {
    _client: Client,
    _summary_options: SummaryOptions,
}

impl NoaaWeatherServer {
    /// Builds the empty tool server around the production NOAA client.
    pub fn new() -> anyhow::Result<Self> {
        let user_agent = concat!(
            "noaa-weather-mcp/",
            env!("CARGO_PKG_VERSION"),
            " (+https://github.com/seferino-fernandez/noaa_weather)"
        );
        Ok(Self {
            _client: Client::builder(user_agent).build()?,
            _summary_options: SummaryOptions::default(),
        })
    }
}

impl ServerHandler for NoaaWeatherServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(ServerCapabilities::builder().build()).with_server_info(
            Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        );
        info.instructions = Some(
            "NOAA Weather MCP scaffold. Weather tools will be added in the next vertical slice."
                .to_owned(),
        );
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_advertises_no_tools() {
        let info = NoaaWeatherServer::new()
            .expect("the built-in NOAA client configuration must be valid")
            .get_info();

        assert_eq!(info.server_info.name, "noaa_weather_mcp");
        assert!(info.capabilities.tools.is_none());
    }
}
