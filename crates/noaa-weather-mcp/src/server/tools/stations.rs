//! Observation and terminal-aerodrome-forecast tools.

use noaa_weather_client::geo::{Feature, FeatureCollection};
use noaa_weather_client::stations::{
    LatestObservationQuery, Observation, ObservationStation, ObservationsQuery, StationsQuery,
    TerminalAerodromeForecast, TerminalAerodromeForecastsResponse,
};
use noaa_weather_client::{OffsetDateTime, StationId};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct LatestObservationArguments {
    station_id: StationId,
    #[serde(flatten)]
    query: LatestObservationQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ObservationsArguments {
    station_id: StationId,
    #[serde(flatten)]
    query: ObservationsQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct TafListArguments {
    station_id: StationId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct TafGetArguments {
    station_id: StationId,
    issued: OffsetDateTime,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct StationArguments {
    station_id: StationId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ObservationAtArguments {
    station_id: StationId,
    time: OffsetDateTime,
}

#[rmcp::tool_router(router = stations_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "stations_observation_latest",
        description = "Return the latest surface observation reported by one NOAA station.",
        annotations(
            title = "Latest Station Observation",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_observation_latest(
        &self,
        Parameters(arguments): Parameters<LatestObservationArguments>,
    ) -> Result<Json<Feature<Observation>>, ToolFailure> {
        self.client
            .stations()
            .latest_observation(&arguments.station_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "stations_observations",
        description = "Return one NOAA page of surface observations for a station.",
        annotations(
            title = "Station Observations",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_observations(
        &self,
        Parameters(arguments): Parameters<ObservationsArguments>,
    ) -> Result<Json<FeatureCollection<Observation>>, ToolFailure> {
        self.client
            .stations()
            .observations(&arguments.station_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "stations_taf_list",
        description = "Return metadata for the current Terminal Aerodrome Forecasts at a station.",
        annotations(
            title = "Station TAF List",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_taf_list(
        &self,
        Parameters(arguments): Parameters<TafListArguments>,
    ) -> Result<Json<TerminalAerodromeForecastsResponse>, ToolFailure> {
        self.client
            .stations()
            .tafs(&arguments.station_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "stations_taf_get",
        description = "Return one decoded Terminal Aerodrome Forecast by station and issue time.",
        annotations(title = "Station TAF", read_only_hint = true, open_world_hint = true)
    )]
    async fn stations_taf_get(
        &self,
        Parameters(arguments): Parameters<TafGetArguments>,
    ) -> Result<Json<TerminalAerodromeForecast>, ToolFailure> {
        self.client
            .stations()
            .taf(&arguments.station_id, arguments.issued.timestamp())
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}

#[rmcp::tool_router(router = stations_remaining_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns metadata for one NOAA observation station.
    #[rmcp::tool(
        name = "stations_get",
        description = "Return NOAA metadata for one surface observation station.",
        annotations(
            title = "Get NOAA Observation Station",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_get(
        &self,
        Parameters(arguments): Parameters<StationArguments>,
    ) -> Result<Json<Feature<ObservationStation>>, ToolFailure> {
        self.client
            .stations()
            .get(&arguments.station_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    /// Returns one page of NOAA observation stations.
    #[rmcp::tool(
        name = "stations_list",
        description = "Return one NOAA page of observation stations matching optional identifiers, areas, and pagination fields.",
        annotations(
            title = "List NOAA Observation Stations",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_list(
        &self,
        Parameters(query): Parameters<StationsQuery>,
    ) -> Result<Json<FeatureCollection<ObservationStation>>, ToolFailure> {
        self.client
            .stations()
            .list(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    /// Returns one surface observation recorded at an exact time.
    #[rmcp::tool(
        name = "stations_observation_at",
        description = "Return the NOAA surface observation recorded by one station at an exact instant.",
        annotations(
            title = "Get Station Observation at Time",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn stations_observation_at(
        &self,
        Parameters(arguments): Parameters<ObservationAtArguments>,
    ) -> Result<Json<Feature<Observation>>, ToolFailure> {
        self.client
            .stations()
            .observation_at(&arguments.station_id, arguments.time.timestamp())
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}
