//! Radar tools.

use noaa_weather_client::RadarStationId;
use noaa_weather_client::radar::{
    RadarQueueHost, RadarQueueQuery, RadarQueuesResponse, RadarServerQuery, RadarServerTelemetry,
    RadarServersQuery, RadarServersResponse, RadarSpgdsResponse, RadarStationAlarmsResponse,
    RadarStationQuery, RadarStationTelemetry, RadarStationsQuery, RadarStationsResponse,
    SpgdsQuery,
};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct RadarQueueArguments {
    host: RadarQueueHost,
    #[serde(flatten)]
    query: RadarQueueQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct RadarServerArguments {
    server_id: String,
    #[serde(flatten)]
    query: RadarServerQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct RadarStationArguments {
    station_id: RadarStationId,
    #[serde(flatten)]
    query: RadarStationQuery,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct RadarStationIdArguments {
    station_id: RadarStationId,
}

#[rmcp::tool_router(router = radar_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "radar_queue",
        description = "Return radar data-queue entries for one queue host, matching optional time, station, type, feed, resolution, and limit filters.",
        annotations(
            title = "Get Radar Queue",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radar_queue(
        &self,
        Parameters(arguments): Parameters<RadarQueueArguments>,
    ) -> Result<Json<RadarQueuesResponse>, ToolFailure> {
        self.client
            .radar()
            .queue(&arguments.host, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radar_servers",
        description = "Return radar servers, optionally filtered by reporting host.",
        annotations(
            title = "List Radar Servers",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radar_servers(
        &self,
        Parameters(query): Parameters<RadarServersQuery>,
    ) -> Result<Json<RadarServersResponse>, ToolFailure> {
        self.client
            .radar()
            .servers(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radar_server_get",
        description = "Return one radar server by its server-issued identifier, optionally as seen from a reporting host.",
        annotations(
            title = "Get Radar Server",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radar_server_get(
        &self,
        Parameters(arguments): Parameters<RadarServerArguments>,
    ) -> Result<Json<RadarServerTelemetry>, ToolFailure> {
        self.client
            .radar()
            .server(&arguments.server_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radar_stations",
        description = "Return radar stations matching optional station-type, reporting-host, and queue-host filters.",
        annotations(
            title = "List Radar Stations",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radar_stations(
        &self,
        Parameters(query): Parameters<RadarStationsQuery>,
    ) -> Result<Json<RadarStationsResponse>, ToolFailure> {
        self.client
            .radar()
            .stations(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radar_station_get",
        description = "Return one radar station, optionally as seen from reporting and queue hosts.",
        annotations(
            title = "Get Radar Station",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radar_station_get(
        &self,
        Parameters(arguments): Parameters<RadarStationArguments>,
    ) -> Result<Json<RadarStationTelemetry>, ToolFailure> {
        self.client
            .radar()
            .station(&arguments.station_id, &arguments.query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radar_station_alarms",
        description = "Return the alarms raised by one radar station.",
        annotations(
            title = "List Radar Station Alarms",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radar_station_alarms(
        &self,
        Parameters(RadarStationIdArguments { station_id }): Parameters<RadarStationIdArguments>,
    ) -> Result<Json<RadarStationAlarmsResponse>, ToolFailure> {
        self.client
            .radar()
            .station_alarms(&station_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radar_spgds",
        description = "Return SPGDS radar-host telemetry, optionally filtered by publication interval.",
        annotations(
            title = "Get Radar SPGDS Telemetry",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radar_spgds(
        &self,
        Parameters(query): Parameters<SpgdsQuery>,
    ) -> Result<Json<RadarSpgdsResponse>, ToolFailure> {
        self.client
            .radar()
            .spgds(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}
