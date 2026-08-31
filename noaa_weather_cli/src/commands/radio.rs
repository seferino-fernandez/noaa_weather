use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::radio as radio_api;

use crate::output::Output;

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
/// * `output` - The configured output policy.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &RadioCommands,
    output: &Output,
    config: &Configuration,
) -> Result<()> {
    match command {
        RadioCommands::Point(args) => {
            output
                .show(
                    format!(
                        "getting radio broadcast for point {},{}",
                        args.latitude, args.longitude
                    ),
                    radio_api::get_point_radio(config, args.latitude, args.longitude),
                )
                .await
        }
        RadioCommands::Station(args) => {
            output
                .show(
                    format!("getting radio broadcast for station {}", args.call_sign),
                    radio_api::get_area_radio(config, &args.call_sign),
                )
                .await
        }
        RadioCommands::Transmitters(args) => {
            output
                .show(
                    "listing radio transmitters",
                    radio_api::get_radio_transmitters(config, args.cursor.as_deref()),
                )
                .await
        }
        RadioCommands::Transmitter(args) => {
            output
                .show(
                    format!("getting radio transmitter {}", args.call_sign),
                    radio_api::get_radio_transmitter(config, &args.call_sign),
                )
                .await
        }
        RadioCommands::Zone(args) => {
            output
                .show(
                    format!(
                        "listing radio transmitters for county zone {}",
                        args.zone_id
                    ),
                    radio_api::get_radio_transmitters_for_county_zone(config, &args.zone_id),
                )
                .await
        }
    }
}
