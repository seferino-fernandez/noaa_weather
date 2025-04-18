use anyhow::{Result, anyhow};
use clap::{Args, Subcommand, value_parser};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::gridpoints as gridpoints_api;
use noaa_weather_client::models::{GridpointForecastUnits, NwsForecastOfficeId};
use serde_json::Value;

/// Common arguments for gridpoint-based commands.
#[derive(Args, Debug, Clone)]
pub struct GridpointLocationArgs {
    /// NWS forecast office ID (e.g., TOP, LWX).
    #[arg(long, value_parser = value_parser!(NwsForecastOfficeId))]
    office_id: NwsForecastOfficeId,

    /// Grid X coordinate.
    #[arg(short, long)]
    x: i32,

    /// Grid Y coordinate.
    #[arg(short, long)]
    y: i32,
}

/// Arguments specific to forecast endpoints.
#[derive(Args, Debug, Clone)]
pub struct ForecastArgs {
    /// Feature flags to enable experimental API features.
    #[arg(long)]
    feature_flags: Option<Vec<String>>,

    /// Units for forecast data (us or si).
    #[arg(long, value_parser = value_parser!(GridpointForecastUnits))]
    units: Option<GridpointForecastUnits>,
}

/// Arguments specific to the stations endpoint.
#[derive(Args, Debug, Clone)]
pub struct StationsArgs {
    /// Limit the number of stations returned.
    #[arg(long)]
    limit: Option<i32>,

    /// Pagination cursor for fetching subsequent results.
    #[arg(long)]
    cursor: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum GridpointCommands {
    /// Get raw numerical forecast data for a gridpoint.
    Gridpoint {
        #[clap(flatten)]
        location: GridpointLocationArgs,
    },
    /// Get textual forecast for a gridpoint.
    Forecast {
        #[clap(flatten)]
        location: GridpointLocationArgs,
        #[clap(flatten)]
        forecast_opts: ForecastArgs,
    },
    /// Get textual hourly forecast for a gridpoint.
    Hourly {
        #[clap(flatten)]
        location: GridpointLocationArgs,
        #[clap(flatten)]
        forecast_opts: ForecastArgs,
    },
    /// Get observation stations usable for a gridpoint.
    Stations {
        #[clap(flatten)]
        location: GridpointLocationArgs,
        #[clap(flatten)]
        station_opts: StationsArgs,
    },
}

pub async fn handle_command(command: GridpointCommands, config: &Configuration) -> Result<Value> {
    match command {
        GridpointCommands::Gridpoint { location } => {
            let result =
                gridpoints_api::gridpoint(config, location.office_id, location.x, location.y)
                    .await
                    .map_err(|e| anyhow!("getting raw gridpoint data: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        GridpointCommands::Forecast {
            location,
            forecast_opts,
        } => {
            let result = gridpoints_api::gridpoint_forecast(
                config,
                location.office_id,
                location.x,
                location.y,
                forecast_opts.feature_flags,
                forecast_opts.units,
            )
            .await
            .map_err(|e| anyhow!("getting gridpoint forecast: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        GridpointCommands::Hourly {
            location,
            forecast_opts,
        } => {
            let result = gridpoints_api::gridpoint_forecast_hourly(
                config,
                location.office_id,
                location.x,
                location.y,
                forecast_opts.feature_flags,
                forecast_opts.units,
            )
            .await
            .map_err(|e| anyhow!("getting hourly gridpoint forecast: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        GridpointCommands::Stations {
            location,
            station_opts,
        } => {
            let result = gridpoints_api::gridpoint_stations(
                config,
                location.office_id,
                location.x,
                location.y,
                station_opts.limit,
                station_opts.cursor.as_deref(),
            )
            .await
            .map_err(|e| anyhow!("getting gridpoint stations: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
