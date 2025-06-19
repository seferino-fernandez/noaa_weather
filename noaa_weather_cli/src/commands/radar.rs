use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::radar as radar_api;
use noaa_weather_client::apis::radar::RadarDataQueueQueryParams;
use noaa_weather_client::models::RadarQueueHost;

use crate::utils::format::write_output;
use crate::{Cli, tables};

const DEFAULT_RADAR_DATA_QUEUE_LIMIT: i32 = 10;

/// Subcommands for interacting with NWS radar data endpoints.
#[derive(Subcommand, Debug, Clone)]
#[command(
    about = "Access radar stations, servers, data queues, and wind profilers",
    long_about = "Provides access to various endpoints related to NOAA radar stations, servers, data queues, and wind profilers."
)]
pub enum RadarCommand {
    /// Get metadata for a specific radar wind profiler station.
    WindProfiler(RadarWindProfilerArgs),
    /// Get metadata and recent entries for a radar data queue on a specific host.
    DataQueue(RadarDataQueueArgs),
    /// Get metadata for a specific radar server by its ID.
    Server(RadarServerArgs),
    /// Get a list of radar servers, optionally filtered by reporting host.
    Servers(RadarServersArgs),
    /// Get metadata for a specific radar station by its ID.
    Station(RadarStationArgs),
    /// Get alarm metadata for a specific radar station.
    StationAlarms(RadarStationAlarmsArgs),
    /// Get a list of radar stations, optionally filtered by type or host.
    Stations(RadarStationsArgs),
}

/// Arguments for the `profiler` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata for a radar wind profiler station.")]
pub struct RadarWindProfilerArgs {
    /// The ID of the radar wind profiler station (e.g., "HWPA2").
    #[arg(required = true)]
    id: String,

    /// Optional: Specify a time for the data (ISO 8601 format or relative time like "-1hour").
    #[arg(long)]
    time: Option<String>,

    /// Optional: Specify a time interval (ISO 8601 duration format, e.g., "PT1H").
    #[arg(long)]
    interval: Option<String>,
}

/// Arguments for the `data-queue` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata and entries for a radar data queue.")]
pub struct RadarDataQueueArgs {
    /// The host name of the radar queue server (e.g., "rds").
    #[arg(required = true, value_enum)]
    host: RadarQueueHost,

    /// Optional: Limit the number of queue entries returned.
    /// A limit is required or the API will return an error.
    /// Default is 10.
    #[arg(long)]
    limit: Option<i32>,

    /// Optional: Filter by arrival time range (ISO 8601 interval, e.g., "start/end", "start/", "/end").
    #[arg(long)]
    arrived: Option<String>,

    /// Optional: Filter by creation time range (ISO 8601 interval).
    #[arg(long)]
    created: Option<String>,

    /// Optional: Filter by publication time range (ISO 8601 interval).
    #[arg(long)]
    published: Option<String>,

    /// Optional: Filter by radar station ID (e.g., "KIWA").
    #[arg(long)]
    station: Option<String>,

    /// Optional: Filter by data type (e.g., "LEVEL2").
    #[arg(long)]
    r#type: Option<String>,

    /// Optional: Filter by feed type.
    #[arg(long)]
    feed: Option<String>,

    /// Optional: Filter by data resolution.
    #[arg(long)]
    resolution: Option<i32>,
}

/// Arguments for the `server` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata for a specific radar server.")]
pub struct RadarServerArgs {
    /// The ID of the radar server (e.g., "ldm1").
    #[arg(required = true)]
    id: String,

    /// Optional: Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,
}

/// Arguments for the `servers` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get a list of radar servers.")]
pub struct RadarServersArgs {
    /// Optional: Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,
}

/// Arguments for the `station` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata for a specific radar station.")]
pub struct RadarStationArgs {
    /// The ID of the radar station (e.g., "KABQ", "HWPA2").
    #[arg(required = true)]
    station_id: String,

    /// Optional: Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,

    /// Optional: Filter by host server.
    #[arg(long, value_enum)]
    host: Option<RadarQueueHost>,
}

/// Arguments for the `station-alarms` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get alarm metadata for a specific radar station.")]
pub struct RadarStationAlarmsArgs {
    /// The ID of the radar station (e.g., "KABQ").
    #[arg(required = true)]
    station_id: String,
}

/// Arguments for the `stations` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get a list of radar stations.")]
pub struct RadarStationsArgs {
    /// Optional: Filter by station type(s) (e.g., "WSR-88D", "TDWR"). Can be specified multiple times.
    #[arg(long = "stationType")]
    station_type: Option<Vec<String>>,

    /// Optional: Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,

    /// Optional: Filter by host server.
    #[arg(long)]
    host: Option<RadarQueueHost>,
}

/// Handles the execution of radar-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `RadarCommand` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific radar subcommand and its arguments to execute.
/// * `cli` - The CLI arguments.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &RadarCommand,
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        RadarCommand::WindProfiler(args) => {
            let result = radar_api::get_radar_wind_profiler(
                config,
                &args.id,
                args.time.as_deref(),
                args.interval.as_deref(),
            )
            .await?;
            write_output(
                cli.output.as_deref(),
                &serde_json::to_string_pretty(&result)?,
            )?;
            Ok(())
        }
        RadarCommand::DataQueue(args) => {
            let limit = args.limit.unwrap_or(DEFAULT_RADAR_DATA_QUEUE_LIMIT);
            let params = RadarDataQueueQueryParams {
                limit: Some(limit),
                arrived: args.arrived.as_deref(),
                created: args.created.as_deref(),
                published: args.published.as_deref(),
                station: args.station.as_deref(),
                r#type: args.r#type.as_deref(),
                feed: args.feed.as_deref(),
                resolution: args.resolution,
            };
            let result = radar_api::get_radar_data_queue(config, &args.host, params).await?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::radar::create_radar_data_queue_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        RadarCommand::Server(args) => {
            let result =
                radar_api::get_radar_server(config, &args.id, args.reporting_host.as_deref())
                    .await?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::radar::create_radar_server_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        RadarCommand::Servers(args) => {
            let result =
                radar_api::get_radar_servers(config, args.reporting_host.as_deref()).await?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::radar::create_radar_servers_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        RadarCommand::Station(args) => {
            let result = radar_api::get_radar_station(
                config,
                &args.station_id,
                args.reporting_host.as_deref(),
                args.host.as_ref(),
            )
            .await?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::radar::create_radar_station_feature_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        RadarCommand::StationAlarms(args) => {
            let result = radar_api::get_radar_station_alarms(config, &args.station_id).await?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::radar::create_radar_station_alarms_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        RadarCommand::Stations(args) => {
            let result = radar_api::get_radar_stations(
                config,
                args.station_type.clone(),
                args.reporting_host.as_deref(),
                args.host.as_ref(),
            )
            .await?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::radar::create_radar_stations_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
    }
}
