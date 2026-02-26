//! Weather Alerts Example
//!
//! This example demonstrates how to:
//! - Get active alerts with various filters
//! - Get alerts for specific areas and zones
//! - Display alert details in a user-friendly format
//!
//! Run with: just example-alerts
//! Or: cargo run --example weather_alerts --manifest-path noaa_weather_client/Cargo.toml

use noaa_weather_client::apis::alerts;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::{AlertSeverity, AreaCode, StateTerritoryCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    println!("NOAA Weather Alerts Example\n");

    // 1. Get count of all active alerts
    println!("[1] Getting alert count...");
    match alerts::get_active_alerts_count(&config).await {
        Ok(count_response) => {
            println!(
                "  Total active alerts: {}",
                count_response.total.unwrap_or(0)
            );
            if let Some(land) = count_response.land {
                println!("  Land alerts: {}", land);
            }
            if let Some(marine) = count_response.marine {
                println!("  Marine alerts: {}", marine);
            }
        }
        Err(error) => {
            eprintln!("  Error getting alert count: {}", error);
        }
    }

    // 2. Get alerts for California
    println!("\n[2] Getting alerts for California...");
    match alerts::get_active_alerts_for_area(
        &config,
        &AreaCode::StateTerritoryCode(StateTerritoryCode::Ca),
    )
    .await
    {
        Ok(ca_alerts) => {
            println!("  Found {} alerts for California", ca_alerts.features.len());

            for (index, alert_feature) in ca_alerts.features.iter().take(3).enumerate() {
                if let Some(alert_props) = &alert_feature.properties {
                    println!("\n    Alert #{}", index + 1);
                    println!(
                        "    Event: {}",
                        alert_props.event.as_deref().unwrap_or("Unknown")
                    );
                    println!(
                        "    Headline: {}",
                        alert_props
                            .headline
                            .as_ref()
                            .and_then(|h| h.as_ref())
                            .map(|s| s.as_str())
                            .unwrap_or("No headline")
                    );
                    println!(
                        "    Areas: {}",
                        alert_props.area_desc.as_deref().unwrap_or("Unknown")
                    );

                    if let Some(severity) = &alert_props.severity {
                        println!("    Severity: {}", severity);
                    }

                    if let Some(urgency) = &alert_props.urgency {
                        println!("    Urgency: {}", urgency);
                    }

                    if let Some(expires) = &alert_props.expires {
                        println!("    Expires: {}", expires);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("  Error getting California alerts: {}", error);
        }
    }

    // 3. Get available alert types
    println!("\n[3] Getting available alert types...");
    match alerts::get_alert_types(&config).await {
        Ok(types_response) => {
            if let Some(event_types) = &types_response.event_types {
                println!("  Available alert types ({} total):", event_types.len());
                for alert_type in event_types.iter().take(10) {
                    println!("    - {}", alert_type);
                }
                if event_types.len() > 10 {
                    println!("    ... and {} more", event_types.len() - 10);
                }
            }
        }
        Err(error) => {
            eprintln!("  Error getting alert types: {}", error);
        }
    }

    // 4. Get alerts with specific filters
    println!("\n[4] Getting high-severity alerts...");
    let severity_params = alerts::ActiveAlertsParams {
        severity: Some(vec![
            AlertSeverity::Minor,
            AlertSeverity::Moderate,
            AlertSeverity::Severe,
            AlertSeverity::Extreme,
        ]),
        ..Default::default()
    };
    match alerts::get_active_alerts(&config, severity_params).await {
        Ok(severe_alerts) => {
            println!(
                "  Found {} high-severity alerts",
                severe_alerts.features.len()
            );

            for (index, alert_feature) in severe_alerts.features.iter().enumerate() {
                if let Some(alert_props) = &alert_feature.properties {
                    println!(
                        "    {}. {} - {}",
                        index + 1,
                        alert_props.event.as_deref().unwrap_or("Unknown"),
                        alert_props
                            .severity
                            .as_ref()
                            .map(|s| s.to_string())
                            .unwrap_or("Unknown severity".to_string())
                    );
                }
            }
        }
        Err(error) => {
            eprintln!("  Error getting severe alerts: {}", error);
        }
    }

    // 5. Get specific alert by ID (if any alerts exist)
    println!("\n[5] Getting detailed information for a specific alert...");
    let single_alert_params = alerts::ActiveAlertsParams::default();
    match alerts::get_active_alerts(&config, single_alert_params).await {
        Ok(any_alerts) => {
            if let Some(first_alert) = any_alerts.features.first() {
                if let Some(alert_id) = &first_alert.properties.as_ref().and_then(|p| p.id.as_ref())
                {
                    println!("  Getting details for alert: {}", alert_id);

                    match alerts::get_alert(&config, alert_id).await {
                        Ok(detailed_alert) => {
                            let props = &detailed_alert.properties;
                            println!("    Alert Details:");
                            println!("    Event: {}", props.event.as_deref().unwrap_or("Unknown"));
                            println!(
                                "    Sender: {}",
                                props.sender_name.as_deref().unwrap_or("Unknown")
                            );

                            if let Some(description) = &props.description {
                                let short_desc = if description.len() > 200 {
                                    format!("{}...", &description[..200])
                                } else {
                                    description.clone()
                                };
                                println!("    Description: {}", short_desc);
                            }

                            if let Some(Some(instruction)) = &props.instruction {
                                let short_instruction = if instruction.len() > 200 {
                                    format!("{}...", &instruction[..200])
                                } else {
                                    instruction.clone()
                                };
                                println!("    Instructions: {}", short_instruction);
                            }
                        }
                        Err(error) => {
                            eprintln!("    Error getting alert details: {}", error);
                        }
                    }
                }
            } else {
                println!("  No alerts currently available for detailed lookup");
            }
        }
        Err(error) => {
            eprintln!("  Error getting sample alert: {}", error);
        }
    }

    println!("\nWeather alerts example completed!");
    println!("\nPro tip: You can filter alerts by:");
    println!("   - Area (state/territory code like 'CA', 'TX')");
    println!("   - Zone (specific forecast zone like 'CAZ006')");
    println!("   - Severity (Minor, Moderate, Severe, Extreme)");
    println!("   - Message type (Alert, Update, Cancel)");

    Ok(())
}
