use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{Attribute, Cell, CellAlignment, Color, ContentArrangement, Table};
use noaa_weather_client::models::{ActiveAlertCounts, Alert, AlertEventTypes, AlertSeverity};
use noaa_weather_client::{Feature, FeatureCollection};

use super::{DefaultPresentation, DefaultPresenter, PresentationError};
use crate::output::PresentationDocument;

/// Formats a collection of alerts into a comfy table.
/// Displays a summary of each alert, highlighting severity with color.
fn create_alerts_table(
    alerts_data: &FeatureCollection<Alert>,
    presenter: &DefaultPresenter,
) -> Table {
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
        return table;
    }

    for feature in &alerts_data.features {
        let alert_properties = &feature.properties;

        let mut severity_cell = Cell::new(alert_properties.severity.to_string());
        match alert_properties.severity {
            AlertSeverity::Extreme => {
                severity_cell = severity_cell.fg(Color::Red).add_attribute(Attribute::Bold);
            }
            AlertSeverity::Severe => severity_cell = severity_cell.fg(Color::Red),
            AlertSeverity::Moderate => severity_cell = severity_cell.fg(Color::Yellow),
            AlertSeverity::Minor => severity_cell = severity_cell.fg(Color::Green),
            AlertSeverity::Unknown => {}
        }

        let event_headline = format!(
            "{}\n\n{}",
            presenter.text(Some(&alert_properties.sender_name)),
            presenter.text(alert_properties.headline.as_deref())
        );
        let effective_date = presenter.offset_date_time(&alert_properties.effective);
        let expires_date = presenter.offset_date_time(&alert_properties.expires);

        let effective_date = format!("{effective_date}\nto\n{expires_date}");
        table.add_row(vec![
            Cell::new(event_headline),
            Cell::new(presenter.text(Some(&alert_properties.area_desc))),
            Cell::new(effective_date),
            severity_cell,
            Cell::new(presenter.text(alert_properties.instruction.as_deref())),
            Cell::new(presenter.text(Some(alert_properties.id.as_str()))),
        ]);
    }
    table
}

/// Formats a single alert's details into a comfy table.
fn create_single_alert_table(alert_data: &Feature<Alert>, presenter: &DefaultPresenter) -> Table {
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
        format!("ID: {}", presenter.text(Some(alert.id.as_str()))),
        format!("Event: {}", presenter.text(Some(&alert.event))),
        format!("Headline: {}", presenter.text(alert.headline.as_deref())),
        format!(
            "Area Description: {}",
            presenter.text(Some(&alert.area_desc))
        ),
        format!("Sender Name: {}", presenter.text(Some(&alert.sender_name))),
        format!("Sent: {}", presenter.offset_date_time(&alert.sent)),
        format!(
            "Effective: {}",
            presenter.offset_date_time(&alert.effective)
        ),
        format!(
            "Onset: {}",
            presenter.optional_offset_date_time(alert.onset.as_ref())
        ),
        format!("Expires: {}", presenter.offset_date_time(&alert.expires)),
        format!(
            "Ends: {}",
            presenter.optional_offset_date_time(alert.ends.as_ref())
        ),
        format!("Status: {}", alert.status),
        format!("Message Type: {}", alert.message_type),
        format!("Category: {}", alert.category),
        format!("Severity: {}", alert.severity),
        format!("Certainty: {}", alert.certainty),
        format!("Urgency: {}", alert.urgency),
        format!(
            "Instruction: {}",
            presenter.text(alert.instruction.as_deref())
        ),
        format!(
            "Response: {}",
            presenter.text(alert.response.map(|value| value.to_string()).as_deref())
        ),
    ];
    let affected_zones = alert
        .affected_zones
        .iter()
        .map(|zone| presenter.resource_identifier(Some(zone)))
        .collect::<Vec<_>>()
        .join(", ");
    let formatted_affected_zones = presenter.text(Some(&affected_zones));

    details.push(format!("Affected Zones: {formatted_affected_zones}"));
    if let Some(note) = &alert.note {
        details.push(format!("Note: {note}"));
    }
    let description = presenter.text(alert.description.as_deref());

    table.add_row(vec![Cell::new(details.join("\n")), Cell::new(description)]);
    table
}

/// Formats the active alerts count into a comfy table.
fn create_alert_count_table(count_data: &ActiveAlertCounts) -> Table {
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
        count_data.total, count_data.land, count_data.marine
    );
    let mut active_alerts_by_area_data = String::new();
    for (area_key, count_val) in &count_data.areas {
        active_alerts_by_area_data.push_str(&format!("{area_key}: {count_val}\n"));
    }

    let mut active_alerts_by_marine_region_data = String::new();
    for (region_key, count_val) in &count_data.regions {
        active_alerts_by_marine_region_data.push_str(&format!("{region_key}: {count_val}\n"));
    }

    let mut active_alerts_by_zone_data = String::new();
    for (zone_key, count_val) in &count_data.zones {
        active_alerts_by_zone_data.push_str(&format!("{zone_key}: {count_val}\n"));
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
fn create_alert_types_table(types_data: &AlertEventTypes) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Available NWS Alert Event Types")
            .add_attribute(comfy_table::Attribute::Bold)
            .set_alignment(CellAlignment::Center),
    ]);

    if types_data.event_types.is_empty() {
        table.add_row(vec![Cell::new("No event types found.")]);
    } else {
        for event_type_str in &types_data.event_types {
            table.add_row(vec![Cell::new(event_type_str)]);
        }
    }
    table
}

impl DefaultPresentation for FeatureCollection<Alert> {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_alerts_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for Feature<Alert> {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_single_alert_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for ActiveAlertCounts {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_alert_count_table(self)))
    }
}

impl DefaultPresentation for AlertEventTypes {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_alert_types_table(self)))
    }
}
