use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::radio::TransmittersQuery;
use noaa_weather_client::{CallSign, Client, Coordinates, Cursor, ZoneId};

use super::Run;
use crate::output::Output;

/// Arguments for getting the radio broadcast for a geographic point.
#[derive(Args, Debug, Clone)]
pub struct PointRadioArgs {
    /// Point as LAT,LON in decimal degrees (e.g., 33.4484,-112.0740).
    #[arg(value_name = "LAT,LON")]
    pub point: Coordinates,
}

/// Arguments for getting the radio broadcast for a transmitter station.
#[derive(Args, Debug, Clone)]
pub struct StationRadioArgs {
    /// Transmitter call sign (e.g., KEC94).
    pub call_sign: CallSign,
}

/// Arguments for listing NOAA Weather Radio transmitters.
#[derive(Args, Debug, Clone)]
pub struct RadioTransmittersArgs {
    /// Opaque pagination cursor from a previous page (see pagination.next in --json output)
    #[arg(long)]
    cursor: Option<Cursor>,
}

/// Arguments for a county-zone transmitter lookup.
#[derive(Args, Debug, Clone)]
pub struct RadioZoneArgs {
    /// NWS county zone identifier (for example, AZC013).
    zone_id: ZoneId,
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
    /// Example: `noaa-weather radio point 33.4484,-112.0740`
    #[clap(name = "point")]
    Point(PointRadioArgs),
    /// Get the NOAA Weather Radio broadcast for a transmitter station.
    ///
    /// Example: `noaa-weather radio station KEC94`
    #[clap(name = "station")]
    Station(StationRadioArgs),
}

impl Run for RadioCommands {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        let radio = client.radio();
        match self {
            RadioCommands::Point(args) => output.show(radio.for_point(args.point)).await,
            RadioCommands::Station(args) => output.show(radio.broadcast(&args.call_sign)).await,
            RadioCommands::Transmitters(args) => {
                let query = TransmittersQuery {
                    cursor: args.cursor.clone(),
                };
                output.show(radio.transmitters(&query)).await
            }
            RadioCommands::Transmitter(args) => {
                output.show(radio.transmitter(&args.call_sign)).await
            }
            RadioCommands::Zone(args) => {
                output
                    .show(radio.transmitters_for_county(&args.zone_id))
                    .await
            }
        }
    }
}
