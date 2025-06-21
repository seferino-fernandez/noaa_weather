use crate::utils::format::write_output;

use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::points as points_api;

use crate::{Cli, tables};

/// Arguments requiring a specific geographical point.
#[derive(Args, Debug, Clone)]
pub struct PointArgs {
    /// Geographical point specified as "latitude,longitude" (e.g., "39.7456,-97.0892").
    point: String,
}

/// Access metadata and nearby stations for a specific geographical point.
#[derive(Subcommand, Debug, Clone)]
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
/// * `cli` - The CLI arguments, including the `--json` flag.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &PointCommands,
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        PointCommands::Metadata(args) => {
            let result = points_api::get_point(config, &args.point)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting point metadata: {}", e))?;

            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::points::create_point_metadata_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        PointCommands::Stations(args) => {
            let result = points_api::get_point_stations(config, &args.point)
                .await
                .map_err(|error| anyhow::anyhow!("Error getting point stations: {}", error))?;

            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
    }
}
