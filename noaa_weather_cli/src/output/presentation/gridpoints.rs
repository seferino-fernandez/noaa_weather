use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    Gridpoint12hForecastGeoJson, GridpointGeoJson, GridpointHourlyForecastGeoJson,
    QuantitativeValue,
};

use crate::output::{HumanDocument, HumanPresentation};
use crate::utils::format::{format_datetime_human_readable, format_dewpoint};

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
    table.load_style(UTF8_FULL_CONDENSED);
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

/// Formats the multi-day 12-hour forecast into a comfy table.
pub fn create_forecast_table(forecast_data: &Gridpoint12hForecastGeoJson) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Period", "Time", "Temp", "Wind", "Forecast"]);

    let props = &forecast_data.properties;
    if let Some(periods) = &props.periods {
        for period in periods {
            let temp_str = period.temperature.as_ref().map_or_else(
                || "N/A".to_owned(),
                |temperature| format_quantitative_value(temperature),
            );

            let wind_str = format_wind(
                period.wind_speed.as_deref(),
                period.wind_gust.as_ref().and_then(Option::as_deref),
                period
                    .wind_direction
                    .and_then(|direction| direction)
                    .map(|direction| direction.to_string())
                    .as_deref(),
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
pub fn create_hourly_forecast_table(forecast_data: &GridpointHourlyForecastGeoJson) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "Hour", "Temp", "Dewpoint", "Precip", "Humidity", "Wind", "Forecast",
    ]);

    let props = &forecast_data.properties;
    if let Some(periods) = &props.periods {
        for period in periods {
            let temp_str = period.temperature.as_ref().map_or_else(
                || "N/A".to_owned(),
                |temperature| format_quantitative_value(temperature),
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

            let wind_str = format_wind(
                period.wind_speed.as_deref(),
                period.wind_gust.as_ref().and_then(Option::as_deref),
                period
                    .wind_direction
                    .and_then(|direction| direction)
                    .map(|direction| direction.to_string())
                    .as_deref(),
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

fn format_quantitative_value(value: &QuantitativeValue) -> String {
    let Some(number) = value.value.flatten() else {
        return "N/A".to_owned();
    };
    let number = if number.fract() == 0.0 {
        format!("{number:.0}")
    } else {
        number.to_string()
    };
    match value.unit_code.as_deref() {
        Some(unit) => format!(
            "{number} {}",
            unit.rsplit([':', '/']).next().unwrap_or(unit)
        ),
        None => number,
    }
}

fn format_wind(
    speed: Option<&QuantitativeValue>,
    gust: Option<&QuantitativeValue>,
    direction: Option<&str>,
) -> String {
    let mut parts = vec![speed.map_or_else(|| "N/A".to_owned(), format_quantitative_value)];
    if let Some(direction) = direction {
        parts.push(direction.to_owned());
    }
    if let Some(gust) = gust {
        parts.push(format!("gust {}", format_quantitative_value(gust)));
    }
    parts.join(" ")
}

impl HumanPresentation for GridpointGeoJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_gridpoint_table(self))
    }
}

impl HumanPresentation for Gridpoint12hForecastGeoJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_forecast_table(self))
    }
}

impl HumanPresentation for GridpointHourlyForecastGeoJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_hourly_forecast_table(self))
    }
}

#[cfg(test)]
mod tests {
    use super::{format_quantitative_value, format_wind};
    use noaa_weather_client::models::QuantitativeValue;

    fn measurement(value: f64, unit: &str) -> QuantitativeValue {
        QuantitativeValue {
            value: Some(Some(value)),
            unit_code: Some(unit.to_owned()),
            ..QuantitativeValue::default()
        }
    }

    #[test]
    fn formats_quantitative_temperature_without_debug_output() {
        assert_eq!(
            format_quantitative_value(&measurement(72.0, "wmoUnit:degF")),
            "72 degF"
        );
    }

    #[test]
    fn formats_quantitative_speed_and_gust_units() {
        assert_eq!(
            format_wind(
                Some(&measurement(12.0, "wmoUnit:km_h-1")),
                Some(&measurement(
                    20.5,
                    "https://codes.wmo.int/common/unit/km_h-1"
                )),
                Some("NW"),
            ),
            "12 km_h-1 NW gust 20.5 km_h-1"
        );
    }
}
