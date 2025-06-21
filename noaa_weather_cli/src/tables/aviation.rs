use std::{borrow, string};

use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    CenterWeatherAdvisoryCollectionGeoJson, CenterWeatherAdvisoryGeoJson, CwsuOffice,
    SigmetCollectionGeoJson, SigmetGeoJson,
};

use crate::utils::format::format_datetime_human_readable;

/// Formats a CWSU office's details into a `comfy_table::Table`.
pub fn create_cwsu_table(office: &CwsuOffice) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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

    let office_id = office
        .id
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("N/A");
    let name = office
        .name
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("N/A");

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
    let address_cell_content = if final_address_str.is_empty() {
        "N/A".to_owned()
    } else {
        final_address_str
    };

    // For phone, email, website, and region, also ensure empty strings become "N/A"
    // The original .map_or("N/A", |v| v) for phone would print an empty string if phone_number was Some("").
    // The .filter(|s| !s.is_empty()) pattern handles this more robustly.
    let phone = office
        .phone_number
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("N/A");
    let email = office
        .email
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("N/A");
    let website = office
        .website_url
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("N/A");
    let region = office
        .nws_region
        .as_deref()
        .filter(|value| !value.is_empty())
        .unwrap_or("N/A");

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
pub fn create_cwa_table(cwa: &CenterWeatherAdvisoryGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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
    let office_id = cwa.properties.as_ref().id.as_deref().unwrap_or("N/A");
    let issue_time = cwa.properties.as_ref().issue_time.as_deref();
    let issue_time_str = format_datetime_human_readable(issue_time);
    let cwsu = cwa.properties.as_ref().cwsu;
    let cwsu_str = cwsu.map_or_else(|| "N/A".to_owned(), |cwsu_value| cwsu_value.to_string());
    let sequence = cwa.properties.as_ref().sequence;
    let sequence_str = sequence.map_or_else(
        || "N/A".to_owned(),
        |sequence_value| sequence_value.to_string(),
    );
    let start = cwa.properties.as_ref().start.as_deref();
    let end = cwa.properties.as_ref().end.as_deref();
    let start_and_end = format!(
        "{}\nto\n{}",
        format_datetime_human_readable(start),
        format_datetime_human_readable(end)
    );
    let observed_property = cwa.properties.as_ref().observed_property.as_deref();
    let observed_property_str =
        observed_property.map_or_else(|| "N/A".to_owned(), borrow::ToOwned::to_owned);
    let text = cwa.properties.as_ref().text.as_deref();
    let text_str = text.map_or_else(|| "N/A".to_owned(), borrow::ToOwned::to_owned);
    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(issue_time_str),
        Cell::new(cwsu_str),
        Cell::new(sequence_str),
        Cell::new(start_and_end),
        Cell::new(observed_property_str),
        Cell::new(text_str),
    ]);
    table
}

/// Formats a collection of aviation center weather advisories into a comfy table.
pub fn create_cwas_table(cwas: &CenterWeatherAdvisoryCollectionGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
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
    for cwa in &cwas.features {
        let office_id = cwa
            .properties
            .as_ref()
            .unwrap()
            .id
            .as_deref()
            .unwrap_or("N/A");
        let issue_time = cwa.properties.as_ref().unwrap().issue_time.as_deref();
        let issue_time_str = format_datetime_human_readable(issue_time);
        let cwsu_binding = cwa
            .properties
            .as_ref()
            .unwrap()
            .cwsu
            .as_ref()
            .map(string::ToString::to_string);
        let cwsu = cwsu_binding.as_deref().unwrap_or("N/A");
        let sequence = cwa.properties.as_ref().unwrap().sequence;
        let sequence_str = sequence.map_or_else(
            || "N/A".to_owned(),
            |sequence_value| sequence_value.to_string(),
        );
        let start = cwa.properties.as_ref().unwrap().start.as_deref();
        let end = cwa.properties.as_ref().unwrap().end.as_deref();
        let start_and_end = format!(
            "{}\nto\n{}",
            format_datetime_human_readable(start),
            format_datetime_human_readable(end)
        );
        let observed_property = cwa
            .properties
            .as_ref()
            .unwrap()
            .observed_property
            .as_deref()
            .unwrap_or("N/A");
        let text = cwa
            .properties
            .as_ref()
            .unwrap()
            .text
            .as_deref()
            .unwrap_or("N/A");

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
    table
}

/// Formats a single aviation SIGMET into a comfy table.
pub fn create_sigmet_table(sigmet: &SigmetGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
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
    let office_id = sigmet.properties.as_ref().id.as_deref().unwrap_or("N/A");
    let issue_time = sigmet.properties.as_ref().issue_time.as_deref();
    let issue_time_str = format_datetime_human_readable(issue_time);
    let fir = sigmet
        .properties
        .as_ref()
        .fir
        .clone()
        .flatten()
        .unwrap_or("N/A".to_owned());
    let atsu = sigmet.properties.as_ref().atsu.as_deref().unwrap_or("N/A");
    let sequence = sigmet
        .properties
        .as_ref()
        .sequence
        .clone()
        .flatten()
        .unwrap_or("N/A".to_owned());
    let start = sigmet.properties.as_ref().start.as_deref();
    let end = sigmet.properties.as_ref().end.as_deref();
    let start_and_end = format!(
        "{}\nto\n{}",
        format_datetime_human_readable(start),
        format_datetime_human_readable(end)
    );
    let phenomenon = sigmet
        .properties
        .as_ref()
        .phenomenon
        .clone()
        .flatten()
        .unwrap_or("N/A".to_owned());
    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(issue_time_str),
        Cell::new(fir),
        Cell::new(atsu),
        Cell::new(sequence),
        Cell::new(phenomenon),
        Cell::new(start_and_end),
    ]);
    table
}

/// Formats a collection of aviation SIGMETs into a comfy table.
pub fn create_sigmets_table(sigmets: &SigmetCollectionGeoJson) -> Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
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
    for sigmet in &sigmets.features {
        let office_id = sigmet.properties.as_ref().id.as_deref().unwrap_or("N/A");
        let issue_time = sigmet.properties.as_ref().issue_time.as_deref();
        let issue_time_str = format_datetime_human_readable(issue_time);
        let fir = sigmet
            .properties
            .as_ref()
            .fir
            .clone()
            .flatten()
            .unwrap_or("N/A".to_owned());
        let atsu = sigmet.properties.as_ref().atsu.as_deref().unwrap_or("N/A");
        let sequence = sigmet
            .properties
            .as_ref()
            .sequence
            .clone()
            .flatten()
            .unwrap_or("N/A".to_owned());
        let start = sigmet.properties.as_ref().start.as_deref();
        let end = sigmet.properties.as_ref().end.as_deref();
        let start_and_end = format!(
            "{}\nto\n{}",
            format_datetime_human_readable(start),
            format_datetime_human_readable(end)
        );
        let phenomenon = sigmet
            .properties
            .as_ref()
            .phenomenon
            .clone()
            .flatten()
            .unwrap_or("N/A".to_owned());
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
    table
}
