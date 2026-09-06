//! Alert tools.

use noaa_weather_client::alerts::{
    ActiveAlertCounts, ActiveAlertsQuery, Alert, AlertEventTypes, AlertsQuery,
};
use noaa_weather_client::geo::{AreaCode, Feature, FeatureCollection, MarineRegionCode};
use noaa_weather_client::{AlertId, ZoneId};
use rmcp::handler::server::wrapper::Parameters;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GetAlertParams {
    alert_id: AlertId,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ActiveAlertsForAreaParams {
    #[schemars(with = "String")]
    area: AreaCode,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ActiveAlertsForMarineRegionParams {
    #[schemars(with = "String")]
    region: MarineRegionCode,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ActiveAlertsForZoneParams {
    zone_id: ZoneId,
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

#[rmcp::tool_router(router = alerts_remaining_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns active NOAA weather alerts for one state, territory, or marine area.
    #[rmcp::tool(
        name = "alerts_active_for_area",
        description = "Return active NOAA weather alerts for one state, territory, or marine area code.",
        annotations(
            title = "Get Active NOAA Alerts for Area",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_active_for_area(
        &self,
        Parameters(parameters): Parameters<ActiveAlertsForAreaParams>,
    ) -> Result<rmcp::Json<FeatureCollection<Alert>>, ToolFailure> {
        self.client
            .alerts()
            .active_for_area(&parameters.area)
            .await
            .map(rmcp::Json)
            .map_err(ToolFailure::from)
    }

    /// Returns active NOAA weather alerts for one marine region.
    #[rmcp::tool(
        name = "alerts_active_for_marine_region",
        description = "Return active NOAA weather alerts for one NOAA marine region code.",
        annotations(
            title = "Get Active NOAA Alerts for Marine Region",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_active_for_marine_region(
        &self,
        Parameters(parameters): Parameters<ActiveAlertsForMarineRegionParams>,
    ) -> Result<rmcp::Json<FeatureCollection<Alert>>, ToolFailure> {
        self.client
            .alerts()
            .active_for_marine_region(parameters.region)
            .await
            .map(rmcp::Json)
            .map_err(ToolFailure::from)
    }

    /// Returns active NOAA weather alerts for one public zone or county.
    #[rmcp::tool(
        name = "alerts_active_for_zone",
        description = "Return active NOAA weather alerts for one NOAA public zone or county identifier.",
        annotations(
            title = "Get Active NOAA Alerts for Zone",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_active_for_zone(
        &self,
        Parameters(parameters): Parameters<ActiveAlertsForZoneParams>,
    ) -> Result<rmcp::Json<FeatureCollection<Alert>>, ToolFailure> {
        self.client
            .alerts()
            .active_for_zone(&parameters.zone_id)
            .await
            .map(rmcp::Json)
            .map_err(ToolFailure::from)
    }

    /// Returns counts of currently active NOAA weather alerts.
    #[rmcp::tool(
        name = "alerts_active_count",
        description = "Return counts of currently active NOAA weather alerts by land, marine region, area, and zone.",
        annotations(
            title = "Count Active NOAA Weather Alerts",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_active_count(&self) -> Result<rmcp::Json<ActiveAlertCounts>, ToolFailure> {
        self.client
            .alerts()
            .active_count()
            .await
            .map(rmcp::Json)
            .map_err(ToolFailure::from)
    }

    /// Returns one page of NOAA weather alerts, including past alerts.
    #[rmcp::tool(
        name = "alerts_search",
        description = "Return one NOAA page of weather alerts, including past alerts, matching optional time, status, message, location, severity, and pagination filters.",
        annotations(
            title = "Search NOAA Weather Alerts",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_search(
        &self,
        Parameters(query): Parameters<AlertsQuery>,
    ) -> Result<rmcp::Json<FeatureCollection<Alert>>, ToolFailure> {
        self.client
            .alerts()
            .search(&query)
            .await
            .map(rmcp::Json)
            .map_err(ToolFailure::from)
    }

    /// Returns the event types recognized by NOAA's alert system.
    #[rmcp::tool(
        name = "alerts_types",
        description = "Return the weather-alert event types recognized by NOAA's alert system.",
        annotations(
            title = "List NOAA Weather Alert Types",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn alerts_types(&self) -> Result<rmcp::Json<AlertEventTypes>, ToolFailure> {
        self.client
            .alerts()
            .types()
            .await
            .map(rmcp::Json)
            .map_err(ToolFailure::from)
    }
}
