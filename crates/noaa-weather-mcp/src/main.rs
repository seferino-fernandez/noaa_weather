//! `noaa-weather-mcp`: stdio Model Context Protocol server for NOAA Weather.

#![forbid(unsafe_code)]

mod commands;
mod server;

use std::path::PathBuf;

use clap::{CommandFactory as _, Parser, Subcommand};
use clap_complete::CompleteEnv;
use rmcp::ServiceExt as _;

use commands::completions::CompletionShell;
use server::NoaaWeatherServer;

#[derive(Debug, Parser)]
#[command(
    name = "noaa-weather-mcp",
    author,
    version,
    about = "Serve the NOAA weather.gov API over MCP using stdio"
)]
struct Cli {
    /// Generate an artifact instead of starting the stdio server.
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate a static shell completion script.
    Completions {
        /// Shell to generate the completion script for.
        shell: CompletionShell,
    },
    /// Generate man pages for the server and every subcommand.
    Man {
        /// Directory to write the generated man pages into.
        out_dir: PathBuf,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    CompleteEnv::with_factory(Cli::command).complete();

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Completions { shell }) => {
            commands::completions::run(shell, Cli::command());
        }
        Some(Command::Man { out_dir }) => {
            commands::man::run(Cli::command(), &out_dir)?;
        }
        None => {
            let service = NoaaWeatherServer::new()?
                .serve(rmcp::transport::io::stdio())
                .await?;
            service.waiting().await?;
        }
    }

    Ok(())
}
