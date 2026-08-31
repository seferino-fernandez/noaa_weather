use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::gridpoints as gridpoints_api;
use noaa_weather_client::models::{GridpointForecastUnits, NwsForecastOfficeId};

use crate::output::Output;

/// Common arguments for identifying a specific NWS gridpoint.
#[derive(Args, Debug, Clone)]
pub struct GridpointLocationArgs {
    /// NWS forecast office ID (e.g., TOP, LWX).
    /// Use the `points` command to find the office for a location.
    #[arg(long, value_enum)]
    forecast_office_id: NwsForecastOfficeId,

    /// Grid X coordinate.
    /// Use the `points` command to find grid coordinates.
    /// The grid coordinates must be greater than 0.
    #[arg(short, long, value_parser = clap::value_parser!(i32).range(1..))]
    x: i32,

    /// Grid Y coordinate.
    /// Use the `points` command to find grid coordinates.
    /// The grid coordinates must be greater than 0.
    #[arg(short, long, value_parser = clap::value_parser!(i32).range(1..))]
    y: i32,
}

/// Access forecast data for specific NWS gridpoints.
///
/// Gridpoints represent a 2.5km square area used by the NWS for forecasts.
/// Use the `points` command to find the correct gridpoint (office ID, X, Y)
/// for a given latitude/longitude.
#[derive(Subcommand, Debug, Clone)]
pub enum GridpointCommands {
    /// Get raw numerical forecast data layers for a gridpoint.
    ///
    /// Returns detailed data like temperature, humidity, wind speed, etc.,
    /// for various time intervals.
    /// Example: `noaa-weather gridpoints gridpoint --forecast-office-id TOP -x 31 -y 80`
    Gridpoint {
        #[clap(flatten)]
        location: GridpointLocationArgs,
    },
    /// Get the multi-day textual forecast for a gridpoint.
    ///
    /// Returns a human-readable forecast summary broken down into periods (e.g., "Tonight", "Thursday").
    /// Example: `noaa-weather gridpoints forecast --forecast-office-id PSR -x 159 -y 100 --units si`
    Forecast {
        #[clap(flatten)]
        location: GridpointLocationArgs,
        /// Specify units for forecast data (`us` for US customary, `si` for Metric).
        #[arg(long, value_enum)]
        units: Option<GridpointForecastUnits>,
    },
    /// Get the hourly textual forecast for a gridpoint.
    ///
    /// Returns a human-readable forecast summary broken down by hour.
    /// Example: `noaa-weather gridpoints hourly --forecast-office-id PSR -x 159 -y 100`
    ForecastHourly {
        #[clap(flatten)]
        location: GridpointLocationArgs,
        /// Specify units for forecast data (`us` for US customary, `si` for Metric).
        #[arg(long, value_enum)]
        units: Option<GridpointForecastUnits>,
    },
    /// List observation stations usable for retrieving observations for a gridpoint.
    ///
    /// Returns a list of nearby stations that can provide current weather conditions.
    /// Example: `noaa-weather gridpoints stations --forecast-office-id PSR -x 159 -y 100 --limit 5`
    Stations {
        #[clap(flatten)]
        location: GridpointLocationArgs,
        /// Limit the number of observation stations returned by the API.
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=500))]
        limit: Option<i32>,
    },
}

/// Handles the execution of gridpoint-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `GridpointCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific gridpoint subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &GridpointCommands,
    output: &Output,
    config: &Configuration,
) -> Result<()> {
    match command {
        GridpointCommands::Gridpoint { location } => {
            output
                .show(
                    "getting raw gridpoint data",
                    gridpoints_api::get_gridpoint(
                        config,
                        location.forecast_office_id,
                        location.x,
                        location.y,
                    ),
                )
                .await?;
        }
        GridpointCommands::Forecast { location, units } => {
            output
                .show(
                    "getting gridpoint forecast",
                    gridpoints_api::get_gridpoint_forecast(
                        config,
                        location.forecast_office_id,
                        location.x,
                        location.y,
                        *units,
                    ),
                )
                .await?;
        }
        GridpointCommands::ForecastHourly { location, units } => {
            output
                .show(
                    "getting hourly gridpoint forecast",
                    gridpoints_api::get_gridpoint_forecast_hourly(
                        config,
                        location.forecast_office_id,
                        location.x,
                        location.y,
                        *units,
                    ),
                )
                .await?;
        }
        GridpointCommands::Stations { location, limit } => {
            output
                .show(
                    "getting gridpoint stations",
                    gridpoints_api::get_gridpoint_stations(
                        config,
                        location.forecast_office_id,
                        location.x,
                        location.y,
                        *limit,
                    ),
                )
                .await?;
        }
    }
    Ok(())
}
