use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::radar as radar_api;
use serde_json::Value;

#[derive(Subcommand, Debug)]
#[command(
    about = "Access radar station and server information",
    long_about = "Provides access to various endpoints related to NOAA radar stations, servers, queues, and profilers."
)]
pub enum RadarCommand {
    Profiler(ProfilerArgs),
    Queue(QueueArgs),
    Server(ServerArgs),
    Servers(ServersArgs),
    Station(StationArgs),
    StationAlarms(StationAlarmsArgs),
    Stations(StationsArgs),
}

#[derive(Args, Debug)]
#[command(about = "Get metadata for a radar wind profiler")]
pub struct ProfilerArgs {
    /// The ID of the radar wind profiler station.
    #[arg(required = true)]
    station_id: String,

    /// Specific time for the data (ISO 8601 format or relative time).
    #[arg(long)]
    time: Option<String>,

    /// Time interval for the data (ISO 8601 duration format).
    #[arg(long)]
    interval: Option<String>,
}

#[derive(Args, Debug)]
#[command(about = "Get metadata about a radar queue")]
pub struct QueueArgs {
    /// The host name of the radar queue.
    #[arg(required = true)]
    host: String,

    /// Limit the number of results.
    #[arg(long)]
    limit: Option<i32>,

    /// Filter by arrival time range (ISO 8601 format, e.g., "start/end", "start/", "/end").
    #[arg(long)]
    arrived: Option<String>,

    /// Filter by creation time range (ISO 8601 format).
    #[arg(long)]
    created: Option<String>,

    /// Filter by publication time range (ISO 8601 format).
    #[arg(long)]
    published: Option<String>,

    /// Filter by radar station ID.
    #[arg(long)]
    station: Option<String>,

    /// Filter by data type.
    #[arg(long)]
    r#type: Option<String>,

    /// Filter by feed type.
    #[arg(long)]
    feed: Option<String>,

    /// Filter by resolution.
    #[arg(long)]
    resolution: Option<i32>,
}

#[derive(Args, Debug)]
#[command(about = "Get metadata for a specific radar server")]
pub struct ServerArgs {
    /// The ID of the radar server.
    #[arg(required = true)]
    id: String,

    /// Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,
}

#[derive(Args, Debug)]
#[command(about = "Get a list of radar servers")]
pub struct ServersArgs {
    /// Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,
}

#[derive(Args, Debug)]
#[command(about = "Get metadata for a specific radar station")]
pub struct StationArgs {
    /// The ID of the radar station.
    #[arg(required = true)]
    station_id: String,

    /// Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,

    /// Filter by host server.
    #[arg(long)]
    host: Option<String>,
}

#[derive(Args, Debug)]
#[command(about = "Get alarm metadata for a specific radar station")]
pub struct StationAlarmsArgs {
    /// The ID of the radar station.
    #[arg(required = true)]
    station_id: String,
}

#[derive(Args, Debug)]
#[command(about = "Get a list of radar stations")]
pub struct StationsArgs {
    /// Filter by station type(s). Can be specified multiple times.
    #[arg(long = "stationType")]
    station_type: Option<Vec<String>>,

    /// Filter by reporting host.
    #[arg(long)]
    reporting_host: Option<String>,

    /// Filter by host server.
    #[arg(long)]
    host: Option<String>,
}

pub async fn handle_command(command: RadarCommand, config: &Configuration) -> Result<Value> {
    match command {
        RadarCommand::Profiler(args) => {
            let result = radar_api::radar_profiler(
                config,
                &args.station_id,
                args.time.as_deref(),
                args.interval.as_deref(),
            )
            .await?;
            Ok(result)
        }
        RadarCommand::Queue(args) => {
            let result = radar_api::radar_queue(
                config,
                &args.host,
                args.limit,
                args.arrived.as_deref(),
                args.created.as_deref(),
                args.published.as_deref(),
                args.station.as_deref(),
                args.r#type.as_deref(),
                args.feed.as_deref(),
                args.resolution,
            )
            .await?;
            Ok(result)
        }
        RadarCommand::Server(args) => {
            let result =
                radar_api::radar_server(config, &args.id, args.reporting_host.as_deref()).await?;
            Ok(result)
        }
        RadarCommand::Servers(args) => {
            let result = radar_api::radar_servers(config, args.reporting_host.as_deref()).await?;
            Ok(result)
        }
        RadarCommand::Station(args) => {
            let result = radar_api::radar_station(
                config,
                &args.station_id,
                args.reporting_host.as_deref(),
                args.host.as_deref(),
            )
            .await?;
            Ok(result)
        }
        RadarCommand::StationAlarms(args) => {
            let result = radar_api::radar_station_alarms(config, &args.station_id).await?;
            Ok(result)
        }
        RadarCommand::Stations(args) => {
            let result = radar_api::radar_stations(
                config,
                args.station_type,
                args.reporting_host.as_deref(),
                args.host.as_deref(),
            )
            .await?;
            Ok(result)
        }
    }
}
