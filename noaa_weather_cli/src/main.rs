use anyhow::Result;
use clap::Parser;
use noaa_weather_client::apis::configuration::Configuration;
use std::fs::File;
use std::io::Write;

mod commands;
mod utils;
use commands::{Commands, alerts, gridpoints, offices, points, radar, stations, zones};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Fetches weather forecasts and alerts from the NOAA API."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    /// Controls logging level.
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,

    /// Output file path
    #[arg(short, long, global = true)]
    output: Option<String>,
}

/// Write output to either stdout or a file
fn write_output(output_path: Option<&str>, content: &str) -> Result<()> {
    if let Some(path) = output_path {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
    } else {
        println!("{}", content);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Configuration::new();

    let result = match cli.command {
        Commands::Alerts { command } => alerts::handle_command(*command, &config).await?,
        Commands::Gridpoints { command } => gridpoints::handle_command(*command, &config).await?,
        Commands::Offices { command } => offices::handle_command(*command, &config).await?,
        Commands::Points { command } => points::handle_command(*command, &config).await?,
        Commands::Radar { command } => radar::handle_command(*command, &config).await?,
        Commands::Stations { command } => stations::handle_command(*command, &config).await?,
        Commands::Zones { command } => zones::handle_command(*command, &config).await?,
    };

    let output = if cli.json {
        serde_json::to_string_pretty(&result)?
    } else {
        format!("{:#?}", result)
    };

    write_output(cli.output.as_deref(), &output)?;

    Ok(())
}
