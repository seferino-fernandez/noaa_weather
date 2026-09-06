//! Forecast-zone tools.

#![cfg_attr(
    not(test),
    allow(dead_code, reason = "the family router is composed by integration")
)]

use noaa_weather_client::ZoneId;
use noaa_weather_client::geo::{Feature, FeatureCollection};
use noaa_weather_client::stations::{Observation, ObservationStation};
use noaa_weather_client::zones::{
    Zone, ZoneForecast, ZoneObservationsQuery, ZoneQuery, ZoneStationsQuery, ZoneType, ZonesQuery,
};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ZoneArguments {
    /// Zone type used in the `/zones/{type}` request path.
    zone_type: ZoneType,
    /// Forecast zone or county identifier used in the request path.
    zone_id: ZoneId,
    #[serde(flatten)]
    query: ZoneQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ZoneForecastArguments {
    /// Zone type used in the `/zones/{type}` request path.
    zone_type: ZoneType,
    /// Forecast zone or county identifier used in the request path.
    zone_id: ZoneId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ZonesOfTypeArguments {
    /// Required request-path type, distinct from the optional `type` query filter.
    zone_type: ZoneType,
    #[serde(flatten)]
    query: ZonesQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ZoneObservationsArguments {
    /// Forecast-zone identifier used in the request path.
    zone_id: ZoneId,
    #[serde(flatten)]
    query: ZoneObservationsQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ZoneStationsArguments {
    /// Forecast-zone identifier used in the request path.
    zone_id: ZoneId,
    #[serde(flatten)]
    query: ZoneStationsQuery,
}

#[rmcp::tool_router(router = zones_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "zones_get",
        description = "Return metadata for one NOAA zone, optionally at a specified effective time.",
        annotations(title = "Get NOAA Zone", read_only_hint = true, open_world_hint = true)
    )]
    async fn zones_get(
        &self,
        Parameters(arguments): Parameters<ZoneArguments>,
    ) -> Result<Json<Feature<Zone>>, ToolFailure> {
        self.client
            .zones()
            .get(arguments.zone_type, &arguments.zone_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "zones_forecast",
        description = "Return the current textual forecast for one NOAA zone.",
        annotations(
            title = "Get NOAA Zone Forecast",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn zones_forecast(
        &self,
        Parameters(arguments): Parameters<ZoneForecastArguments>,
    ) -> Result<Json<Feature<ZoneForecast>>, ToolFailure> {
        self.client
            .zones()
            .forecast(arguments.zone_type, &arguments.zone_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "zones_list",
        description = "Return one NOAA page of zones matching optional identifier, area, region, type, point, geometry, limit, and effective-time filters.",
        annotations(
            title = "List NOAA Zones",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn zones_list(
        &self,
        Parameters(query): Parameters<ZonesQuery>,
    ) -> Result<Json<FeatureCollection<Zone>>, ToolFailure> {
        self.client
            .zones()
            .list(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "zones_list_of_type",
        description = "Return one NOAA page of zones under a required path type, optionally narrowed by identifier, area, region, type, point, geometry, limit, and effective time.",
        annotations(
            title = "List NOAA Zones of Type",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn zones_list_of_type(
        &self,
        Parameters(arguments): Parameters<ZonesOfTypeArguments>,
    ) -> Result<Json<FeatureCollection<Zone>>, ToolFailure> {
        self.client
            .zones()
            .list_of_type(arguments.zone_type, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "zones_observations",
        description = "Return one NOAA page of recent observations from stations in a forecast zone; NOAA's returned next link incorrectly targets one station and is not followed.",
        annotations(
            title = "Get NOAA Zone Observations",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn zones_observations(
        &self,
        Parameters(arguments): Parameters<ZoneObservationsArguments>,
    ) -> Result<Json<FeatureCollection<Observation>>, ToolFailure> {
        self.client
            .zones()
            .observations(&arguments.zone_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "zones_stations",
        description = "Return one NOAA page of observation stations for a forecast zone. Do not copy the response pagination.next cursor: NOAA points it to /stations, where it returns an empty page; increase limit instead.",
        annotations(
            title = "Get NOAA Zone Stations",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn zones_stations(
        &self,
        Parameters(arguments): Parameters<ZoneStationsArguments>,
    ) -> Result<Json<FeatureCollection<ObservationStation>>, ToolFailure> {
        self.client
            .zones()
            .stations(&arguments.zone_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}
