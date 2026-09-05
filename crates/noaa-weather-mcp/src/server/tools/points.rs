//! Point metadata and point-to-forecast tools.

use super::super::NoaaWeatherServer;

#[rmcp::tool_router(router = points_router, vis = "pub(super)")]
impl NoaaWeatherServer {}
