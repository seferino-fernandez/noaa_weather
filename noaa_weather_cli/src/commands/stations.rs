use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::{AreaCode, StateTerritoryCode};
use serde_json::Value;
use std::str::FromStr;

#[derive(Args, Debug)]
pub struct StationIdArgs {
    /// Station ID (e.g., KPHX)
    #[arg(short = 's', long)]
    station_id: String,
}

#[derive(Subcommand, Debug)]
pub enum StationCommands {
    /// Get metadata for a specific observation station
    Metadata(StationIdArgs),
    /// List observation stations
    List {
        /// Filter by station ID (comma-separated)
        #[arg(long, value_delimiter = ',')]
        id: Option<Vec<String>>,
        /// Filter by US state/territory abbreviation (comma-separated, e.g., AZ,CA)
        #[arg(long, value_delimiter = ',')]
        state: Option<Vec<String>>,
        /// Limit the number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Pagination cursor
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Get the latest observation for a station
    LatestObservation {
        #[clap(flatten)]
        station_args: StationIdArgs,
        /// Require quality controlled data
        #[arg(long)]
        require_qc: Option<bool>,
    },
    /// List observations for a station
    Observations {
        #[clap(flatten)]
        station_args: StationIdArgs,
        /// Start time (ISO 8601 format)
        #[arg(long)]
        start: Option<String>,
        /// End time (ISO 8601 format)
        #[arg(long)]
        end: Option<String>,
        /// Limit the number of results
        #[arg(short, long)]
        limit: Option<i32>,
    },
    /// Get a specific observation by time
    Observation {
        #[clap(flatten)]
        station_args: StationIdArgs,
        /// Observation time (ISO 8601 format)
        #[arg(long)]
        time: String,
    },
    /// Get Terminal Aerodrome Forecasts (TAFs) for a station
    Tafs(StationIdArgs),
    /// Get a specific TAF by date and time
    Taf {
        #[clap(flatten)]
        station_args: StationIdArgs,
        /// Date (YYYY-MM-DD)
        #[arg(long)]
        date: String,
        /// Time (e.g., "06:00:00Z", "2023-10-26T06:00:00+00:00")
        #[arg(long)]
        time: String,
    },
}

pub async fn handle_command(command: StationCommands, config: &Configuration) -> Result<Value> {
    match command {
        StationCommands::Metadata(args) => {
            let result = noaa_weather_client::apis::stations::obs_station(config, &args.station_id)
                .await
                .map_err(|e| anyhow!("Error getting station metadata: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::List {
            id,
            state,
            limit,
            cursor,
        } => {
            // Parse state strings into StateTerritoryCode enums, then wrap in AreaCode
            let states_parsed = state
                .map(|states| {
                    states
                        .into_iter()
                        .map(|s| StateTerritoryCode::from_str(&s))
                        .collect::<Result<Vec<_>, _>>()
                        .map(|stc_vec| {
                            stc_vec
                                .into_iter()
                                .map(AreaCode::StateTerritoryCode)
                                .collect()
                        })
                })
                .transpose()
                .map_err(|e| anyhow!("Invalid state code provided: {e}"))?;

            let result = noaa_weather_client::apis::stations::obs_stations(
                config,
                id,
                states_parsed,
                limit,
                cursor.as_deref(),
            )
            .await
            .map_err(|e| anyhow!("Error listing stations: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::LatestObservation {
            station_args,
            require_qc,
        } => {
            let result = noaa_weather_client::apis::stations::station_observation_latest(
                config,
                &station_args.station_id,
                require_qc,
            )
            .await
            .map_err(|e| anyhow!("Error getting latest observation: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Observations {
            station_args,
            start,
            end,
            limit,
        } => {
            let result = noaa_weather_client::apis::stations::station_observation_list(
                config,
                &station_args.station_id,
                start,
                end,
                limit,
            )
            .await
            .map_err(|e| anyhow!("Error listing observations: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Observation { station_args, time } => {
            let result = noaa_weather_client::apis::stations::station_observation_time(
                config,
                &station_args.station_id,
                time,
            )
            .await
            .map_err(|e| anyhow!("Error getting observation by time: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Tafs(args) => {
            let result = noaa_weather_client::apis::stations::tafs(config, &args.station_id)
                .await
                .map_err(|e| anyhow!("Error getting TAFs: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Taf {
            station_args,
            date,
            time,
        } => {
            let result = noaa_weather_client::apis::stations::taf(
                config,
                &station_args.station_id,
                date,
                &time,
            )
            .await
            .map_err(|e| anyhow!("Error getting specific TAF: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
