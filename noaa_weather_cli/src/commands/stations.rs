use anyhow::{Result, anyhow};
use clap::Subcommand;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::stations as station_api;
use noaa_weather_client::models::{AreaCode, StateTerritoryCode};
use serde_json::Value;
use std::str::FromStr;

/// Access data related to NWS observation stations.
#[derive(Subcommand, Debug)]
pub enum StationCommands {
    /// Get metadata for a specific observation station.
    ///
    /// Example: `noaa-weather stations metadata --id KPHX`
    Metadata {
        /// Station ID (e.g., KPHX, KDEN).
        #[arg(short, long)]
        id: String,
    },
    /// List observation stations, optionally filtered.
    ///
    /// Example: `noaa-weather stations list --state AZ --limit 10`
    List {
        /// Optional: Filter by station ID(s) (comma-separated).
        #[arg(long, value_delimiter = ',')]
        id: Option<Vec<String>>,
        /// Optional: Filter by US state/territory abbreviation(s) (comma-separated, e.g., AZ,CA).
        #[arg(long, value_delimiter = ',')]
        state: Option<Vec<String>>,
        /// Optional: Limit the number of results returned.
        #[arg(short, long)]
        limit: Option<i32>,
        /// Optional: Pagination cursor for fetching subsequent pages.
        #[arg(long)]
        cursor: Option<String>,
    },
    /// Get the latest observation for a specific station.
    ///
    /// Example: `noaa-weather stations latest-observation --station-id KPHX`
    LatestObservation {
        /// Station ID (e.g., KPHX, KDEN).
        #[arg(short = 's', long)]
        station_id: String,
        /// Optional: Only return quality controlled data.
        #[arg(long)]
        require_qc: Option<bool>,
    },
    /// List recent observations for a specific station, optionally filtered by time.
    ///
    /// Example: `noaa-weather stations observations --station-id KPHX --limit 5`
    /// Example: `noaa-weather stations observations --station-id KPHX --start "-PT2H" --end "-PT1H"`
    Observations {
        /// Station ID (e.g., KPHX).
        #[arg(short = 's', long)]
        station_id: String,
        /// Optional: Start time (ISO 8601 format or relative duration like "-PT1H").
        #[arg(long)]
        start: Option<String>,
        /// Optional: End time (ISO 8601 format or relative duration like "-PT1H").
        #[arg(long)]
        end: Option<String>,
        /// Optional: Limit the number of results.
        #[arg(short, long)]
        limit: Option<i32>,
    },
    /// Get a single observation for a station at a specific time.
    ///
    /// Requires an exact ISO 8601 timestamp matching an observation time.
    /// Example: `noaa-weather stations observation --station-id KPHX --time "2023-10-27T18:53:00+00:00"`
    Observation {
        /// Station ID (e.g., KPHX).
        #[arg(short = 's', long)]
        station_id: String,
        /// Exact observation time (ISO 8601 format).
        #[arg(long)]
        time: String,
    },
    /// Get the latest Terminal Aerodrome Forecasts (TAFs) for an airport station.
    ///
    /// Example: `noaa-weather stations tafs --station-id KPHX`
    Tafs {
        /// Airport Station ID (typically ICAO identifier, e.g., KPHX, KLAX).
        #[arg(short = 's', long)]
        station_id: String,
    },
    /// Get a specific Terminal Aerodrome Forecast (TAF) by date and time.
    ///
    /// Note: This is less common than fetching the latest TAFs using the `tafs` subcommand.
    /// Example: `noaa-weather stations taf --station-id KPHX --date 2025-05-03 --time 1800`
    Taf {
        /// Airport Station ID (e.g., KPHX).
        #[arg(short = 's', long)]
        station_id: String,
        /// Date of the TAF (YYYY-MM-DD).
        #[arg(long)]
        date: String,
        /// Time of the TAF (HHMM format, UTC).
        #[arg(long)]
        time: String,
    },
}

/// Handles the execution of station-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided `StationCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific station subcommand and its arguments to execute.
/// * `config` - The application configuration containing API details.
///
/// # Returns
///
/// A `Result` containing the JSON `Value` of the API response on success,
/// or an `anyhow::Error` if an error occurs during the API call or processing.
pub async fn handle_command(command: StationCommands, config: &Configuration) -> Result<Value> {
    match command {
        StationCommands::Metadata { id } => {
            let result = station_api::get_observation_station(config, &id)
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

            let result = station_api::get_observation_stations(
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
            station_id,
            require_qc,
        } => {
            let result = station_api::get_latest_observations(config, &station_id, require_qc)
                .await
                .map_err(|e| anyhow!("Error getting latest observation: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Observations {
            station_id,
            start,
            end,
            limit,
        } => {
            let result = station_api::get_observations(config, &station_id, start, end, limit)
                .await
                .map_err(|e| anyhow!("Error listing observations: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Observation { station_id, time } => {
            let result = station_api::get_observation_by_time(config, &station_id, time)
                .await
                .map_err(|e| anyhow!("Error getting observation by time: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Tafs { station_id } => {
            let result = station_api::get_terminal_aerodrome_forecasts(config, &station_id)
                .await
                .map_err(|e| anyhow!("Error getting TAFs: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
        StationCommands::Taf {
            station_id,
            date,
            time,
        } => {
            let result =
                station_api::get_terminal_aerodrome_forecast(config, &station_id, date, &time)
                    .await
                    .map_err(|e| anyhow!("Error getting specific TAF: {e}"))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
