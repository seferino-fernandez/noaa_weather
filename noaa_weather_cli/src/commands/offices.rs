use anyhow::{Context as _, Result, anyhow, bail};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::BinaryPayload;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::offices as offices_api;
use noaa_weather_client::models::NwsOfficeId;

use crate::utils::format::write_output;
use crate::{Cli, tables};

/// Arguments requiring a NWS office ID.
#[derive(Args, Debug, Clone)]
pub struct OfficeIdArgs {
    /// NWS office ID (three-letter identifier, e.g., PSR, WRH, NWS).
    #[arg(long)]
    id: NwsOfficeId,
}

/// Access metadata and headlines for NWS offices.
#[derive(Subcommand, Debug, Clone)]
pub enum OfficeCommands {
    /// Get metadata for a specific NWS forecast office.
    ///
    /// Returns details like address, contact info, and responsible areas.
    /// Example: `noaa-weather offices metadata --id PSR`
    Metadata(OfficeIdArgs),
    /// Get recent news headlines for a specific NWS forecast office.
    ///
    /// Example: `noaa-weather offices headlines --id PSR`
    Headlines(OfficeIdArgs),
    /// Get a specific news headline by its ID for an NWS forecast office.
    ///
    /// Headline IDs can be found in the output of the `headlines` subcommand.
    /// Example: `noaa-weather offices headline --id PSR --headline-id "..."`
    Headline {
        #[clap(flatten)]
        office_args: OfficeIdArgs,
        /// Specific headline ID to retrieve.
        #[arg(long)]
        headline_id: String,
    },
    /// Get metadata for the active office briefing, if one exists.
    Briefing(OfficeIdArgs),
    /// Download a briefing PDF by document ID.
    BriefingDownload {
        #[clap(flatten)]
        office_args: OfficeIdArgs,
        /// Briefing document identifier from `offices briefing`.
        #[arg(long)]
        document_id: String,
    },
    /// Download the latest briefing PDF.
    BriefingDownloadLatest(OfficeIdArgs),
    /// Get metadata for active office weather stories.
    WeatherStories(OfficeIdArgs),
    /// Download a weather-story image by story ID.
    WeatherStoryImage {
        #[clap(flatten)]
        office_args: OfficeIdArgs,
        /// Weather-story identifier from `offices weather-stories`.
        #[arg(long)]
        story_id: String,
    },
}

fn binary_output_path<'a>(cli: &'a Cli, operation: &str) -> Result<&'a str> {
    if cli.json {
        bail!("{operation} produces binary data; --json cannot be used");
    }
    cli.output
        .as_deref()
        .ok_or_else(|| anyhow!("{operation} requires --output <PATH>"))
}

fn write_binary(path: &str, payload: &BinaryPayload, operation: &str) -> Result<()> {
    std::fs::write(path, payload.as_bytes())
        .with_context(|| format!("{operation}: writing binary output to {path}"))
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
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        OfficeCommands::Metadata(args) => {
            let result = offices_api::get_forecast_office(config, &args.id)
                .await
                .map_err(|e| anyhow!("Error getting NWS forecast office metadata: {e}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::offices::create_office_metadata_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        OfficeCommands::Headlines(args) => {
            let result = offices_api::get_forecast_office_headlines(config, &args.id)
                .await
                .map_err(|error| anyhow!("Error getting NWS forecast office headlines: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::offices::create_office_headlines_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        OfficeCommands::Headline {
            office_args,
            headline_id,
        } => {
            let result =
                offices_api::get_forecast_office_headline(config, &office_args.id, headline_id)
                    .await
                    .map_err(|error| {
                        anyhow!("Error getting NWS forecast office headline: {error}")
                    })?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::offices::create_office_headline_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        OfficeCommands::Briefing(args) => {
            let result = offices_api::get_forecast_office_briefing(config, &args.id)
                .await
                .context("getting NWS forecast office briefing")?;
            let content = if cli.json {
                serde_json::to_string_pretty(&result)?
            } else {
                tables::offices::create_office_briefing_table(&result).to_string()
            };
            write_output(cli.output.as_deref(), &content)
        }
        OfficeCommands::BriefingDownload {
            office_args,
            document_id,
        } => {
            let operation = "downloading NWS forecast office briefing document";
            let output = binary_output_path(&cli, operation)?;
            let result = offices_api::get_forecast_office_briefing_document(
                config,
                &office_args.id,
                document_id,
            )
            .await
            .with_context(|| operation)?;
            write_binary(output, &result, operation)
        }
        OfficeCommands::BriefingDownloadLatest(args) => {
            let operation = "downloading latest NWS forecast office briefing document";
            let output = binary_output_path(&cli, operation)?;
            let result =
                offices_api::get_latest_forecast_office_briefing_document(config, &args.id)
                    .await
                    .with_context(|| operation)?;
            write_binary(output, &result, operation)
        }
        OfficeCommands::WeatherStories(args) => {
            let result = offices_api::get_forecast_office_weather_stories(config, &args.id)
                .await
                .context("getting NWS forecast office weather stories")?;
            let content = if cli.json {
                serde_json::to_string_pretty(&result)?
            } else {
                tables::offices::create_office_weather_stories_table(&result).to_string()
            };
            write_output(cli.output.as_deref(), &content)
        }
        OfficeCommands::WeatherStoryImage {
            office_args,
            story_id,
        } => {
            let operation = "downloading NWS forecast office weather-story image";
            let output = binary_output_path(&cli, operation)?;
            let result = offices_api::get_forecast_office_weather_story_image(
                config,
                &office_args.id,
                story_id,
            )
            .await
            .with_context(|| operation)?;
            write_binary(output, &result, operation)
        }
    }
}
