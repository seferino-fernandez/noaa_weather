use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::{Client, Coordinates};

use super::Run;
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

impl Run for PointCommands {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        match self {
            PointCommands::Metadata(args) => output.show(client.points().get(args.point)).await,
        }
    }
}
