//! Drawing a [`Summary`] with box-drawing characters.

use comfy_table::presets::{UTF8_FULL, UTF8_FULL_CONDENSED};
use comfy_table::{CellAlignment, ColumnConstraint, Table};
use noaa_weather_summary::render::format_value;
use noaa_weather_summary::{Align, Cell, Column, Emphasis, Fact, Section, Summary, Value};

use super::{RenderOptions, Width, style};

/// Renders a summary for a terminal.
///
/// Title first, bold and colored by the summary's emphasis; the subtitle
/// under it, dimmed; then one block per section, separated by blank lines;
/// then one `note: …` line per trailing note. Facts are a condensed
/// two-column table, tables are full-bordered, prose is wrapped text, and an
/// empty section is its message alone.
///
/// Every line outside a table is wrapped before it is styled, so an escape
/// sequence never counts against the width.
pub(super) fn render(summary: &Summary, options: &RenderOptions) -> String {
    let styled = options.styled();
    let width = options.width();
    let mut blocks: Vec<String> = Vec::new();

    let mut header = style::heading_line(&wrap(&summary.title, width), summary.emphasis, styled);
    if let Some(subtitle) = summary.subtitle.as_deref().filter(|text| !text.is_empty()) {
        header.push('\n');
        header.push_str(&style::dimmed_line(&wrap(subtitle, width), styled));
    }
    blocks.push(header);

    for section in &summary.sections {
        blocks.push(render_section(section, options));
    }

    for note in &summary.notes {
        blocks.push(style::dimmed_line(
            &wrap(&format!("note: {note}"), width),
            styled,
        ));
    }

    blocks.join("\n\n")
}

fn render_section(section: &Section, options: &RenderOptions) -> String {
    match section {
        Section::Facts { heading, facts } => {
            with_heading(heading.as_deref(), facts_table(facts, options), options)
        }
        Section::Table {
            heading,
            columns,
            rows,
        } => with_heading(
            heading.as_deref(),
            values_table(columns, rows, options),
            options,
        ),
        Section::Prose { heading, text, .. } => {
            with_heading(heading.as_deref(), wrap(text, options.width()), options)
        }
        Section::Empty { message, .. } => message.clone(),
    }
}

/// A heading line above a block, or the block alone when there is none.
fn with_heading(heading: Option<&str>, block: String, options: &RenderOptions) -> String {
    match heading.filter(|heading| !heading.is_empty()) {
        None => block,
        Some(heading) => format!(
            "{}\n{block}",
            style::heading_line(
                &wrap(heading, options.width()),
                Emphasis::None,
                options.styled()
            )
        ),
    }
}

/// Label and value, one row each, with no header row to repeat.
fn facts_table(facts: &[Fact], options: &RenderOptions) -> String {
    let mut table = Table::new();
    table.load_style(UTF8_FULL_CONDENSED);
    options.apply(&mut table);

    let mut values: Vec<String> = Vec::with_capacity(facts.len());
    for fact in facts {
        let value = text_of(&fact.value, options);
        table.add_row(vec![
            style::cell(fact.label.to_string(), Emphasis::None),
            style::cell(value.clone(), fact.emphasis),
        ]);
        values.push(value);
    }

    let identifiers = facts
        .iter()
        .zip(&values)
        .filter(|(fact, _)| is_identifier(&fact.value))
        .map(|(_, value)| value.as_str());
    keep_intact(&mut table, 1, widest(identifiers), 2, options);

    table.to_string()
}

fn values_table(columns: &[Column], rows: &[Vec<Cell>], options: &RenderOptions) -> String {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    options.apply(&mut table);

    table.set_header(
        columns
            .iter()
            .map(|column| style::header_cell(&column.title).set_alignment(alignment(column.align)))
            .collect::<Vec<_>>(),
    );

    let rendered: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| text_of(&cell.value, options))
                .collect()
        })
        .collect();

    for (row, texts) in rows.iter().zip(&rendered) {
        table.add_row(
            row.iter()
                .zip(texts)
                .zip(columns)
                .map(|((cell, text), column)| {
                    style::cell(text.clone(), cell.emphasis).set_alignment(alignment(column.align))
                })
                .collect::<Vec<_>>(),
        );
    }

    for index in 0..columns.len() {
        let identifiers = rows.iter().zip(&rendered).filter_map(|(row, texts)| {
            row.get(index)
                .filter(|cell| is_identifier(&cell.value))
                .and_then(|_| texts.get(index))
                .map(String::as_str)
        });
        keep_intact(
            &mut table,
            index,
            widest(identifiers),
            columns.len(),
            options,
        );
    }

    table.to_string()
}

/// The narrowest a column can be and still say anything.
const MIN_COLUMN_WIDTH: usize = 8;

/// Padding and separators comfy-table spends per column, plus the closing
/// border.
const COLUMN_OVERHEAD: usize = 3;

/// Stops an identifier column from wrapping or truncating — an identifier you
/// cannot copy is useless — for as long as the other columns still have
/// something left to say.
///
/// A 69-character alert URN beside five other columns cannot have both at 100
/// columns, and starving the rest into three-character slivers is worse than
/// wrapping the identifier. `--width 0` always shows every identifier whole.
fn keep_intact(
    table: &mut Table,
    index: usize,
    identifier_width: Option<usize>,
    columns: usize,
    options: &RenderOptions,
) {
    let Some(identifier_width) = identifier_width else {
        return;
    };
    if let Width::Columns(available) = options.width() {
        let others = (columns - 1) * MIN_COLUMN_WIDTH;
        let overhead = columns * COLUMN_OVERHEAD + 1;
        if identifier_width + others + overhead > usize::from(available) {
            return;
        }
    }
    if let Some(column) = table.column_mut(index) {
        column.set_constraint(ColumnConstraint::ContentWidth);
    }
}

/// The width of the widest identifier in a column, or `None` when the column
/// holds no identifiers.
fn widest<'text>(identifiers: impl Iterator<Item = &'text str>) -> Option<usize> {
    identifiers.map(|text| text.chars().count()).max()
}

fn is_identifier(value: &Value) -> bool {
    match value {
        Value::Identifier(_) => true,
        Value::List(values) | Value::Lines(values) => values.iter().any(is_identifier),
        _ => false,
    }
}

/// The summary crate decides what a value says; comfy-table takes the
/// newlines a [`Value::Lines`] produces as they stand.
fn text_of(value: &Value, options: &RenderOptions) -> String {
    format_value(value, &options.summary_options())
}

const fn alignment(align: Align) -> CellAlignment {
    match align {
        Align::Left => CellAlignment::Left,
        Align::Right => CellAlignment::Right,
        Align::Center => CellAlignment::Center,
    }
}

/// Wraps text on spaces, keeping the line breaks it already has.
fn wrap(text: &str, width: Width) -> String {
    let Width::Columns(columns) = width else {
        return text.to_owned();
    };
    let columns = usize::from(columns);

    text.split('\n')
        .map(|line| wrap_line(line, columns))
        .collect::<Vec<_>>()
        .join("\n")
}

fn wrap_line(line: &str, columns: usize) -> String {
    let mut wrapped = String::new();
    let mut used = 0;

    for word in line.split_whitespace() {
        let length = word.chars().count();
        if used == 0 {
            wrapped.push_str(word);
            used = length;
        } else if used + 1 + length <= columns {
            wrapped.push(' ');
            wrapped.push_str(word);
            used += 1 + length;
        } else {
            wrapped.push('\n');
            wrapped.push_str(word);
            used = length;
        }
    }
    wrapped
}

// [`keep_intact`] is pinned on both sides of its threshold by the
// `list_of_alerts_at_*` snapshots in `super::tests`; these cover the pieces a
// snapshot cannot show cheaply.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widest_reports_nothing_for_a_column_without_identifiers() {
        assert_eq!(widest(std::iter::empty()), None);
        assert_eq!(widest(["MIZ044", "MIC163000"].into_iter()), Some(9));
    }

    #[test]
    fn wrapping_breaks_on_spaces_and_keeps_existing_breaks() {
        assert_eq!(
            wrap("one two three four", Width::Columns(9)),
            "one two\nthree\nfour"
        );
        assert_eq!(
            wrap("first line\nsecond line", Width::Columns(40)),
            "first line\nsecond line"
        );
        assert_eq!(
            wrap("one two three four", Width::Unlimited),
            "one two three four"
        );
    }

    #[test]
    fn a_word_longer_than_the_width_keeps_its_own_line() {
        assert_eq!(
            wrap("supercalifragilistic yes", Width::Columns(10)),
            "supercalifragilistic\nyes"
        );
    }

    #[test]
    fn identifiers_are_found_through_lists_and_lines() {
        assert!(is_identifier(&Value::identifier("KDTX")));
        assert!(is_identifier(&Value::list(vec![
            Value::text(Some("a")),
            Value::identifier("KDTX"),
        ])));
        assert!(is_identifier(&Value::lines(vec![Value::identifier(
            "KDTX"
        )])));
        assert!(!is_identifier(&Value::text(Some("KDTX"))));
        assert!(!is_identifier(&Value::Missing));
    }
}
