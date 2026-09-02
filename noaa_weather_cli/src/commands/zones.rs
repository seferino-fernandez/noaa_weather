use anyhow::Result;
use clap::{Args, Subcommand};
use jiff::Timestamp;
use noaa_weather_client::apis::zones::{
    ZoneObservationsQuery, ZoneQuery, ZoneStationsQuery, ZoneType, ZonesQuery,
};
use noaa_weather_client::models::{AreaCode, RegionCode};
use noaa_weather_client::{Client, Coordinates, ZoneId};

use super::parse;
use crate::output::{Output, ZoneObservations};

/// Helper struct for commands requiring both a zone type and ID.
#[derive(Args, Debug, Clone)]
pub struct ZoneTypeAndIdArgs {
    /// Zone identifier (e.g., AZZ540, WVC001)
    #[arg(short, long)]
    id: ZoneId,
    /// Type of zone (land, marine, forecast, public, coastal, offshore, fire, county)
    #[arg(short, long)]
    r#type: ZoneType,
}

/// Access data related to NWS forecast, public, and other zones.
///
/// Zones are geographical areas used by the NWS for issuing forecasts, watches, and warnings.
/// Different types of zones exist (e.g., public, forecast, fire weather).
#[derive(Subcommand, Debug, Clone)]
pub enum ZoneCommands {
    /// List zones, optionally filtered by various criteria.
    ///
    /// Example: `noaa-weather zones list --area AZ --type forecast`
    List {
        /// Filter by zone ID (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        id: Vec<ZoneId>,
        /// Filter by area code (State/Territory or Marine Area, comma-separated)
        #[arg(long, value_delimiter = ',')]
        area: Vec<AreaCode>,
        /// Filter by region code (Land or Marine, comma-separated)
        #[arg(long, value_delimiter = ',')]
        region: Vec<RegionCode>,
        /// Filter by zone type (comma-separated: forecast, public, etc.)
        #[arg(short, long, value_delimiter = ',')]
        r#type: Vec<ZoneType>,
        /// Filter by point as LAT,LON in decimal degrees (e.g., 33.4484,-112.0740)
        #[arg(long, value_name = "LAT,LON")]
        point: Option<Coordinates>,
        /// Include geometry in results (can be large)
        #[arg(long)]
        include_geometry: Option<bool>,
        /// Optional: Limit the number of zones returned (1 to 500).
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=500))]
        limit: Option<u16>,
        /// Filter by effective time (RFC 3339 timestamp or relative age such as 1d)
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        effective: Option<Timestamp>,
    },
    /// Get metadata for a specific zone.
    ///
    /// Example: `noaa-weather zones metadata --type public --id AZZ540`
    Metadata {
        #[clap(flatten)]
        zone_args: ZoneTypeAndIdArgs,
        /// Effective time (RFC 3339 timestamp or relative age such as 1d)
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        effective: Option<Timestamp>,
    },
    /// Get the text forecast for a specific zone.
    ///
    /// Example: `noaa-weather zones forecast --type forecast --id AZZ540`
    Forecast {
        #[clap(flatten)]
        zone_args: ZoneTypeAndIdArgs,
    },
    /// List observation stations within a forecast zone.
    ///
    /// Example: `noaa-weather zones stations --id AZZ540 --limit 10`
    Stations {
        /// Forecast zone identifier (e.g., AZZ540)
        #[arg(short, long)]
        id: ZoneId,
        /// Optional: Limit the number of stations returned (1 to 500).
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=500))]
        limit: Option<u16>,
    },
    /// List recent observations for stations within a forecast zone.
    ///
    /// Example: `noaa-weather zones observations --id AZZ540 --limit 20`
    Observations {
        /// Forecast zone identifier (e.g., AZZ540)
        #[arg(short, long)]
        id: ZoneId,
        /// Start time (RFC 3339 timestamp or relative age such as 6h)
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        start: Option<Timestamp>,
        /// End time (RFC 3339 timestamp or relative age such as 1h)
        #[arg(long, value_parser = parse::time, value_name = "TIME", long_help = parse::TIME_HELP)]
        end: Option<Timestamp>,
        /// Optional: Limit the number of observations returned (1 to 500).
        #[arg(long, value_parser = clap::value_parser!(u16).range(1..=500))]
        limit: Option<u16>,
    },
}

/// Handles the execution of zone-related subcommands.
///
/// Dispatches the command to the matching `client.zones()` method based on
/// the provided [`ZoneCommands`] variant and arguments.
///
/// # Arguments
///
/// * `command` - The specific zone subcommand and its arguments to execute.
/// * `output` - The configured output policy.
/// * `client` - The NOAA API client.
///
pub async fn handle_command(
    command: &ZoneCommands,
    output: &Output,
    client: &Client,
) -> Result<()> {
    let zones = client.zones();
    match command {
        ZoneCommands::List {
            id,
            area,
            region,
            r#type,
            point,
            include_geometry,
            limit,
            effective,
        } => {
            let mut query = ZonesQuery {
                id: id.clone(),
                area: area.clone(),
                region: region.clone(),
                types: r#type.clone(),
                point: *point,
                include_geometry: *include_geometry,
                limit: *limit,
                effective: *effective,
            };
            output
                .show("listing NWS zones", async {
                    // A single type selects the narrower `/zones/{type}` route;
                    // none or several go through `/zones` with a type filter.
                    match r#type.as_slice() {
                        [single] => {
                            query.types.clear();
                            zones.list_of_type(*single, &query).await
                        }
                        _ => zones.list(&query).await,
                    }
                })
                .await
        }
        ZoneCommands::Metadata {
            zone_args,
            effective,
        } => {
            let query = ZoneQuery {
                effective: *effective,
            };
            output
                .show(
                    format!("getting zone {}/{}", zone_args.r#type, zone_args.id),
                    zones.get(zone_args.r#type, &zone_args.id, &query),
                )
                .await
        }
        ZoneCommands::Forecast { zone_args } => {
            output
                .show(
                    format!(
                        "getting forecast for zone {}/{}",
                        zone_args.r#type, zone_args.id
                    ),
                    zones.forecast(zone_args.r#type, &zone_args.id),
                )
                .await
        }
        ZoneCommands::Stations { id, limit } => {
            let query = ZoneStationsQuery {
                limit: *limit,
                cursor: None,
            };
            output
                .show(
                    format!("getting stations for forecast zone {id}"),
                    zones.stations(id, &query),
                )
                .await
        }
        ZoneCommands::Observations {
            id,
            start,
            end,
            limit,
        } => {
            let query = ZoneObservationsQuery {
                start: *start,
                end: *end,
                limit: *limit,
            };
            output
                .show(
                    format!("getting observations for forecast zone {id}"),
                    async { zones.observations(id, &query).await.map(ZoneObservations) },
                )
                .await
        }
    }
}
