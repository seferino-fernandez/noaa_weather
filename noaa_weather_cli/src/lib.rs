//! The `noaa-weather` command line program, as a library.
//!
//! The binary in `src/main.rs` is a thin wrapper around [`run`]. The parts
//! live here so integration tests can import [`Cli`] and walk the clap
//! command tree, which a `bin`-only crate cannot offer them.

use anyhow::Result;
use clap::{CommandFactory as _, FromArgMatches as _, Parser};

use client_args::ClientArgs;
use commands::{Commands, Run as _};
use output::{Operation, Output, OutputArgs};

mod client_args;
mod commands;
mod exit;
mod output;

pub use client_args::{ClientBuildError, Fault};
pub use exit::ExitCode;
pub use output::{OutputFailure, UsageFailure};

/// The whole command line: one subcommand plus the global argument groups.
#[derive(Parser, Debug)]
#[command(
    name = "noaa-weather",
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

/// Parses the command line, runs the chosen command, and reports a failure.
///
/// Returns the code the process should exit with. Argument parsing failures
/// never reach here: clap writes its own message and exits 2 from inside
/// clap's parser, which is why no JSON error line exists for them.
pub async fn run() -> ExitCode {
    let command = Cli::command();
    let root = command.get_name().to_owned();
    let matches = command.get_matches();
    let operation = Operation::from_matches(&root, &matches);
    let Cli {
        command,
        output,
        client,
    } = Cli::from_arg_matches(&matches).unwrap_or_else(|error| error.exit());
    let output = Output::configured(output, operation);

    let Err(error) = execute(&command, &output, &client).await else {
        return ExitCode::Ok;
    };

    let code = exit::classify(&error);
    match output
        .is_machine_readable()
        .then(|| exit::error_line(&error, code))
        .flatten()
    {
        // Only the JSON line, so that everything on standard error parses.
        Some(line) => eprintln!("{line}"),
        None => eprintln!("noaa-weather: {error:#}"),
    }
    code
}

/// Builds a client and runs one command against it.
///
/// # Errors
///
/// Returns the command's error, or the client build failure, unchanged.
async fn execute(command: &Commands, output: &Output, client: &ClientArgs) -> Result<()> {
    let client = client.build()?;
    command.run(&client, output).await
}
