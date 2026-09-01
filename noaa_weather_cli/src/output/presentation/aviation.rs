use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    CenterWeatherAdvisoryCollectionGeoJson, CenterWeatherAdvisoryGeoJson, CwsuOffice,
    SigmetCollectionGeoJson, SigmetGeoJson,
};

use super::{DefaultPresentation, DefaultPresenter, PresentationError};
use crate::output::PresentationDocument;

/// Formats a CWSU office's details into a `comfy_table::Table`.
fn create_cwsu_table(office: &CwsuOffice, presenter: &DefaultPresenter) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Name")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Address")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Phone")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Email")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Website")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Region")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    let office_id = presenter.text(office.id.as_deref());
    let name = presenter.text(office.name.as_deref());

    // Dynamically construct the address string
    // Retrieve and trim address components to handle None, empty, or whitespace-only strings
    let street = office.street.as_deref().unwrap_or("").trim();
    let city = office.city.as_deref().unwrap_or("").trim();
    let state = office.state.as_deref().unwrap_or("").trim();
    let zip_code = office.zip_code.as_deref().unwrap_or("").trim();

    // Build the "City, State Zip" line
    let mut csz_line = String::new();
    if !city.is_empty() {
        csz_line.push_str(city);
    }

    if !state.is_empty() {
        if !csz_line.is_empty() {
            // City was added, so prefix state with ", "
            csz_line.push_str(", ");
        }
        csz_line.push_str(state);
    }

    if !zip_code.is_empty() {
        if !csz_line.is_empty() {
            // Something (city and/or state) was added, so prefix zip with a space
            csz_line.push(' ');
        }
        csz_line.push_str(zip_code);
    }

    // Combine street with the csz_line, using a newline if both are present
    let mut address_lines = Vec::new();
    if !street.is_empty() {
        address_lines.push(street.to_owned());
    }
    if !csz_line.is_empty() {
        address_lines.push(csz_line);
    }

    let final_address_str = address_lines.join("\n");

    // Use "N/A" if the fully constructed address is empty, otherwise use the constructed string.
    let address_cell_content = presenter.text(Some(&final_address_str));

    // For phone, email, website, and region, also ensure empty strings become "N/A"
    // The original .map_or("N/A", |v| v) for phone would print an empty string if phone_number was Some("").
    // The .filter(|s| !s.is_empty()) pattern handles this more robustly.
    let phone = presenter.text(office.phone_number.as_deref());
    let email = presenter.text(office.email.as_deref());
    let website = presenter.text(office.website_url.as_deref());
    let region = presenter.text(office.nws_region.as_deref());

    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(name),
        Cell::new(address_cell_content), // Use the carefully formatted address
        Cell::new(phone),
        Cell::new(email),
        Cell::new(website),
        Cell::new(region),
    ]);
    table
}

/// Formats a single aviation center weather advisory into a comfy table.
fn create_cwa_table(
    cwa: &CenterWeatherAdvisoryGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issue Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("CWSU")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sequence")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Start and End")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Observed Property")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Text")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    let office_id = presenter.text(cwa.properties.as_ref().id.as_deref());
    let issue_time = cwa.properties.as_ref().issue_time.as_deref();
    let issue_time_str = presenter.timestamp("cwa.properties.issue_time", issue_time)?;
    let cwsu = cwa.properties.as_ref().cwsu;
    let cwsu = cwsu.map(|value| value.to_string());
    let cwsu_str = presenter.text(cwsu.as_deref());
    let sequence = cwa.properties.as_ref().sequence;
    let sequence_str = presenter.integer(sequence);
    let start = cwa.properties.as_ref().start.as_deref();
    let end = cwa.properties.as_ref().end.as_deref();
    let start_and_end = format!(
        "{}\nto\n{}",
        presenter.timestamp("cwa.properties.start", start)?,
        presenter.timestamp("cwa.properties.end", end)?
    );
    let observed_property = cwa.properties.as_ref().observed_property.as_deref();
    let observed_property_str = presenter.text(observed_property);
    let text = cwa.properties.as_ref().text.as_deref();
    let text_str = presenter.text(text);
    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(issue_time_str),
        Cell::new(cwsu_str),
        Cell::new(sequence_str),
        Cell::new(start_and_end),
        Cell::new(observed_property_str),
        Cell::new(text_str),
    ]);
    Ok(table)
}

/// Formats a collection of aviation center weather advisories into a comfy table.
fn create_cwas_table(
    cwas: &CenterWeatherAdvisoryCollectionGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issue Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("CWSU")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sequence")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Start and End")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Observed Property")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Text")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    for (index, cwa) in cwas.features.iter().enumerate() {
        let properties = cwa.properties.as_ref().unwrap();
        let office_id = presenter.text(properties.id.as_deref());
        let issue_time = cwa.properties.as_ref().unwrap().issue_time.as_deref();
        let issue_time_str = presenter.timestamp(
            format!("cwas.features[{index}].properties.issue_time"),
            issue_time,
        )?;
        let cwsu = properties.cwsu.map(|value| value.to_string());
        let cwsu = presenter.text(cwsu.as_deref());
        let sequence_str = presenter.integer(properties.sequence);
        let start = properties.start.as_deref();
        let end = properties.end.as_deref();
        let start_and_end = format!(
            "{}\nto\n{}",
            presenter.timestamp(format!("cwas.features[{index}].properties.start"), start)?,
            presenter.timestamp(format!("cwas.features[{index}].properties.end"), end)?
        );
        let observed_property = presenter.text(properties.observed_property.as_deref());
        let text = presenter.text(properties.text.as_deref());

        table.add_row(vec![
            Cell::new(office_id),
            Cell::new(issue_time_str),
            Cell::new(cwsu),
            Cell::new(sequence_str),
            Cell::new(start_and_end),
            Cell::new(observed_property),
            Cell::new(text),
        ]);
    }
    Ok(table)
}

/// Formats a single aviation SIGMET into a comfy table.
fn create_sigmet_table(
    sigmet: &SigmetGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issue Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("FIR")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("ATSU")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sequence")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Phenomenon")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Start and End")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    let office_id = presenter.text(sigmet.properties.as_ref().id.as_deref());
    let issue_time = sigmet.properties.as_ref().issue_time.as_deref();
    let issue_time_str = presenter.timestamp("sigmet.properties.issue_time", issue_time)?;
    let fir = presenter.text(
        sigmet
            .properties
            .as_ref()
            .fir
            .as_ref()
            .and_then(Option::as_deref),
    );
    let atsu = presenter.text(sigmet.properties.as_ref().atsu.as_deref());
    let sequence = presenter.text(
        sigmet
            .properties
            .as_ref()
            .sequence
            .as_ref()
            .and_then(Option::as_deref),
    );
    let start = sigmet.properties.as_ref().start.as_deref();
    let end = sigmet.properties.as_ref().end.as_deref();
    let start_and_end = format!(
        "{}\nto\n{}",
        presenter.timestamp("sigmet.properties.start", start)?,
        presenter.timestamp("sigmet.properties.end", end)?
    );
    let phenomenon = presenter.text(
        sigmet
            .properties
            .as_ref()
            .phenomenon
            .as_ref()
            .and_then(Option::as_deref),
    );
    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(issue_time_str),
        Cell::new(fir),
        Cell::new(atsu),
        Cell::new(sequence),
        Cell::new(phenomenon),
        Cell::new(start_and_end),
    ]);
    Ok(table)
}

/// Formats a collection of aviation SIGMETs into a comfy table.
fn create_sigmets_table(
    sigmets: &SigmetCollectionGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issue Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("FIR")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("ATSU")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Sequence")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Phenomenon")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Start and End")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    for (index, sigmet) in sigmets.features.iter().enumerate() {
        let properties = sigmet.properties.as_ref();
        let office_id = presenter.text(properties.id.as_deref());
        let issue_time_str = presenter.timestamp(
            format!("sigmets.features[{index}].properties.issue_time"),
            properties.issue_time.as_deref(),
        )?;
        let fir = presenter.text(properties.fir.as_ref().and_then(Option::as_deref));
        let atsu = presenter.text(properties.atsu.as_deref());
        let sequence = presenter.text(properties.sequence.as_ref().and_then(Option::as_deref));
        let start = properties.start.as_deref();
        let end = properties.end.as_deref();
        let start_and_end = format!(
            "{}\nto\n{}",
            presenter.timestamp(format!("sigmets.features[{index}].properties.start"), start)?,
            presenter.timestamp(format!("sigmets.features[{index}].properties.end"), end)?
        );
        let phenomenon = presenter.text(properties.phenomenon.as_ref().and_then(Option::as_deref));
        table.add_row(vec![
            Cell::new(office_id),
            Cell::new(issue_time_str),
            Cell::new(fir),
            Cell::new(atsu),
            Cell::new(sequence),
            Cell::new(phenomenon),
            Cell::new(start_and_end),
        ]);
    }
    Ok(table)
}

impl DefaultPresentation for CwsuOffice {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_cwsu_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for CenterWeatherAdvisoryGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_cwa_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for CenterWeatherAdvisoryCollectionGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_cwas_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for SigmetGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_sigmet_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for SigmetCollectionGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_sigmets_table(
            self, presenter,
        )?))
    }
}
