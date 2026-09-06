//! NOAA Weather Radio tools.

use noaa_weather_client::radio::{
    RadioBroadcast, RadioTransmitter, RadioTransmitterCollection, TransmittersQuery,
};
use noaa_weather_client::{CallSign, Coordinates, ZoneId};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct PointArguments {
    point: Coordinates,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct CallSignArguments {
    call_sign: CallSign,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct CountyArguments {
    zone_id: ZoneId,
}

#[rmcp::tool_router(router = radio_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "radio_broadcast_for_point",
        description = "Return the decoded NOAA Weather Radio broadcast serving a latitude and longitude point.",
        annotations(
            title = "Get Radio Broadcast for Point",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radio_broadcast_for_point(
        &self,
        Parameters(PointArguments { point }): Parameters<PointArguments>,
    ) -> Result<Json<RadioBroadcast>, ToolFailure> {
        self.client
            .radio()
            .for_point(point)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radio_broadcast",
        description = "Return the decoded current NOAA Weather Radio broadcast for one transmitter call sign.",
        annotations(
            title = "Get Radio Broadcast",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radio_broadcast(
        &self,
        Parameters(CallSignArguments { call_sign }): Parameters<CallSignArguments>,
    ) -> Result<Json<RadioBroadcast>, ToolFailure> {
        self.client
            .radio()
            .broadcast(&call_sign)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radio_transmitters",
        description = "Return one NOAA page of Weather Radio transmitter metadata.",
        annotations(
            title = "List Radio Transmitters",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radio_transmitters(
        &self,
        Parameters(query): Parameters<TransmittersQuery>,
    ) -> Result<Json<RadioTransmitterCollection>, ToolFailure> {
        self.client
            .radio()
            .transmitters(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radio_transmitter_get",
        description = "Return metadata for one NOAA Weather Radio transmitter by call sign.",
        annotations(
            title = "Get Radio Transmitter",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radio_transmitter_get(
        &self,
        Parameters(CallSignArguments { call_sign }): Parameters<CallSignArguments>,
    ) -> Result<Json<RadioTransmitter>, ToolFailure> {
        self.client
            .radio()
            .transmitter(&call_sign)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "radio_transmitters_for_county",
        description = "Return the NOAA Weather Radio transmitters serving one county zone.",
        annotations(
            title = "List Radio Transmitters for County",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn radio_transmitters_for_county(
        &self,
        Parameters(CountyArguments { zone_id }): Parameters<CountyArguments>,
    ) -> Result<Json<RadioTransmitterCollection>, ToolFailure> {
        self.client
            .radio()
            .transmitters_for_county(&zone_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}
