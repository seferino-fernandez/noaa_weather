//! Gridpoint forecast tools.

use noaa_weather_client::GridpointId;
use noaa_weather_client::geo::Feature;
use noaa_weather_client::gridpoints::{Forecast, ForecastQuery};
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct GridpointForecastParameters {
    /// NOAA forecast office and grid coordinates as `OFFICE/x,y`.
    gridpoint_id: GridpointId,
    /// Existing forecast options exposed as flat tool arguments.
    #[serde(flatten)]
    query: ForecastQuery,
}

#[rmcp::tool_router(router = gridpoints_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns the multi-day textual forecast for one NOAA grid cell.
    #[rmcp::tool(
        name = "gridpoints_forecast",
        description = "Get the multi-day NOAA textual forecast for a forecast-office grid cell.",
        annotations(
            title = "Get NOAA Gridpoint Forecast",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn gridpoints_forecast(
        &self,
        Parameters(parameters): Parameters<GridpointForecastParameters>,
    ) -> Result<rmcp::Json<Feature<Forecast>>, ToolFailure> {
        let forecast = self
            .client
            .gridpoints()
            .forecast(&parameters.gridpoint_id, &parameters.query)
            .await?;
        Ok(rmcp::Json(forecast))
    }

    /// Returns the hour-by-hour textual forecast for one NOAA grid cell.
    #[rmcp::tool(
        name = "gridpoints_forecast_hourly",
        description = "Get the hour-by-hour NOAA textual forecast for a forecast-office grid cell.",
        annotations(
            title = "Get NOAA Hourly Gridpoint Forecast",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn gridpoints_forecast_hourly(
        &self,
        Parameters(parameters): Parameters<GridpointForecastParameters>,
    ) -> Result<rmcp::Json<Feature<Forecast>>, ToolFailure> {
        let forecast = self
            .client
            .gridpoints()
            .forecast_hourly(&parameters.gridpoint_id, &parameters.query)
            .await?;
        Ok(rmcp::Json(forecast))
    }
}
