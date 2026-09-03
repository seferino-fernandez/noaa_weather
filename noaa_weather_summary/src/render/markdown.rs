//! GitHub-flavored markdown rendering.

use crate::{Align, Cell, Emphasis, Section, Summary};

use super::{RangeStyle, RenderOptions, format_value, heading_or_empty};

/// Markdown reads as prose, where an en dash between two numbers is the
/// ordinary typography for a range.
const RANGE: RangeStyle = RangeStyle::Dash;

/// Renders a summary as GitHub-flavored markdown.
///
/// Layout: `# title`, an italic subtitle, `### heading` per section when one
/// is present, facts as a bullet list, tables as GFM tables with an alignment
/// row, prose as a paragraph, empty sections as an italic message and notes as
/// a trailing bullet list. Facts and table cells with [`Emphasis::Warning`] or
/// [`Emphasis::Danger`] are bold.
pub fn render(summary: &Summary, options: &RenderOptions) -> String {
    let mut blocks: Vec<String> = Vec::new();

    let mut header = format!("# {}", summary.title);
    if let Some(subtitle) = summary.subtitle.as_deref().filter(|s| !s.is_empty()) {
        header.push_str(&format!("\n_{subtitle}_"));
    }
    blocks.push(header);

    for section in &summary.sections {
        blocks.push(render_section(section, options));
    }

    if !summary.notes.is_empty() {
        let notes = summary
            .notes
            .iter()
            .map(|note| format!("- {note}"))
            .collect::<Vec<_>>()
            .join("\n");
        blocks.push(notes);
    }

    let mut out = blocks.join("\n\n");
    out.push('\n');
    out
}

fn render_section(section: &Section, options: &RenderOptions) -> String {
    let mut lines: Vec<String> = Vec::new();
    match section {
        Section::Facts { heading, facts } => {
            push_heading(&mut lines, heading.as_ref());
            for fact in facts {
                let value = emphasize(format_value(&fact.value, options, RANGE), fact.emphasis);
                lines.push(format!("- **{}:** {value}", fact.label));
            }
        }
        Section::Table {
            heading,
            columns,
            rows,
        } => {
            push_heading(&mut lines, heading.as_ref());
            lines.push(table_row(columns.iter().map(|column| cell(&column.title))));
            lines.push(table_row(columns.iter().map(|column| match column.align {
                Align::Left => ":---".to_owned(),
                Align::Right => "---:".to_owned(),
                Align::Center => ":---:".to_owned(),
            })));
            for row in rows {
                lines.push(table_row(row.iter().map(|entry: &Cell| {
                    emphasize(
                        cell(&format_value(&entry.value, options, RANGE)),
                        entry.emphasis,
                    )
                })));
            }
        }
        Section::Prose { heading, text, .. } => {
            push_heading(&mut lines, heading.as_ref());
            lines.push(text.clone());
        }
        Section::Empty { message, .. } => lines.push(format!("_{message}_")),
    }
    lines.join("\n")
}

fn push_heading(lines: &mut Vec<String>, heading: Option<&String>) {
    if let Some(heading) = heading_or_empty(heading) {
        lines.push(format!("### {heading}"));
        lines.push(String::new());
    }
}

fn table_row(cells: impl Iterator<Item = String>) -> String {
    let mut row = String::from("|");
    for cell in cells {
        row.push(' ');
        row.push_str(&cell);
        row.push_str(" |");
    }
    row
}

/// Escapes pipes and turns newlines into line breaks so a cell cannot break
/// the table while a [`crate::Value::Lines`] still reads one value per line.
fn cell(text: &str) -> String {
    text.replace('|', "\\|").replace('\n', "<br>")
}

fn emphasize(text: String, emphasis: Emphasis) -> String {
    match emphasis {
        Emphasis::Warning | Emphasis::Danger => format!("**{text}**"),
        Emphasis::None | Emphasis::Info | Emphasis::Notice => text,
    }
}
