use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use noaa_weather_client::models::{
    NwsConnectDocumentMetadata, Office, OfficeBriefingResponse, OfficeHeadline,
    OfficeHeadlineCollection, OfficeWeatherStory, OfficeWeatherStoryCollection,
};

use super::{DefaultPresentation, DefaultPresenter, PresentationError};
use crate::output::PresentationDocument;

fn add_document_row(
    table: &mut Table,
    document: &NwsConnectDocumentMetadata,
    presenter: &DefaultPresenter,
) -> Result<(), PresentationError> {
    table.add_row(vec![
        Cell::new(presenter.text(document.id.as_deref())),
        Cell::new(presenter.text(document.title.as_deref())),
        Cell::new(presenter.text(document.description.as_deref())),
        Cell::new(
            presenter.timestamp("office.document.start_time", document.start_time.as_deref())?,
        ),
        Cell::new(presenter.timestamp("office.document.end_time", document.end_time.as_deref())?),
        Cell::new(presenter.text(document.download.as_deref())),
    ]);
    Ok(())
}

fn document_table() -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(["ID", "Title", "Description", "Starts", "Ends", "Download"]);
    table
}

/// Formats active office briefing metadata. An empty table means no active briefing.
fn create_office_briefing_table(
    response: &OfficeBriefingResponse,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = document_table();
    if let Some(briefing) = &response.briefing {
        add_document_row(&mut table, briefing, presenter)?;
    }
    Ok(table)
}

/// Formats active office weather-story metadata.
fn create_office_weather_stories_table(
    stories: &OfficeWeatherStoryCollection,
    presenter: &DefaultPresenter,
) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header([
        "ID",
        "Title",
        "Description",
        "Alt Text",
        "Order",
        "Download",
    ]);
    for story in &stories.stories {
        add_weather_story_row(&mut table, story, presenter);
    }
    table
}

fn add_weather_story_row(
    table: &mut Table,
    story: &OfficeWeatherStory,
    presenter: &DefaultPresenter,
) {
    table.add_row(vec![
        Cell::new(presenter.text(story.id.as_deref())),
        Cell::new(presenter.text(story.title.as_deref())),
        Cell::new(presenter.text(story.description.as_deref())),
        Cell::new(presenter.text(story.alt_text.as_deref())),
        Cell::new(presenter.integer(story.order)),
        Cell::new(presenter.text(story.download.as_deref())),
    ]);
}

/// Formats an `Office`'s metadata into a `comfy_table::Table`.
///
/// This function constructs a table displaying various attributes of an `Office`.
///
fn create_office_metadata_table(office: &Office, presenter: &DefaultPresenter) -> Table {
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

    // Handle simple fields with robust "N/A" for None or empty strings
    let office_id = presenter.text(office.id.as_deref());
    let name = presenter.text(office.name.as_deref());

    // Dynamically construct the address string, handling nested Option
    let (street, city, state, zip_code) =
        office
            .address
            .as_ref()
            .map_or(("", "", "", ""), |addr_details| {
                (
                    addr_details.street_address.as_deref().unwrap_or("").trim(),
                    addr_details.city.as_deref().unwrap_or("").trim(),
                    addr_details.state.as_deref().unwrap_or("").trim(),
                    addr_details.zip_code.as_deref().unwrap_or("").trim(),
                )
            });

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
        address_lines.push(street.to_owned());
    }
    if !csz_line.is_empty() {
        address_lines.push(csz_line);
    }

    let final_address_str = address_lines.join("\n");

    let address_cell_content = presenter.text(Some(&final_address_str));

    let phone = presenter.text(office.phone_number.as_deref());
    let email = presenter.text(office.email.as_deref());
    let website = presenter.text(office.website_url.as_deref());
    let region = presenter.text(office.nws_region.as_deref());

    table.add_row(vec![
        Cell::new(office_id),
        Cell::new(name),
        Cell::new(address_cell_content),
        Cell::new(phone),
        Cell::new(email),
        Cell::new(website),
        Cell::new(region),
    ]);
    table
}

/// Formats an Office's headlines into a `comfy_table::Table`.
///
/// This function constructs a table displaying various attributes of an Office.
///
fn create_office_headlines_table(
    office_headlines: &OfficeHeadlineCollection,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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
    for (index, headline) in office_headlines.at_graph.iter().enumerate() {
        let headline_id = presenter.text(headline.id.as_deref());
        let title = presenter.text(headline.title.as_deref());
        let summary = presenter.text(headline.summary.as_ref().and_then(Option::as_deref));
        let issuance_time_readable = presenter.timestamp(
            format!("office_headlines.at_graph[{index}].issuance_time"),
            headline.issuance_time.as_deref(),
        )?;
        let link = presenter.text(headline.link.as_deref());

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

/// Formats an Office's headline into a `comfy_table::Table`.
///
/// This function constructs a table displaying various attributes of an Office.
///
fn create_office_headline_table(
    office_headline: &OfficeHeadline,
    presenter: &DefaultPresenter,
) -> Result<Table, PresentationError> {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
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
    let headline_id = presenter.text(office_headline.id.as_deref());
    let title = presenter.text(office_headline.title.as_deref());
    let summary = presenter.text(office_headline.summary.as_ref().and_then(Option::as_deref));
    let issuance_time_readable = presenter.timestamp(
        "office_headline.issuance_time",
        office_headline.issuance_time.as_deref(),
    )?;
    let link = presenter.text(office_headline.link.as_deref());

    table.add_row(vec![
        Cell::new(headline_id),
        Cell::new(title),
        Cell::new(summary),
        Cell::new(issuance_time_readable),
        Cell::new(link),
    ]);

    Ok(table)
}

impl DefaultPresentation for Office {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_office_metadata_table(
            self, presenter,
        )))
    }
}

impl DefaultPresentation for OfficeHeadlineCollection {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_office_headlines_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for OfficeHeadline {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_office_headline_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for OfficeBriefingResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_office_briefing_table(
            self, presenter,
        )?))
    }
}

impl DefaultPresentation for OfficeWeatherStoryCollection {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(
            create_office_weather_stories_table(self, presenter),
        ))
    }
}
