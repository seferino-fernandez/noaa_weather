use std::str::FromStr;

use anyhow::{Result, anyhow};
use clap::Subcommand;
use noaa_weather_client::apis::alerts as alerts_api;
use noaa_weather_client::apis::alerts::{ActiveAlertsParams, GetAlertsParams};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::{
    self, AlertCertainty, AlertSeverity, AlertUrgency, MarineRegionCode,
};

use crate::utils::format::write_output;
use crate::utils::parse::{parse_area_codes, parse_string_args_into_vec};
use crate::{Cli, tables};

/// Subcommands for interacting with the NWS Alerts API.
#[derive(Subcommand, Debug, Clone)]
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
/// * `cli`: The CLI arguments.
/// * `config`: The API client configuration.
///
pub async fn handle_command(
    command: &AlertCommands,
    cli: Cli,
    config: &Configuration,
) -> Result<()> {
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
            let area_parsed = parse_area_codes(area.clone())?;
            let region_parsed = parse_string_args_into_vec::<MarineRegionCode>(region.clone())?;
            let urgency_parsed = parse_string_args_into_vec::<AlertUrgency>(urgency.clone())?;
            let severity_parsed = parse_string_args_into_vec::<AlertSeverity>(severity.clone())?;
            let certainty_parsed = parse_string_args_into_vec::<AlertCertainty>(certainty.clone())?;

            let params = ActiveAlertsParams {
                status: status.clone(),
                message_type: message_type.clone(),
                event: event.clone(),
                code: code.clone(),
                area: area_parsed,
                point: point.as_deref(),
                region: region_parsed,
                region_type: region_type.as_deref(),
                zone: zone.clone(),
                urgency: urgency_parsed,
                severity: severity_parsed,
                certainty: certainty_parsed,
                limit: *limit,
            };

            let result = alerts_api::get_active_alerts(config, params)
                .await
                .map_err(|e| anyhow!("Error fetching active alerts: {}", e))?;

            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_alerts_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AlertCommands::Area { area } => {
            let result = alerts_api::get_active_alerts_for_area(config, area)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for area {}: {}", area, e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_alerts_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AlertCommands::Count => {
            let result = alerts_api::get_active_alerts_count(config)
                .await
                .map_err(|e| anyhow!("Error fetching active alert count: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_alert_count_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AlertCommands::Region { region } => {
            let region_parsed = models::MarineRegionCode::from_str(region)
                .map_err(|e| anyhow!("Invalid marine region code '{}': {}", region, e))?;
            let result = alerts_api::get_active_alerts_for_region(config, region_parsed)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for region {}: {}", region, e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_alerts_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AlertCommands::Zone { zone_id } => {
            let result = alerts_api::get_active_alerts_for_zone(config, zone_id)
                .await
                .map_err(|e| anyhow!("Error fetching alerts for zone {}: {}", zone_id, e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_alerts_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
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
            let area_parsed = parse_area_codes(area.clone())?;
            let region_parsed =
                parse_string_args_into_vec::<models::MarineRegionCode>(region.clone())?;
            let urgency_parsed =
                parse_string_args_into_vec::<models::AlertUrgency>(urgency.clone())?;
            let severity_parsed =
                parse_string_args_into_vec::<models::AlertSeverity>(severity.clone())?;
            let certainty_parsed =
                parse_string_args_into_vec::<models::AlertCertainty>(certainty.clone())?;

            let params = GetAlertsParams {
                active: *active,
                start: start.clone(),
                end: end.clone(),
                status: status.clone(),
                message_type: message_type.clone(),
                event: event.clone(),
                code: code.clone(),
                area: area_parsed,
                point: point.as_deref(),
                region: region_parsed,
                region_type: region_type.as_deref(),
                zone: zone.clone(),
                urgency: urgency_parsed,
                severity: severity_parsed,
                certainty: certainty_parsed,
                limit: *limit,
                cursor: cursor.as_deref(),
            };

            let result = alerts_api::get_alerts(config, params)
                .await
                .map_err(|e| anyhow!("Error querying alerts: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_alerts_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AlertCommands::Alert { id } => {
            let result = alerts_api::get_alert(config, id)
                .await
                .map_err(|e| anyhow!("Error getting alert {}: {}", id, e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_single_alert_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
        AlertCommands::Types => {
            let result = alerts_api::get_alert_types(config)
                .await
                .map_err(|e| anyhow!("Error fetching alert types: {}", e))?;
            if cli.json {
                write_output(
                    cli.output.as_deref(),
                    &serde_json::to_string_pretty(&result)?,
                )?;
            } else {
                let table = tables::alerts::create_alert_types_table(&result)?;
                write_output(cli.output.as_deref(), &table.to_string())?;
            }
            Ok(())
        }
    }
}
