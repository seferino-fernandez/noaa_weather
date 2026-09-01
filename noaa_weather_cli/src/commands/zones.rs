use anyhow::Result;
use clap::{Args, Subcommand};
use noaa_weather_client::Client;
use noaa_weather_client::apis::zones::{self as zones_api, GetZonesByTypeParams, GetZonesParams};
use noaa_weather_client::models::{AreaCode, NwsZoneType, RegionCode};

use crate::output::{Output, ZoneObservations};

/// Helper struct for commands requiring both a zone type and ID.
#[derive(Args, Debug, Clone)]
pub struct ZoneTypeAndIdArgs {
    /// Zone identifier (e.g., AZZ540, WVC001)
    #[arg(short, long)]
    id: String,
    /// Type of zone (forecast, public, coastal, offshore, fire, county)
    #[arg(short, long, value_enum)]
    r#type: NwsZoneType,
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
        id: Option<Vec<String>>,
        /// Filter by area code (State/Territory or Marine Area, comma-separated)
        #[arg(long, value_delimiter = ',', value_enum)]
        area: Option<Vec<AreaCode>>,
        /// Filter by region code (Land or Marine, comma-separated)
        #[arg(long, value_delimiter = ',', value_enum)]
        region: Option<Vec<RegionCode>>,
        /// Filter by zone type (comma-separated: forecast, public, etc.)
        #[arg(short, long, value_delimiter = ',', value_enum)]
        r#type: Option<Vec<NwsZoneType>>,
        /// Filter by point (latitude,longitude)
        #[arg(long)]
        point: Option<String>,
        /// Include geometry in results (can be large)
        #[arg(long)]
        include_geometry: Option<bool>,
        /// Optional: Limit the number of zones returned.
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=500))]
        limit: Option<i32>,
        /// Filter by effective date (ISO 8601 format)
        #[arg(long)]
        effective: Option<String>,
    },
    /// Get metadata for a specific zone.
    ///
    /// Example: `noaa-weather zones metadata --type public --id AZZ540`
    Metadata {
        #[clap(flatten)]
        zone_args: ZoneTypeAndIdArgs,
        /// Effective date (ISO 8601 format)
        #[arg(long)]
        effective: Option<String>,
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
        id: String,
        /// Optional: Limit the number of stations returned.
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=500))]
        limit: Option<i32>,
    },
    /// List recent observations for stations within a forecast zone.
    ///
    /// Example: `noaa-weather zones observations --id AZZ540 --limit 20`
    Observations {
        /// Forecast zone identifier (e.g., AZZ540)
        #[arg(short, long)]
        id: String,
        /// Start time (ISO 8601 format)
        #[arg(long)]
        start: Option<String>,
        /// End time (ISO 8601 format)
        #[arg(long)]
        end: Option<String>,
        /// Optional: Limit the number of observations returned.
        #[arg(long, value_parser = clap::value_parser!(i32).range(1..=500))]
        limit: Option<i32>,
    },
}

/// Handles the execution of zone-related subcommands.
///
/// Dispatches the command to the appropriate API function based on the
/// provided [`ZoneCommands`] variant and arguments.
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
            let point_ref = point.as_deref();

            output
                .show("listing NWS zones", async {
                    match r#type {
                        None => {
                            let params = GetZonesParams {
                                id: id.clone(),
                                area: area.clone(),
                                region: region.clone(),
                                r#type: None,
                                point: point_ref,
                                include_geometry: *include_geometry,
                                limit: *limit,
                                effective: effective.clone(),
                            };
                            zones_api::get_zones(client, params).await
                        }
                        Some(types) if types.len() == 1 => {
                            let single_type = types[0];
                            let params = GetZonesByTypeParams {
                                id: id.clone(),
                                area: area.clone(),
                                region: region.clone(),
                                type_filter: None,
                                point: point_ref,
                                include_geometry: *include_geometry,
                                limit: *limit,
                                effective: effective.clone(),
                            };
                            zones_api::get_zones_by_type(client, single_type, params).await
                        }
                        Some(types) => {
                            let params = GetZonesParams {
                                id: id.clone(),
                                area: area.clone(),
                                region: region.clone(),
                                r#type: Some(types.clone()),
                                point: point_ref,
                                include_geometry: *include_geometry,
                                limit: *limit,
                                effective: effective.clone(),
                            };
                            zones_api::get_zones(client, params).await
                        }
                    }
                })
                .await
        }
        ZoneCommands::Metadata {
            zone_args,
            effective,
        } => {
            output
                .show(
                    format!("getting zone {}/{}", zone_args.r#type, zone_args.id),
                    zones_api::get_zone(client, zone_args.r#type, &zone_args.id, effective.clone()),
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
                    zones_api::get_current_zone_forecast(
                        client,
                        &zone_args.r#type.to_string(),
                        &zone_args.id,
                    ),
                )
                .await
        }
        ZoneCommands::Stations { id, limit } => {
            output
                .show(
                    format!("getting stations for forecast zone {id}"),
                    zones_api::get_stations_by_zone(client, id, *limit, None),
                )
                .await
        }
        ZoneCommands::Observations {
            id,
            start,
            end,
            limit,
        } => {
            output
                .show(
                    format!("getting observations for forecast zone {id}"),
                    async {
                        zones_api::get_zone_observations(
                            client,
                            id,
                            start.clone(),
                            end.clone(),
                            *limit,
                        )
                        .await
                        .map(ZoneObservations)
                    },
                )
                .await
        }
    }
}
