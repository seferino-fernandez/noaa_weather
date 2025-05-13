use anyhow::Result;
use comfy_table::presets::{UTF8_FULL_CONDENSED, UTF8_HORIZONTAL_ONLY};
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    CenterWeatherAdvisoryCollectionGeoJson, CenterWeatherAdvisoryGeoJson, Office,
    SigmetCollectionGeoJson, SigmetGeoJson,
};

use crate::utils::format::format_datetime_human_readable;

/// Formats a collection of aviation offices into a comfy table.
pub fn create_cwsu_table(office: &Office) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec![
        Cell::new("ID").set_alignment(CellAlignment::Center),
        Cell::new("Name").set_alignment(CellAlignment::Center),
        Cell::new("Address").set_alignment(CellAlignment::Center),
        Cell::new("Phone").set_alignment(CellAlignment::Center),
        Cell::new("Email").set_alignment(CellAlignment::Center),
        Cell::new("Website").set_alignment(CellAlignment::Center),
        Cell::new("Region").set_alignment(CellAlignment::Center),
    ]);
    let office_id = office.id.as_deref().unwrap_or("N/A");
    let name = office.name.as_deref().unwrap_or("N/A");
    let address = format!(
        "{}\n{}, {} {}",
        office
            .address
            .as_ref()
            .unwrap()
            .street
            .as_deref()
            .unwrap_or(""),
        office
            .address
            .as_ref()
            .unwrap()
            .city
            .as_deref()
            .unwrap_or(""),
        office
            .address
            .as_ref()
            .unwrap()
            .state
            .as_deref()
            .unwrap_or(""),
        office
            .address
            .as_ref()
            .unwrap()
            .zip_code
            .as_deref()
            .unwrap_or(""),
    );
    let phone = office.phone_number.as_deref().unwrap_or("N/A");
    let email = office.email.as_deref().unwrap_or("N/A");
    let website = office.website_url.as_deref().unwrap_or("N/A");
    let region = office.nws_region.as_deref().unwrap_or("N/A");

    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(name),
        Cell::new(address),
        Cell::new(phone),
        Cell::new(email),
        Cell::new(website),
        Cell::new(region),
    ]);
    Ok(table)
}

/// Formats a single aviation center weather advisory into a comfy table.
pub fn create_cwa_table(cwa: &CenterWeatherAdvisoryGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec![
        Cell::new("ID").set_alignment(CellAlignment::Center),
        Cell::new("Issue Time").set_alignment(CellAlignment::Center),
        Cell::new("CWSU").set_alignment(CellAlignment::Center),
        Cell::new("Sequence").set_alignment(CellAlignment::Center),
        Cell::new("Start and End").set_alignment(CellAlignment::Center),
        Cell::new("Observed Property").set_alignment(CellAlignment::Center),
        Cell::new("Text").set_alignment(CellAlignment::Center),
    ]);
    let office_id = cwa.properties.as_ref().id.as_deref().unwrap_or("N/A");
    let issue_time = cwa.properties.as_ref().issue_time.as_deref();
    let issue_time_str = format_datetime_human_readable(issue_time);
    let cwsu = cwa.properties.as_ref().cwsu;
    let cwsu_str = if let Some(cwsu) = cwsu {
        cwsu.to_string()
    } else {
        "N/A".to_string()
    };
    let sequence = cwa.properties.as_ref().sequence;
    let sequence_str = if let Some(sequence) = sequence {
        sequence.to_string()
    } else {
        "N/A".to_string()
    };
    let start = cwa.properties.as_ref().start.as_deref();
    let end = cwa.properties.as_ref().end.as_deref();
    let start_and_end = format!(
        "{}\nto\n{}",
        format_datetime_human_readable(start),
        format_datetime_human_readable(end)
    );
    let observed_property = cwa.properties.as_ref().observed_property.as_deref();
    let observed_property_str = if let Some(observed_property) = observed_property {
        observed_property.to_string()
    } else {
        "N/A".to_string()
    };
    let text = cwa.properties.as_ref().text.as_deref();
    let text_str = if let Some(text) = text {
        text.to_string()
    } else {
        "N/A".to_string()
    };
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
pub fn create_cwas_table(cwas: &CenterWeatherAdvisoryCollectionGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_HORIZONTAL_ONLY);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec![
        Cell::new("ID").set_alignment(CellAlignment::Center),
        Cell::new("Issue Time").set_alignment(CellAlignment::Center),
        Cell::new("CWSU").set_alignment(CellAlignment::Center),
        Cell::new("Sequence").set_alignment(CellAlignment::Center),
        Cell::new("Start and End").set_alignment(CellAlignment::Center),
        Cell::new("Observed Property").set_alignment(CellAlignment::Center),
        Cell::new("Text").set_alignment(CellAlignment::Center),
    ]);
    for cwa in cwas.features.iter() {
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
            .map(|cwsu| cwsu.to_string());
        let cwsu = cwsu_binding.as_deref().unwrap_or("N/A");
        let sequence = cwa.properties.as_ref().unwrap().sequence;
        let sequence_str = if let Some(sequence) = sequence {
            sequence.to_string()
        } else {
            "N/A".to_string()
        };
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
    Ok(table)
}

/// Formats a single aviation SIGMET into a comfy table.
pub fn create_sigmet_table(sigmet: &SigmetGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec![
        Cell::new("ID").set_alignment(CellAlignment::Center),
        Cell::new("Issue Time").set_alignment(CellAlignment::Center),
        Cell::new("FIR").set_alignment(CellAlignment::Center),
        Cell::new("ATSU").set_alignment(CellAlignment::Center),
        Cell::new("Sequence").set_alignment(CellAlignment::Center),
        Cell::new("Phenomenon").set_alignment(CellAlignment::Center),
        Cell::new("Start and End").set_alignment(CellAlignment::Center),
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
    let sequence_str = sequence.to_string();
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
    let phenomenon_str = phenomenon.to_string();
    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(issue_time_str),
        Cell::new(fir),
        Cell::new(atsu),
        Cell::new(sequence_str),
        Cell::new(phenomenon_str),
        Cell::new(start_and_end),
    ]);
    Ok(table)
}

/// Formats a collection of aviation SIGMETs into a comfy table.
pub fn create_sigmets_table(sigmets: &SigmetCollectionGeoJson) -> Result<Table> {
    let mut table = Table::new();
    table.load_preset(UTF8_HORIZONTAL_ONLY);
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_header(vec![
        Cell::new("ID").set_alignment(CellAlignment::Center),
        Cell::new("Issue Time").set_alignment(CellAlignment::Center),
        Cell::new("FIR").set_alignment(CellAlignment::Center),
        Cell::new("ATSU").set_alignment(CellAlignment::Center),
        Cell::new("Sequence").set_alignment(CellAlignment::Center),
        Cell::new("Phenomenon").set_alignment(CellAlignment::Center),
        Cell::new("Start and End").set_alignment(CellAlignment::Center),
    ]);
    for sigmet in sigmets.features.iter() {
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
        let sequence_str = sequence.to_string();
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
        let phenomenon_str = phenomenon.to_string();
        table.add_row(vec![
            Cell::new(office_id),
            Cell::new(issue_time_str),
            Cell::new(fir),
            Cell::new(atsu),
            Cell::new(sequence_str),
            Cell::new(phenomenon_str),
            Cell::new(start_and_end),
        ]);
    }
    Ok(table)
}
