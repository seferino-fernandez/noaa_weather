use crate::utils::format::{format_datetime_human_readable, format_dewpoint};
use anyhow::Result;
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

/// Formats raw gridpoint data into a comfy table.
pub fn format_gridpoint_table(gridpoint_data: &GridpointGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
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
                props
                    .elevation
                    .as_ref()
                    .and_then(|qv| qv.unit_code.as_deref())
                    .unwrap_or("m")
            )
        })
        .unwrap_or_else(|| "N/A".to_string());
    table.add_row(vec!["Elevation", &elevation_str]);

    Ok(table)
}

/// Formats the multi-day forecast into a comfy table.
pub fn format_forecast_table(forecast_data: &GridpointForecastGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec!["Period", "Time", "Temp", "Wind", "Forecast"]);

    let props = &forecast_data.properties;
    let default_unit = props.units.map(|u| u.to_string());

    if let Some(periods) = &props.periods {
        for period in periods {
            let temp_str = period.temperature.as_ref().map_or_else(
                || "N/A".to_string(),
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
                    .unwrap_or_else(|| "N/A".to_string()),
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
                &format!("{} to {}", start_time_formatted, end_time_formatted),
                &temp_str,
                &wind_str.trim(),
                period.short_forecast.as_deref().unwrap_or("-"),
            ]);
        }
    } else {
        table.add_row(vec![
            Cell::new("No forecast periods found.").set_alignment(CellAlignment::Center),
        ]);
    }

    Ok(table)
}

/// Formats the hourly forecast into a comfy table.
pub fn format_hourly_forecast_table(forecast_data: &GridpointForecastGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec![
        "Hour", "Temp", "Dewpoint", "Precip", "Humidity", "Wind", "Forecast",
    ]);

    let props = &forecast_data.properties;
    let default_unit = props.units.map(|u| u.to_string());

    if let Some(periods) = &props.periods {
        for period in periods {
            let temp_str = period.temperature.as_ref().map_or_else(
                || "N/A".to_string(),
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
                        // format_dewpoint from utils::temperature expects value as String,
                        // api unit as Option<&str>, and target unit as Option<&str>.
                        // We pass None for target unit, meaning it will format based on api_unit.
                        format_dewpoint(
                            value.to_string(),
                            quantitative_value.unit_code.as_deref(),
                            None, // Display as per API unit; conversion (if any) handled by API based on request.
                        )
                    })
                })
                .unwrap_or_else(|| "N/A".to_string());

            let precip_str = period.probability_of_precipitation.as_ref().map_or_else(
                || "N/A".to_string(),
                |pop_qv| {
                    pop_qv
                        .value
                        .flatten()
                        .map_or_else(|| "N/A".to_string(), |v| format!("{:.0}%", v))
                },
            );

            let humidity_str = period.relative_humidity.as_ref().map_or_else(
                || "N/A".to_string(),
                |rh_qv| {
                    rh_qv
                        .value
                        .flatten()
                        .map_or_else(|| "N/A".to_string(), |v| format!("{:.0}%", v))
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
                    .unwrap_or_else(|| "N/A".to_string()),
                period
                    .wind_direction
                    .map(|gridpoint_forecast_period_wind_direction| {
                        gridpoint_forecast_period_wind_direction
                            .unwrap_or_default()
                            .to_string()
                    })
                    .unwrap_or_else(|| "".to_string())
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
            Cell::new("No hourly forecast periods found.").set_alignment(CellAlignment::Center),
        ]);
    }

    Ok(table)
}

// Helper to format temperature (which can be QuantitativeValue or Integer)
fn format_temperature(temp: &GridpointForecastPeriodTemperature, unit: Option<&str>) -> String {
    match temp {
        GridpointForecastPeriodTemperature::QuantitativeValue(qv) => {
            let value_str = qv
                .value
                .flatten()
                .map_or_else(|| "N/A".to_string(), |v| format!("{:.0}", v));
            let unit_str = qv.unit_code.as_deref().unwrap_or(unit.unwrap_or("?"));
            format!(
                "{}°{}",
                value_str,
                unit_str.split(':').next_back().unwrap_or(unit_str)
            )
        }
        GridpointForecastPeriodTemperature::Integer(i) => {
            format!("{}°{}", i, unit.unwrap_or("?"))
        }
    }
}
