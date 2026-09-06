//! Tool-family routers and shared tool behavior.

mod alerts;
mod aviation;
mod error;
mod glossary;
mod gridpoints;
mod offices;
mod points;
mod products;
mod radar;
mod radio;
mod stations;
#[cfg(test)]
pub(super) mod test_support;
#[cfg(test)]
mod tests;
mod zones;

use rmcp::handler::server::router::tool::ToolRouter;

use super::NoaaWeatherServer;

pub(super) fn router() -> ToolRouter<NoaaWeatherServer> {
    NoaaWeatherServer::points_router()
        + NoaaWeatherServer::alerts_router()
        + NoaaWeatherServer::alerts_remaining_router()
        + NoaaWeatherServer::aviation_router()
        + NoaaWeatherServer::glossary_router()
        + NoaaWeatherServer::gridpoints_router()
        + NoaaWeatherServer::gridpoints_remaining_router()
        + NoaaWeatherServer::offices_router()
        + NoaaWeatherServer::products_router()
        + NoaaWeatherServer::radar_router()
        + NoaaWeatherServer::radio_router()
        + NoaaWeatherServer::stations_router()
        + NoaaWeatherServer::stations_remaining_router()
        + NoaaWeatherServer::zones_router()
}
