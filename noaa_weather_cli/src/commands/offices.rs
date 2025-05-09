use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::offices as offices_api;

use crate::Cli;

/// Arguments requiring a NWS forecast office ID.
#[derive(Args, Debug, Clone)]
pub struct OfficeIdArgs {
    /// NWS forecast office ID (three-letter identifier, e.g., PSR, BOX, TOP).
    #[arg(long)]
    id: String,
}

/// Access metadata and headlines for NWS forecast offices.
#[derive(Subcommand, Debug, Clone)]
pub enum OfficeCommands {
    /// Get metadata for a specific NWS forecast office.
    ///
    /// Returns details like address, contact info, and responsible areas.
    /// Example: `noaa-weather offices metadata --id TOP`
    Metadata(OfficeIdArgs),
    /// Get recent news headlines for a specific NWS forecast office.
    ///
    /// Example: `noaa-weather offices headlines --id TOP`
    Headlines(OfficeIdArgs),
    /// Get a specific news headline by its ID for an NWS forecast office.
    ///
    /// Headline IDs can be found in the output of the `headlines` subcommand.
    /// Example: `noaa-weather offices headline --id TOP --headline-id "..."`
    Headline {
        #[clap(flatten)]
        office_args: OfficeIdArgs,
        /// Specific headline ID to retrieve.
        #[arg(long)]
        headline_id: String,
    },
}

/// Handles the execution of office-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `OfficeCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific office subcommand and its arguments to execute.
/// * `cli` - The CLI arguments.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &OfficeCommands,
    _cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        OfficeCommands::Metadata(args) => {
            let _result = offices_api::get_forecast_office(config, &args.id)
                .await
                .map_err(|e| anyhow!("Error getting NWS forecast office metadata: {e}"))?;
            Ok(())
        }
        OfficeCommands::Headlines(args) => {
            let _result = offices_api::get_forecast_office_headlines(config, &args.id)
                .await
                .map_err(|e| anyhow!("Error getting NWS forecast office headlines: {e}"))?;
            Ok(())
        }
        OfficeCommands::Headline {
            office_args,
            headline_id,
        } => {
            let _result =
                offices_api::get_forecast_office_headline(config, &office_args.id, headline_id)
                    .await
                    .map_err(|e| anyhow!("Error getting NWS forecast office headline: {e}"))?;
            Ok(())
        }
    }
}
