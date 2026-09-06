//! `noaa-weather-mcp`: stdio Model Context Protocol server for NOAA Weather.

#![forbid(unsafe_code)]

mod commands;
mod server;

use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::LazyLock;

use clap::{CommandFactory as _, Parser, Subcommand};
use clap_complete::CompleteEnv;
use rmcp::ServiceExt as _;

use commands::completions::CompletionShell;
use server::NoaaWeatherServer;

const DEFAULT_MAX_RESPONSE_BYTES: &str = "10485760";

static COMMAND_VERSION: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{} (api.weather.gov spec {})",
        env!("CARGO_PKG_VERSION"),
        noaa_weather_client::API_SPEC_VERSION
    )
});

fn command_version() -> &'static str {
    COMMAND_VERSION.as_str()
}

#[derive(Debug, Parser)]
#[command(
    name = "noaa-weather-mcp",
    author,
    version = command_version(),
    about = "Serve the NOAA weather.gov API over MCP using stdio"
)]
struct Cli {
    /// Maximum size in bytes of one structured JSON or raw binary tool result.
    #[arg(
        long,
        global = true,
        env = "NOAA_WEATHER_MCP_MAX_RESPONSE_BYTES",
        default_value = DEFAULT_MAX_RESPONSE_BYTES,
        value_name = "BYTES",
        allow_hyphen_values = true
    )]
    max_response_bytes: String,

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

impl Cli {
    fn parsed_max_response_bytes(&self) -> Result<NonZeroUsize, clap::Error> {
        self.max_response_bytes.parse().map_err(|error| {
            Cli::command().error(
                clap::error::ErrorKind::ValueValidation,
                format!(
                    "invalid value {:?} for '--max-response-bytes <BYTES>': {error}",
                    self.max_response_bytes
                ),
            )
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    CompleteEnv::with_factory(Cli::command).complete();

    let cli = Cli::parse();
    let max_response_bytes = cli
        .parsed_max_response_bytes()
        .unwrap_or_else(|error| error.exit());
    match cli.command {
        Some(Command::Completions { shell }) => {
            commands::completions::run(shell, Cli::command());
        }
        Some(Command::Man { out_dir }) => {
            commands::man::run(Cli::command(), &out_dir)?;
        }
        None => {
            let service = NoaaWeatherServer::new(max_response_bytes)?
                .serve(rmcp::transport::io::stdio())
                .await?;
            service.waiting().await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_limit_defaults_to_ten_mebibytes() {
        let cli = Cli::try_parse_from(["noaa-weather-mcp", "completions", "bash"])
            .expect("default CLI must parse");
        assert_eq!(
            cli.parsed_max_response_bytes()
                .expect("default response limit must be valid")
                .get(),
            10_485_760
        );
    }

    #[test]
    fn explicit_response_limit_overrides_the_default() {
        let cli = Cli::try_parse_from([
            "noaa-weather-mcp",
            "--max-response-bytes",
            "2048",
            "completions",
            "bash",
        ])
        .expect("nonzero response limit must parse");
        assert_eq!(
            cli.parsed_max_response_bytes()
                .expect("explicit response limit must be valid")
                .get(),
            2_048
        );
    }

    #[test]
    fn zero_response_limit_is_a_usage_error() {
        let cli = Cli::try_parse_from([
            "noaa-weather-mcp",
            "--max-response-bytes",
            "0",
            "completions",
            "bash",
        ])
        .expect("Clap must select the raw CLI value");
        let error = cli
            .parsed_max_response_bytes()
            .expect_err("zero must not mean unlimited");
        assert_eq!(error.exit_code(), 2);
    }
}
