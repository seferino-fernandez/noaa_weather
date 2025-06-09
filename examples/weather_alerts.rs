//! Weather Alerts Example
//!
//! This example demonstrates how to:
//! - Get active alerts with various filters
//! - Get alerts for specific areas and zones
//! - Display alert details in a user-friendly format
//!
//! Run with: cargo run --example weather_alerts

use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::alerts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new();

    println!("🚨 NOAA Weather Alerts Example\n");

    // 1. Get count of all active alerts
    println!("1️⃣ Getting alert count...");
    match alerts::get_active_alerts_count(&config, None).await {
        Ok(count_response) => {
            println!("  ✅ Total active alerts: {}", count_response.total.unwrap_or(0));
            if let Some(land) = count_response.land {
                println!("  🌍 Land alerts: {}", land);
            }
            if let Some(marine) = count_response.marine {
                println!("  🌊 Marine alerts: {}", marine);
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting alert count: {}", error);
        }
    }

    // 2. Get alerts for California
    println!("\n2️⃣ Getting alerts for California...");
    match alerts::get_active_alerts_for_area(&config, "CA").await {
        Ok(ca_alerts) => {
            println!("  ✅ Found {} alerts for California", ca_alerts.features.len());

            for (index, alert_feature) in ca_alerts.features.iter().take(3).enumerate() {
                if let Some(alert_props) = &alert_feature.properties {
                    println!("\n    Alert #{}", index + 1);
                    println!("    📢 Event: {}", alert_props.event.as_deref().unwrap_or("Unknown"));
                    println!("    📋 Headline: {}",
                        alert_props.headline.as_ref()
                            .and_then(|h| h.as_ref())
                            .unwrap_or("No headline")
                    );
                    println!("    🗺️  Areas: {}", alert_props.area_desc.as_deref().unwrap_or("Unknown"));

                    if let Some(severity) = &alert_props.severity {
                        println!("    ⚠️  Severity: {}", severity);
                    }

                    if let Some(urgency) = &alert_props.urgency {
                        println!("    ⏰ Urgency: {}", urgency);
                    }

                    if let Some(expires) = &alert_props.expires {
                        println!("    ⏳ Expires: {}", expires);
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting California alerts: {}", error);
        }
    }

    // 3. Get available alert types
    println!("\n3️⃣ Getting available alert types...");
    match alerts::get_alert_types(&config).await {
        Ok(types_response) => {
            if let Some(event_types) = &types_response.event_types {
                println!("  ✅ Available alert types ({} total):", event_types.len());
                for alert_type in event_types.iter().take(10) {
                    println!("    • {}", alert_type);
                }
                if event_types.len() > 10 {
                    println!("    ... and {} more", event_types.len() - 10);
                }
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting alert types: {}", error);
        }
    }

    // 4. Get alerts with specific filters
    println!("\n4️⃣ Getting high-severity alerts...");
    match alerts::get_active_alerts(
        &config,
        None,               // status
        None,               // message_type
        Some("Minor,Moderate,Severe,Extreme"), // severity
        Some(5),            // limit
        None,               // cursor
        None,               // area
        None                // zone
    ).await {
        Ok(severe_alerts) => {
            println!("  ✅ Found {} high-severity alerts", severe_alerts.features.len());

            for (index, alert_feature) in severe_alerts.features.iter().enumerate() {
                if let Some(alert_props) = &alert_feature.properties {
                    println!("    {}. {} - {}",
                        index + 1,
                        alert_props.event.as_deref().unwrap_or("Unknown"),
                        alert_props.severity.as_ref().map(|s| s.to_string()).unwrap_or("Unknown severity".to_string())
                    );
                }
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting severe alerts: {}", error);
        }
    }

    // 5. Get specific alert by ID (if any alerts exist)
    println!("\n5️⃣ Getting detailed information for a specific alert...");
    match alerts::get_active_alerts(&config, None, None, None, Some(1), None, None, None).await {
        Ok(any_alerts) => {
            if let Some(first_alert) = any_alerts.features.first() {
                if let Some(alert_id) = &first_alert.properties.as_ref().and_then(|p| p.id.as_ref()) {
                    println!("  📋 Getting details for alert: {}", alert_id);

                    match alerts::get_alert(&config, alert_id).await {
                        Ok(detailed_alert) => {
                            if let Some(props) = &detailed_alert.properties {
                                println!("    ✅ Alert Details:");
                                println!("    📢 Event: {}", props.event.as_deref().unwrap_or("Unknown"));
                                println!("    📤 Sender: {}", props.sender_name.as_deref().unwrap_or("Unknown"));

                                if let Some(description) = &props.description {
                                    let short_desc = if description.len() > 200 {
                                        format!("{}...", &description[..200])
                                    } else {
                                        description.clone()
                                    };
                                    println!("    📝 Description: {}", short_desc);
                                }

                                if let Some(instruction) = &props.instruction {
                                    let short_instruction = if instruction.len() > 200 {
                                        format!("{}...", &instruction[..200])
                                    } else {
                                        instruction.clone()
                                    };
                                    println!("    📋 Instructions: {}", short_instruction);
                                }
                            }
                        }
                        Err(error) => {
                            eprintln!("    ❌ Error getting alert details: {}", error);
                        }
                    }
                }
            } else {
                println!("  ℹ️  No alerts currently available for detailed lookup");
            }
        }
        Err(error) => {
            eprintln!("  ❌ Error getting sample alert: {}", error);
        }
    }

    println!("\n✨ Weather alerts example completed!");
    println!("\n💡 Pro tip: You can filter alerts by:");
    println!("   - Area (state/territory code like 'CA', 'TX')");
    println!("   - Zone (specific forecast zone like 'CAZ006')");
    println!("   - Severity (Minor, Moderate, Severe, Extreme)");
    println!("   - Message type (Alert, Update, Cancel)");

    Ok(())
}
