use crate::utils::format::write_output;
use crate::{Cli, tables};
use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::aviation as aviation_api;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::NwsCenterWeatherServiceUnitId;

/// Arguments for fetching a specific Center Weather Advisory (CWA).
#[derive(Args, Debug, Clone)]
pub struct CwaArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long, value_enum)]
    cwsu_id: NwsCenterWeatherServiceUnitId,

    /// Date of the advisory in YYYY-MM-DD format.
    #[arg(long)]
    date: String,

    /// Sequence number of the advisory (must be >= 100).
    #[arg(long, value_parser = clap::value_parser!(i32).range(100..))]
    sequence: i32,
}

/// Arguments for fetching all current CWAs for a CWSU.
#[derive(Args, Debug, Clone)]
pub struct CwasArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long, value_enum)]
    cwsu_id: NwsCenterWeatherServiceUnitId,
}

/// Arguments for fetching metadata about a CWSU.
#[derive(Args, Debug, Clone)]
pub struct CwsuArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long, value_enum)]
    cwsu_id: NwsCenterWeatherServiceUnitId,
}

/// Arguments for fetching a specific SIGMET/AIRMET.
#[derive(Args, Debug, Clone)]
pub struct SigmetArgs {
    /// Air Traffic Service Unit (ATSU) identifier (e.g., KKCI, ANC).
    #[arg(long)]
    atsu: String,

    /// Date of issuance in YYYY-MM-DD format.
    #[arg(long)]
    date: String,

    /// Time of issuance in HHMM format (UTC).
    #[arg(long)]
    time: String,
}

/// Arguments for querying available SIGMET/AIRMET products with filters.
#[derive(Args, Debug, Clone)]
pub struct SigmetsArgs {
    /// Start time for filtering (ISO 8601 format, e.g., "2023-10-27T12:00:00Z").
    #[arg(long)]
    start: Option<String>,

    /// End time for filtering (ISO 8601 format).
    #[arg(long)]
    end: Option<String>,

    /// Date for filtering (YYYY-MM-DD format).
    #[arg(long)]
    date: Option<String>,

    /// Air Traffic Service Unit (ATSU) identifier (e.g., KKCI, ANC).
    #[arg(long)]
    atsu: Option<String>,

    /// Sequence number for filtering.
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
    /// Get a specific SIGMET/AIRMET product identified by Air Traffic Service Unit (ATSU), date, and time.
    ///
    /// Example: `noaa-weather aviation sigmet --atsu "KKCI" --date 2025-04-18 --time 1430`
    Sigmet(SigmetArgs),
    /// Query available SIGMET/AIRMET products with optional filters.
    ///
    /// Use flags like --atsu, --date, --start, --end, --sequence to narrow results.
    /// Example: `noaa-weather aviation sigmets --atsu "KKCI" --date 2025-04-18`
    Sigmets(SigmetsArgs),
}

/// Handles the execution of aviation-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `AviationCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific aviation subcommand and its arguments to execute.
/// * `cli` - The CLI arguments.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &AviationCommands,
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        AviationCommands::Cwa(args) => {
            let result = aviation_api::get_center_weather_advisories_by_date_and_sequence(
                config,
                args.cwsu_id,
                args.date.clone(),
                args.sequence,
            )
            .await
            .map_err(|e| anyhow!("Error getting CWA: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::aviation::create_cwa_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AviationCommands::Cwas(args) => {
            let result = aviation_api::get_center_weather_advisories(config, args.cwsu_id)
                .await
                .map_err(|e| anyhow!("Error getting CWAs for CWSU: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::aviation::create_cwas_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AviationCommands::Cwsu(args) => {
            let result = aviation_api::get_center_weather_service_unit(config, args.cwsu_id)
                .await
                .map_err(|e| anyhow!("Error getting CWSU metadata: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::aviation::create_cwsu_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AviationCommands::Sigmet(args) => {
            let _result =
                aviation_api::get_sigmet(config, &args.atsu, args.date.clone(), &args.time)
                    .await
                    .map_err(|e| anyhow!("Error getting SIGMET: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&_result)?,
                )?;
            } else {
                let table = tables::aviation::create_sigmet_table(&_result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AviationCommands::Sigmets(args) => {
            let _result = aviation_api::get_sigmets(
                config,
                args.start.clone(),
                args.end.clone(),
                args.date.clone(),
                args.atsu.as_deref(),
                args.sequence.as_deref(),
            )
            .await
            .map_err(|e| anyhow!("Error querying SIGMETs: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&_result)?,
                )?;
            } else {
                let table = tables::aviation::create_sigmets_table(&_result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
    }
}
