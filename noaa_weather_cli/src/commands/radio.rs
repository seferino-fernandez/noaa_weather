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

/// Arguments for listing NOAA Weather Radio transmitters.
#[derive(Args, Debug, Clone)]
pub struct RadioTransmittersArgs {
    /// Pagination cursor returned by a previous request.
    #[arg(long)]
    cursor: Option<String>,
}

/// Arguments for a county-zone transmitter lookup.
#[derive(Args, Debug, Clone)]
pub struct RadioZoneArgs {
    /// NWS county zone identifier (for example, AZC013).
    zone_id: String,
}

/// Access NOAA Weather Radio broadcast information.
#[derive(Subcommand, Debug, Clone)]
pub enum RadioCommands {
    /// List NOAA Weather Radio transmitters.
    Transmitters(RadioTransmittersArgs),
    /// Get metadata for one NOAA Weather Radio transmitter.
    Transmitter(StationRadioArgs),
    /// List transmitters serving an NWS county zone.
    Zone(RadioZoneArgs),
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
        RadioCommands::Transmitters(args) => {
            let result = radio_api::get_radio_transmitters(config, args.cursor.as_deref())
                .await
                .map_err(|error| anyhow!("listing radio transmitters: {error}"))?;
            let content = if cli.json {
                serde_json::to_string_pretty(&result)?
            } else {
                tables::radio::create_radio_transmitters_table(&result).to_string()
            };
            write_output(cli.output.as_deref(), &content)
        }
        RadioCommands::Transmitter(args) => {
            let result = radio_api::get_radio_transmitter(config, &args.call_sign)
                .await
                .map_err(|error| {
                    anyhow!("getting radio transmitter {}: {error}", args.call_sign)
                })?;
            let content = if cli.json {
                serde_json::to_string_pretty(&result)?
            } else {
                tables::radio::create_radio_transmitter_table(&result).to_string()
            };
            write_output(cli.output.as_deref(), &content)
        }
        RadioCommands::Zone(args) => {
            let result = radio_api::get_radio_transmitters_for_county_zone(config, &args.zone_id)
                .await
                .map_err(|error| {
                    anyhow!(
                        "listing radio transmitters for county zone {}: {error}",
                        args.zone_id
                    )
                })?;
            let content = if cli.json {
                serde_json::to_string_pretty(&result)?
            } else {
                tables::radio::create_radio_transmitters_table(&result).to_string()
            };
            write_output(cli.output.as_deref(), &content)
        }
    }
}
