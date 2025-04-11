use anyhow::{Result, anyhow};
use clap::{Args, Subcommand, value_parser};
use noaa_weather_client::apis::configuration::Configuration;
use noaa_weather_client::models::NwsZoneType;
use serde_json::Value;

use crate::utils::parse::{parse_area_codes, parse_region_codes, parse_string_args_into_vec};

#[derive(Args, Debug)]
pub struct ZoneTypeAndIdArgs {
    /// Type of zone (forecast, public, coastal, offshore, fire, county)
    #[arg(short, long, value_parser = value_parser!(NwsZoneType))]
    r#type: NwsZoneType,
    /// Zone identifier (e.g., AZZ540, WVC001)
    #[arg(short, long)]
    zone_id: String,
}

#[derive(Subcommand, Debug)]
pub enum ZoneCommands {
    /// List zones with optional filters
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
    /// Get metadata for a specific zone
    Get {
        #[clap(flatten)]
        zone_args: ZoneTypeAndIdArgs,
        /// Effective date (ISO 8601 format)
        #[arg(long)]
        effective: Option<String>,
    },
    /// Get the forecast for a specific zone
    Forecast {
        /// Type of zone (forecast, public, coastal, offshore, fire, county)
        #[arg(short, long, value_parser = value_parser!(NwsZoneType))]
        r#type: NwsZoneType,
        /// Zone identifier (e.g., AZZ540, WVC001)
        #[arg(short, long)]
        zone_id: String,
    },
    /// List observation stations within a forecast zone
    Stations {
        /// Forecast zone identifier (e.g., AZZ540)
        #[arg(short, long)]
        zone_id: String,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<i32>,
        /// Pagination cursor
        #[arg(long)]
        cursor: Option<String>,
    },
    /// List observations for a forecast zone
    Observations {
        /// Forecast zone identifier (e.g., AZZ540)
        #[arg(short, long)]
        zone_id: String,
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

            let result = match type_parsed {
                None => {
                    // Call the general list endpoint if no type filter
                    noaa_weather_client::apis::zones::zone_list(
                        config,
                        id,
                        area_parsed,
                        region_parsed,
                        None,
                        point.as_deref(),
                        include_geometry,
                        limit,
                        effective,
                    )
                    .await
                    .map_err(|e| anyhow!("Error listing zones: {}", e))?
                }
                Some(types) => {
                    if types.len() == 1 {
                        // Call the type-specific list endpoint if exactly one type filter
                        let single_type = types.into_iter().next().unwrap();
                        noaa_weather_client::apis::zones::zone_list_type(
                            config,
                            single_type,
                            id,
                            area_parsed,
                            region_parsed,
                            None,
                            point.as_deref(),
                            include_geometry,
                            limit,
                            effective,
                        )
                        .await
                        .map_err(|e| {
                            anyhow!("Error listing zones of type {}: {}", single_type, e)
                        })?
                    } else {
                        // Call general list endpoint with type filter if multiple types
                        noaa_weather_client::apis::zones::zone_list(
                            config,
                            id,
                            area_parsed,
                            region_parsed,
                            Some(types),
                            point.as_deref(),
                            include_geometry,
                            limit,
                            effective,
                        )
                        .await
                        .map_err(|e| anyhow!("Error listing zones with multiple types: {}", e))?
                    }
                }
            };

            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Get {
            zone_args,
            effective,
        } => {
            // zone_args.r#type is already parsed by clap using value_parser
            let result = noaa_weather_client::apis::zones::zone(
                config,
                zone_args.r#type,
                &zone_args.zone_id,
                effective,
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "Error getting zone {}/{}: {}",
                    zone_args.r#type,
                    zone_args.zone_id,
                    e
                )
            })?;
            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Forecast { r#type, zone_id } => {
            // r#type is already parsed by clap using value_parser
            // API expects type as string, use the Display impl
            let result = noaa_weather_client::apis::zones::zone_forecast(
                config,
                &r#type.to_string(),
                &zone_id,
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "Error getting forecast for zone {}/{}: {}",
                    r#type,
                    zone_id,
                    e
                )
            })?;
            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Stations {
            zone_id,
            limit,
            cursor,
        } => {
            let result = noaa_weather_client::apis::zones::zone_stations(
                config,
                &zone_id,
                limit,
                cursor.as_deref(),
            )
            .await
            .map_err(|e| {
                anyhow!(
                    "Error getting stations for forecast zone {}: {}",
                    zone_id,
                    e
                )
            })?;
            Ok(serde_json::to_value(result)?)
        }
        ZoneCommands::Observations {
            zone_id,
            start,
            end,
            limit,
        } => {
            let result =
                noaa_weather_client::apis::zones::zone_obs(config, &zone_id, start, end, limit)
                    .await
                    .map_err(|e| {
                        anyhow!(
                            "Error getting observations for forecast zone {}: {}",
                            zone_id,
                            e
                        )
                    })?;
            Ok(serde_json::to_value(result)?)
        }
    }
}
