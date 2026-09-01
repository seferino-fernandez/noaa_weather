use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use noaa_weather_client::models::GlossaryResponse;

use super::{DefaultPresentation, DefaultPresenter, PresentationError};
use crate::output::PresentationDocument;

/// Creates a concise table of NWS glossary terms and definitions.
fn create_glossary_table(glossary: &GlossaryResponse, presenter: &DefaultPresenter) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Term").add_attribute(Attribute::Bold),
        Cell::new("Definition").add_attribute(Attribute::Bold),
    ]);

    for entry in &glossary.glossary {
        table.add_row(vec![
            Cell::new(presenter.text(entry.term.as_deref())),
            Cell::new(presenter.text(entry.definition.as_deref())),
        ]);
    }

    table
}

impl DefaultPresentation for GlossaryResponse {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::table(create_glossary_table(
            self, presenter,
        )))
    }
}
