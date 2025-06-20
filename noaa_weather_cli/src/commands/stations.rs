use anyhow::{Result, anyhow};
use clap::Subcommand;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::stations as station_api;
use noaa_weather_client::models::{AreaCode, StateTerritoryCode};
use std::str::FromStr as _;

use crate::utils::format::write_output;
use crate::{Cli, tables};

/// Access data related to NWS observation stations.
#[derive(Subcommand, Debug, Clone)]
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
        /// Optional: Limit the number of observation stations returned by the API.
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=500))]
        limit: Option<i32>,
    },

    /// Get the latest observation for a specific station.
    ///
    /// Example: `noaa-weather stations latest-observation --station-id KPHX`
    LatestObservation {
        /// Station ID (e.g., KPHX, KDEN).
        #[arg(short = 's', long)]
        station_id: String,
        /// Optional: Only return quality controlled data.
        #[arg(long, default_value_t = false)]
        require_quality_controlled: bool,
    },

    /// List recent observations for a specific station, optionally filtered by time.
    ///
    /// Example: `noaa-weather stations observations --station-id KPHX --limit 5`
    /// Example: `noaa-weather stations observations --station-id KPHX --start "-PT2H" --end "-PT1H"`
    Observations {
        /// Station ID (e.g., KPHX).
        #[arg(long)]
        station_id: String,
        /// Optional: Start time (ISO 8601 format or relative duration like "-PT1H").
        #[arg(long)]
        start: Option<String>,
        /// Optional: End time (ISO 8601 format or relative duration like "-PT1H").
        #[arg(long)]
        end: Option<String>,
        /// Optional: Limit the number of observations returned by the API.
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=500))]
        limit: Option<i32>,
    },
    /// Get a single observation for a station at a specific time.
    ///
    /// Requires an exact ISO 8601 timestamp matching an observation time.
    /// Example: `noaa-weather stations observation --station-id KPHX --time "2023-10-27T18:53:00+00:00"`
    Observation {
        /// Station ID (e.g., KPHX).
        #[arg(long)]
        station_id: String,
        /// Exact observation time (ISO 8601 format).
        #[arg(long)]
        time: String,
    },
    /// Get the metadata for Terminal Aerodrome Forecasts (TAFs) for an airport station.
    ///
    /// Example: `noaa-weather stations terminal-aerodrome-forecasts --station-id KPHX`
    TerminalAerodromeForecasts {
        /// Airport Station ID (typically ICAO identifier, e.g., KPHX, KLAX).
        #[arg(long)]
        station_id: String,
    },
    /// Get a specific Terminal Aerodrome Forecast (TAF) by date and time.
    ///
    /// Example: `noaa-weather stations terminal-aerodrome-forecast --station-id KPHX --date 2025-05-03 --time 1800`
    TerminalAerodromeForecast {
        /// Airport Station ID (e.g., KPHX).
        #[arg(long)]
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
/// * `cli` - The CLI arguments.
/// * `config` - The application configuration containing API details.
///
pub async fn handle_command(
    command: &StationCommands,
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
    match command {
        StationCommands::Metadata { id } => {
            let result = station_api::get_observation_station(config, id)
                .await
                .map_err(|error| anyhow!("Error getting station metadata: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_observation_station_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        StationCommands::List { id, state, limit } => {
            // Parse state strings into StateTerritoryCode enums, then wrap in AreaCode
            let states_parsed = state
                .as_ref()
                .map(|states| {
                    states
                        .iter()
                        .map(|state_code| StateTerritoryCode::from_str(state_code))
                        .collect::<Result<Vec<_>, _>>()
                        .map(|stc_vec| {
                            stc_vec
                                .into_iter()
                                .map(AreaCode::StateTerritoryCode)
                                .collect()
                        })
                })
                .transpose()
                .map_err(|error| anyhow!("Invalid state code provided: {error}"))?;

            let result = station_api::get_observation_stations(
                config,
                id.clone(),
                states_parsed,
                *limit,
                None,
            )
            .await
            .map_err(|error| anyhow!("Error listing stations: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        StationCommands::LatestObservation {
            station_id,
            require_quality_controlled,
        } => {
            let result = station_api::get_latest_observations(
                config,
                station_id,
                Some(*require_quality_controlled),
            )
            .await
            .map_err(|error| anyhow!("Error getting latest observation: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_observation_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        StationCommands::Observations {
            station_id,
            start,
            end,
            limit,
        } => {
            let result = station_api::get_observations(
                config,
                station_id,
                start.clone(),
                end.clone(),
                *limit,
            )
            .await
            .map_err(|error| anyhow!("Error listing observations: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_observations_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        StationCommands::Observation { station_id, time } => {
            let result = station_api::get_observation_by_time(config, station_id, time.clone())
                .await
                .map_err(|error| anyhow!("Error getting observation by time: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_observation_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        StationCommands::TerminalAerodromeForecasts { station_id } => {
            let result = station_api::get_terminal_aerodrome_forecasts(config, station_id)
                .await
                .map_err(|error| anyhow!("Error getting TAFs: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_tafs_metadata_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        StationCommands::TerminalAerodromeForecast {
            station_id,
            date,
            time,
        } => {
            let result = station_api::get_terminal_aerodrome_forecast(
                config,
                station_id,
                date.clone(),
                time,
            )
            .await
            .map_err(|error| anyhow!("Error getting specific TAF: {error}"))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::stations::create_stations_taf_table(&result);
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
    }
}
