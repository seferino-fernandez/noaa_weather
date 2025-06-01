use anyhow::Result;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{Office, OfficeHeadline, OfficeHeadlineCollection};

use crate::utils::format::format_datetime_human_readable;

/// Formats an Office's metadata into a comfy_table::Table.
///
/// This function constructs a table displaying various attributes of an Office.
///
pub fn create_office_metadata_table(office: &Office) -> Result<Table> {
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

    // Handle simple fields with robust "N/A" for None or empty strings
    let office_id = office
        .id
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");
    let name = office
        .name
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");

    // Dynamically construct the address string, handling nested Option
    let (street, city, state, zip_code) = if let Some(addr_details) = office.address.as_ref() {
        // If office.address is Some, extract its fields, trim, and default to empty string if None
        (
            addr_details.street_address.as_deref().unwrap_or("").trim(),
            addr_details.city.as_deref().unwrap_or("").trim(),
            addr_details.state.as_deref().unwrap_or("").trim(),
            addr_details.zip_code.as_deref().unwrap_or("").trim(),
        )
    } else {
        // If office.address is None, all address components are effectively empty
        ("", "", "", "")
    };

    // Build the "City, State Zip" line from extracted components
    let mut csz_line = String::new();
    if !city.is_empty() {
        csz_line.push_str(city);
    }

    if !state.is_empty() {
        if !csz_line.is_empty() {
            // City was added
            csz_line.push_str(", ");
        }
        csz_line.push_str(state);
    }

    if !zip_code.is_empty() {
        if !csz_line.is_empty() {
            // City and/or state was added
            csz_line.push(' ');
        }
        csz_line.push_str(zip_code);
    }

    // Combine street with csz_line
    let mut address_lines = Vec::new();
    if !street.is_empty() {
        address_lines.push(street.to_string());
    }
    if !csz_line.is_empty() {
        address_lines.push(csz_line);
    }

    let final_address_str = address_lines.join("\n");

    let address_cell_content = if final_address_str.is_empty() {
        "N/A".to_string()
    } else {
        final_address_str
    };

    let phone = office
        .phone_number
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");
    let email = office
        .email
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");
    let website = office
        .website_url
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");
    let region = office
        .nws_region
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");

    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(name),
        Cell::new(address_cell_content),
        Cell::new(phone),
        Cell::new(email),
        Cell::new(website),
        Cell::new(region),
    ]);
    Ok(table)
}

/// Formats an Office's headlines into a comfy_table::Table.
///
/// This function constructs a table displaying various attributes of an Office.
///
pub fn create_office_headlines_table(office_headlines: &OfficeHeadlineCollection) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Title")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Summary")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issuance Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Link")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    for headline in office_headlines.at_graph.iter() {
        let headline_id = headline
            .id
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("N/A");
        let title = headline
            .title
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("N/A");
        let summary = headline
            .summary
            .clone()
            .flatten()
            .filter(|s| !s.is_empty())
            .unwrap_or("N/A".to_owned());
        let issuance_time = headline.issuance_time.as_deref().filter(|s| !s.is_empty());
        let issuance_time_readable = format_datetime_human_readable(issuance_time);
        let link = headline
            .link
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("N/A");

        table.add_row(vec![
            Cell::new(headline_id),
            Cell::new(title),
            Cell::new(summary),
            Cell::new(issuance_time_readable),
            Cell::new(link),
        ]);
    }

    Ok(table)
}

/// Formats an Office's headline into a comfy_table::Table.
///
/// This function constructs a table displaying various attributes of an Office.
///
pub fn create_office_headline_table(office_headline: &OfficeHeadline) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Title")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Summary")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Issuance Time")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Link")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);
    let headline_id = office_headline
        .id
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");
    let title = office_headline
        .title
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");
    let summary = office_headline
        .summary
        .clone()
        .flatten()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A".to_owned());
    let issuance_time = office_headline
        .issuance_time
        .as_deref()
        .filter(|s| !s.is_empty());
    let issuance_time_readable = format_datetime_human_readable(issuance_time);
    let link = office_headline
        .link
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("N/A");

    table.add_row(vec![
        Cell::new(headline_id),
        Cell::new(title),
        Cell::new(summary),
        Cell::new(issuance_time_readable),
        Cell::new(link),
    ]);

    Ok(table)
}
