use anyhow::Result;
use clap::Parser;
use noaa_weather_client::apis::configuration::Configuration;

mod commands;
mod tables;
mod utils;

use commands::{
    Commands, alerts, aviation, gridpoints, offices, points, products, radar, stations, zones,
};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "Fetches weather forecasts and alerts from the NOAA Weather API."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Output file path
    #[arg(short, long, global = true)]
    output: Option<String>,
}

#[tokio::main]
async fn main() {
    if let Err(e) = try_main().await {
        eprintln!("noaa-weather: {e}");
        std::process::exit(1);
    }
}

async fn try_main() -> Result<()> {
    let cli = Cli::parse();

    let config = Configuration::new();

    match &cli.command {
        Commands::Alerts { command } => {
            alerts::handle_command(command, cli.clone(), &config).await?
        }
        Commands::Gridpoints { command } => {
            gridpoints::handle_command(command, cli.clone(), &config).await?
        }
        Commands::Offices { command } => {
            offices::handle_command(command, cli.clone(), &config).await?
        }
        Commands::Points { command } => {
            points::handle_command(command, cli.clone(), &config).await?
        }
        Commands::Radar { command } => radar::handle_command(command, cli.clone(), &config).await?,
        Commands::Stations { command } => {
            stations::handle_command(command, cli.clone(), &config).await?
        }
        Commands::Zones { command } => zones::handle_command(command, cli.clone(), &config).await?,
        Commands::Aviation { command } => {
            aviation::handle_command(command, cli.clone(), &config).await?
        }
        Commands::Products { command } => {
            products::handle_command(command, cli.clone(), &config).await?
        }
    };

    Ok(())
}
