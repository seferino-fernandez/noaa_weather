use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use serde_json::Value;

#[derive(Args, Debug)]
pub struct OfficeIdArgs {
    /// NWS forecast office ID (e.g., PSR, BOX)
    #[arg(long)]
    office_id: String,
}

#[derive(Subcommand, Debug)]
pub enum OfficeCommands {
    /// Get metadata for a specific NWS forecast office
    Metadata(OfficeIdArgs),
    /// Get news headlines for an office
    Headlines(OfficeIdArgs),
    /// Get a specific news headline for an office
    Headline {
        #[clap(flatten)]
        office_args: OfficeIdArgs,
        /// Specific headline ID
        #[arg(long)]
        headline_id: String,
    },
}

pub async fn handle_command(command: OfficeCommands, config: &Configuration) -> Result<Value> {
    match command {
        OfficeCommands::Metadata(args) => {
            let result = noaa_weather_client::apis::offices::office(config, &args.office_id)
                .await
                .map_err(|e| anyhow!("Error getting office metadata: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        OfficeCommands::Headlines(args) => {
            let result =
                noaa_weather_client::apis::offices::office_headlines(config, &args.office_id)
                    .await
                    .map_err(|e| anyhow!("Error getting office headlines: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        OfficeCommands::Headline {
            office_args,
            headline_id,
        } => {
            let result = noaa_weather_client::apis::offices::office_headline(
                config,
                &office_args.office_id,
                &headline_id,
            )
            .await
            .map_err(|e| anyhow!("Error getting specific office headline: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
