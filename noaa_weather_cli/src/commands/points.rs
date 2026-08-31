use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::points as points_api;

use crate::output::Output;

/// Arguments requiring a specific geographical point.
#[derive(Args, Debug, Clone)]
pub struct PointArgs {
    /// Latitude of the point (e.g., 39.7456).
    pub latitude: f64,
    /// Longitude of the point (e.g., -97.0892).
    pub longitude: f64,
}

/// Access metadata for a specific geographical point.
#[derive(Subcommand, Debug, Clone)]
pub enum PointCommands {
    /// Get metadata for a specific latitude/longitude point.
    ///
    /// Returns information like the responsible forecast office, grid coordinates,
    /// forecast zone, and links to relevant forecast endpoints.
    /// Example: `noaa-weather points metadata 39.7456 -- -97.0892`
    Metadata(PointArgs),
}

/// Handles the execution of point-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `PointCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific point subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &PointCommands,
    output: &Output,
    config: &Configuration,
) -> Result<()> {
    match command {
        PointCommands::Metadata(args) => {
            output
                .show(
                    "getting point metadata",
                    points_api::get_point(config, args.latitude, args.longitude),
                )
                .await
        }
    }
}
