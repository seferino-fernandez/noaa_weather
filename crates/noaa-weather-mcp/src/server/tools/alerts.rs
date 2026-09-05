//! Alert tools.

use super::super::NoaaWeatherServer;

#[rmcp::tool_router(router = alerts_router, vis = "pub(super)")]
impl NoaaWeatherServer {}
