//! Tool-family routers and shared tool behavior.

mod alerts;
#[allow(
    dead_code,
    reason = "family tools consume shared error projection after foundation"
)]
mod error;
mod gridpoints;
mod points;
mod stations;
#[cfg(test)]
pub(super) mod test_support;
#[cfg(test)]
mod tests;

use rmcp::handler::server::router::tool::ToolRouter;

use super::NoaaWeatherServer;

pub(super) fn router() -> ToolRouter<NoaaWeatherServer> {
    NoaaWeatherServer::points_router()
        + NoaaWeatherServer::alerts_router()
        + NoaaWeatherServer::gridpoints_router()
        + NoaaWeatherServer::stations_router()
}
