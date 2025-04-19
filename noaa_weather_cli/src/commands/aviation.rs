use anyhow::{Result, anyhow};
use clap::{Args, Subcommand, value_parser};
use noaa_weather_client::apis::aviation as aviation_api;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::NwsCenterWeatherServiceUnitId;
use serde_json::Value;

#[derive(Args, Debug)]
pub struct CwaArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long, value_parser = value_parser!(NwsCenterWeatherServiceUnitId))]
    cwsu_id: NwsCenterWeatherServiceUnitId,

    /// Date in YYYY-MM-DD format.
    #[arg(long)]
    date: String,

    /// Advisory sequence number. Minimum value is 100.
    #[arg(long, value_parser = clap::value_parser!(i32).range(100..))]
    sequence: i32,
}

#[derive(Args, Debug)]
pub struct CwasArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long, value_parser = value_parser!(NwsCenterWeatherServiceUnitId))]
    cwsu_id: NwsCenterWeatherServiceUnitId,
}

#[derive(Args, Debug)]
pub struct CwsuArgs {
    /// Center Weather Service Unit (CWSU) ID (e.g., ZAB, ZDC).
    #[arg(long, value_parser = value_parser!(NwsCenterWeatherServiceUnitId))]
    cwsu_id: NwsCenterWeatherServiceUnitId,
}

#[derive(Args, Debug)]
pub struct SigmetArgs {
    /// Air Traffic Service Unit (ATSU) identifier.
    #[arg(long)]
    atsu: String,

    /// Date in YYYY-MM-DD format.
    #[arg(long)]
    date: String,

    /// Time in HHMM format (UTC).
    #[arg(long)]
    time: String,
}

#[derive(Args, Debug)]
pub struct SigmetsArgs {
    /// Start time for filtering (ISO 8601 format).
    #[arg(long)]
    start: Option<String>,

    /// End time for filtering (ISO 8601 format).
    #[arg(long)]
    end: Option<String>,

    /// Date for filtering (YYYY-MM-DD format).
    #[arg(long)]
    date: Option<String>,

    /// Air Traffic Service Unit (ATSU) identifier for filtering.
    #[arg(long)]
    atsu: Option<String>,

    /// Sequence number for filtering.
    #[arg(long)]
    sequence: Option<String>,
}

/// Access aviation weather products like CWAs and SIGMETs.
#[derive(Subcommand, Debug)]
pub enum AviationCommands {
    /// Get a specific Center Weather Advisory (CWA).
    Cwa(CwaArgs),
    /// Get all current CWAs for a Center Weather Service Unit (CWSU).
    Cwas(CwasArgs),
    /// Get metadata for a Center Weather Service Unit (CWSU).
    Cwsu(CwsuArgs),
    /// Get a specific SIGMET/AIRMET product identified by ATSU, date, and time.
    Sigmet(SigmetArgs),
    /// Query available SIGMET/AIRMET products with filters.
    /// Use flags like --atsu, --date, --start, --end, --sequence to narrow results.
    Sigmets(SigmetsArgs),
}

/// Handles the execution of aviation-related commands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided arguments.
///
/// # Arguments
///
/// * `command` - The specific aviation command to execute.
/// * `config` - The application configuration containing API details.
///
/// # Returns
///
/// A `Result` containing the JSON `Value` of the API response or an `Error`.
pub async fn handle_command(command: AviationCommands, config: &Configuration) -> Result<Value> {
    match command {
        AviationCommands::Cwa(args) => {
            let result = aviation_api::cwa(config, args.cwsu_id, args.date, args.sequence)
                .await
                .map_err(|e| anyhow!("getting CWA: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AviationCommands::Cwas(args) => {
            let result = aviation_api::cwas(config, args.cwsu_id)
                .await
                .map_err(|e| anyhow!("getting CWAs for CWSU: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AviationCommands::Cwsu(args) => {
            let result = aviation_api::cwsu(config, args.cwsu_id)
                .await
                .map_err(|e| anyhow!("getting CWSU metadata: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AviationCommands::Sigmet(args) => {
            let result = aviation_api::sigmet(config, &args.atsu, args.date, &args.time)
                .await
                .map_err(|e| anyhow!("getting SIGMET: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AviationCommands::Sigmets(args) => {
            let result = aviation_api::sigmets(
                config,
                args.start,
                args.end,
                args.date,
                args.atsu.as_deref(),
                args.sequence.as_deref(),
            )
            .await
            .map_err(|e| anyhow!("querying SIGMETs: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
