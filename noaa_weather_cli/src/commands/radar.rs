use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::radar::{
    RadarQueueQuery, RadarServerQuery, RadarServersQuery, RadarStationQuery, RadarStationsQuery,
    SpgdsQuery, WindProfilerQuery,
};
use noaa_weather_client::models::RadarQueueHost;
use noaa_weather_client::{Client, Interval, RadarStationId};

use super::Run;
use crate::output::Output;

const DEFAULT_RADAR_DATA_QUEUE_LIMIT: u16 = 10;

const INTERVAL_HELP: &str = "An ISO 8601 time interval in any of its four forms: start/end, \
    start/duration, duration/end, or a bare duration, with RFC 3339 timestamps and ISO 8601 \
    durations (for example 2026-08-30T00:00:00Z/PT1H or PT1H).";

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
    /// Publication interval (ISO 8601 interval, e.g., "2026-08-30T00:00:00Z/PT1H").
    #[arg(long, value_name = "INTERVAL", long_help = INTERVAL_HELP)]
    published: Option<Interval>,
}

/// Arguments for the `profiler` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata for a radar wind profiler station.")]
pub struct RadarWindProfilerArgs {
    /// The ID of the radar wind profiler station (e.g., "HWPA2").
    #[arg(long, required = true)]
    id: String,

    /// Optional: Sampling interval of the data (ISO 8601 interval, e.g., "PT1H").
    #[arg(long, value_name = "INTERVAL", long_help = INTERVAL_HELP)]
    interval: Option<Interval>,

    /// Optional: Time range of the data (ISO 8601 interval, e.g., "2026-08-30T00:00:00Z/PT1H").
    #[arg(long, value_name = "INTERVAL", long_help = INTERVAL_HELP)]
    time: Option<Interval>,
}

/// Arguments for the `data-queue` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get metadata and entries for a radar data queue.")]
pub struct RadarDataQueueArgs {
    /// Optional: Filter by arrival time range (ISO 8601 interval, e.g., "2026-08-30T00:00:00Z/PT1H").
    #[arg(long, value_name = "INTERVAL", long_help = INTERVAL_HELP)]
    arrived: Option<Interval>,

    /// The host name of the radar queue server (rds or tds).
    #[arg(long, required = true)]
    host: RadarQueueHost,

    /// Optional: Limit the number of queue entries returned (1 through 50,000).
    /// A limit is required or the API will return an error.
    /// Default is 10.
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..=50_000))]
    limit: Option<u16>,

    /// Optional: Filter by creation time range (ISO 8601 interval).
    #[arg(long, value_name = "INTERVAL", long_help = INTERVAL_HELP)]
    created: Option<Interval>,

    /// Optional: Filter by publication time range (ISO 8601 interval).
    #[arg(long, value_name = "INTERVAL", long_help = INTERVAL_HELP)]
    published: Option<Interval>,

    /// Optional: Filter by radar station ID (e.g., "KIWA").
    #[arg(long)]
    station: Option<RadarStationId>,

    /// Optional: Filter by data type (e.g., "LEVEL2").
    #[arg(long)]
    r#type: Option<String>,

    /// Optional: Filter by feed type.
    #[arg(long)]
    feed: Option<String>,

    /// Optional: Filter by data resolution.
    #[arg(long)]
    resolution: Option<u32>,
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
    /// The ID of the radar station (four or five letters or digits, e.g., "KFSX" or the profiler "HWPA2").
    #[arg(long, required = true)]
    station_id: RadarStationId,

    /// Optional: Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,

    /// Optional: Filter by host server (rds or tds).
    #[arg(long)]
    host: Option<RadarQueueHost>,
}

/// Arguments for the `station-alarms` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get alarm metadata for a specific radar station.")]
pub struct RadarStationAlarmsArgs {
    /// The ID of the radar station (e.g., "KABQ").
    #[arg(long, required = true)]
    station_id: RadarStationId,
}

/// Arguments for the `stations` subcommand.
#[derive(Args, Debug, Clone)]
#[command(about = "Get a list of radar stations.")]
pub struct RadarStationsArgs {
    /// Optional: Filter by station type(s) (e.g., "WSR-88D", "TDWR"). Can be specified multiple times.
    #[arg(long)]
    station_type: Vec<String>,

    /// Optional: Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,

    /// Optional: Filter by host server (rds or tds).
    #[arg(long)]
    host: Option<RadarQueueHost>,
}

impl Run for RadarCommand {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        let radar = client.radar();
        match self {
            RadarCommand::WindProfiler(args) => {
                let query = WindProfilerQuery {
                    time: args.time,
                    interval: args.interval,
                };
                output.raw_json(radar.wind_profiler(&args.id, &query)).await
            }
            RadarCommand::DataQueue(args) => {
                let query = RadarQueueQuery {
                    limit: Some(args.limit.unwrap_or(DEFAULT_RADAR_DATA_QUEUE_LIMIT)),
                    arrived: args.arrived,
                    created: args.created,
                    published: args.published,
                    station: args.station.clone(),
                    data_type: args.r#type.clone(),
                    feed: args.feed.clone(),
                    resolution: args.resolution,
                };
                output.show(radar.queue(&args.host, &query)).await
            }
            RadarCommand::Server(args) => {
                let query = RadarServerQuery {
                    reporting_host: args.reporting_host.clone(),
                };
                output.show(radar.server(&args.id, &query)).await
            }
            RadarCommand::Servers(args) => {
                let query = RadarServersQuery {
                    reporting_host: args.reporting_host.clone(),
                };
                output.show(radar.servers(&query)).await
            }
            RadarCommand::Station(args) => {
                let query = RadarStationQuery {
                    reporting_host: args.reporting_host.clone(),
                    host: args.host.clone(),
                };
                output.show(radar.station(&args.station_id, &query)).await
            }
            RadarCommand::StationAlarms(args) => {
                output.show(radar.station_alarms(&args.station_id)).await
            }
            RadarCommand::Stations(args) => {
                let query = RadarStationsQuery {
                    station_type: args.station_type.clone(),
                    reporting_host: args.reporting_host.clone(),
                    host: args.host.clone(),
                };
                output.show(radar.stations(&query)).await
            }
            RadarCommand::Spgds(args) => {
                let query = SpgdsQuery {
                    published: args.published,
                };
                output.show(radar.spgds(&query)).await
            }
        }
    }
}
