use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::{Client, OfficeId};

use super::{Run, parse};
use crate::output::Output;

/// Arguments requiring a NWS office ID.
#[derive(Args, Debug, Clone)]
pub struct OfficeIdArgs {
    /// NWS office ID (three-letter identifier, e.g., PSR, WRH, NWS).
    #[arg(long, long_help = parse::office_long_help("NWS office ID"))]
    id: OfficeId,
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

impl Run for OfficeCommands {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        let offices = client.offices();
        match self {
            OfficeCommands::Metadata(args) => output.show(offices.get(&args.id)).await,
            OfficeCommands::Headlines(args) => output.show(offices.headlines(&args.id)).await,
            OfficeCommands::Headline {
                office_args,
                headline_id,
            } => {
                output
                    .show(offices.headline(&office_args.id, headline_id))
                    .await
            }
            OfficeCommands::Briefing(args) => output.show(offices.briefing(&args.id)).await,
            OfficeCommands::BriefingDownload {
                office_args,
                document_id,
            } => {
                output
                    .download(offices.briefing_document(&office_args.id, document_id))
                    .await
            }
            OfficeCommands::BriefingDownloadLatest(args) => {
                output
                    .download(offices.latest_briefing_document(&args.id))
                    .await
            }
            OfficeCommands::WeatherStories(args) => {
                output.show(offices.weather_stories(&args.id)).await
            }
            OfficeCommands::WeatherStoryImage {
                office_args,
                story_id,
            } => {
                output
                    .download(offices.weather_story_image(&office_args.id, story_id))
                    .await
            }
        }
    }
}
