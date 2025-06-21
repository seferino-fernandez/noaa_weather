use crate::utils::format::get_zone_from_url;

use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, CellAlignment, Table};
use noaa_weather_client::models::PointGeoJson;

/// Formats point metadata into a `comfy_table::Table`.
pub fn create_point_metadata_table(point_data: &PointGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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
        get_zone_from_url(properties.forecast_office.clone())
    );
    add_row_if_some!(table, "Grid ID", properties.grid_id);
    add_row_if_some!(table, "Grid X", properties.grid_x);
    add_row_if_some!(table, "Grid Y", properties.grid_y);
    add_row_if_some!(
        table,
        "Forecast Zone",
        get_zone_from_url(properties.forecast_zone.clone())
    );
    add_row_if_some!(
        table,
        "County Zone",
        get_zone_from_url(properties.county.clone())
    );
    add_row_if_some!(
        table,
        "Fire Weather Zone",
        get_zone_from_url(properties.fire_weather_zone.clone())
    );
    add_row_if_some!(table, "Time Zone", properties.time_zone);
    add_row_if_some!(
        table,
        "Radar Station",
        get_zone_from_url(properties.radar_station.clone())
    );

    table
}
