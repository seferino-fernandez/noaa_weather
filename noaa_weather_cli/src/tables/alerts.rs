use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};
use noaa_weather_client::models::{
    ActiveAlertsCountResponse, Alert, AlertCollectionGeoJson, AlertGeoJson, AlertSeverity,
    AlertTypesResponse,
};

use crate::utils::format::{
    format_datetime_human_readable, format_optional_number, get_zone_from_url,
};

/// Formats a collection of alerts into a comfy table.
/// Displays a summary of each alert, highlighting severity with color.
pub fn create_alerts_table(alerts_data: &AlertCollectionGeoJson) -> comfy_table::Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Alert")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Areas Affected")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Effective")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Severity")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Instructions")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("ID")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    if alerts_data.features.is_empty() {
        table.add_row(vec![
            Cell::new("No active alerts found.")
                .add_attribute(comfy_table::Attribute::Bold)
                .set_alignment(CellAlignment::Center),
        ]);
        return table;
    }

    for feature in &alerts_data.features {
        if let Some(alert_properties_box) = &feature.properties {
            let alert_properties = &**alert_properties_box;

            let mut severity_cell = Cell::new(
                alert_properties
                    .severity
                    .map_or_else(|| "N/A".to_string(), |s| s.to_string()),
            );
            if let Some(severity_value) = alert_properties.severity {
                match severity_value {
                    AlertSeverity::Extreme => {
                        severity_cell = severity_cell.fg(Color::Red).add_attribute(Attribute::Bold);
                    }
                    AlertSeverity::Severe => severity_cell = severity_cell.fg(Color::Red),
                    AlertSeverity::Moderate => severity_cell = severity_cell.fg(Color::Yellow),
                    AlertSeverity::Minor => severity_cell = severity_cell.fg(Color::Green),
                    AlertSeverity::Unknown => {}
                }
            }

            let event_headline = format!(
                "{}\n\n{}",
                alert_properties.sender_name.as_deref().unwrap_or("N/A"),
                alert_properties
                    .headline
                    .clone()
                    .flatten()
                    .as_deref()
                    .unwrap_or("N/A")
            );
            let effective_date =
                format_datetime_human_readable(alert_properties.effective.as_deref());
            let expires_date = format_datetime_human_readable(alert_properties.expires.as_deref());

            let effective_date = format!("{effective_date}\nto\n{expires_date}");
            table.add_row(vec![
                Cell::new(event_headline),
                Cell::new(alert_properties.area_desc.as_deref().unwrap_or("N/A")),
                Cell::new(effective_date),
                severity_cell,
                Cell::new(
                    alert_properties
                        .instruction
                        .clone()
                        .flatten()
                        .as_deref()
                        .unwrap_or("N/A"),
                ),
                Cell::new(alert_properties.id.as_deref().unwrap_or("N/A")),
            ]);
        }
    }
    table
}

/// Formats a single alert's details into a comfy table.
pub fn create_single_alert_table(alert_data: &AlertGeoJson) -> comfy_table::Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Alert Details")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Alert Description")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    let alert: &Alert = &alert_data.properties;

    // Collect all details as strings
    let mut details = vec![
        format!(
            "ID: {}",
            alert.id.clone().unwrap_or_else(|| "N/A".to_string())
        ),
        format!(
            "Event: {}",
            alert.event.clone().unwrap_or_else(|| "N/A".to_string())
        ),
        format!(
            "Headline: {}",
            alert
                .headline
                .clone()
                .flatten()
                .unwrap_or_else(|| "N/A".to_string())
        ),
        format!(
            "Area Description: {}",
            alert.area_desc.clone().unwrap_or_else(|| "N/A".to_string())
        ),
        format!(
            "Sender Name: {}",
            alert
                .sender_name
                .clone()
                .unwrap_or_else(|| "N/A".to_string())
        ),
        format!(
            "Sent: {}",
            format_datetime_human_readable(alert.sent.as_deref())
        ),
        format!(
            "Effective: {}",
            format_datetime_human_readable(alert.effective.as_deref())
        ),
        format!(
            "Onset: {}",
            format_datetime_human_readable(alert.onset.clone().flatten().as_deref())
        ),
        format!(
            "Expires: {}",
            format_datetime_human_readable(alert.expires.as_deref())
        ),
        format!(
            "Ends: {}",
            format_datetime_human_readable(alert.ends.clone().flatten().as_deref())
        ),
        format!(
            "Status: {}",
            alert
                .status
                .map_or("N/A".to_string(), |status| status.to_string())
        ),
        format!(
            "Message Type: {}",
            alert
                .message_type
                .map_or("N/A".to_string(), |message_type| message_type.to_string())
        ),
        format!(
            "Category: {}",
            alert
                .category
                .map_or("N/A".to_string(), |category| category.to_string())
        ),
        format!(
            "Severity: {}",
            alert
                .severity
                .map_or("N/A".to_string(), |severity| severity.to_string())
        ),
        format!(
            "Certainty: {}",
            alert
                .certainty
                .map_or("N/A".to_string(), |certainty| certainty.to_string())
        ),
        format!(
            "Urgency: {}",
            alert
                .urgency
                .map_or("N/A".to_string(), |urgency| urgency.to_string())
        ),
        format!(
            "Instruction: {}",
            alert
                .instruction
                .clone()
                .flatten()
                .unwrap_or_else(|| "N/A".to_string())
        ),
        format!(
            "Response: {}",
            alert
                .response
                .map_or("N/A".to_string(), |response| response.to_string())
        ),
    ];
    let formatted_affected_zones = if let Some(affected_zones_list) = &alert.affected_zones {
        affected_zones_list
            .iter()
            .filter_map(|zone| get_zone_from_url(Some(zone.clone())))
            .collect::<Vec<String>>()
            .join(", ")
    } else {
        "N/A".to_string()
    };

    details.push(format!("Affected Zones: {formatted_affected_zones}"));
    let description = alert
        .description
        .clone()
        .unwrap_or_else(|| "N/A".to_string());

    table.add_row(vec![Cell::new(details.join("\n")), Cell::new(description)]);
    table
}

/// Formats the active alerts count into a comfy table.
pub fn create_alert_count_table(count_data: &ActiveAlertsCountResponse) -> comfy_table::Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Active Alerts Summary")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Alerts by Area (State/Territory)")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Alerts by Marine Region")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
        Cell::new("Alerts by Zone")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    let active_alerts_summary_data = format!(
        "Total Active Alerts: {}\nLand Alerts: {}\nMarine Alerts: {}",
        format_optional_number(count_data.total),
        format_optional_number(count_data.land),
        format_optional_number(count_data.marine)
    );
    let mut active_alerts_by_area_data = String::new();
    if let Some(areas_map) = &count_data.areas
        && !areas_map.is_empty()
    {
        for (area_key, count_val) in areas_map {
            active_alerts_by_area_data.push_str(&format!("{area_key}: {count_val}\n"));
        }
    }

    let mut active_alerts_by_marine_region_data = String::new();
    if let Some(regions_map) = &count_data.regions
        && !regions_map.is_empty()
    {
        for (region_key, count_val) in regions_map {
            active_alerts_by_marine_region_data.push_str(&format!("{region_key}: {count_val}\n"));
        }
    }

    let mut active_alerts_by_zone_data = String::new();
    if let Some(zones_map) = &count_data.zones
        && !zones_map.is_empty()
    {
        for (zone_key, count_val) in zones_map {
            active_alerts_by_zone_data.push_str(&format!("{zone_key}: {count_val}\n"));
        }
    }

    table.add_row(vec![
        Cell::new(active_alerts_summary_data),
        Cell::new(active_alerts_by_area_data),
        Cell::new(active_alerts_by_marine_region_data),
        Cell::new(active_alerts_by_zone_data),
    ]);
    table
}

/// Formats the list of alert types into a comfy table.
pub fn create_alert_types_table(types_data: &AlertTypesResponse) -> comfy_table::Table {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Available NWS Alert Event Types")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    if let Some(event_types_vec) = &types_data.event_types {
        if event_types_vec.is_empty() {
            table.add_row(vec![Cell::new("No event types found.")]);
        } else {
            for event_type_str in event_types_vec {
                table.add_row(vec![Cell::new(event_type_str)]);
            }
        }
    } else {
        table.add_row(vec![Cell::new("No event types available.")]);
    }
    table
}
