use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::points as points_api;
use serde_json::Value;

/// Arguments requiring a specific geographical point.
#[derive(Args, Debug)]
pub struct PointArgs {
    /// Geographical point specified as "latitude,longitude" (e.g., "39.7456,-97.0892").
    point: String,
}

/// Access metadata and nearby stations for a specific geographical point.
#[derive(Subcommand, Debug)]
pub enum PointCommands {
    /// Get metadata for a specific latitude,longitude point.
    ///
    /// Returns information like the responsible forecast office, grid coordinates,
    /// forecast zone, and links to relevant forecast endpoints.
    /// Example: `noaa-weather points metadata "39.7456,-97.0892"`
    Metadata(PointArgs),
    /// Get a list of observation stations near a specific latitude,longitude point.
    ///
    /// Example: `noaa-weather points stations "39.7456,-97.0892"`
    Stations(PointArgs),
}

/// Handles the execution of point-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `PointCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific point subcommand and its arguments to execute.
/// * `config` - The application configuration containing API details.
///
/// # Returns
///
/// A `Result` containing the JSON `Value` of the API response on success,
/// or an `anyhow::Error` if an error occurs during the API call or processing.
pub async fn handle_command(command: PointCommands, config: &Configuration) -> Result<Value> {
    match command {
        PointCommands::Metadata(args) => {
            let result = points_api::get_point(config, &args.point)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting point metadata: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        PointCommands::Stations(args) => {
            let result = points_api::get_point_stations(config, &args.point)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting point stations: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
