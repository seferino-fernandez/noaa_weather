use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use noaa_weather_client::models::{RadioBroadcast, RadioTransmitter, RadioTransmitterCollection};

fn add_transmitter_row(table: &mut Table, transmitter: &RadioTransmitter) {
    table.add_row(vec![
        Cell::new(transmitter.call_sign.as_deref().unwrap_or("N/A")),
        Cell::new(transmitter.frequency.as_deref().unwrap_or("N/A")),
        Cell::new(transmitter.site_name.as_deref().unwrap_or("N/A")),
        Cell::new(transmitter.city.as_deref().unwrap_or("N/A")),
        Cell::new(transmitter.state.as_deref().unwrap_or("N/A")),
        Cell::new(transmitter.same_codes.len()),
        Cell::new(transmitter.counties.len()),
    ]);
}

fn transmitter_table() -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        Cell::new("Call Sign").add_attribute(Attribute::Bold),
        Cell::new("Frequency").add_attribute(Attribute::Bold),
        Cell::new("Site").add_attribute(Attribute::Bold),
        Cell::new("City").add_attribute(Attribute::Bold),
        Cell::new("State").add_attribute(Attribute::Bold),
        Cell::new("SAME Codes").add_attribute(Attribute::Bold),
        Cell::new("Counties").add_attribute(Attribute::Bold),
    ]);
    table
}

/// Creates a concise table for a transmitter collection.
pub fn create_radio_transmitters_table(collection: &RadioTransmitterCollection) -> Table {
    let mut table = transmitter_table();
    for transmitter in &collection.transmitters {
        add_transmitter_row(&mut table, transmitter);
    }
    table
}

/// Creates a concise table for one transmitter.
pub fn create_radio_transmitter_table(transmitter: &RadioTransmitter) -> Table {
    let mut table = transmitter_table();
    add_transmitter_row(&mut table, transmitter);
    table
}

/// Formats a `RadioBroadcast` (SSML document) into human-readable text.
///
/// Iterates through paragraphs and sentences, extracting the full text of each
/// sentence. Metadata marks are displayed as bracketed annotations between paragraphs.
pub fn format_radio_broadcast(broadcast: &RadioBroadcast) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "NOAA Weather Radio Broadcast (lang: {})\n",
        broadcast.lang
    ));
    output.push_str(&"=".repeat(60));
    output.push('\n');

    for (i, paragraph) in broadcast.paragraphs.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }

        for sentence in &paragraph.sentences {
            let text = sentence.full_text();
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                output.push_str(trimmed);
                output.push(' ');
            }
        }

        if !paragraph.sentences.is_empty() {
            // Trim trailing space and add newline
            if output.ends_with(' ') {
                output.pop();
            }
            output.push('\n');
        }

        for mark in &paragraph.marks {
            output.push_str(&format!("  [mark: {}]\n", mark.name));
        }
    }

    output
}
