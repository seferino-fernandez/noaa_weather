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

    // Helper macro to add rows for `Option<T>` properties
    macro_rules! add_row_if_some {
        ($table:ident, $label:expr, $value:expr) => {
            if let Some(ref val) = $value {
                $table.add_row(vec![$label, &format!("{val}")]);
            }
        };
        ($table:ident, $label:expr, $value:expr, $formatter:expr) => {
            if let Some(ref val) = $value {
                $table.add_row(vec![$label, &$formatter(val)]);
            }
        };
    }

    add_row_if_some!(
        table,
        "Forecast Office",
        properties.forecast_office,
        |value: &String| presenter.resource_identifier(Some(value))
    );
    add_row_if_some!(table, "Grid ID", properties.grid_id);
    add_row_if_some!(table, "Grid X", properties.grid_x);
    add_row_if_some!(table, "Grid Y", properties.grid_y);
    add_row_if_some!(
        table,
        "Forecast Zone",
        properties.forecast_zone,
        |value: &String| presenter.resource_identifier(Some(value))
    );
    add_row_if_some!(table, "County Zone", properties.county, |value: &String| {
        presenter.resource_identifier(Some(value))
    });
    add_row_if_some!(
        table,
        "Fire Weather Zone",
        properties.fire_weather_zone,
        |value: &String| presenter.resource_identifier(Some(value))
    );
    add_row_if_some!(
        table,
        "Time Zone",
        properties.time_zone,
        |value: &String| { presenter.text(Some(value)) }
    );
    add_row_if_some!(
        table,
        "Radar Station",
        properties.radar_station,
        |value: &String| presenter.resource_identifier(Some(value))
    );

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
