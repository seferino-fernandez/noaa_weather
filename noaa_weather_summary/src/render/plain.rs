//! Plain text rendering, without markup.

use crate::{Align, Section, Summary};

use super::{RenderOptions, format_value, heading_or_empty};

/// Renders a summary as plain text.
///
/// Same walk as the markdown renderer: title, subtitle, one block per section
/// separated by blank lines, notes last. Facts are `Label: value`; tables are
/// columns padded with spaces and aligned per [`Align`]. Emphasis is ignored
/// because plain text has no markup for it. Facts and prose keep the newlines
/// a [`crate::Value::Lines`] produces; table cells join them with `; ` so a
/// row stays one line.
pub fn render(summary: &Summary, options: &RenderOptions) -> String {
    let mut blocks: Vec<String> = Vec::new();

    let mut header = summary.title.clone();
    if let Some(subtitle) = summary.subtitle.as_deref().filter(|s| !s.is_empty()) {
        header.push('\n');
        header.push_str(subtitle);
    }
    blocks.push(header);

    for section in &summary.sections {
        blocks.push(render_section(section, options));
    }

    if !summary.notes.is_empty() {
        blocks.push(
            summary
                .notes
                .iter()
                .map(|note| format!("- {note}"))
                .collect::<Vec<_>>()
                .join("\n"),
        );
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
                lines.push(format!(
                    "{}: {}",
                    fact.label,
                    format_value(&fact.value, options)
                ));
            }
        }
        Section::Table {
            heading,
            columns,
            rows,
        } => {
            push_heading(&mut lines, heading.as_ref());
            let header: Vec<String> = columns
                .iter()
                .map(|column| column.title.to_string())
                .collect();
            let body: Vec<Vec<String>> = rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| format_value(&cell.value, options).replace('\n', "; "))
                        .collect()
                })
                .collect();
            let widths: Vec<usize> = (0..columns.len())
                .map(|index| {
                    std::iter::once(&header)
                        .chain(body.iter())
                        .filter_map(|row| row.get(index))
                        .map(|text| text.chars().count())
                        .max()
                        .unwrap_or(0)
                })
                .collect();
            let aligns: Vec<Align> = columns.iter().map(|column| column.align).collect();
            lines.push(pad_row(&header, &widths, &aligns));
            for row in &body {
                lines.push(pad_row(row, &widths, &aligns));
            }
        }
        Section::Prose { heading, text, .. } => {
            push_heading(&mut lines, heading.as_ref());
            lines.push(text.clone());
        }
        Section::Empty { message, .. } => lines.push(message.clone()),
    }
    lines.join("\n")
}

fn push_heading(lines: &mut Vec<String>, heading: Option<&String>) {
    if let Some(heading) = heading_or_empty(heading) {
        lines.push(heading.to_owned());
    }
}

fn pad_row(cells: &[String], widths: &[usize], aligns: &[Align]) -> String {
    let row = cells
        .iter()
        .zip(widths)
        .zip(aligns)
        .map(|((cell, width), align)| match align {
            Align::Left => format!("{cell:<width$}"),
            Align::Right => format!("{cell:>width$}"),
            Align::Center => format!("{cell:^width$}"),
        })
        .collect::<Vec<_>>()
        .join("  ");
    row.trim_end().to_owned()
}
