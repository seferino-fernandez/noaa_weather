use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, CellAlignment, Table};
use noaa_weather_client::Feature;
use noaa_weather_client::models::Point;

use super::{DefaultPresentation, DefaultPresenter, PresentationError};
use crate::output::PresentationDocument;

/// Formats point metadata into a `comfy_table::Table`.
fn create_point_metadata_table(point_data: &Feature<Point>, presenter: &DefaultPresenter) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_header(vec![
        Cell::new("Property")
            .add_attribute(Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Value")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    let properties = &point_data.properties;

    table.add_row(vec![
        "Forecast Office",
        &presenter.resource_identifier(Some(&properties.forecast_office)),
    ]);
    table.add_row(vec!["Grid ID", &properties.grid_id.to_string()]);
    table.add_row(vec!["Grid X", &properties.grid_x.to_string()]);
    table.add_row(vec!["Grid Y", &properties.grid_y.to_string()]);
    table.add_row(vec![
        "Forecast Zone",
        &presenter.resource_identifier(Some(&properties.forecast_zone)),
    ]);
    table.add_row(vec![
        "County Zone",
        &presenter.resource_identifier(Some(&properties.county)),
    ]);
    table.add_row(vec![
        "Fire Weather Zone",
        &presenter.resource_identifier(Some(&properties.fire_weather_zone)),
    ]);
    table.add_row(vec![
        "Time Zone",
        &presenter.text(properties.time_zone.iana_name()),
    ]);
    table.add_row(vec![
        "Radar Station",
        &presenter.resource_identifier(Some(&properties.radar_station)),
    ]);

    table
}

impl DefaultPresentation for Feature<Point> {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_point_metadata_table(
            self, presenter,
        )))
    }
}
