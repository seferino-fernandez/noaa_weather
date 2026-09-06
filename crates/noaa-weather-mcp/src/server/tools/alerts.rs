//! Alert tools.

use noaa_weather_client::AlertId;
use noaa_weather_client::alerts::{ActiveAlertsQuery, Alert};
use noaa_weather_client::geo::{Feature, FeatureCollection};
use rmcp::handler::server::wrapper::Parameters;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GetAlertParams {
    alert_id: AlertId,
}

#[rmcp::tool_router(router = alerts_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns one page of currently active NOAA weather alerts matching the supplied filters.
    #[rmcp::tool(
        name = "alerts_active",
        description = "Return one NOAA page of currently active weather alerts matching optional status, message, event, location, urgency, severity, and certainty filters.",
        annotations(
            title = "Get Active NOAA Weather Alerts",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_active(
        &self,
        Parameters(query): Parameters<ActiveAlertsQuery>,
    ) -> Result<rmcp::Json<FeatureCollection<Alert>>, ToolFailure> {
        let alerts = self.client.alerts().active(&query).await?;
        Ok(rmcp::Json(alerts))
    }

    /// Returns one NOAA weather alert by its canonical alert identifier.
    #[rmcp::tool(
        name = "alerts_get",
        description = "Return one NOAA weather alert by its canonical alert identifier.",
        annotations(
            title = "Get NOAA Weather Alert",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_get(
        &self,
        Parameters(GetAlertParams { alert_id }): Parameters<GetAlertParams>,
    ) -> Result<rmcp::Json<Feature<Alert>>, ToolFailure> {
        let alert = self.client.alerts().get(&alert_id).await?;
        Ok(rmcp::Json(alert))
    }
}
