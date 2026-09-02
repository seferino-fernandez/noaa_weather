use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::{Client, Coordinates};

use crate::output::Output;

/// Arguments requiring a specific geographical point.
#[derive(Args, Debug, Clone)]
pub struct PointArgs {
    /// Point as LAT,LON in decimal degrees (e.g., 39.7456,-97.0892).
    /// Latitude must be within -90..=90 and longitude within -180..=180; values are rounded to four decimals.
    #[arg(value_name = "LAT,LON")]
    pub point: Coordinates,
}

/// Access metadata for a specific geographical point.
#[derive(Subcommand, Debug, Clone)]
pub enum PointCommands {
    /// Get metadata for a specific latitude/longitude point.
    ///
    /// Returns information like the responsible forecast office, grid coordinates,
    /// forecast zone, and links to relevant forecast endpoints.
    /// Example: `noaa-weather points metadata 39.7456,-97.0892`
    Metadata(PointArgs),
}

/// Handles the execution of point-related subcommands.
///
/// Dispatches the command to the matching `client.points()` method based on
/// the provided `PointCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific point subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `client` - The NOAA API client.
///
pub async fn handle_command(
    command: &PointCommands,
    output: &Output,
    client: &Client,
) -> Result<()> {
    match command {
        PointCommands::Metadata(args) => {
            output
                .show(
                    format!("getting point metadata for {}", args.point),
                    client.points().get(args.point),
                )
                .await
        }
    }
}
