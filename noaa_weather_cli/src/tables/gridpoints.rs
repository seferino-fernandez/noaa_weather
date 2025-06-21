use crate::utils::format::{format_datetime_human_readable, format_dewpoint};
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    GridpointForecastGeoJson, GridpointForecastPeriodTemperature, GridpointGeoJson,
};

macro_rules! add_row_if_some {
    ($table:ident, $label:expr, $value:expr) => {
        if let Some(ref val) = $value {
            $table.add_row(vec![$label, &format!("{}", val)]);
        }
    };
}

/// Formats raw gridpoint data into a `comfy_table::Table`.
pub fn create_gridpoint_table(gridpoint_data: &GridpointGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Property", "Value"]);

    let props = &gridpoint_data.properties;

    add_row_if_some!(table, "Forecast Office", props.forecast_office);
    add_row_if_some!(table, "Grid ID", props.grid_id);
    add_row_if_some!(table, "Grid X", props.grid_x);
    add_row_if_some!(table, "Grid Y", props.grid_y);
    add_row_if_some!(table, "Update Time", props.update_time);

    // Add elevation if available
    let elevation_str = props
        .elevation
        .as_ref()
        .and_then(|qv| qv.value)
        .flatten()
        .map(|v| {
            format!(
                "{:.1} {}",
                v,
                {
                    let qv = props.elevation.as_ref();
                    qv.unwrap().unit_code.as_deref()
                }
                .unwrap_or("m")
            )
        })
        .unwrap_or_else(|| "N/A".to_owned());
    table.add_row(vec!["Elevation", &elevation_str]);

    table
}

/// Formats the multi-day forecast into a comfy table.
pub fn create_forecast_table(forecast_data: &GridpointForecastGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Period", "Time", "Temp", "Wind", "Forecast"]);

    let props = &forecast_data.properties;
    let default_unit = props.units.map(|unit| unit.to_string());

    if let Some(periods) = &props.periods {
        for period in periods {
            let temp_str = period.temperature.as_ref().map_or_else(
                || "N/A".to_owned(),
                |gridpoint_forecast_period_temperature| {
                    format_temperature(
                        gridpoint_forecast_period_temperature,
                        period
                            .temperature_unit
                            .map(|gridpoint_forecast_period_temperature_unit| {
                                gridpoint_forecast_period_temperature_unit.to_string()
                            })
                            .as_deref()
                            .or(default_unit.as_deref()),
                    )
                },
            );

            let wind_str = format!(
                "{} {}",
                period
                    .wind_speed
                    .as_ref()
                    .map(|gridpoint_forecast_period_wind_speed| {
                        gridpoint_forecast_period_wind_speed.to_string()
                    })
                    .unwrap_or_else(|| "N/A".to_owned()),
                period
                    .wind_direction
                    .map(|gridpoint_forecast_period_wind_direction| {
                        gridpoint_forecast_period_wind_direction.unwrap_or_default()
                    })
                    .unwrap_or_default()
            );

            let start_time_formatted = format_datetime_human_readable(period.start_time.as_deref());
            let end_time_formatted = format_datetime_human_readable(period.end_time.as_deref());

            table.add_row(vec![
                period.name.as_deref().unwrap_or("-"),
                &format!("{start_time_formatted} to {end_time_formatted}"),
                &temp_str,
                &wind_str.trim(),
                period.short_forecast.as_deref().unwrap_or("-"),
            ]);
        }
    } else {
        table.add_row(vec![
            Cell::new("No forecast periods found.")
                .add_attribute(comfy_table::Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ]);
    }

    table
}

/// Formats the hourly forecast into a comfy table.
pub fn create_hourly_forecast_table(forecast_data: &GridpointForecastGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "Hour", "Temp", "Dewpoint", "Precip", "Humidity", "Wind", "Forecast",
    ]);

    let props = &forecast_data.properties;
    let default_unit = props.units.map(|unit| unit.to_string());

    if let Some(periods) = &props.periods {
        for period in periods {
            let temp_str = period.temperature.as_ref().map_or_else(
                || "N/A".to_owned(),
                |gridpoint_forecast_period_temperature| {
                    format_temperature(
                        gridpoint_forecast_period_temperature,
                        period
                            .temperature_unit
                            .map(|gridpoint_forecast_period_temperature_unit| {
                                gridpoint_forecast_period_temperature_unit.to_string()
                            })
                            .as_deref()
                            .or(default_unit.as_deref()),
                    )
                },
            );

            let dewpoint_str = period
                .dewpoint
                .as_ref()
                .and_then(|quantitative_value| {
                    quantitative_value.value.flatten().map(|value| {
                        format_dewpoint(
                            value.to_string(),
                            quantitative_value.unit_code.as_deref(),
                            None,
                        )
                    })
                })
                .unwrap_or_else(|| "N/A".to_owned());

            let precip_str = period.probability_of_precipitation.as_ref().map_or_else(
                || "N/A".to_owned(),
                |pop_qv| {
                    pop_qv
                        .value
                        .flatten()
                        .map_or_else(|| "N/A".to_owned(), |value| format!("{value:.0}%"))
                },
            );

            let humidity_str = period.relative_humidity.as_ref().map_or_else(
                || "N/A".to_owned(),
                |rh_qv| {
                    rh_qv
                        .value
                        .flatten()
                        .map_or_else(|| "N/A".to_owned(), |value| format!("{value:.0}%"))
                },
            );

            let wind_str = format!(
                "{} {}",
                period
                    .wind_speed
                    .as_ref()
                    .map(|gridpoint_forecast_period_wind_speed| {
                        gridpoint_forecast_period_wind_speed.to_string()
                    })
                    .unwrap_or_else(|| "N/A".to_owned()),
                period
                    .wind_direction
                    .map(|gridpoint_forecast_period_wind_direction| {
                        gridpoint_forecast_period_wind_direction
                            .unwrap_or_default()
                            .to_string()
                    })
                    .unwrap_or_else(String::new)
            );
            let time_formatted = format_datetime_human_readable(period.start_time.as_deref());

            table.add_row(vec![
                &time_formatted,
                &temp_str,
                &dewpoint_str,
                &precip_str,
                &humidity_str,
                wind_str.trim(),
                period.short_forecast.as_deref().unwrap_or("-"),
            ]);
        }
    } else {
        table.add_row(vec![
            Cell::new("No hourly forecast periods found.")
                .add_attribute(comfy_table::Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ]);
    }

    table
}

// Helper to format temperature (which can be QuantitativeValue or Integer)
fn format_temperature(temp: &GridpointForecastPeriodTemperature, unit: Option<&str>) -> String {
    match temp {
        GridpointForecastPeriodTemperature::QuantitativeValue(qv) => {
            let value_str = qv
                .value
                .flatten()
                .map_or_else(|| "N/A".to_owned(), |value| format!("{value:.0}"));
            let unit_str = qv.unit_code.as_deref().unwrap_or(unit.unwrap_or("?"));
            format!(
                "{}\u{b0}{}",
                value_str,
                unit_str.split(':').next_back().unwrap_or(unit_str)
            )
        }
        GridpointForecastPeriodTemperature::Integer(i) => {
            format!("{}\u{b0}{}", i, unit.unwrap_or("?"))
        }
    }
}
