use noaa_weather_client::models::RadioBroadcast;

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
