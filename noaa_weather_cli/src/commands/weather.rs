use anyhow::Result;
use clap::Subcommand;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::city_coordinates::{USCity, USState};
use serde_json::Value;
use std::str::FromStr;

#[derive(Subcommand, Debug)]
pub enum WeatherCommands {
    /// Get current weather for a US city
    City {
        /// The city to get weather for (e.g. Phoenix)
        city: String,
        /// The state the city is in (e.g. AZ)
        state: String,
    },
}

pub async fn handle_command(command: WeatherCommands, config: &Configuration) -> Result<Value> {
    match command {
        WeatherCommands::City { city, state } => {
            // Convert state string to USState enum
            let state =
                USState::from_str(&state).map_err(|e| anyhow::anyhow!("Invalid state: {}", e))?;

            // Convert city string to USCity enum
            let city = USCity::from_str(&city, state)
                .map_err(|e| anyhow::anyhow!("Invalid city: {}", e))?;

            let result = noaa_weather_client::apis::stations::get_city_weather(config, city)
                .await
                .map_err(|e| anyhow::anyhow!("Error getting city weather: {}", e))?;

            Ok(serde_json::to_value(result)?)
        }
    }
}
