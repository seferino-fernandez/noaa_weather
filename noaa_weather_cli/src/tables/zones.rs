use anyhow::Result;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    MetarPhenomenon, Observation, ObservationCloudLayersInner, ObservationGeoJson, Zone,
    ZoneCollectionGeoJson, ZoneForecastGeoJson, ZoneGeoJson, ZoneState,
};

use crate::utils::format::{
    format_datetime_human_readable, format_observation_wind, get_zone_from_url, opt_value_unit_val,
};

/// Creates a table listing all zones with key summary information.
///
/// This function processes a `ZoneCollectionGeoJson`, which contains a list of zones,
/// and formats them into a table. Each row represents a zone, displaying its ID, name,
/// type, state, time zones, forecast office, and a summary of observation stations.
///
/// # Arguments
/// * `zone_collection`: A reference to the `ZoneCollectionGeoJson` struct.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
pub fn create_zones_table(zone_collection: &ZoneCollectionGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Zone ID")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Type")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("State")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time Zone(s)")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Forecast Office")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Observation Stations")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for feature_geojson in &zone_collection.features {
        let properties: &Zone = &feature_geojson.properties;

        table.add_row(create_zone_row(properties));
    }

    Ok(table)
}

/// Creates a table listing the metadata for a single zone.
///
/// This function processes a `ZoneGeoJson`, which contains the metadata for a single zone,
/// and formats it into a table. Each row represents a zone, displaying its ID, name,
/// type, state, time zones, forecast office, and a summary of observation stations.
///
/// # Arguments
/// * `zone_geo`: A reference to the `ZoneGeoJson` struct.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
pub fn create_zone_metadata_table(zone_geo: &ZoneGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Zone ID")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Type")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("State")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time Zone(s)")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Forecast Office")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Observation Stations")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    table.add_row(create_zone_row(&zone_geo.properties));

    Ok(table)
}

/// Creates a table listing the forecast for a single zone.
///
/// This function processes a `ZoneForecastGeoJson`, which contains the forecast for a single zone,
/// and formats it into a table. Each row represents a forecast period, displaying its name and
/// detailed forecast.
///
/// # Arguments
/// * `zone_forecast`: A reference to the `ZoneForecastGeoJson` struct.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
pub fn create_zone_forecast_table(zone_forecast: &ZoneForecastGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Day/Night")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Forecast")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    let properties = &zone_forecast.properties;

    match &properties.periods {
        Some(periods_vec) if !periods_vec.is_empty() => {
            for period_item in periods_vec {
                let name_cell = Cell::new(&period_item.name);
                let forecast_cell = Cell::new(&period_item.detailed_forecast);

                table.add_row(vec![name_cell, forecast_cell]);
            }
        }
        _ => {
            // Handles None or empty Vec.
            // Add a single cell that will be displayed in the first column.
            // comfy-table will handle rendering this appropriately given the headers.
            table.add_row(vec![
                Cell::new("No forecast periods available.")
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Italic),
            ]);
        }
    }

    Ok(table)
}

fn create_zone_row(zone: &Zone) -> Vec<Cell> {
    let zone_id_str = zone.id.as_deref().unwrap_or("N/A").to_string();
    let name_str = zone.name.as_deref().unwrap_or("N/A").to_string();

    let zone_type_display = zone
        .r#type
        .as_ref()
        .map(|zone_type| format!("{:?}", zone_type))
        .unwrap_or_else(|| "N/A".to_string());

    let state_display = zone
        .state
        .as_ref()
        .map(|boxed_zone_state_ref| {
            let actual_zone_state: &ZoneState = boxed_zone_state_ref.as_ref();
            match actual_zone_state {
                ZoneState::StateTerritoryCode(state_code_val) => {
                    format!("{:?}", state_code_val).to_uppercase()
                }
                ZoneState::String(string_val) => string_val.to_uppercase(),
            }
        })
        .unwrap_or_else(|| "N/A".to_string());

    let time_zones_display = zone
        .time_zone
        .as_ref()
        .map_or("N/A".to_string(), |time_zones| {
            if time_zones.is_empty() {
                "N/A".to_string()
            } else {
                time_zones.join(",\n")
            }
        });

    let forecast_office_display = get_zone_from_url(zone.forecast_office.as_deref())
        .unwrap_or_else(|| zone.forecast_office.as_deref().unwrap_or("N/A").to_string());

    let obs_stations_display =
        zone.observation_stations
            .as_ref()
            .map_or("N/A".to_string(), |stations| {
                if stations.is_empty() {
                    "None".to_string()
                } else {
                    let station_ids: Vec<String> = stations
                        .iter()
                        .filter_map(|url| get_zone_from_url(Some(url)))
                        .collect();

                    if station_ids.is_empty() && !stations.is_empty() {
                        format!("{} station URL(s)", stations.len())
                    } else if station_ids.is_empty() {
                        "None".to_string()
                    } else {
                        // Show all station IDs four in a row, then wrap
                        let mut station_ids_str = String::new();
                        for (i, station_id) in station_ids.iter().enumerate() {
                            station_ids_str.push_str(station_id);
                            if (i + 1) % 4 == 0 {
                                station_ids_str.push('\n');
                            } else {
                                station_ids_str.push_str(", ");
                            }
                        }
                        // Remove trailing comma and space if any
                        station_ids_str.trim_end_matches(", ").to_string()
                    }
                }
            });
    vec![
        Cell::new(zone_id_str),
        Cell::new(name_str),
        Cell::new(zone_type_display),
        Cell::new(state_display),
        Cell::new(time_zones_display),
        Cell::new(forecast_office_display),
        Cell::new(obs_stations_display),
    ]
}

/// Formats cloud layers from an observation.
fn format_observation_clouds(
    cloud_layers_field: Option<&Option<Vec<ObservationCloudLayersInner>>>,
) -> String {
    match cloud_layers_field {
        Some(Some(layers)) if !layers.is_empty() => layers
            .iter()
            .map(|layer| {
                let amount = &layer.amount;
                let base_str = opt_value_unit_val(&Some(layer.base.as_ref().clone()));
                format!("{} at {}", amount, base_str)
            })
            .collect::<Vec<String>>()
            .join("\n"),
        Some(Some(_)) => "Clear".to_string(),
        Some(None) => "N/A (not reported)".to_string(),
        None => "N/A".to_string(),
    }
}

/// Formats present weather phenomena.
fn format_observation_present_weather(weather_opt: Option<&Vec<MetarPhenomenon>>) -> String {
    match weather_opt {
        Some(phenomena) if !phenomena.is_empty() => phenomena
            .iter()
            .map(|p| p.raw_string.clone())
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join(" "),
        _ => "N/A".to_string(),
    }
}

/// Creates a table listing the latest observations from stations within a zone.
///
/// Each row represents a single observation from a station.
///
/// # Arguments
/// * `observations_features`: A slice of `ObservationGeoJson` features. This typically comes
///   from the `features` array of an `ObservationCollectionGeoJson` response.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
pub fn create_zone_observations_table(
    observations_features: &[ObservationGeoJson],
) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Station")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time (UTC)")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Weather")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Temp.")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Dewpoint")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sea Level Pressure")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Visibility")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Clouds")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    if observations_features.is_empty() {
        table.add_row(vec![
            Cell::new("No observations available for this zone.")
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Italic),
        ]);
        return Ok(table);
    }

    for obs_feature in observations_features {
        let properties: &Observation = &obs_feature.properties;

        let station_name = properties.station_name.as_deref().unwrap_or("N/A");
        let station_id = properties.station_id.as_deref().unwrap_or("N/A");
        let station_name_code = format!("{}\n({})", station_name, station_id);
        let timestamp = format_datetime_human_readable(properties.timestamp.as_deref());

        let temp = opt_value_unit_val(&properties.temperature);
        let dewpoint = opt_value_unit_val(&properties.dewpoint);

        let wind = format_observation_wind(
            properties.wind_speed.as_ref().cloned(),
            properties.wind_direction.as_ref().cloned(),
        );

        // Prioritize Sea Level Pressure, fallback to Barometric if SLP is not available
        let pressure = opt_value_unit_val(
            &properties
                .sea_level_pressure
                .as_ref()
                .or(properties.barometric_pressure.as_ref())
                .cloned(),
        );

        let visibility = opt_value_unit_val(&properties.visibility);
        let clouds = format_observation_clouds(properties.cloud_layers.as_ref());

        // For weather description, use textDescription. If empty, use formatted presentWeather.
        let mut weather_description = properties
            .text_description
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_string();
        if weather_description.is_empty() {
            weather_description =
                format_observation_present_weather(properties.present_weather.as_ref());
            if weather_description.is_empty() {
                weather_description = "N/A".to_string();
            }
        }

        table.add_row(vec![
            Cell::new(station_name_code),
            Cell::new(timestamp),
            Cell::new(weather_description),
            Cell::new(temp),
            Cell::new(dewpoint),
            Cell::new(wind),
            Cell::new(pressure),
            Cell::new(visibility),
            Cell::new(clouds),
        ]);
    }

    Ok(table)
}
