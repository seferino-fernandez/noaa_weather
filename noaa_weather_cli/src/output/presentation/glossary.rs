use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use noaa_weather_client::models::GlossaryResponse;

use crate::output::{HumanDocument, HumanPresentation};

/// Creates a concise table of NWS glossary terms and definitions.
pub fn create_glossary_table(glossary: &GlossaryResponse) -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Term").add_attribute(Attribute::Bold),
        Cell::new("Definition").add_attribute(Attribute::Bold),
    ]);

    for entry in &glossary.glossary {
        table.add_row(vec![
            Cell::new(entry.term.as_deref().unwrap_or("N/A")),
            Cell::new(entry.definition.as_deref().unwrap_or("N/A")),
        ]);
    }

    table
}

impl HumanPresentation for GlossaryResponse {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::table(create_glossary_table(self))
    }
}
