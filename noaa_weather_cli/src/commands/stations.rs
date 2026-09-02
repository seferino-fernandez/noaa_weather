use anyhow::Result;
use clap::Subcommand;
use jiff::Timestamp;
use noaa_weather_client::apis::stations::{
    LatestObservationQuery, ObservationsQuery, StationsQuery,
};
use noaa_weather_client::models::AreaCode;
use noaa_weather_client::{Client, Cursor, StationId};

use super::parse;
use crate::output::Output;

/// Access data related to NWS observation stations.
#[derive(Subcommand, Debug, Clone)]
pub enum StationCommands {
    /// Get metadata for a specific observation station.
    ///
    /// Example: `noaa-weather stations metadata --id KPHX`
    Metadata {
        /// Station ID (e.g., KPHX, KDEN).
        #[arg(short, long)]
        id: StationId,
    },

    /// List observation stations, optionally filtered.
    ///
    /// Example: `noaa-weather stations list --state AZ --limit 10`
    List {
        /// Optional: Filter by station ID(s) (comma-separated).
        #[arg(long, value_delimiter = ',')]
        id: Vec<StationId>,
        /// Optional: Filter by US state/territory or marine area code(s) (comma-separated, e.g., AZ,CA).
        #[arg(long, value_delimiter = ',')]
        state: Vec<AreaCode>,
        /// Optional: Limit the number of observation stations returned by the API (1 to 500).
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=500))]
        limit: Option<u16>,
        /// Opaque pagination cursor from a previous page (see pagination.next in --json output)
        #[arg(long)]
        cursor: Option<Cursor>,
    },

    /// Get the latest observation for a specific station.
    ///
    /// Example: `noaa-weather stations latest-observation --station-id KPHX`
    LatestObservation {
        /// Station ID (e.g., KPHX, KDEN).
        #[arg(short = 's', long)]
        station_id: StationId,
        /// Optional: Only return quality controlled data.
        #[arg(long, default_value_t = false)]
        require_quality_controlled: bool,
    },

    /// List recent observations for a specific station, optionally filtered by time.
    ///
    /// Example: `noaa-weather stations observations --station-id KPHX --limit 5`
    /// Example: `noaa-weather stations observations --station-id KPHX --start 2h --end 1h`
    Observations {
        /// Station ID (e.g., KPHX).
        #[arg(long)]
        station_id: StationId,
        /// Optional: Start time (RFC 3339 timestamp or relative age such as 6h).
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        start: Option<Timestamp>,
        /// Optional: End time (RFC 3339 timestamp or relative age such as 1h).
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        end: Option<Timestamp>,
        /// Optional: Limit the number of observations returned by the API (1 to 500).
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=500))]
        limit: Option<u16>,
        /// Opaque pagination cursor from a previous page (see pagination.next in --json output)
        #[arg(long)]
        cursor: Option<Cursor>,
    },
    /// Get a single observation for a station at a specific time.
    ///
    /// Requires an exact timestamp matching an observation time.
    /// Example: `noaa-weather stations observation --station-id KPHX --time "2023-10-27T18:53:00+00:00"`
    Observation {
        /// Station ID (e.g., KPHX).
        #[arg(long)]
        station_id: StationId,
        /// Exact observation time (RFC 3339 timestamp; sent to NOAA in UTC).
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        time: Timestamp,
    },
    /// Get the metadata for Terminal Aerodrome Forecasts (TAFs) for an airport station.
    ///
    /// Example: `noaa-weather stations terminal-aerodrome-forecasts --station-id KPHX`
    TerminalAerodromeForecasts {
        /// Airport Station ID (typically ICAO identifier, e.g., KPHX, KLAX).
        #[arg(long)]
        station_id: StationId,
    },
    /// Get a specific Terminal Aerodrome Forecast (TAF) by its issue time.
    ///
    /// Example: `noaa-weather stations terminal-aerodrome-forecast --station-id KPHX --issued 2025-05-03T18:00:00Z`
    TerminalAerodromeForecast {
        /// Airport Station ID (e.g., KPHX).
        #[arg(long)]
        station_id: StationId,
        /// Issue time of the TAF (RFC 3339 timestamp). NOAA addresses a TAF by its UTC date and HHMM minute, so seconds are dropped.
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        issued: Timestamp,
    },
}

/// Handles the execution of station-related subcommands.
///
/// Dispatches the command to the matching `client.stations()` method based
/// on the provided `StationCommands` variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific station subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `client` - The NOAA API client.
///
pub async fn handle_command(
    command: &StationCommands,
    output: &Output,
    client: &Client,
) -> Result<()> {
    let stations = client.stations();
    match command {
        StationCommands::Metadata { id } => {
            output
                .show(format!("getting station {id} metadata"), stations.get(id))
                .await
        }
        StationCommands::List {
            id,
            state,
            limit,
            cursor,
        } => {
            let query = StationsQuery {
                id: id.clone(),
                state: state.clone(),
                limit: *limit,
                cursor: cursor.clone(),
            };
            output
                .show("listing observation stations", stations.list(&query))
                .await
        }
        StationCommands::LatestObservation {
            station_id,
            require_quality_controlled,
        } => {
            let query = LatestObservationQuery {
                require_qc: Some(*require_quality_controlled),
            };
            output
                .show(
                    format!("getting latest observation for station {station_id}"),
                    stations.latest_observation(station_id, &query),
                )
                .await
        }
        StationCommands::Observations {
            station_id,
            start,
            end,
            limit,
            cursor,
        } => {
            let query = ObservationsQuery {
                start: *start,
                end: *end,
                limit: *limit,
                cursor: cursor.clone(),
            };
            output
                .show(
                    format!("listing observations for station {station_id}"),
                    stations.observations(station_id, &query),
                )
                .await
        }
        StationCommands::Observation { station_id, time } => {
            output
                .show(
                    format!("getting observation for station {station_id} at {time}"),
                    stations.observation_at(station_id, *time),
                )
                .await
        }
        StationCommands::TerminalAerodromeForecasts { station_id } => {
            output
                .show(
                    format!("getting TAFs for station {station_id}"),
                    stations.tafs(station_id),
                )
                .await
        }
        StationCommands::TerminalAerodromeForecast { station_id, issued } => {
            output
                .show(
                    format!("getting TAF for station {station_id} issued at {issued}"),
                    stations.taf(station_id, *issued),
                )
                .await
        }
    }
}
