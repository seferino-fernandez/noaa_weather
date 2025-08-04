//! Basic usage example for the NOAA Weather Client
//!
//! This example demonstrates how to:
//! - Get point metadata for coordinates
//! - Fetch active weather alerts
//! - Get current weather observations
//! - Retrieve forecasts
//!
//! Run with: just example-basic
//! Or: cargo run --example basic_usage --manifest-path noaa_weather_client/Cargo.toml

use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::{alerts, points, stations};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    println!("🌦️ NOAA Weather Client - Basic Usage Example\n");

    // Example coordinates (Kansas City, MO)
    let coordinates = "39.7456,-94.5692";
    println!(
        "📍 Getting weather information for coordinates: {}",
        coordinates
    );

    // 1. Get point metadata
    println!("\n1️⃣ Getting point metadata...");
    match points::get_point(&config, coordinates).await {
        Ok(point_data) => {
            let properties = &point_data.properties;
            println!("  ✅ Forecast Office: {:?}", properties.forecast_office);
            println!(
                "  ✅ Grid Coordinates: {},{}",
                properties.grid_x.unwrap_or(0),
                properties.grid_y.unwrap_or(0)
            );
            if let Some(time_zone) = &properties.time_zone {
                println!("  ✅ Time Zone: {}", time_zone);
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting point data: {}", error);
        }
    }

    // 2. Get active weather alerts (limited to first 5)
    println!("\n2️⃣ Getting active weather alerts...");
    let alert_params = alerts::ActiveAlertsParams::default();
    match alerts::get_active_alerts(&config, alert_params).await {
        Ok(alerts_data) => {
            println!("  ✅ Found {} active alerts", alerts_data.features.len());
            for (index, alert_feature) in alerts_data.features.iter().take(3).enumerate() {
                if let Some(properties) = &alert_feature.properties {
                    println!(
                        "    {}. {}",
                        index + 1,
                        properties.event.as_deref().unwrap_or("Unknown Event")
                    );
                    if let Some(areas) = &properties.area_desc {
                        println!("       Areas: {}", areas);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting alerts: {}", error);
        }
    }

    // 3. Get stations near the point
    println!("\n3️⃣ Getting nearby weather stations...");
    match points::get_point_stations(&config, coordinates).await {
        Ok(stations_data) => {
            println!(
                "  ✅ Found {} nearby stations",
                stations_data.features.len()
            );

            // Try to get observation from the first station
            if let Some(first_station) = stations_data.features.first()
                && let Some(station_id) = &first_station.properties.station_identifier
            {
                println!(
                    "  📡 Getting latest observation from station: {}",
                    station_id
                );

                match stations::get_latest_observations(&config, station_id, None).await {
                    Ok(observation) => {
                        let props = &observation.properties;
                        println!(
                            "    ✅ Station: {}",
                            props.station_name.as_deref().unwrap_or("Unknown")
                        );

                        if let Some(temp) = &props.temperature
                            && let Some(temp_value) = temp.value
                        {
                            println!("    🌡️  Temperature: {:.1}°C", temp_value);
                        }

                        if let Some(desc) = &props.text_description {
                            println!("    ☁️  Conditions: {}", desc);
                        }

                        if let Some(timestamp) = &props.timestamp {
                            println!("    ⏰ Observed: {}", timestamp);
                        }
                    }
                    Err(error) => {
                        eprintln!("    ❌ Error getting observation: {}", error);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting stations: {}", error);
        }
    }

    println!("\n✨ Example completed!");
    println!("\n💡 Try running other examples:");
    println!("   just example-alerts");
    println!("   just examples        # Run all examples");

    Ok(())
}
