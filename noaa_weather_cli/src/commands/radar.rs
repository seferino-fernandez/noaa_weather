use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::Client;
use noaa_weather_client::apis::radar as radar_api;
use noaa_weather_client::apis::radar::RadarDataQueueQueryParams;
use noaa_weather_client::models::RadarQueueHost;

use crate::output::Output;

const DEFAULT_RADAR_DATA_QUEUE_LIMIT: i32 = 10;

/// Subcommands for interacting with NWS radar data endpoints.
#[derive(Subcommand, Debug, Clone)]
#[command(
    about = "Access radar stations, servers, data queues, and wind profilers",
    long_about = "Provides access to various endpoints related to NOAA radar stations, servers, data queues, and wind profilers."
)]
pub enum RadarCommand {
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
    /// Get SPGDS host telemetry, optionally filtered by publication interval.
    Spgds(RadarSpgdsArgs),
    /// Get metadata for a specific radar wind profiler station.
    WindProfiler(RadarWindProfilerArgs),
}

/// Arguments for the `spgds` subcommand.
#[derive(Args, Debug, Clone)]
pub struct RadarSpgdsArgs {
    /// Publication interval accepted by the NWS API.
    #[arg(long)]
    published: Option<String>,
}

/// Arguments for the `profiler` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata for a radar wind profiler station.")]
pub struct RadarWindProfilerArgs {
    /// The ID of the radar wind profiler station (e.g., "HWPA2").
    #[arg(long, required = true)]
    id: String,

    /// Optional: Specify a time interval (ISO 8601 duration format, e.g., "PT1H").
    #[arg(long)]
    interval: Option<String>,

    /// Optional: Specify a time for the data (ISO 8601 format or relative time like "-1hour").
    #[arg(long)]
    time: Option<String>,
}

/// Arguments for the `data-queue` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata and entries for a radar data queue.")]
pub struct RadarDataQueueArgs {
    /// Optional: Filter by arrival time range (ISO 8601 interval, e.g., "start/end", "start/", "/end").
    #[arg(long)]
    arrived: Option<String>,

    /// The host name of the radar queue server (e.g., "rds").
    #[arg(long, required = true, value_enum)]
    host: RadarQueueHost,

    /// Optional: Limit the number of queue entries returned (1 through 50,000).
    /// A limit is required or the API will return an error.
    /// Default is 10.
    #[arg(long, value_parser = clap::value_parser!(i32).range(1..=50_000))]
    limit: Option<i32>,

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
    #[arg(long, required = true)]
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
    #[arg(long, required = true)]
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
    #[arg(long, required = true)]
    station_id: String,
}

/// Arguments for the `stations` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get a list of radar stations.")]
pub struct RadarStationsArgs {
    /// Optional: Filter by station type(s) (e.g., "WSR-88D", "TDWR"). Can be specified multiple times.
    #[arg(long)]
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
/// * `output` - The configured output policy.
/// * `client` - The NOAA API client.
///
pub async fn handle_command(
    command: &RadarCommand,
    output: &Output,
    client: &Client,
) -> Result<()> {
    match command {
        RadarCommand::WindProfiler(args) => {
            output
                .raw_json(
                    format!("getting radar wind-profiler data for {}", args.id),
                    radar_api::get_radar_wind_profiler(
                        client,
                        &args.id,
                        args.time.as_deref(),
                        args.interval.as_deref(),
                    ),
                )
                .await
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
            output
                .show(
                    format!("getting radar data queue for host {}", args.host),
                    radar_api::get_radar_data_queue(client, &args.host, params),
                )
                .await
        }
        RadarCommand::Server(args) => {
            output
                .show(
                    format!("getting radar server {}", args.id),
                    radar_api::get_radar_server(client, &args.id, args.reporting_host.as_deref()),
                )
                .await
        }
        RadarCommand::Servers(args) => {
            output
                .show(
                    "listing radar servers",
                    radar_api::get_radar_servers(client, args.reporting_host.as_deref()),
                )
                .await
        }
        RadarCommand::Station(args) => {
            output
                .show(
                    format!("getting radar station {}", args.station_id),
                    radar_api::get_radar_station(
                        client,
                        &args.station_id,
                        args.reporting_host.as_deref(),
                        args.host.as_ref(),
                    ),
                )
                .await
        }
        RadarCommand::StationAlarms(args) => {
            output
                .show(
                    format!("getting alarms for radar station {}", args.station_id),
                    radar_api::get_radar_station_alarms(client, &args.station_id),
                )
                .await
        }
        RadarCommand::Stations(args) => {
            output
                .show(
                    "listing radar stations",
                    radar_api::get_radar_stations(
                        client,
                        args.station_type.clone(),
                        args.reporting_host.as_deref(),
                        args.host.as_ref(),
                    ),
                )
                .await
        }
        RadarCommand::Spgds(args) => {
            output
                .show(
                    "getting radar SPGDS telemetry",
                    radar_api::get_radar_spgds(client, args.published.as_deref()),
                )
                .await
        }
    }
}
