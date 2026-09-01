//! Basic usage example for the NOAA Weather Client
//!
//! This example demonstrates how to:
//! - Get point metadata for coordinates
//! - Fetch active weather alerts
//!
//! Run with: just example-basic
//! Or: cargo run --example basic_usage --manifest-path noaa_weather_client/Cargo.toml

use noaa_weather_client::Client;
use noaa_weather_client::apis::{alerts, points};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(
        "noaa-weather-examples/1.0 (+https://github.com/seferino-fernandez/noaa_weather)",
    )
    .build()?;

    println!("NOAA Weather Client - Basic Usage Example\n");

    // Example coordinates (Kansas City, MO)
    let latitude = 39.7456;
    let longitude = -94.5692;
    println!(
        "Getting weather information for coordinates: {},{}",
        latitude, longitude
    );

    // 1. Get point metadata
    println!("\n[1] Getting point metadata...");
    match points::get_point(&client, latitude, longitude).await {
        Ok(point_data) => {
            let properties = &point_data.properties;
            println!("  Forecast Office: {:?}", properties.forecast_office);
            println!(
                "  Grid Coordinates: {},{}",
                properties.grid_x.unwrap_or(0),
                properties.grid_y.unwrap_or(0)
            );
            if let Some(time_zone) = &properties.time_zone {
                println!("  Time Zone: {}", time_zone);
            }
        }
        Err(error) => {
            eprintln!("  Error getting point data: {}", error);
        }
    }

    // 2. Get active weather alerts (limited to first 3)
    println!("\n[2] Getting active weather alerts...");
    let alert_params = alerts::ActiveAlertsParams::default();
    match alerts::get_active_alerts(&client, alert_params).await {
        Ok(alerts_data) => {
            println!("  Found {} active alerts", alerts_data.features.len());
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
            eprintln!("  Error getting alerts: {}", error);
        }
    }

    println!("\nExample completed!");
    println!("\nTry running other examples:");
    println!("   just example-alerts");
    println!("   just examples        # Run all examples");

    Ok(())
}
