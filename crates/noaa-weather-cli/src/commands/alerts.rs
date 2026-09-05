use anyhow::Result;
use clap::Subcommand;
use jiff::Timestamp;
use noaa_weather_client::apis::alerts::{ActiveAlertsQuery, AlertsQuery, RegionType};
use noaa_weather_client::models::{
    AlertCertainty, AlertMessageType, AlertSeverity, AlertStatus, AlertUrgency, AreaCode,
    MarineRegionCode,
};
use noaa_weather_client::{AlertId, Client, Coordinates, Cursor, ZoneId};

use super::{Run, parse};
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
        #[arg(long, value_delimiter = ',')]
        status: Vec<AlertStatus>,

        /// Filter by alert message type (alert, update, cancel).
        #[arg(long, value_delimiter = ',')]
        message_type: Vec<AlertMessageType>,

        /// Filter by alert event type (e.g., "Tornado Warning", "Flood Watch").
        #[arg(long)]
        event: Vec<String>,

        /// Filter by alert code (NWS public zone/county or SAME code).
        #[arg(long, value_delimiter = ',')]
        code: Vec<String>,

        /// Filter by area code (State/Territory or Marine Area, comma-separated).
        /// This parameter is incompatible with the following parameters: point, marine-region, region-type, zone.
        #[arg(long, value_delimiter = ',')]
        area: Vec<AreaCode>,

        /// Filter by point as LAT,LON in decimal degrees (e.g., 39.7456,-97.0892).
        /// This parameter is incompatible with the following parameters: area, marine-region, region-type, zone.
        #[arg(long, value_name = "LAT,LON")]
        point: Option<Coordinates>,

        /// Filter by marine region code (AL, AT, GL, GM, PA, PI).
        /// This parameter is incompatible with the following parameters: area, point, region-type, zone
        #[arg(long, value_delimiter = ',')]
        marine_region: Vec<MarineRegionCode>,

        /// Filter by region type (land or marine).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, zone.
        #[arg(long)]
        region_type: Option<RegionType>,

        /// Filter by zone ID (forecast or county, e.g., CAZ043, comma-separated).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, region-type
        #[arg(long, value_delimiter = ',')]
        zone: Vec<ZoneId>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        urgency: Vec<AlertUrgency>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        severity: Vec<AlertSeverity>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        certainty: Vec<AlertCertainty>,
    },

    /// Get active alerts for a specific area (State/Territory or Marine Area).
    ///
    /// Example: `noaa-weather alerts area --area CA` or `noaa-weather alerts area --area GM`
    Area {
        /// The state/territory abbreviation or marine area code (e.g., "AL", "GM").
        #[arg(long)]
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
        #[arg(long)]
        marine_region: MarineRegionCode,
    },

    /// Get active alerts for a specific NWS zone (Public Zone or County).
    ///
    /// Example: `noaa-weather alerts zone --zone-id CAZ043`
    Zone {
        /// Zone ID (e.g., "CAZ043", "CAC073") to get alerts for.
        #[arg(long)]
        zone_id: ZoneId,
    },

    /// List alerts with various filters and pagination.
    ///
    /// Queries the NWS API for alerts, allowing filtering by time range,
    /// status, location, and other criteria.
    List {
        /// Start of the query period (RFC 3339 timestamp or relative age such as 6h).
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        start: Option<Timestamp>,

        /// End of the query period (RFC 3339 timestamp or relative age such as 1h).
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        end: Option<Timestamp>,

        /// Filter by alert status (actual, exercise, test, draft).
        #[arg(long, value_delimiter = ',')]
        status: Vec<AlertStatus>,

        /// Filter by alert message type (alert, update, cancel).
        #[arg(long, value_delimiter = ',')]
        message_type: Vec<AlertMessageType>,

        /// Filter by alert event type (e.g., "Tornado Warning").
        #[arg(long, value_delimiter = ',')]
        event: Vec<String>,

        /// Filter by alert code (NWS public zone/county or SAME code, comma-separated).
        #[arg(long, value_delimiter = ',')]
        code: Vec<String>,

        /// Filter by area code (State/Territory or Marine Area, comma-separated).
        /// This parameter is incompatible with the following parameters: point, marine-region, region-type, zone
        #[arg(long, value_delimiter = ',')]
        area: Vec<AreaCode>,

        /// Filter by point as LAT,LON in decimal degrees (e.g., 39.7456,-97.0892).
        /// This parameter is incompatible with the following parameters: area, marine-region, region-type, zone
        #[arg(long, value_name = "LAT,LON")]
        point: Option<Coordinates>,

        /// Filter by marine region code (e.g., AL, AT, GL, comma-separated).
        /// This parameter is incompatible with the following parameters: area, point, region-type, zone
        #[arg(long, value_delimiter = ',')]
        marine_region: Vec<MarineRegionCode>,

        /// Filter by region type (land or marine).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, zone
        #[arg(long)]
        region_type: Option<RegionType>,

        /// Filter by zone ID (forecast or county, e.g., CAZ043, comma-separated).
        /// This parameter is incompatible with the following parameters: area, point, marine-region, region-type
        #[arg(long, value_delimiter = ',')]
        zone: Vec<ZoneId>,

        /// Filter by urgency (Immediate, Expected, Future, Past, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        urgency: Vec<AlertUrgency>,

        /// Filter by severity (Extreme, Severe, Moderate, Minor, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        severity: Vec<AlertSeverity>,

        /// Filter by certainty (Observed, Likely, Possible, Unlikely, Unknown, comma-separated).
        #[arg(long, value_delimiter = ',')]
        certainty: Vec<AlertCertainty>,

        /// Limit number of results returned by the API (1 to 500).
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=500))]
        limit: Option<u16>,

        /// Opaque pagination cursor from a previous page (see pagination.next in --json output)
        #[arg(long)]
        cursor: Option<Cursor>,
    },

    /// Get a single alert by its unique NWS ID.
    ///
    /// Example: `noaa-weather alerts alert --id urn:oid:2.49.0.1.840.0.dcc6cd9527d1f8732519ea87f13d3810e9ef672c.001.1`
    Alert {
        /// Unique Alert ID (e.g., "urn:oid:2.49.0.1.840.0.dcc6cd9527d1f8732519ea87f13d3810e9ef672c.001.1").
        #[arg(long)]
        id: AlertId,
    },

    /// List available alert event types recognized by the NWS API.
    Types,
}

impl Run for AlertCommands {
    async fn run(&self, client: &Client, output: &Output) -> Result<()> {
        let alerts = client.alerts();
        match self {
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
                let query = ActiveAlertsQuery {
                    status: status.clone(),
                    message_type: message_type.clone(),
                    event: event.clone(),
                    code: code.clone(),
                    area: area.clone(),
                    point: *point,
                    region: marine_region.clone(),
                    region_type: *region_type,
                    zone: zone.clone(),
                    urgency: urgency.clone(),
                    severity: severity.clone(),
                    certainty: certainty.clone(),
                };

                output.show(alerts.active(&query)).await
            }
            AlertCommands::Area { area } => output.show(alerts.active_for_area(area)).await,
            AlertCommands::Count => output.show(alerts.active_count()).await,
            AlertCommands::MarineRegion { marine_region } => {
                output
                    .show(alerts.active_for_marine_region(*marine_region))
                    .await
            }
            AlertCommands::Zone { zone_id } => output.show(alerts.active_for_zone(zone_id)).await,
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
                cursor,
            } => {
                let query = AlertsQuery {
                    start: *start,
                    end: *end,
                    status: status.clone(),
                    message_type: message_type.clone(),
                    event: event.clone(),
                    code: code.clone(),
                    area: area.clone(),
                    point: *point,
                    region: marine_region.clone(),
                    region_type: *region_type,
                    zone: zone.clone(),
                    urgency: urgency.clone(),
                    severity: severity.clone(),
                    certainty: certainty.clone(),
                    limit: *limit,
                    cursor: cursor.clone(),
                };

                output.show(alerts.search(&query)).await
            }
            AlertCommands::Alert { id } => output.show(alerts.get(id)).await,
            AlertCommands::Types => output.show(alerts.types()).await,
        }
    }
}
