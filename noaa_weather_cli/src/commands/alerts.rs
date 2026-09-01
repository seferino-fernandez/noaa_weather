use anyhow::Result;
use clap::Subcommand;
use noaa_weather_client::Client;
use noaa_weather_client::apis::alerts as alerts_api;
use noaa_weather_client::apis::alerts::{ActiveAlertsParams, GetAlertsParams};
use noaa_weather_client::models::{
    AlertCertainty, AlertMessageType, AlertSeverity, AlertStatus, AlertUrgency, AreaCode,
    MarineRegionCode, RegionType,
};

use crate::output::Output;

/// Subcommands for interacting with the NWS Alerts API.
#[derive(Subcommand, Debug, Clone)]
pub enum AlertCommands {
    /// List active alerts, optionally filtering by various criteria.
    ///
    /// Fetches currently active alerts from the NWS API. You can filter results
    /// based on status, type, location, severity, and more.
    Active {
        /// Filter by alert status (actual, exercise, system, test, draft).
        #[arg(long, value_delimiter = ',', value_enum)]
        status: Option<Vec<AlertStatus>>,

        /// Filter by alert message type (alert, update, cancel).
        #[arg(long, value_delimiter = ',', value_enum)]
        message_type: Option<Vec<AlertMessageType>>,

        /// Filter by alert event type (e.g., "Tornado Warning", "Flood Watch").
        #[arg(long)]
        event: Option<Vec<String>>,

        /// Filter by alert code (NWS public zone/county or SAME code).
        #[arg(long, value_delimiter = ',')]
        code: Option<Vec<String>>,

        /// Filter by area code (State/Territory or Marine Area, comma-separated).
        /// This parameter is incompatible with the following parameters: point, marine-region, region-type, zone.
        #[arg(long, value_delimiter = ',', value_enum)]
        area: Option<Vec<AreaCode>>,

        /// Filter by point (latitude,longitude).
        /// This parameter is incompatible with the following parameters: area, marine-region, region-type, zone.
        #[arg(long)]
        point: Option<String>,

        /// Filter by marine region code (AL, AT, GL, GM, PA, PI).
        /// This parameter is incompatible with the following parameters: area, point, region-type, zone
        #[arg(long, value_delimiter = ',', value_enum)]
        marine_region: Option<Vec<MarineRegionCode>>,

        /// Filter by region type (land or marine).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, zone.
        #[arg(long, value_enum)]
        region_type: Option<RegionType>,

        /// Filter by Zone ID (forecast or county).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, region-type
        #[arg(long, value_delimiter = ',')]
        zone: Option<Vec<String>>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',', value_enum)]
        urgency: Option<Vec<AlertUrgency>>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',', value_enum)]
        severity: Option<Vec<AlertSeverity>>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',', value_enum)]
        certainty: Option<Vec<AlertCertainty>>,
    },

    /// Get active alerts for a specific area (State/Territory or Marine Area).
    ///
    /// Example: `noaa-weather alerts area --area CA` or `noaa-weather alerts area --area GM`
    Area {
        /// The state/territory abbreviation or marine area code (e.g., "AL", "GM").
        #[arg(long, value_enum)]
        area: AreaCode,
    },

    /// Get the total count of active alerts, optionally summarized.
    Count,

    /// Get active alerts for a specific marine region.
    ///
    /// Marine region codes:
    ///  - AL: Alaska waters
    ///  - AT: Atlantic Ocean
    ///  - GL: Great Lakes
    ///  - GM: Gulf of Mexico
    ///  - PA: Eastern Pacific Ocean and U.S. West Coast
    ///  - PI: Central and Western Pacific
    ///
    /// Example: `noaa-weather alerts marine-region --marine-region AT`
    MarineRegion {
        /// Marine region code (AL, AT, GL, GM, PA, PI).
        #[arg(long, value_enum)]
        marine_region: MarineRegionCode,
    },

    /// Get active alerts for a specific NWS zone (Public Zone or County).
    ///
    /// Example: `noaa-weather alerts zone --zone-id CAZ043`
    Zone {
        /// Zone ID (e.g., "CAZ043", "CAC073") to get alerts for.
        #[arg(long)]
        zone_id: String,
    },

    /// List alerts with various filters and pagination.
    ///
    /// Queries the NWS API for alerts, allowing filtering by time range,
    /// status, location, and other criteria.
    List {
        /// Start time for query (ISO 8601 format, e.g., "2023-10-26T14:00:00Z").
        #[arg(long)]
        start: Option<String>,

        /// End time for query (ISO 8601 format).
        #[arg(long)]
        end: Option<String>,

        /// Filter by alert status (actual, exercise, test, draft).
        #[arg(long, value_delimiter = ',', value_enum)]
        status: Option<Vec<AlertStatus>>,

        /// Filter by alert message type (alert, update, cancel).
        #[arg(long, value_delimiter = ',', value_enum)]
        message_type: Option<Vec<AlertMessageType>>,

        /// Filter by alert event type (e.g., "Tornado Warning").
        #[arg(long, value_delimiter = ',')]
        event: Option<Vec<String>>,

        /// Filter by alert code (NWS public zone/county or SAME code, comma-separated).
        #[arg(long, value_delimiter = ',')]
        code: Option<Vec<String>>,

        /// Filter by area code (State/Territory or Marine Area, comma-separated).
        /// This parameter is incompatible with the following parameters: point, marine-region, region-type, zone
        #[arg(long, value_delimiter = ',', value_enum)]
        area: Option<Vec<AreaCode>>,

        /// Filter by point (latitude,longitude).
        /// This parameter is incompatible with the following parameters: area, marine-region, region-type, zone
        #[arg(long)]
        point: Option<String>,

        /// Filter by marine region code (e.g., AL, AT, GL, comma-separated).
        /// This parameter is incompatible with the following parameters: area, point, region-type, zone
        #[arg(long, value_delimiter = ',', value_enum)]
        marine_region: Option<Vec<MarineRegionCode>>,

        /// Filter by region type (land or marine).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, zone
        #[arg(long, value_enum)]
        region_type: Option<RegionType>,

        /// Filter by Zone ID (forecast or county).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, region-type
        #[arg(long, value_delimiter = ',')]
        zone: Option<Vec<String>>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',', value_enum)]
        urgency: Option<Vec<AlertUrgency>>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',', value_enum)]
        severity: Option<Vec<AlertSeverity>>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',', value_enum)]
        certainty: Option<Vec<AlertCertainty>>,

        /// Limit number of results returned by the API.
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=500))]
        limit: Option<i32>,
    },

    /// Get a single alert by its unique NWS ID.
    ///
    /// Example: `noaa-weather alerts alert --id urn:oid:2.49.0.1.840.0.dcc6cd9527d1f8732519ea87f13d3810e9ef672c.001.1`
    Alert {
        /// Unique Alert ID (e.g., "urn:oid:2.49.0.1.840.0.dcc6cd9527d1f8732519ea87f13d3810e9ef672c.001.1").
        #[arg(long)]
        id: String,
    },

    /// List available alert event types recognized by the NWS API.
    Types,
}

/// Handles the dispatch of alert-related subcommands.
///
/// Takes a parsed `AlertCommands` enum variant and the API [`Client`],
/// calls the appropriate `noaa_weather_client` function, and returns the
/// result as a `serde_json::Value`.
///
/// # Arguments
///
/// * `command`: The specific alert subcommand to execute.
/// * `output`: The configured output policy.
/// * `client`: The NOAA API client.
///
pub async fn handle_command(
    command: &AlertCommands,
    output: &Output,
    client: &Client,
) -> Result<()> {
    match command {
        AlertCommands::Active {
            status,
            message_type,
            event,
            code,
            area,
            point,
            marine_region,
            region_type,
            zone,
            urgency,
            severity,
            certainty,
        } => {
            let params = ActiveAlertsParams {
                status: status.clone(),
                message_type: message_type.clone(),
                event: event.clone(),
                code: code.clone(),
                area: area.clone(),
                point: point.as_deref(),
                region: marine_region.clone(),
                region_type: *region_type,
                zone: zone.clone(),
                urgency: urgency.clone(),
                severity: severity.clone(),
                certainty: certainty.clone(),
            };

            output
                .show(
                    "fetching active alerts",
                    alerts_api::get_active_alerts(client, params),
                )
                .await
        }
        AlertCommands::Area { area } => {
            output
                .show(
                    format!("fetching active alerts for area {area}"),
                    alerts_api::get_active_alerts_for_area(client, area),
                )
                .await
        }
        AlertCommands::Count => {
            output
                .show(
                    "fetching active alert count",
                    alerts_api::get_active_alerts_count(client),
                )
                .await
        }
        AlertCommands::MarineRegion { marine_region } => {
            output
                .show(
                    format!("fetching active alerts for marine region {marine_region}"),
                    alerts_api::get_active_alerts_for_marine_region(client, *marine_region),
                )
                .await
        }
        AlertCommands::Zone { zone_id } => {
            output
                .show(
                    format!("fetching active alerts for zone {zone_id}"),
                    alerts_api::get_active_alerts_for_zone(client, zone_id),
                )
                .await
        }
        AlertCommands::List {
            start,
            end,
            status,
            message_type,
            event,
            code,
            area,
            point,
            marine_region,
            region_type,
            zone,
            urgency,
            severity,
            certainty,
            limit,
        } => {
            let params = GetAlertsParams {
                start: start.clone(),
                end: end.clone(),
                status: status.clone(),
                message_type: message_type.clone(),
                event: event.clone(),
                code: code.clone(),
                area: area.clone(),
                point: point.as_deref(),
                region: marine_region.clone(),
                region_type: *region_type,
                zone: zone.clone(),
                urgency: urgency.clone(),
                severity: severity.clone(),
                certainty: certainty.clone(),
                limit: *limit,
                cursor: None,
            };

            output
                .show("querying alerts", alerts_api::get_alerts(client, params))
                .await
        }
        AlertCommands::Alert { id } => {
            output
                .show(
                    format!("getting alert {id}"),
                    alerts_api::get_alert(client, id),
                )
                .await
        }
        AlertCommands::Types => {
            output
                .show("fetching alert types", alerts_api::get_alert_types(client))
                .await
        }
    }
}
