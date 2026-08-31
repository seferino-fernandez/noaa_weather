use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    GeoJsonGeometry, ObservationCollectionGeoJson, ObservationGeoJson,
    ObservationStationCollectionGeoJson, ObservationStationGeoJson,
};

use crate::output::{HumanDocument, HumanPresentation};
use crate::utils::format::{
    format_datetime_human_readable, format_optional_value_unit, get_zone_from_url,
};

/// Creates a table listing all observation stations with key summary information.
///
/// This function processes a `ObservationStationCollectionGeoJson`, which contains a list of observation stations,
/// and formats them into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_stations_table(station_data: &ObservationStationCollectionGeoJson) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Station ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Elevation (m)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time Zone")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Point (Coords)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Zones")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    for feature in &station_data.features {
        table.add_row(create_station_row(feature));
    }

    table
}

/// Creates a table listing a single observation station with key summary information.
///
/// This function processes a `ObservationStationGeoJson`, which contains a single observation station,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_observation_station_table(observation_station: &ObservationStationGeoJson) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Station ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Elevation (m)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time Zone")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Point (Coords)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Zones")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    table.add_row(create_station_row(observation_station));

    table
}

/// Creates a table listing the latest observation for a single observation station.
///
/// This function processes an `ObservationGeoJson`, which contains a single observation,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_stations_observation_table(observation: &ObservationGeoJson) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let props = &observation.properties;

    let station_id_str =
        get_zone_from_url(props.station.as_ref()).unwrap_or_else(|| "N/A".to_owned());

    let title = format!("Station: {station_id_str} - Observation");
    table.set_header(vec![
        Cell::new(title)
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    table.add_row(vec![
        Cell::new("Timestamp").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_datetime_human_readable(props.timestamp.as_deref())),
    ]);

    table.add_row(vec![
        Cell::new("Text Description").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(props.text_description.as_deref().unwrap_or("N/A")),
    ]);

    table.add_row(vec![
        Cell::new("Temperature").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.temperature)),
    ]);

    table.add_row(vec![
        Cell::new("Dewpoint").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.dewpoint)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Direction").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_direction)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Speed").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_speed)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Gust").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_gust)),
    ]);

    table.add_row(vec![
        Cell::new("Barometric Pressure").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.barometric_pressure)),
    ]);

    table.add_row(vec![
        Cell::new("Sea Level Pressure").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.sea_level_pressure)),
    ]);

    table.add_row(vec![
        Cell::new("Visibility").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.visibility)),
    ]);

    table.add_row(vec![
        Cell::new("Relative Humidity").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.relative_humidity)),
    ]);

    table.add_row(vec![
        Cell::new("Wind Chill").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.wind_chill)),
    ]);

    table.add_row(vec![
        Cell::new("Heat Index").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(format_optional_value_unit(&props.heat_index)),
    ]);

    table
}

/// Creates a table listing the latest observation for a single observation station.
///
/// This function processes an `ObservationCollectionGeoJson`, which contains a single observation,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
pub fn create_stations_observations_table(observations: &ObservationCollectionGeoJson) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Timestamp")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Temperature")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Dewpoint")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Direction")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Speed")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Gust")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Barometric Pressure")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sea Level Pressure")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Visibility")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Relative Humidity")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Wind Chill")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Heat Index")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    let observations_features = &observations.features;

    for observation in observations_features {
        let timestamp_str =
            format_datetime_human_readable(observation.properties.timestamp.as_deref());
        let temperature_str = format_optional_value_unit(&observation.properties.temperature);
        let dewpoint_str = format_optional_value_unit(&observation.properties.dewpoint);
        let wind_direction_str = format_optional_value_unit(&observation.properties.wind_direction);
        let wind_speed_str = format_optional_value_unit(&observation.properties.wind_speed);
        let wind_gust_str = format_optional_value_unit(&observation.properties.wind_gust);
        let barometric_pressure_str =
            format_optional_value_unit(&observation.properties.barometric_pressure);
        let sea_level_pressure_str =
            format_optional_value_unit(&observation.properties.sea_level_pressure);
        let visibility_str = format_optional_value_unit(&observation.properties.visibility);
        let relative_humidity_str =
            format_optional_value_unit(&observation.properties.relative_humidity);
        let wind_chill_str = format_optional_value_unit(&observation.properties.wind_chill);
        let heat_index_str = format_optional_value_unit(&observation.properties.heat_index);

        table.add_row(vec![
            Cell::new(timestamp_str),
            Cell::new(temperature_str),
            Cell::new(dewpoint_str),
            Cell::new(wind_direction_str),
            Cell::new(wind_speed_str),
            Cell::new(wind_gust_str),
            Cell::new(barometric_pressure_str),
            Cell::new(sea_level_pressure_str),
            Cell::new(visibility_str),
            Cell::new(relative_humidity_str),
            Cell::new(wind_chill_str),
            Cell::new(heat_index_str),
        ]);
    }

    table
}

#[cfg(feature = "xml")]
mod taf;

#[cfg(feature = "xml")]
pub use taf::{create_stations_taf_table, create_stations_tafs_metadata_table};

/// Creates a row for a single observation station.
///
/// This function processes an `ObservationStationGeoJson`, which contains a single observation station,
/// and formats it into a row. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
fn create_station_row(observation_station: &ObservationStationGeoJson) -> Vec<String> {
    let station = &observation_station.properties;

    let elevation_str = format_optional_value_unit(&station.elevation);

    let point_str = observation_station.geometry.as_ref().map_or_else(
        || "N/A".to_owned(),
        |geo_json_geometry| match geo_json_geometry.as_ref() {
            GeoJsonGeometry::GeoJsonPoint(point) => {
                format!("{:?}", point.coordinates)
            }
            GeoJsonGeometry::GeoJsonLineString(_)
            | GeoJsonGeometry::GeoJsonPolygon(_)
            | GeoJsonGeometry::GeoJsonMultiPoint(_)
            | GeoJsonGeometry::GeoJsonMultiLineString(_)
            | GeoJsonGeometry::GeoJsonMultiPolygon(_) => "N/A".to_owned(),
        },
    );

    let timezone_str = station
        .time_zone
        .clone()
        .unwrap_or_else(|| "N/A".to_owned());

    let forecast_zone =
        get_zone_from_url(station.forecast.clone()).unwrap_or_else(|| "N/A".to_owned());

    let county = get_zone_from_url(station.county.clone()).unwrap_or_else(|| "N/A".to_owned());

    let fire_weather_zone =
        get_zone_from_url(station.fire_weather_zone.clone()).unwrap_or_else(|| "N/A".to_owned());

    let zones = format!(
        "Forecast Zone: {forecast_zone}\nCounty: {county}\nFire Weather Zone: {fire_weather_zone}"
    );

    vec![
        station
            .station_identifier
            .as_deref()
            .unwrap_or("N/A")
            .to_owned(),
        station.name.as_deref().unwrap_or("N/A").to_owned(),
        elevation_str,
        timezone_str,
        point_str,
        zones,
    ]
}

impl HumanPresentation for ObservationStationCollectionGeoJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_stations_table(self))
    }
}

impl HumanPresentation for ObservationStationGeoJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_observation_station_table(self))
    }
}

impl HumanPresentation for ObservationGeoJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_stations_observation_table(self))
    }
}

impl HumanPresentation for ObservationCollectionGeoJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_stations_observations_table(self))
    }
}

#[cfg(feature = "xml")]
impl HumanPresentation for noaa_weather_client::models::TerminalAerodromeForecastsResponse {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_stations_tafs_metadata_table(self))
    }
}

#[cfg(feature = "xml")]
impl HumanPresentation for noaa_weather_client::models::TerminalAerodromeForecast {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_stations_taf_table(self))
    }
}
