use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::Subcommand;
use noaa_weather_client::apis::alerts as alerts_api;
use noaa_weather_client::apis::alerts::{ActiveAlertsParams, GetAlertsParams};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::{
    self, AlertCertainty, AlertSeverity, AlertUrgency, MarineRegionCode,
};
use serde_json::Value;

use crate::utils::parse::{parse_area_codes, parse_string_args_into_vec};

/// Subcommands for interacting with the NWS Alerts API.
#[derive(Subcommand, Debug)]
pub enum AlertCommands {
    /// List active alerts, optionally filtering by various criteria.
    ///
    /// Fetches currently active alerts from the NWS API. You can filter results
    /// based on status, type, location, severity, and more.
    Active {
        /// Filter by status (e.g., actual, exercise, test).
        #[arg(long)]
        status: Option<Vec<String>>,

        /// Filter by message type (alert, update, cancel).
        #[arg(long)]
        message_type: Option<Vec<String>>,

        /// Filter by event type (e.g., "Tornado Warning", "Flood Watch").
        #[arg(long)]
        event: Option<Vec<String>>,

        /// Filter by alert code (NWS public zone/county or SAME code).
        #[arg(long)]
        code: Option<Vec<String>>,

        /// Filter by area code (State/Territory or Marine Area, comma-separated).
        #[arg(long, value_delimiter = ',')]
        area: Option<Vec<String>>,

        /// Filter by point (latitude,longitude).
        #[arg(long)]
        point: Option<String>,

        /// Filter by marine region code (e.g., AL, AT, GL, comma-separated).
        #[arg(long, value_delimiter = ',')]
        region: Option<Vec<String>>,

        /// Filter by region type (land or marine).
        #[arg(long)]
        region_type: Option<String>,

        /// Filter by zone ID (NWS Public Zone or County, comma-separated).
        #[arg(long, value_delimiter = ',')]
        zone: Option<Vec<String>>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        urgency: Option<Vec<String>>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        severity: Option<Vec<String>>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        certainty: Option<Vec<String>>,

        /// Limit number of results returned by the API.
        #[arg(long)]
        limit: Option<i32>,
    },

    /// Get active alerts for a specific area (State/Territory or Marine Area).
    ///
    /// Example: `noaa-weather alerts area CA` or `noaa-weather alerts area GM`
    Area {
        /// The state/territory abbreviation or marine area code (e.g., "AL", "GM").
        area: String,
    },

    /// Get the total count of active alerts, optionally summarized.
    Count,

    /// Get active alerts for a specific marine region.
    ///
    /// Example: `noaa-weather alerts region AT`
    Region {
        /// Marine region code (AL, AT, GL, GM, PA, PI).
        region: String,
    },

    /// Get active alerts for a specific NWS zone (Public Zone or County).
    ///
    /// Example: `noaa-weather alerts zone CAZ043`
    Zone {
        /// Zone ID (e.g., "CAZ043", "CAC073") to get alerts for.
        zone_id: String,
    },

    /// List alerts (active or inactive) with various filters and pagination.
    ///
    /// Queries the NWS API for alerts, allowing filtering by time range,
    /// status, location, and other criteria. Supports pagination using `--cursor`.
    List {
        /// Include active alerts only (if set, overrides start/end times).
        #[arg(long)]
        active: Option<bool>,

        /// Start time for query (ISO 8601 format, e.g., "2023-10-26T14:00:00Z").
        #[arg(long)]
        start: Option<String>,

        /// End time for query (ISO 8601 format).
        #[arg(long)]
        end: Option<String>,

        /// Filter by status (e.g., actual, exercise, test, comma-separated).
        #[arg(long, value_delimiter = ',')]
        status: Option<Vec<String>>,

        /// Filter by message type (alert, update, cancel, comma-separated).
        #[arg(long, value_delimiter = ',')]
        message_type: Option<Vec<String>>,

        /// Filter by event type (e.g., "Tornado Warning", comma-separated).
        #[arg(long, value_delimiter = ',')]
        event: Option<Vec<String>>,

        /// Filter by alert code (NWS public zone/county or SAME code, comma-separated).
        #[arg(long, value_delimiter = ',')]
        code: Option<Vec<String>>,

        /// Filter by area code (State/Territory or Marine Area, comma-separated).
        #[arg(long, value_delimiter = ',')]
        area: Option<Vec<String>>,

        /// Filter by point (latitude,longitude).
        #[arg(long)]
        point: Option<String>,

        /// Filter by marine region code (e.g., AL, AT, GL, comma-separated).
        #[arg(long, value_delimiter = ',')]
        region: Option<Vec<String>>,

        /// Filter by region type (land or marine).
        #[arg(long)]
        region_type: Option<String>,

        /// Filter by zone ID (Public or County, comma-separated).
        #[arg(long, value_delimiter = ',')]
        zone: Option<Vec<String>>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        urgency: Option<Vec<String>>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        severity: Option<Vec<String>>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        certainty: Option<Vec<String>>,

        /// Limit number of results returned by the API.
        #[arg(long)]
        limit: Option<i32>,

        /// Cursor for pagination (obtained from previous query results).
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Get a single alert by its unique NWS ID.
    ///
    /// Example: `noaa-weather alerts alert NWS-ID-XYZ123`
    Alert {
        /// Unique Alert ID (e.g., "NWS-TOW-ORD-201809141900").
        id: String,
    },

    /// List available alert event types recognized by the NWS API.
    Types,
}

/// Handles the dispatch of alert-related subcommands.
///
/// Takes a parsed `AlertCommands` enum variant and the API `Configuration`,
/// calls the appropriate `noaa_weather_client` function, and returns the
/// result as a `serde_json::Value`.
///
/// # Arguments
///
/// * `command`: The specific alert subcommand to execute.
/// * `config`: The API client configuration.
///
/// # Returns
///
/// A `Result` containing the JSON response from the API on success,
/// or an `anyhow::Error` if an error occurs during parsing, API call, or
/// result serialization.
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
            let region_parsed = parse_string_args_into_vec::<MarineRegionCode>(region)?;
            let urgency_parsed = parse_string_args_into_vec::<AlertUrgency>(urgency)?;
            let severity_parsed = parse_string_args_into_vec::<AlertSeverity>(severity)?;
            let certainty_parsed = parse_string_args_into_vec::<AlertCertainty>(certainty)?;

            let params = ActiveAlertsParams {
                status,
                message_type,
                event,
                code,
                area: area_parsed,
                point: point.as_deref(),
                region: region_parsed,
                region_type: region_type.as_deref(),
                zone,
                urgency: urgency_parsed,
                severity: severity_parsed,
                certainty: certainty_parsed,
                limit,
            };

            let result = alerts_api::get_active_alerts(config, params)
                .await
                .map_err(|e| anyhow!("Error fetching active alerts: {}", e))?;

            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Area { area } => {
            let result = alerts_api::get_active_alerts_for_area(config, &area)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for area {}: {}", area, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Count => {
            let result = alerts_api::get_active_alerts_count(config)
                .await
                .map_err(|e| anyhow!("Error fetching active alert count: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Region { region } => {
            let region_parsed = models::MarineRegionCode::from_str(&region)
                .map_err(|e| anyhow!("Invalid marine region code '{}': {}", region, e))?;
            let result = alerts_api::get_active_alerts_for_region(config, region_parsed)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for region {}: {}", region, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Zone { zone_id } => {
            let result = alerts_api::get_active_alerts_for_zone(config, &zone_id)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for zone {}: {}", zone_id, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::List {
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
            let region_parsed = parse_string_args_into_vec::<models::MarineRegionCode>(region)?;
            let urgency_parsed = parse_string_args_into_vec::<models::AlertUrgency>(urgency)?;
            let severity_parsed = parse_string_args_into_vec::<models::AlertSeverity>(severity)?;
            let certainty_parsed = parse_string_args_into_vec::<models::AlertCertainty>(certainty)?;

            let params = GetAlertsParams {
                active,
                start,
                end,
                status,
                message_type,
                event,
                code,
                area: area_parsed,
                point: point.as_deref(),
                region: region_parsed,
                region_type: region_type.as_deref(),
                zone,
                urgency: urgency_parsed,
                severity: severity_parsed,
                certainty: certainty_parsed,
                limit,
                cursor: cursor.as_deref(),
            };

            let result = alerts_api::get_alerts(config, params)
                .await
                .map_err(|e| anyhow!("Error querying alerts: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Alert { id } => {
            let result = alerts_api::get_alert(config, &id)
                .await
                .map_err(|e| anyhow!("Error getting alert {}: {}", id, e))?;
            Ok(serde_json::to_value(result)?)
        }
        AlertCommands::Types => {
            let result = alerts_api::get_alert_types(config)
                .await
                .map_err(|e| anyhow!("Error fetching alert types: {}", e))?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
