use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};
use noaa_weather_client::models::{
    ActiveAlertsCountResponse, Alert, AlertCollectionGeoJson, AlertGeoJson, AlertSeverity,
    AlertTypesResponse,
};

use super::{DefaultPresentation, DefaultPresenter, PresentationError};
use crate::output::PresentationDocument;

/// Formats a collection of alerts into a comfy table.
/// Displays a summary of each alert, highlighting severity with color.
fn create_alerts_table(
    alerts_data: &AlertCollectionGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
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
        return Ok(table);
    }

    for feature in &alerts_data.features {
        if let Some(alert_properties_box) = &feature.properties {
            let alert_properties = &**alert_properties_box;

            let severity = alert_properties.severity.map(|value| value.to_string());
            let mut severity_cell = Cell::new(presenter.text(severity.as_deref()));
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
                presenter.text(alert_properties.sender_name.as_deref()),
                presenter.text(
                    alert_properties
                        .headline
                        .as_ref()
                        .and_then(Option::as_deref)
                )
            );
            let effective_date = presenter.timestamp(
                "alerts.features[].properties.effective",
                alert_properties.effective.as_deref(),
            )?;
            let expires_date = presenter.timestamp(
                "alerts.features[].properties.expires",
                alert_properties.expires.as_deref(),
            )?;

            let effective_date = format!("{effective_date}\nto\n{expires_date}");
            table.add_row(vec![
                Cell::new(event_headline),
                Cell::new(presenter.text(alert_properties.area_desc.as_deref())),
                Cell::new(effective_date),
                severity_cell,
                Cell::new(
                    presenter.text(
                        alert_properties
                            .instruction
                            .as_ref()
                            .and_then(Option::as_deref),
                    ),
                ),
                Cell::new(presenter.text(alert_properties.id.as_deref())),
            ]);
        }
    }
    Ok(table)
}

/// Formats a single alert's details into a comfy table.
fn create_single_alert_table(
    alert_data: &AlertGeoJson,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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
        format!("ID: {}", presenter.text(alert.id.as_deref())),
        format!("Event: {}", presenter.text(alert.event.as_deref())),
        format!(
            "Headline: {}",
            presenter.text(alert.headline.as_ref().and_then(Option::as_deref))
        ),
        format!(
            "Area Description: {}",
            presenter.text(alert.area_desc.as_deref())
        ),
        format!(
            "Sender Name: {}",
            presenter.text(alert.sender_name.as_deref())
        ),
        format!(
            "Sent: {}",
            presenter.timestamp("alert.properties.sent", alert.sent.as_deref())?
        ),
        format!(
            "Effective: {}",
            presenter.timestamp("alert.properties.effective", alert.effective.as_deref())?
        ),
        format!(
            "Onset: {}",
            presenter.timestamp(
                "alert.properties.onset",
                alert.onset.as_ref().and_then(Option::as_deref),
            )?
        ),
        format!(
            "Expires: {}",
            presenter.timestamp("alert.properties.expires", alert.expires.as_deref())?
        ),
        format!(
            "Ends: {}",
            presenter.timestamp(
                "alert.properties.ends",
                alert.ends.as_ref().and_then(Option::as_deref),
            )?
        ),
        format!(
            "Status: {}",
            presenter.text(alert.status.map(|value| value.to_string()).as_deref())
        ),
        format!(
            "Message Type: {}",
            presenter.text(alert.message_type.map(|value| value.to_string()).as_deref())
        ),
        format!(
            "Category: {}",
            presenter.text(alert.category.map(|value| value.to_string()).as_deref())
        ),
        format!(
            "Severity: {}",
            presenter.text(alert.severity.map(|value| value.to_string()).as_deref())
        ),
        format!(
            "Certainty: {}",
            presenter.text(alert.certainty.map(|value| value.to_string()).as_deref())
        ),
        format!(
            "Urgency: {}",
            presenter.text(alert.urgency.map(|value| value.to_string()).as_deref())
        ),
        format!(
            "Instruction: {}",
            presenter.text(alert.instruction.as_ref().and_then(Option::as_deref))
        ),
        format!(
            "Response: {}",
            presenter.text(alert.response.map(|value| value.to_string()).as_deref())
        ),
    ];
    let affected_zones = alert.affected_zones.as_ref().map(|zones| {
        zones
            .iter()
            .map(|zone| presenter.resource_identifier(Some(zone)))
            .collect::<Vec<_>>()
            .join(", ")
    });
    let formatted_affected_zones = presenter.text(affected_zones.as_deref());

    details.push(format!("Affected Zones: {formatted_affected_zones}"));
    if let Some(Some(note)) = &alert.note {
        details.push(format!("Note: {note}"));
    }
    let description = presenter.text(alert.description.as_deref());

    table.add_row(vec![Cell::new(details.join("\n")), Cell::new(description)]);
    Ok(table)
}

/// Formats the active alerts count into a comfy table.
fn create_alert_count_table(
    count_data: &ActiveAlertsCountResponse,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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
        presenter.integer(count_data.total),
        presenter.integer(count_data.land),
        presenter.integer(count_data.marine)
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
fn create_alert_types_table(types_data: &AlertTypesResponse) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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

impl DefaultPresentation for AlertCollectionGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_alerts_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for AlertGeoJson {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_single_alert_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for ActiveAlertsCountResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_alert_count_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for AlertTypesResponse {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_alert_types_table(self)))
    }
}
