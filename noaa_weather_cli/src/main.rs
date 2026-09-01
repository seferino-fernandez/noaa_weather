use anyhow::Result;
use clap::Parser;
use noaa_weather_client::apis::configuration::Configuration;

use output::{Output, OutputArgs};

mod commands;
mod output;

#[cfg(feature = "radio")]
use commands::radio;
use commands::{
    Commands, alerts, aviation, glossary, gridpoints, offices, points, products, radar, stations,
    zones,
};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Fetches weather forecasts and alerts from the NOAA Weather API."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    output: OutputArgs,
}

#[tokio::main]
async fn main() {
    if let Err(error) = try_main().await {
        eprintln!("noaa-weather: {error:#}");
        std::process::exit(1);
    }
}

async fn try_main() -> Result<()> {
    let Cli { command, output } = Cli::parse();
    let output = Output::configured(output);
    let config = Configuration::default();

    match &command {
        Commands::Alerts { command } => {
            alerts::handle_command(command, &output, &config).await?;
        }
        Commands::Gridpoints { command } => {
            gridpoints::handle_command(command, &output, &config).await?;
        }
        Commands::Glossary => glossary::handle_command(&output, &config).await?,
        Commands::Offices { command } => {
            offices::handle_command(command, &output, &config).await?;
        }
        Commands::Points { command } => {
            points::handle_command(command, &output, &config).await?;
        }
        Commands::Radar { command } => radar::handle_command(command, &output, &config).await?,
        Commands::Stations { command } => {
            stations::handle_command(command, &output, &config).await?;
        }
        Commands::Zones { command } => zones::handle_command(command, &output, &config).await?,
        Commands::Aviation { command } => {
            aviation::handle_command(command, &output, &config).await?;
        }
        Commands::Products { command } => {
            products::handle_command(command, &output, &config).await?;
        }
        #[cfg(feature = "radio")]
        Commands::Radio { command } => {
            radio::handle_command(command, &output, &config).await?;
        }
    }

    Ok(())
}
