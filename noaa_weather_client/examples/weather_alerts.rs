//! Weather Alerts Example
//!
//! This example demonstrates how to:
//! - Get active alerts with various filters
//! - Get alerts for specific areas and zones
//! - Display alert details in a user-friendly format
//!
//! Run with: just example-alerts
//! Or: cargo run --example weather_alerts --manifest-path noaa_weather_client/Cargo.toml

use noaa_weather_client::apis::alerts::ActiveAlertsQuery;
use noaa_weather_client::models::{AlertSeverity, AreaCode, StateTerritoryCode};
use noaa_weather_client::{AlertId, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(
        "noaa-weather-examples/1.0 (+https://github.com/seferino-fernandez/noaa_weather)",
    )
    .build()?;
    let alerts = client.alerts();

    println!("NOAA Weather Alerts Example\n");

    // 1. Get count of all active alerts
    println!("[1] Getting alert count...");
    match alerts.active_count().await {
        Ok(counts) => {
            println!("  Total active alerts: {}", counts.total);
            println!("  Land alerts: {}", counts.land);
            println!("  Marine alerts: {}", counts.marine);
            println!("  States/territories with alerts: {}", counts.areas.len());
        }
        Err(error) => {
            eprintln!("  Error getting alert count: {}", error);
        }
    }

    // 2. Get alerts for California
    println!("\n[2] Getting alerts for California...");
    match alerts
        .active_for_area(&AreaCode::StateTerritoryCode(StateTerritoryCode::Ca))
        .await
    {
        Ok(ca_alerts) => {
            println!("  Found {} alerts for California", ca_alerts.len());

            for (index, alert) in ca_alerts.iter().take(3).enumerate() {
                println!("\n    Alert #{}", index + 1);
                println!("    Event: {}", alert.event);
                println!(
                    "    Headline: {}",
                    alert.headline.as_deref().unwrap_or("No headline")
                );
                println!("    Areas: {}", alert.area_desc);
                println!("    Severity: {}", alert.severity);
                println!("    Urgency: {}", alert.urgency);
                // `expires` is an OffsetDateTime: it prints with the offset
                // NOAA sent, such as 2026-09-02T04:45:00-07:00.
                println!("    Expires: {}", alert.expires);
                let zones: Vec<String> = alert
                    .affected_zone_ids()
                    .map(|zone| zone.to_string())
                    .collect();
                println!("    Zones: {}", zones.join(", "));
            }
        }
        Err(error) => {
            eprintln!("  Error getting California alerts: {}", error);
        }
    }

    // 3. Get available alert types
    println!("\n[3] Getting available alert types...");
    match alerts.types().await {
        Ok(types) => {
            let event_types = &types.event_types;
            println!("  Available alert types ({} total):", event_types.len());
            for alert_type in event_types.iter().take(10) {
                println!("    - {}", alert_type);
            }
            if event_types.len() > 10 {
                println!("    ... and {} more", event_types.len() - 10);
            }
        }
        Err(error) => {
            eprintln!("  Error getting alert types: {}", error);
        }
    }

    // 4. Get alerts with specific filters. Unset filters stay absent from
    //    the request thanks to struct-update syntax.
    println!("\n[4] Getting high-severity alerts...");
    let severity_query = ActiveAlertsQuery {
        severity: vec![
            AlertSeverity::Minor,
            AlertSeverity::Moderate,
            AlertSeverity::Severe,
            AlertSeverity::Extreme,
        ],
        ..Default::default()
    };
    match alerts.active(&severity_query).await {
        Ok(severe_alerts) => {
            println!("  Found {} high-severity alerts", severe_alerts.len());

            for (index, alert) in severe_alerts.iter().enumerate() {
                println!("    {}. {} - {}", index + 1, alert.event, alert.severity);
            }
        }
        Err(error) => {
            eprintln!("  Error getting severe alerts: {}", error);
        }
    }

    // 5. Get specific alert by ID (if any alerts exist)
    println!("\n[5] Getting detailed information for a specific alert...");
    match alerts.active(&ActiveAlertsQuery::default()).await {
        Ok(any_alerts) => {
            // `feature.id` is the NOAA self-link URL; the URN NOAA accepts
            // in `/alerts/{id}` is the typed `AlertId` inside `properties`.
            let first_id: Option<&AlertId> = any_alerts
                .features
                .first()
                .map(|alert| &alert.properties.id);
            match first_id {
                Some(alert_id) => {
                    println!("  Getting details for alert: {}", alert_id);

                    match alerts.get(alert_id).await {
                        Ok(detailed_alert) => {
                            let props = &detailed_alert.properties;
                            println!("    Alert Details:");
                            println!("    Event: {}", props.event);
                            println!("    Sender: {}", props.sender_name);
                            println!("    Sent: {}", props.sent);

                            if let Some(description) = &props.description {
                                let short_desc = if description.len() > 200 {
                                    format!("{}...", &description[..200])
                                } else {
                                    description.clone()
                                };
                                println!("    Description: {}", short_desc);
                            }

                            if let Some(instruction) = &props.instruction {
                                let short_instruction = if instruction.len() > 200 {
                                    format!("{}...", &instruction[..200])
                                } else {
                                    instruction.clone()
                                };
                                println!("    Instructions: {}", short_instruction);
                            }

                            if let Some(replaced_by) = &props.replaced_by {
                                println!("    Replaced by: {}", replaced_by);
                            }
                        }
                        Err(error) => {
                            eprintln!("    Error getting alert details: {}", error);
                        }
                    }
                }
                None => println!("  No alerts currently available for detailed lookup"),
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
