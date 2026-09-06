//! Point metadata and point-to-forecast tools.

use noaa_weather_client::Coordinates;
use noaa_weather_client::geo::Feature;
use noaa_weather_client::gridpoints::Forecast;
use noaa_weather_client::points::Point;
use rmcp::handler::server::wrapper::Parameters;
use schemars::JsonSchema;
use serde::Deserialize;

use super::super::NoaaWeatherServer;
use super::error::ToolFailure;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct PointParameters {
    /// Latitude and longitude as `latitude,longitude` in decimal degrees.
    point: Coordinates,
}

#[rmcp::tool_router(router = points_router, vis = "pub(super)")]
impl NoaaWeatherServer {
    /// Returns NOAA metadata for the forecast grid and zones covering a point.
    #[rmcp::tool(
        name = "points_get",
        description = "Get NOAA metadata for the forecast grid and zones covering a latitude/longitude point.",
        annotations(
            title = "Get NOAA Point Metadata",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn points_get(
        &self,
        Parameters(parameters): Parameters<PointParameters>,
    ) -> Result<rmcp::Json<Feature<Point>>, ToolFailure> {
        let point = self.client.points().get(parameters.point).await?;
        Ok(rmcp::Json(point))
    }

    /// Resolves a point to its NOAA grid cell and returns the multi-day forecast.
    #[rmcp::tool(
        name = "points_forecast",
        description = "Resolve a latitude/longitude point to its NOAA grid cell and get the multi-day textual forecast.",
        annotations(
            title = "Get NOAA Forecast for Point",
            read_only_hint = true,
            open_world_hint = true
        )
    )]
    async fn points_forecast(
        &self,
        Parameters(parameters): Parameters<PointParameters>,
    ) -> Result<rmcp::Json<Feature<Forecast>>, ToolFailure> {
        let forecast = self.client.points().forecast_for(parameters.point).await?;
        Ok(rmcp::Json(forecast))
    }
}
