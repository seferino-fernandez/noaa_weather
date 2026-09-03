//! The `noaa-weather` command line program, as a library.
//!
//! The binary in `src/main.rs` is a thin wrapper around [`try_main`]. The
//! parts live here so integration tests can import [`Cli`] and walk the clap
//! command tree, which a `bin`-only crate cannot offer them.

use anyhow::Result;
use clap::Parser;

use client_args::ClientArgs;
use output::{Output, OutputArgs};

mod client_args;
mod commands;
mod output;

pub use client_args::{ClientBuildError, Fault};

use commands::radio;
use commands::{
    Commands, alerts, aviation, glossary, gridpoints, offices, points, products, radar, stations,
    zones,
};

/// The whole command line: one subcommand plus the global argument groups.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Fetches weather forecasts and alerts from the NOAA Weather API.",
    after_long_help = client_args::ENVIRONMENT_HELP
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    output: OutputArgs,

    #[command(flatten)]
    client: ClientArgs,
}

/// Parses the command line, builds a client, and runs the chosen command.
///
/// # Errors
///
/// Returns the command's error, or the client build failure, unchanged.
/// Argument parsing failures exit the process from inside clap.
pub async fn try_main() -> Result<()> {
    let Cli {
        command,
        output,
        client,
    } = Cli::parse();
    let output = Output::configured(output);
    let client = client.build()?;

    match &command {
        Commands::Alerts { command } => {
            alerts::handle_command(command, &output, &client).await?;
        }
        Commands::Gridpoints { command } => {
            gridpoints::handle_command(command, &output, &client).await?;
        }
        Commands::Glossary => glossary::handle_command(&output, &client).await?,
        Commands::Offices { command } => {
            offices::handle_command(command, &output, &client).await?;
        }
        Commands::Points { command } => {
            points::handle_command(command, &output, &client).await?;
        }
        Commands::Radar { command } => radar::handle_command(command, &output, &client).await?,
        Commands::Stations { command } => {
            stations::handle_command(command, &output, &client).await?;
        }
        Commands::Zones { command } => zones::handle_command(command, &output, &client).await?,
        Commands::Aviation { command } => {
            aviation::handle_command(command, &output, &client).await?;
        }
        Commands::Products { command } => {
            products::handle_command(command, &output, &client).await?;
        }
        Commands::Radio { command } => {
            radio::handle_command(command, &output, &client).await?;
        }
    }

    Ok(())
}
