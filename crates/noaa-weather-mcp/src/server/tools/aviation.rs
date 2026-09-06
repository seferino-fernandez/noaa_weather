//! Aviation tools.

use std::borrow::Cow;

use jiff::civil::Date;
use noaa_weather_client::aviation::{CenterWeatherAdvisory, CwsuOffice, Sigmet, SigmetsQuery};
use noaa_weather_client::geo::{Feature, FeatureCollection};
use noaa_weather_client::{AtsuId, CwsuId, OffsetDateTime};
use rmcp::Json;
use rmcp::handler::server::wrapper::Parameters;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(try_from = "u32")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct CwaSequence(u32);

impl TryFrom<u32> for CwaSequence {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 100 {
            return Err("CWA sequence must be at least 100");
        }
        Ok(Self(value))
    }
}

impl JsonSchema for CwaSequence {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> Cow<'static, str> {
        "CwaSequence".into()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        schemars::json_schema!({
            "type": "integer",
            "format": "uint32",
            "minimum": 100,
        })
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct CwsuArguments {
    cwsu_id: CwsuId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct CwaArguments {
    cwsu_id: CwsuId,
    date: Date,
    sequence: CwaSequence,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct AtsuArguments {
    atsu_id: AtsuId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct AtsuDateArguments {
    atsu_id: AtsuId,
    date: Date,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code, reason = "used when the family router is composed")]
struct SigmetArguments {
    atsu_id: AtsuId,
    issued: OffsetDateTime,
}

#[rmcp::tool_router(router = aviation_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    #[rmcp::tool(
        name = "aviation_cwsu_get",
        description = "Return metadata for one NOAA Center Weather Service Unit.",
        annotations(
            title = "Get Center Weather Service Unit",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn aviation_cwsu_get(
        &self,
        Parameters(CwsuArguments { cwsu_id }): Parameters<CwsuArguments>,
    ) -> Result<Json<CwsuOffice>, ToolFailure> {
        self.client
            .aviation()
            .cwsu(&cwsu_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "aviation_cwas",
        description = "Return the current NOAA Center Weather Advisories from one Center Weather Service Unit.",
        annotations(
            title = "List Center Weather Advisories",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn aviation_cwas(
        &self,
        Parameters(CwsuArguments { cwsu_id }): Parameters<CwsuArguments>,
    ) -> Result<Json<FeatureCollection<CenterWeatherAdvisory>>, ToolFailure> {
        self.client
            .aviation()
            .cwas(&cwsu_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "aviation_cwa_get",
        description = "Return one NOAA Center Weather Advisory by service unit, UTC issue date, and sequence number.",
        annotations(
            title = "Get Center Weather Advisory",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn aviation_cwa_get(
        &self,
        Parameters(CwaArguments {
            cwsu_id,
            date,
            sequence,
        }): Parameters<CwaArguments>,
    ) -> Result<Json<Feature<CenterWeatherAdvisory>>, ToolFailure> {
        self.client
            .aviation()
            .cwa(&cwsu_id, date, sequence.0)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "aviation_sigmets",
        description = "Return SIGMET and AIRMET products matching optional issue-time, date, issuing-unit, and sequence filters.",
        annotations(
            title = "Search SIGMETs and AIRMETs",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn aviation_sigmets(
        &self,
        Parameters(query): Parameters<SigmetsQuery>,
    ) -> Result<Json<FeatureCollection<Sigmet>>, ToolFailure> {
        self.client
            .aviation()
            .sigmets(&query)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "aviation_sigmets_for_atsu",
        description = "Return SIGMET and AIRMET products from one Air Traffic Service Unit.",
        annotations(
            title = "List SIGMETs for ATSU",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn aviation_sigmets_for_atsu(
        &self,
        Parameters(AtsuArguments { atsu_id }): Parameters<AtsuArguments>,
    ) -> Result<Json<FeatureCollection<Sigmet>>, ToolFailure> {
        self.client
            .aviation()
            .sigmets_for_atsu(&atsu_id)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "aviation_sigmets_for_atsu_on_date",
        description = "Return SIGMET and AIRMET products from one Air Traffic Service Unit on one UTC date.",
        annotations(
            title = "List SIGMETs for ATSU on Date",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn aviation_sigmets_for_atsu_on_date(
        &self,
        Parameters(AtsuDateArguments { atsu_id, date }): Parameters<AtsuDateArguments>,
    ) -> Result<Json<FeatureCollection<Sigmet>>, ToolFailure> {
        self.client
            .aviation()
            .sigmets_for_atsu_on(&atsu_id, date)
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }

    #[rmcp::tool(
        name = "aviation_sigmet_get",
        description = "Return one SIGMET or AIRMET by issuing unit and issue instant.",
        annotations(
            title = "Get SIGMET or AIRMET",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn aviation_sigmet_get(
        &self,
        Parameters(SigmetArguments { atsu_id, issued }): Parameters<SigmetArguments>,
    ) -> Result<Json<Feature<Sigmet>>, ToolFailure> {
        self.client
            .aviation()
            .sigmet(&atsu_id, issued.timestamp())
            .await
            .map(Json)
            .map_err(ToolFailure::from)
    }
}
