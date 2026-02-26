use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::radio as radio_api;

use crate::utils::format::write_output;
use crate::{Cli, tables};

/// Arguments for getting the radio broadcast for a geographic point.
#[derive(Args, Debug, Clone)]
pub struct PointRadioArgs {
    /// Latitude of the point (e.g., 33.4484).
    pub latitude: f64,
    /// Longitude of the point (e.g., -112.0740).
    pub longitude: f64,
}

/// Arguments for getting the radio broadcast for a transmitter station.
#[derive(Args, Debug, Clone)]
pub struct StationRadioArgs {
    /// Transmitter call sign (e.g., KEC94).
    pub call_sign: String,
}

/// Access NOAA Weather Radio broadcast information.
#[derive(Subcommand, Debug, Clone)]
pub enum RadioCommands {
    /// Get the NOAA Weather Radio broadcast for a geographic point.
    ///
    /// Example: `noaa-weather radio point 33.4484 -- -112.0740`
    #[clap(name = "point")]
    Point(PointRadioArgs),
    /// Get the NOAA Weather Radio broadcast for a transmitter station.
    ///
    /// Example: `noaa-weather radio station KEC94`
    #[clap(name = "station")]
    Station(StationRadioArgs),
}

/// Handles the execution of radio-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `RadioCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific radio subcommand and its arguments to execute.
/// * `cli` - The CLI arguments, including the `--json` flag and output path.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &RadioCommands,
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        RadioCommands::Point(args) => {
            let result = radio_api::get_point_radio(config, args.latitude, args.longitude)
                .await
                .map_err(|error| anyhow!("getting radio broadcast for point: {}", error))?;
            let content = if cli.json {
                serde_json::to_string_pretty(&result)?
            } else {
                tables::radio::format_radio_broadcast(&result)
            };
            write_output(cli.output.as_deref(), &content)?;
            Ok(())
        }
        RadioCommands::Station(args) => {
            let result = radio_api::get_area_radio(config, &args.call_sign)
                .await
                .map_err(|error| anyhow!("getting radio broadcast for station: {}", error))?;
            let content = if cli.json {
                serde_json::to_string_pretty(&result)?
            } else {
                tables::radio::format_radio_broadcast(&result)
            };
            write_output(cli.output.as_deref(), &content)?;
            Ok(())
        }
    }
}
