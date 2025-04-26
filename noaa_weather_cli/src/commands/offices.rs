use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::offices as offices_api;
use serde_json::Value;

/// Arguments requiring a NWS forecast office ID.
#[derive(Args, Debug)]
pub struct OfficeIdArgs {
    /// NWS forecast office ID (three-letter identifier, e.g., PSR, BOX, TOP).
    #[arg(long)]
    id: String,
}

/// Access metadata and headlines for NWS forecast offices.
#[derive(Subcommand, Debug)]
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
/// * `config` - The application configuration containing API details.
///
/// # Returns
///
/// A `Result` containing the JSON `Value` of the API response on success,
/// or an `anyhow::Error` if an error occurs during the API call or processing.
pub async fn handle_command(command: OfficeCommands, config: &Configuration) -> Result<Value> {
    match command {
        OfficeCommands::Metadata(args) => {
            let result = offices_api::get_forecast_office(config, &args.id)
                .await
                .map_err(|e| anyhow!("Error getting NWS forecast office metadata: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        OfficeCommands::Headlines(args) => {
            let result = offices_api::get_forecast_office_headlines(config, &args.id)
                .await
                .map_err(|e| anyhow!("Error getting NWS forecast office headlines: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        OfficeCommands::Headline {
            office_args,
            headline_id,
        } => {
            let result =
                offices_api::get_forecast_office_headline(config, &office_args.id, &headline_id)
                    .await
                    .map_err(|e| anyhow!("Error getting NWS forecast office headline: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
