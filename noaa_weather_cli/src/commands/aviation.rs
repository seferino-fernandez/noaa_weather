use anyhow::Result;
use clap::{Args, Subcommand};
use jiff::Timestamp;
use jiff::civil::Date;
use noaa_weather_client::apis::aviation::SigmetsQuery;
use noaa_weather_client::{AtsuId, Client, CwsuId};

use super::parse;
use crate::output::Output;

/// Arguments for fetching a specific Center Weather Advisory (CWA).
#[derive(Args, Debug, Clone)]
pub struct CwaArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long)]
    cwsu_id: CwsuId,

    /// Date of the advisory in YYYY-MM-DD format.
    #[arg(long, value_name = "YYYY-MM-DD")]
    date: Date,

    /// Sequence number of the advisory (must be >= 100).
    #[arg(long, value_parser = clap::value_parser!(u32).range(100..))]
    sequence: u32,
}

/// Arguments for fetching all current CWAs for a CWSU.
#[derive(Args, Debug, Clone)]
pub struct CwasArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long)]
    cwsu_id: CwsuId,
}

/// Arguments for fetching metadata about a CWSU.
#[derive(Args, Debug, Clone)]
pub struct CwsuArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long)]
    cwsu_id: CwsuId,
}

/// Arguments for fetching a specific SIGMET/AIRMET.
#[derive(Args, Debug, Clone)]
pub struct SigmetArgs {
    /// Air Traffic Service Unit (ATSU) identifier (e.g., KKCI).
    #[arg(long)]
    atsu: AtsuId,

    /// Issue time of the product (RFC 3339 timestamp or relative age such as 2h).
    /// NOAA addresses the product by its UTC date and HHMM minute, so seconds are dropped.
    #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
    issued: Timestamp,
}

/// Arguments for querying available SIGMET/AIRMET products with filters.
#[derive(Args, Debug, Clone)]
pub struct SigmetsArgs {
    /// Start time for filtering (RFC 3339 timestamp or relative age such as 6h).
    #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
    start: Option<Timestamp>,

    /// End time for filtering (RFC 3339 timestamp or relative age such as 1h).
    #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
    end: Option<Timestamp>,

    /// Date for filtering (YYYY-MM-DD format).
    #[arg(long, value_name = "YYYY-MM-DD")]
    date: Option<Date>,

    /// Air Traffic Service Unit (ATSU) identifier (e.g., KKCI).
    #[arg(long)]
    atsu: Option<AtsuId>,

    /// Sequence number for filtering (e.g., 52C).
    #[arg(long)]
    sequence: Option<String>,
}

/// Access aviation weather products like CWAs and SIGMETs.
#[derive(Subcommand, Debug, Clone)]
pub enum AviationCommands {
    /// Get a specific Center Weather Advisory (CWA) by CWSU ID, date, and sequence number.
    ///
    /// Example: `noaa-weather aviation cwa --cwsu-id ZJX --date 2025-06-12 --sequence 101`
    Cwa(CwaArgs),
    /// Get all current Center Weather Advisories (CWAs) for a Center Weather Service Unit (CWSU).
    ///
    /// Example: `noaa-weather aviation cwas --cwsu-id ZJX`
    Cwas(CwasArgs),
    /// Get metadata for a Center Weather Service Unit (CWSU).
    ///
    /// Example: `noaa-weather aviation cwsu --cwsu-id ZJX`
    Cwsu(CwsuArgs),
    /// Get a specific SIGMET/AIRMET product identified by Air Traffic Service Unit (ATSU) and issue time.
    ///
    /// Example: `noaa-weather aviation sigmet --atsu KKCI --issued 2025-04-18T14:30:00Z`
    Sigmet(SigmetArgs),
    /// Query available SIGMET/AIRMET products with optional filters.
    ///
    /// Use flags like --atsu, --date, --start, --end, --sequence to narrow results.
    /// Example: `noaa-weather aviation sigmets --atsu KKCI --date 2025-04-18`
    Sigmets(SigmetsArgs),
}

/// Handles the execution of aviation-related subcommands.
///
/// Dispatches the command to the matching `client.aviation()` method based
/// on the provided `AviationCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific aviation subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `client` - The NOAA API client.
///
pub async fn handle_command(
    command: &AviationCommands,
    output: &Output,
    client: &Client,
) -> Result<()> {
    let aviation = client.aviation();
    match command {
        AviationCommands::Cwa(args) => {
            output
                .show(
                    format!(
                        "getting CWA {} for {} on {}",
                        args.sequence, args.cwsu_id, args.date
                    ),
                    aviation.cwa(&args.cwsu_id, args.date, args.sequence),
                )
                .await
        }
        AviationCommands::Cwas(args) => {
            output
                .show(
                    format!("getting CWAs for CWSU {}", args.cwsu_id),
                    aviation.cwas(&args.cwsu_id),
                )
                .await
        }
        AviationCommands::Cwsu(args) => {
            output
                .show(
                    format!("getting CWSU {} metadata", args.cwsu_id),
                    aviation.cwsu(&args.cwsu_id),
                )
                .await
        }
        AviationCommands::Sigmet(args) => {
            output
                .show(
                    format!(
                        "getting SIGMET from {} issued at {}",
                        args.atsu, args.issued
                    ),
                    aviation.sigmet(&args.atsu, args.issued),
                )
                .await
        }
        AviationCommands::Sigmets(args) => {
            let query = SigmetsQuery {
                start: args.start,
                end: args.end,
                date: args.date,
                atsu: args.atsu.clone(),
                sequence: args.sequence.clone(),
            };
            output
                .show("querying SIGMETs", aviation.sigmets(&query))
                .await
        }
    }
}
