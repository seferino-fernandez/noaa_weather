use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::Feature;
use noaa_weather_client::models::{Forecast, ForecastGenerator, ForecastPeriod, Gridpoint};

use super::{DefaultPresentation, DefaultPresenter, PresentationDocument, PresentationError};

/// Formats raw gridpoint data into a `comfy_table::Table`.
fn create_gridpoint_table(
    gridpoint_data: &Feature<Gridpoint>,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Property", "Value"]);

    let props = &gridpoint_data.properties;

    table.add_row([
        "Forecast Office",
        &presenter.text(Some(&props.forecast_office)),
    ]);
    table.add_row(["Grid ID", &presenter.text(Some(&props.grid_id.to_string()))]);
    table.add_row(["Grid X", &props.grid_x.to_string()]);
    table.add_row(["Grid Y", &props.grid_y.to_string()]);
    table.add_row([
        "Update Time",
        &presenter.offset_date_time(&props.update_time),
    ]);
    table.add_row(["Elevation", &presenter.elevation(&props.elevation)]);

    table
}

/// Renders the wind cell shared by both forecast tables.
fn wind(period: &ForecastPeriod, presenter: &DefaultPresenter) -> String {
    presenter.forecast_wind(
        &period.wind_speed,
        period.wind_gust.as_ref(),
        period
            .wind_direction
            .map(|direction| direction.to_string())
            .as_deref(),
    )
}

/// Formats the multi-day 12-hour forecast into a comfy table.
fn create_forecast_table(forecast_data: &Feature<Forecast>, presenter: &DefaultPresenter) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Period", "Time", "Temp", "Wind", "Forecast"]);

    let periods = &forecast_data.properties.periods;
    if periods.is_empty() {
        table.add_row(vec![
            Cell::new("No forecast periods found.")
                .add_attribute(comfy_table::Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ]);
        return table;
    }

    for period in periods {
        let temp_str = presenter.quantitative_value(&period.temperature);
        let wind_str = wind(period, presenter);
        let start_time_formatted = presenter.offset_date_time(&period.start_time);
        let end_time_formatted = presenter.offset_date_time(&period.end_time);

        table.add_row(vec![
            period.name.as_deref().unwrap_or("-"),
            &format!("{start_time_formatted} to {end_time_formatted}"),
            &temp_str,
            &wind_str.trim(),
            &period.short_forecast,
        ]);
    }

    table
}

/// Formats the hourly forecast into a comfy table.
fn create_hourly_forecast_table(
    forecast_data: &Feature<Forecast>,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "Hour", "Temp", "Dewpoint", "Precip", "Humidity", "Wind", "Forecast",
    ]);

    let periods = &forecast_data.properties.periods;
    if periods.is_empty() {
        table.add_row(vec![
            Cell::new("No hourly forecast periods found.")
                .add_attribute(comfy_table::Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ]);
        return table;
    }

    for period in periods {
        let temp_str = presenter.quantitative_value(&period.temperature);
        let dewpoint_str = period.dewpoint.as_ref().map_or_else(
            || presenter.missing(),
            |value| presenter.rounded_temperature(value),
        );
        let precip_str = presenter.percentage(&period.probability_of_precipitation);
        let humidity_str = period
            .relative_humidity
            .as_ref()
            .map_or_else(|| presenter.missing(), |value| presenter.percentage(value));
        let wind_str = wind(period, presenter);
        let time_formatted = presenter.offset_date_time(&period.start_time);

        table.add_row(vec![
            &time_formatted,
            &temp_str,
            &dewpoint_str,
            &precip_str,
            &humidity_str,
            wind_str.trim(),
            &period.short_forecast,
        ]);
    }

    table
}

impl DefaultPresentation for Feature<Gridpoint> {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_gridpoint_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for Feature<Forecast> {
    /// Both forecast endpoints return this type; the generator says which
    /// one produced it, and the hourly table carries the dewpoint and
    /// humidity columns NOAA only fills in there.
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        let table = match self.properties.forecast_generator {
            ForecastGenerator::Hourly => create_hourly_forecast_table(self, presenter),
            _ => create_forecast_table(self, presenter),
        };
        Ok(PresentationDocument::table(table))
    }
}
