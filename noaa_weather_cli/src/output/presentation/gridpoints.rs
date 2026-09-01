use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    Gridpoint12hForecastGeoJson, GridpointGeoJson, GridpointHourlyForecastGeoJson,
};

use super::{DefaultPresentation, DefaultPresenter, PresentationDocument, PresentationError};

/// Formats raw gridpoint data into a `comfy_table::Table`.
fn create_gridpoint_table(
    gridpoint_data: &GridpointGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Property", "Value"]);

    let props = &gridpoint_data.properties;

    table.add_row([
        "Forecast Office",
        &presenter.text(props.forecast_office.as_deref()),
    ]);
    table.add_row(["Grid ID", &presenter.text(props.grid_id.as_deref())]);
    table.add_row(["Grid X", &presenter.integer(props.grid_x)]);
    table.add_row(["Grid Y", &presenter.integer(props.grid_y)]);
    table.add_row([
        "Update Time",
        &presenter.timestamp(
            "gridpoint.properties.update_time",
            props.update_time.as_deref(),
        )?,
    ]);
    table.add_row([
        "Elevation",
        &presenter.elevation(props.elevation.as_deref()),
    ]);

    Ok(table)
}

/// Formats the multi-day 12-hour forecast into a comfy table.
fn create_forecast_table(
    forecast_data: &Gridpoint12hForecastGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Period", "Time", "Temp", "Wind", "Forecast"]);

    let props = &forecast_data.properties;
    if let Some(periods) = &props.periods {
        for (index, period) in periods.iter().enumerate() {
            let temp_str = presenter.quantitative_value(period.temperature.as_deref());

            let wind_str = presenter.forecast_wind(
                period.wind_speed.as_deref(),
                period.wind_gust.as_ref().and_then(Option::as_deref),
                period
                    .wind_direction
                    .and_then(|direction| direction)
                    .map(|direction| direction.to_string())
                    .as_deref(),
            );

            let start_time_formatted = presenter.timestamp(
                format!("gridpoint forecast period {index} start time"),
                period.start_time.as_deref(),
            )?;
            let end_time_formatted = presenter.timestamp(
                format!("gridpoint forecast period {index} end time"),
                period.end_time.as_deref(),
            )?;

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

    Ok(table)
}

/// Formats the hourly forecast into a comfy table.
fn create_hourly_forecast_table(
    forecast_data: &GridpointHourlyForecastGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "Hour", "Temp", "Dewpoint", "Precip", "Humidity", "Wind", "Forecast",
    ]);

    let props = &forecast_data.properties;
    if let Some(periods) = &props.periods {
        for (index, period) in periods.iter().enumerate() {
            let temp_str = presenter.quantitative_value(period.temperature.as_deref());

            let dewpoint_str = presenter.rounded_temperature(period.dewpoint.as_deref());

            let precip_str = presenter.percentage(period.probability_of_precipitation.as_deref());

            let humidity_str = presenter.percentage(period.relative_humidity.as_deref());

            let wind_str = presenter.forecast_wind(
                period.wind_speed.as_deref(),
                period.wind_gust.as_ref().and_then(Option::as_deref),
                period
                    .wind_direction
                    .and_then(|direction| direction)
                    .map(|direction| direction.to_string())
                    .as_deref(),
            );
            let time_formatted = presenter.timestamp(
                format!("hourly gridpoint forecast period {index} start time"),
                period.start_time.as_deref(),
            )?;

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

    Ok(table)
}

impl DefaultPresentation for GridpointGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_gridpoint_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for Gridpoint12hForecastGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_forecast_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for GridpointHourlyForecastGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_hourly_forecast_table(
            self, presenter,
        )?))
    }
}
