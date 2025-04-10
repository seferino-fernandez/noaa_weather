use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::apis::configuration::Configuration;
use serde_json::Value;

#[derive(Args, Debug)]
pub struct PointArgs {
    /// Latitude,Longitude (e.g., 33.4484,-112.074)
    point: String,
}

#[derive(Subcommand, Debug)]
pub enum PointCommands {
    /// Get metadata for a specific point
    Metadata(PointArgs),
    /// Get observation stations for a specific point
    Stations(PointArgs),
}

pub async fn handle_command(command: PointCommands, config: &Configuration) -> Result<Value> {
    match command {
        PointCommands::Metadata(args) => {
            let result = noaa_weather_client::apis::points::point(config, &args.point)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting point metadata: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        PointCommands::Stations(args) => {
            let result = noaa_weather_client::apis::points::point_stations(config, &args.point)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting point stations: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
