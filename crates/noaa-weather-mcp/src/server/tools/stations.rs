//! Observation and terminal-aerodrome-forecast tools.

use super::super::NoaaWeatherServer;

#[rmcp::tool_router(router = stations_router, vis = "pub(super)")]
impl NoaaWeatherServer {}
