use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    MetarPhenomenon, Observation, ObservationCloudLayersInner, Zone, ZoneForecast, ZoneState,
};
use noaa_weather_client::{Feature, FeatureCollection};
use serde::Serialize;

use crate::output::PresentationDocument;
use crate::output::presentation::{DefaultPresentation, DefaultPresenter, PresentationError};

/// Creates a table listing all zones with key summary information.
///
/// This function processes a zone feature collection, which contains a list of zones,
/// and formats them into a table. Each row represents a zone, displaying its ID, name,
/// type, state, time zones, forecast office, and a summary of observation stations.
///
/// # Arguments
/// * `zone_collection`: A reference to the zone feature collection.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
fn create_zones_table(
    zone_collection: &FeatureCollection<Zone>,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
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

        table.add_row(create_zone_row(properties, presenter));
    }

    table
}

/// Creates a table listing the metadata for a single zone.
///
/// This function processes a zone feature, which contains the metadata for a single zone,
/// and formats it into a table. Each row represents a zone, displaying its ID, name,
/// type, state, time zones, forecast office, and a summary of observation stations.
///
/// # Arguments
/// * `zone_geo`: A reference to the zone feature.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
fn create_zone_metadata_table(zone_geo: &Feature<Zone>, presenter: &DefaultPresenter) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
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

    table.add_row(create_zone_row(&zone_geo.properties, presenter));

    table
}

/// Creates a table listing the forecast for a single zone.
///
/// This function processes a zone forecast feature, which contains the forecast for a single zone,
/// and formats it into a table. Each row represents a forecast period, displaying its name and
/// detailed forecast.
///
/// # Arguments
/// * `zone_forecast`: A reference to the zone forecast feature.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
fn create_zone_forecast_table(
    zone_forecast: &Feature<ZoneForecast>,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
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
                let name_cell = Cell::new(presenter.text(Some(&period_item.name)));
                let forecast_cell = Cell::new(presenter.text(Some(&period_item.detailed_forecast)));

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

    table
}

fn create_zone_row(zone: &Zone, presenter: &DefaultPresenter) -> Vec<Cell> {
    let zone_id_str = presenter.text(zone.id.as_deref());
    let name_str = presenter.text(zone.name.as_deref());

    let zone_type_display = zone.r#type.as_ref().map_or_else(
        || presenter.text(None),
        |zone_type| format!("{zone_type:?}"),
    );

    let state_display = zone
        .state
        .as_ref()
        .map(|boxed_zone_state_ref| {
            let actual_zone_state: &ZoneState = boxed_zone_state_ref.as_ref();
            match actual_zone_state {
                ZoneState::StateTerritoryCode(state_code_val) => {
                    format!("{state_code_val:?}").to_uppercase()
                }
                ZoneState::String(string_val) => string_val.to_uppercase(),
            }
        })
        .unwrap_or_else(|| presenter.text(None));

    let time_zones_display = zone.time_zone.as_ref().map_or_else(
        || presenter.text(None),
        |time_zones| {
            if time_zones.is_empty() {
                presenter.text(None)
            } else {
                time_zones.join(",\n")
            }
        },
    );

    let forecast_office_display = presenter.resource_identifier(zone.forecast_office.as_deref());

    let obs_stations_display = zone.observation_stations.as_ref().map_or_else(
        || presenter.text(None),
        |stations| {
            if stations.is_empty() {
                "None".to_owned()
            } else {
                let station_ids: Vec<String> = stations
                    .iter()
                    .map(|url| presenter.resource_identifier(Some(url)))
                    .collect();

                if station_ids.is_empty() && !stations.is_empty() {
                    format!("{} station URL(s)", stations.len())
                } else if station_ids.is_empty() {
                    "None".to_owned()
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
                    station_ids_str.trim_end_matches(", ").to_owned()
                }
            }
        },
    );
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
    presenter: &DefaultPresenter,
) -> String {
    match cloud_layers_field {
        Some(Some(layers)) if !layers.is_empty() => layers
            .iter()
            .map(|layer| {
                let amount = &layer.amount;
                let base_str = presenter.value_unit(Some(layer.base.as_ref()));
                format!("{amount} at {base_str}")
            })
            .collect::<Vec<String>>()
            .join("\n"),
        Some(Some(_)) => "Clear".to_owned(),
        Some(None) => "N/A (not reported)".to_owned(),
        None => presenter.missing(),
    }
}

/// Formats present weather phenomena.
fn format_observation_present_weather(weather_opt: Option<&Vec<MetarPhenomenon>>) -> String {
    match weather_opt {
        Some(phenomena) if !phenomena.is_empty() => phenomena
            .iter()
            .map(|phenomenon| phenomenon.raw_string.clone())
            .filter(|raw_string| !raw_string.is_empty())
            .collect::<Vec<String>>()
            .join(" "),
        _ => String::new(),
    }
}

/// Creates a table listing the latest observations from stations within a zone.
///
/// Each row represents a single observation from a station.
///
/// # Arguments
/// * `observations_features`: A slice of observation features from a feature collection.
///
/// # Returns
/// A `Result<Table>` which is the `comfy_table::Table` ready for display, or an error.
fn create_zone_observations_table(
    observations_features: &[Feature<Observation>],
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Station")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Time")
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

    for (index, obs_feature) in observations_features.iter().enumerate() {
        let properties: &Observation = &obs_feature.properties;

        let station_name = presenter.text(properties.station_name.as_deref());
        let station_id = presenter.text(properties.station_id.as_deref());
        let station_name_code = format!("{station_name}\n({station_id})");
        let timestamp = presenter.timestamp(
            format!("zone observations.features[{index}].properties.timestamp"),
            properties.timestamp.as_deref(),
        )?;

        let temp = presenter.value_unit(properties.temperature.as_ref());
        let dewpoint = presenter.value_unit(properties.dewpoint.as_ref());

        let wind = presenter.observation_wind(
            properties.wind_speed.as_ref(),
            properties.wind_direction.as_ref(),
        );

        // Prioritize Sea Level Pressure, fallback to Barometric if SLP is not available
        let pressure = presenter.observation_pressure(
            properties.sea_level_pressure.as_ref(),
            properties.barometric_pressure.as_ref(),
        );

        let visibility = presenter.value_unit(properties.visibility.as_ref());
        let clouds = format_observation_clouds(properties.cloud_layers.as_ref(), presenter);

        // For weather description, use textDescription. If empty, use formatted presentWeather.
        let present_weather =
            format_observation_present_weather(properties.present_weather.as_ref());
        let weather_description =
            presenter.observation_weather(properties.text_description.as_deref(), &present_weather);

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

impl DefaultPresentation for FeatureCollection<Zone> {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_zones_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for Feature<Zone> {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_zone_metadata_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for Feature<ZoneForecast> {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_zone_forecast_table(
            self, presenter,
        )))
    }
}

#[derive(Serialize)]
#[serde(transparent)]
pub(crate) struct ZoneObservations(pub(crate) FeatureCollection<Observation>);

impl DefaultPresentation for ZoneObservations {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_zone_observations_table(
            &self.0.features,
            presenter,
        )?))
    }
}
