use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    GeoJsonGeometry, ObservationCollectionGeoJson, ObservationGeoJson,
    ObservationStationCollectionGeoJson, ObservationStationGeoJson,
};

use super::{DefaultPresentation, DefaultPresenter, PresentationDocument, PresentationError};

/// Creates a table listing all observation stations with key summary information.
///
/// This function processes a `ObservationStationCollectionGeoJson`, which contains a list of observation stations,
/// and formats them into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
fn create_stations_table(
    station_data: &ObservationStationCollectionGeoJson,
    presenter: &DefaultPresenter,
) -> Table {
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
        table.add_row(create_station_row(feature, presenter));
    }

    table
}

/// Creates a table listing a single observation station with key summary information.
///
/// This function processes a `ObservationStationGeoJson`, which contains a single observation station,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
fn create_observation_station_table(
    observation_station: &ObservationStationGeoJson,
    presenter: &DefaultPresenter,
) -> Table {
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

    table.add_row(create_station_row(observation_station, presenter));

    table
}

/// Creates a table listing the latest observation for a single observation station.
///
/// This function processes an `ObservationGeoJson`, which contains a single observation,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
fn create_stations_observation_table(
    observation: &ObservationGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    let props = &observation.properties;

    let station_id_str = presenter.resource_identifier(props.station.as_deref());

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
        Cell::new(presenter.timestamp(
            format!("station observation {station_id_str} timestamp"),
            props.timestamp.as_deref(),
        )?),
    ]);

    table.add_row(vec![
        Cell::new("Text Description").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.text(props.text_description.as_deref())),
    ]);

    table.add_row(vec![
        Cell::new("Temperature").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.temperature.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Dewpoint").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.dewpoint.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Wind Direction").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.wind_direction.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Wind Speed").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.wind_speed.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Wind Gust").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.wind_gust.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Barometric Pressure").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.barometric_pressure.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Sea Level Pressure").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.sea_level_pressure.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Visibility").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.visibility.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Relative Humidity").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.relative_humidity.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Wind Chill").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.wind_chill.as_ref())),
    ]);

    table.add_row(vec![
        Cell::new("Heat Index").add_attribute(comfy_table::Attribute::Bold),
        Cell::new(presenter.value_unit(props.heat_index.as_ref())),
    ]);

    Ok(table)
}

/// Creates a table listing the latest observation for a single observation station.
///
/// This function processes an `ObservationCollectionGeoJson`, which contains a single observation,
/// and formats it into a table. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
fn create_stations_observations_table(
    observations: &ObservationCollectionGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
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

    for (index, observation) in observations_features.iter().enumerate() {
        let properties = &observation.properties;
        let station = presenter.resource_identifier(properties.station.as_deref());
        let timestamp_str = presenter.timestamp(
            format!("station observation {index} ({station}) timestamp"),
            properties.timestamp.as_deref(),
        )?;
        let temperature_str = presenter.value_unit(properties.temperature.as_ref());
        let dewpoint_str = presenter.value_unit(properties.dewpoint.as_ref());
        let wind_direction_str = presenter.value_unit(properties.wind_direction.as_ref());
        let wind_speed_str = presenter.value_unit(properties.wind_speed.as_ref());
        let wind_gust_str = presenter.value_unit(properties.wind_gust.as_ref());
        let barometric_pressure_str = presenter.value_unit(properties.barometric_pressure.as_ref());
        let sea_level_pressure_str = presenter.value_unit(properties.sea_level_pressure.as_ref());
        let visibility_str = presenter.value_unit(properties.visibility.as_ref());
        let relative_humidity_str = presenter.value_unit(properties.relative_humidity.as_ref());
        let wind_chill_str = presenter.value_unit(properties.wind_chill.as_ref());
        let heat_index_str = presenter.value_unit(properties.heat_index.as_ref());

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

    Ok(table)
}

#[cfg(feature = "xml")]
mod taf;

#[cfg(feature = "xml")]
use taf::{create_stations_taf_table, create_stations_tafs_metadata_table};

/// Creates a row for a single observation station.
///
/// This function processes an `ObservationStationGeoJson`, which contains a single observation station,
/// and formats it into a row. Each row represents a station, displaying its ID, name,
/// elevation, and time zone.
///
fn create_station_row(
    observation_station: &ObservationStationGeoJson,
    presenter: &DefaultPresenter,
) -> Vec<String> {
    let station = &observation_station.properties;

    let elevation_str = presenter.value_unit(station.elevation.as_ref());

    let point_str = observation_station.geometry.as_ref().map_or_else(
        || presenter.text(None),
        |geo_json_geometry| match geo_json_geometry.as_ref() {
            GeoJsonGeometry::GeoJsonPoint(point) => {
                format!("{:?}", point.coordinates)
            }
            GeoJsonGeometry::GeoJsonLineString(_)
            | GeoJsonGeometry::GeoJsonPolygon(_)
            | GeoJsonGeometry::GeoJsonMultiPoint(_)
            | GeoJsonGeometry::GeoJsonMultiLineString(_)
            | GeoJsonGeometry::GeoJsonMultiPolygon(_) => presenter.text(None),
        },
    );

    let timezone_str = presenter.text(station.time_zone.as_deref());

    let forecast_zone = presenter.resource_identifier(station.forecast.as_deref());

    let county = presenter.resource_identifier(station.county.as_deref());

    let fire_weather_zone = presenter.resource_identifier(station.fire_weather_zone.as_deref());

    let zones = format!(
        "Forecast Zone: {forecast_zone}\nCounty: {county}\nFire Weather Zone: {fire_weather_zone}"
    );

    vec![
        presenter.text(station.station_identifier.as_deref()),
        presenter.text(station.name.as_deref()),
        elevation_str,
        timezone_str,
        point_str,
        zones,
    ]
}

impl DefaultPresentation for ObservationStationCollectionGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_stations_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for ObservationStationGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(
            create_observation_station_table(self, presenter),
        ))
    }
}

impl DefaultPresentation for ObservationGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(
            create_stations_observation_table(self, presenter)?,
        ))
    }
}

impl DefaultPresentation for ObservationCollectionGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(
            create_stations_observations_table(self, presenter)?,
        ))
    }
}

#[cfg(feature = "xml")]
impl DefaultPresentation for noaa_weather_client::models::TerminalAerodromeForecastsResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(
            create_stations_tafs_metadata_table(self, presenter)?,
        ))
    }
}

#[cfg(feature = "xml")]
impl DefaultPresentation for noaa_weather_client::models::TerminalAerodromeForecast {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_stations_taf_table(self)))
    }
}
