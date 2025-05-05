use anyhow::{Result, anyhow};
use clap::{Args, Subcommand, value_parser};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::apis::zones::{self as zones_api, GetZonesByTypeParams, GetZonesParams};
use noaa_weather_client::models::NwsZoneType;
use serde_json::Value;

use crate::utils::parse::{parse_area_codes, parse_region_codes, parse_string_args_into_vec};

/// Helper struct for commands requiring both a zone type and ID.
#[derive(Args, Debug)]
pub struct ZoneTypeAndIdArgs {
    /// Type of zone (forecast, public, coastal, offshore, fire, county)
    #[arg(short, long, value_parser = value_parser!(NwsZoneType))]
    r#type: NwsZoneType,
    /// Zone identifier (e.g., AZZ540, WVC001)
    #[arg(short, long)]
    id: String,
}

/// Access data related to NWS forecast, public, and other zones.
///
/// Zones are geographical areas used by the NWS for issuing forecasts, watches, and warnings.
/// Different types of zones exist (e.g., public, forecast, fire weather).
#[derive(Subcommand, Debug)]
pub enum ZoneCommands {
    /// List zones, optionally filtered by various criteria.
    ///
    /// Example: `noaa-weather zones list --area AZ --type forecast`
    List {
        /// Filter by zone ID (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        id: Option<Vec<String>>,
        /// Filter by area code (State/Territory or Marine Area, comma-separated)
        #[arg(long, value_delimiter = ',')]
        area: Option<Vec<String>>,
        /// Filter by region code (Land or Marine, comma-separated)
        #[arg(long, value_delimiter = ',')]
        region: Option<Vec<String>>,
        /// Filter by zone type (comma-separated: forecast, public, etc.)
        #[arg(short, long, value_delimiter = ',')]
        r#type: Option<Vec<String>>,
        /// Filter by point (latitude,longitude)
        #[arg(long)]
        point: Option<String>,
        /// Include geometry in results (can be large)
        #[arg(long)]
        include_geometry: Option<bool>,
        /// Limit number of results
        #[arg(short, long)]
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
        /// Type of zone (forecast, public, coastal, offshore, fire, county)
        #[arg(short, long, value_parser = value_parser!(NwsZoneType))]
        r#type: NwsZoneType,
        /// Zone identifier (e.g., AZZ540, WVC001)
        #[arg(short, long)]
        id: String,
    },
    /// List observation stations within a forecast zone.
    ///
    /// Example: `noaa-weather zones stations --id AZZ540 --limit 10`
    Stations {
        /// Forecast zone identifier (e.g., AZZ540)
        #[arg(short, long)]
        id: String,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Pagination cursor
        #[arg(long)]
        cursor: Option<String>,
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
        /// Limit number of results
        #[arg(short, long)]
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
/// * `config` - The application configuration containing API details.
///
/// # Returns
///
/// A `Result` containing the JSON `Value` of the API response on success,
/// or an `anyhow::Error` if an error occurs during the API call or processing.
pub async fn handle_command(command: ZoneCommands, config: &Configuration) -> Result<Value> {
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
            let area_parsed = parse_area_codes(area)?;
            let region_parsed = parse_region_codes(region)?;
            let type_parsed = parse_string_args_into_vec::<NwsZoneType>(r#type)?;

            // Need to hold the point string ref for the lifetime of the call
            let point_ref = point.as_deref();

            let result = match type_parsed {
                None => {
                    // Call the general list endpoint if no type filter
                    let params = GetZonesParams {
                        id,
                        area: area_parsed,
                        region: region_parsed,
                        r#type: None,
                        point: point_ref,
                        include_geometry,
                        limit,
                        effective,
                    };
                    zones_api::get_zones(config, params)
                        .await
                        .map_err(|e| anyhow!("Error listing zones: {}", e))?
                }
                Some(types) => {
                    if types.len() == 1 {
                        // Call the type-specific list endpoint if exactly one type filter
                        let single_type = types.into_iter().next().unwrap();
                        let params = GetZonesByTypeParams {
                            id,
                            area: area_parsed,
                            region: region_parsed,
                            type_filter: None,
                            point: point_ref,
                            include_geometry,
                            limit,
                            effective,
                        };
                        zones_api::get_zones_by_type(config, single_type, params)
                            .await
                            .map_err(|e| {
                                anyhow!("Error listing zones of type {}: {}", single_type, e)
                            })?
                    } else {
                        // Call general list endpoint with type filter if multiple types
                        let params = GetZonesParams {
                            id,
                            area: area_parsed,
                            region: region_parsed,
                            r#type: Some(types),
                            point: point_ref,
                            include_geometry,
                            limit,
                            effective,
                        };
                        zones_api::get_zones(config, params).await.map_err(|e| {
                            anyhow!("Error listing zones with multiple types: {}", e)
                        })?
                    }
                }
            };

            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Metadata {
            zone_args,
            effective,
        } => {
            // zone_args.r#type is already parsed by clap using value_parser
            let result = zones_api::get_zone(config, zone_args.r#type, &zone_args.id, effective)
                .await
                .map_err(|e| {
                    anyhow!(
                        "Error getting zone {}/{}: {}",
                        zone_args.r#type,
                        zone_args.id,
                        e
                    )
                })?;
            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Forecast { r#type, id } => {
            // r#type is already parsed by clap using value_parser
            // API expects type as string, use the Display impl
            let result = zones_api::get_current_zone_forecast(config, &r#type.to_string(), &id)
                .await
                .map_err(|e| anyhow!("Error getting forecast for zone {}/{}: {}", r#type, id, e))?;
            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Stations { id, limit, cursor } => {
            let result = zones_api::get_stations_by_zone(config, &id, limit, cursor.as_deref())
                .await
                .map_err(|e| anyhow!("Error getting stations for forecast zone {}: {}", id, e))?;
            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Observations {
            id,
            start,
            end,
            limit,
        } => {
            let result = zones_api::get_zone_observations(config, &id, start, end, limit)
                .await
                .map_err(|e| {
                    anyhow!("Error getting observations for forecast zone {}: {}", id, e)
                })?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
