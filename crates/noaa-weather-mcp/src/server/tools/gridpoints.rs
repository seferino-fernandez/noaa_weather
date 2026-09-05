//! Gridpoint forecast tools.

use super::super::NoaaWeatherServer;

#[rmcp::tool_router(router = gridpoints_router, vis = "pub(super)")]
impl NoaaWeatherServer {}
