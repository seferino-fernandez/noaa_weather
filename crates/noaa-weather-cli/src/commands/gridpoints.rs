use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::gridpoints::{ForecastQuery, GridpointStationsQuery};
use noaa_weather_client::{Client, GridpointId};

use super::Run;
use crate::output::Output;

/// The grid cell every gridpoint command addresses.
#[derive(Args, Debug, Clone)]
pub struct GridpointLocationArgs {
    /// Grid cell as OFFICE/X,Y (e.g., TOP/31,80).
    /// Use the `points metadata` command to find the office and grid coordinates for a location.
    #[arg(value_name = "OFFICE/X,Y")]
    gridpoint: GridpointId,
}

/// Access forecast data for specific NWS gridpoints.
///
/// Gridpoints represent a 2.5km square area used by the NWS for forecasts.
/// Use the `points` command to find the correct gridpoint (OFFICE/X,Y)
/// for a given latitude/longitude.
#[derive(Subcommand, Debug, Clone)]
pub enum GridpointCommands {
    /// Get raw numerical forecast data layers for a gridpoint.
    ///
    /// Returns detailed data like temperature, humidity, wind speed, etc.,
    /// for various time intervals.
    /// Example: `noaa-weather gridpoints gridpoint TOP/31,80`
    Gridpoint {
        #[clap(flatten)]
        location: GridpointLocationArgs,
    },
    /// Get the multi-day textual forecast for a gridpoint.
    ///
    /// Returns a human-readable forecast summary broken down into periods (e.g., "Tonight", "Thursday").
    /// Example: `noaa-weather gridpoints forecast PSR/159,100`
    Forecast {
        #[clap(flatten)]
        location: GridpointLocationArgs,
    },
    /// Get the hourly textual forecast for a gridpoint.
    ///
    /// Returns a human-readable forecast summary broken down by hour.
    /// Example: `noaa-weather gridpoints forecast-hourly PSR/159,100`
    ForecastHourly {
        #[clap(flatten)]
        location: GridpointLocationArgs,
    },
    /// List observation stations usable for retrieving observations for a gridpoint.
    ///
    /// Returns a list of nearby stations that can provide current weather conditions.
    /// Example: `noaa-weather gridpoints stations PSR/159,100 --limit 5`
    Stations {
        #[clap(flatten)]
        location: GridpointLocationArgs,
        /// Limit the number of observation stations returned by the API (1 to 500).
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=500))]
        limit: Option<u16>,
    },
}

impl Run for GridpointCommands {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        let gridpoints = client.gridpoints();
        match self {
            GridpointCommands::Gridpoint { location } => {
                output.show(gridpoints.get(&location.gridpoint)).await
            }
            // NOAA's own `units` parameter is inert under the feature flags this
            // crate always sends, so the request never carries one and the global
            // `--units` decides what the reader sees.
            GridpointCommands::Forecast { location } => {
                output
                    .show(gridpoints.forecast(&location.gridpoint, &ForecastQuery::default()))
                    .await
            }
            GridpointCommands::ForecastHourly { location } => {
                output
                    .show(
                        gridpoints.forecast_hourly(&location.gridpoint, &ForecastQuery::default()),
                    )
                    .await
            }
            GridpointCommands::Stations { location, limit } => {
                output
                    .show(gridpoints.stations(
                        &location.gridpoint,
                        &GridpointStationsQuery { limit: *limit },
                    ))
                    .await
            }
        }
    }
}
