use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::Subcommand;
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::{self, AreaCode, MarineAreaCode, StateTerritoryCode};
use serde_json::Value;

// Helper function to parse a Vec<String> into a Vec of T: FromStr
fn parse_vec_from_str<T: FromStr>(items: Option<Vec<String>>) -> Result<Option<Vec<T>>>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    items
        .map(|strings| {
            strings
                .into_iter()
                .map(|s| T::from_str(&s).map_err(|e| anyhow!("Invalid input '{}': {}", s, e)))
                .collect::<Result<Vec<T>>>()
        })
        .transpose()
}

#[derive(Subcommand, Debug)]
pub enum AlertCommands {
    /// List active alerts
    Active {
        /// Filter by status
        #[arg(long)]
        status: Option<Vec<String>>,

        /// Filter by message type
        #[arg(long)]
        message_type: Option<Vec<String>>,

        /// Filter by event type
        #[arg(long)]
        event: Option<Vec<String>>,

        /// Filter by alert code
        #[arg(long)]
        code: Option<Vec<String>>,

        /// Filter by area code (State/Territory or Marine Area)
        #[arg(long, value_delimiter = ',')]
        area: Option<Vec<String>>,

        /// Filter by point (latitude,longitude)
        #[arg(long)]
        point: Option<String>,

        /// Filter by marine region code (e.g., AL, AT, GL)
        #[arg(long, value_delimiter = ',')]
        region: Option<Vec<String>>,

        /// Filter by region type (land or marine)
        #[arg(long)]
        region_type: Option<String>,

        /// Filter by zone ID (Public or County)
        #[arg(long, value_delimiter = ',')]
        zone: Option<Vec<String>>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown)
        #[arg(long, value_delimiter = ',')]
        urgency: Option<Vec<String>>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown)
        #[arg(long, value_delimiter = ',')]
        severity: Option<Vec<String>>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown)
        #[arg(long, value_delimiter = ',')]
        certainty: Option<Vec<String>>,

        /// Limit number of results
        #[arg(long)]
        limit: Option<i32>,
    },

    /// Get active alerts for a specific area (State/Territory or Marine Area)
    Area {
        /// Area code to get alerts for
        area: String,
    },

    /// Get count of active alerts
    Count,

    /// Get active alerts for a marine region
    Region {
        /// Marine region code (AL, AT, GL, GM, PA, PI)
        region: String,
    },

    /// Get active alerts for a zone (Public Zone or County)
    Zone {
        /// Zone ID to get alerts for
        zone_id: String,
    },

    /// Query alerts with various filters
    Query {
        /// Include active alerts only
        #[arg(long)]
        active: Option<bool>,

        /// Start time for query (ISO 8601 format)
        #[arg(long)]
        start: Option<String>,

        /// End time for query (ISO 8601 format)
        #[arg(long)]
        end: Option<String>,

        /// Filter by status
        #[arg(long, value_delimiter = ',')]
        status: Option<Vec<String>>,

        /// Filter by message type
        #[arg(long, value_delimiter = ',')]
        message_type: Option<Vec<String>>,

        /// Filter by event type
        #[arg(long, value_delimiter = ',')]
        event: Option<Vec<String>>,

        /// Filter by alert code
        #[arg(long, value_delimiter = ',')]
        code: Option<Vec<String>>,

        /// Filter by area code (State/Territory or Marine Area)
        #[arg(long, value_delimiter = ',')]
        area: Option<Vec<String>>,

        /// Filter by point (latitude,longitude)
        #[arg(long)]
        point: Option<String>,

        /// Filter by marine region code (e.g., AL, AT, GL)
        #[arg(long, value_delimiter = ',')]
        region: Option<Vec<String>>,

        /// Filter by region type (land or marine)
        #[arg(long)]
        region_type: Option<String>,

        /// Filter by zone ID (Public or County)
        #[arg(long, value_delimiter = ',')]
        zone: Option<Vec<String>>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown)
        #[arg(long, value_delimiter = ',')]
        urgency: Option<Vec<String>>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown)
        #[arg(long, value_delimiter = ',')]
        severity: Option<Vec<String>>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown)
        #[arg(long, value_delimiter = ',')]
        certainty: Option<Vec<String>>,

        /// Limit number of results
        #[arg(long)]
        limit: Option<i32>,

        /// Cursor for pagination
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Get a single alert by ID
    Get {
        /// Alert ID to retrieve
        id: String,
    },

    /// List available alert types
    Types,
}

// Helper function specifically for parsing AreaCode which can be one of two types
fn parse_area_codes(area_codes: Option<Vec<String>>) -> Result<Option<Vec<AreaCode>>> {
    area_codes
        .map(|codes| {
            codes
                .into_iter()
                .map(|code| {
                    // Try parsing as StateTerritoryCode first
                    StateTerritoryCode::from_str(&code)
                        .map(AreaCode::StateTerritoryCode)
                        .or_else(|_| MarineAreaCode::from_str(&code).map(AreaCode::MarineAreaCode))
                        .map_err(|_| anyhow!("Invalid area code: {}", code))
                })
                .collect::<Result<Vec<_>>>()
        })
        .transpose()
}

pub async fn handle_command(command: AlertCommands, config: &Configuration) -> Result<Value> {
    match command {
        AlertCommands::Active {
            status,
            message_type,
            event,
            code,
            area,
            point,
            region,
            region_type,
            zone,
            urgency,
            severity,
            certainty,
            limit,
        } => {
            let area_parsed = parse_area_codes(area)?;
            let region_parsed = parse_vec_from_str::<models::MarineRegionCode>(region)?;
            let urgency_parsed = parse_vec_from_str::<models::AlertUrgency>(urgency)?;
            let severity_parsed = parse_vec_from_str::<models::AlertSeverity>(severity)?;
            let certainty_parsed = parse_vec_from_str::<models::AlertCertainty>(certainty)?;

            let result = noaa_weather_client::apis::alerts::alerts_active(
                config,
                status,
                message_type,
                event,
                code,
                area_parsed,
                point.as_deref(),
                region_parsed,
                region_type.as_deref(),
                zone,
                urgency_parsed,
                severity_parsed,
                certainty_parsed,
                limit,
            )
            .await
            .map_err(|e| anyhow!("Error fetching active alerts: {}", e))?;

            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Area { area } => {
            let result = noaa_weather_client::apis::alerts::alerts_active_area(config, &area)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for area {}: {}", area, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Count => {
            let result = noaa_weather_client::apis::alerts::alerts_active_count(config)
                .await
                .map_err(|e| anyhow!("Error fetching active alert count: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Region { region } => {
            let region_parsed = models::MarineRegionCode::from_str(&region)
                .map_err(|e| anyhow!("Invalid marine region code '{}': {}", region, e))?;
            let result =
                noaa_weather_client::apis::alerts::alerts_active_region(config, region_parsed)
                    .await
                    .map_err(|e| anyhow!("Error fetching alerts for region {}: {}", region, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Zone { zone_id } => {
            let result = noaa_weather_client::apis::alerts::alerts_active_zone(config, &zone_id)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for zone {}: {}", zone_id, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Query {
            active,
            start,
            end,
            status,
            message_type,
            event,
            code,
            area,
            point,
            region,
            region_type,
            zone,
            urgency,
            severity,
            certainty,
            limit,
            cursor,
        } => {
            let area_parsed = parse_area_codes(area)?;
            let region_parsed = parse_vec_from_str::<models::MarineRegionCode>(region)?;
            let urgency_parsed = parse_vec_from_str::<models::AlertUrgency>(urgency)?;
            let severity_parsed = parse_vec_from_str::<models::AlertSeverity>(severity)?;
            let certainty_parsed = parse_vec_from_str::<models::AlertCertainty>(certainty)?;

            let result = noaa_weather_client::apis::alerts::alerts_query(
                config,
                active,
                start,
                end,
                status,
                message_type,
                event,
                code,
                area_parsed,
                point.as_deref(),
                region_parsed,
                region_type.as_deref(),
                zone,
                urgency_parsed,
                severity_parsed,
                certainty_parsed,
                limit,
                cursor.as_deref(),
            )
            .await
            .map_err(|e| anyhow!("Error querying alerts: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Get { id } => {
            let result = noaa_weather_client::apis::alerts::alerts_single(config, &id)
                .await
                .map_err(|e| anyhow!("Error getting alert {}: {}", id, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Types => {
            let result = noaa_weather_client::apis::alerts::alerts_types(config)
                .await
                .map_err(|e| anyhow!("Error fetching alert types: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
