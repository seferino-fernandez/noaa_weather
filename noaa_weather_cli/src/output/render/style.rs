//! The one mapping from meaning to terminal appearance.
//!
//! Everything the CLI writes gets its color and weight here, so a reader can
//! answer "what does yellow mean" by reading one file.

use comfy_table::{Attribute, Cell, Color};
use crossterm::style::Stylize as _;
use noaa_weather_summary::Emphasis;

/// The color an emphasis reads as, or `None` for ordinary content.
fn color(emphasis: Emphasis) -> Option<Color> {
    match emphasis {
        Emphasis::None => None,
        Emphasis::Info => Some(Color::Green),
        Emphasis::Notice => Some(Color::Cyan),
        Emphasis::Warning => Some(Color::Yellow),
        Emphasis::Danger => Some(Color::Red),
    }
}

/// Whether an emphasis is loud enough to be bold.
///
/// Only [`Emphasis::Danger`] is, which means `Severe` alerts read as bold
/// alongside `Extreme` ones; the Severity column still prints the word.
fn bold(emphasis: Emphasis) -> bool {
    emphasis == Emphasis::Danger
}

/// A table cell carrying its emphasis.
pub(super) fn cell(text: String, emphasis: Emphasis) -> Cell {
    let mut cell = Cell::new(text);
    if let Some(color) = color(emphasis) {
        cell = cell.fg(color);
    }
    if bold(emphasis) {
        cell = cell.add_attribute(Attribute::Bold);
    }
    cell
}

/// A bold header cell.
pub(super) fn header_cell(text: &str) -> Cell {
    Cell::new(text).add_attribute(Attribute::Bold)
}

/// A line outside any table, bold and colored by its emphasis.
pub(super) fn heading_line(text: &str, emphasis: Emphasis, styled: bool) -> String {
    if !styled {
        return text.to_owned();
    }
    let mut content = text.bold();
    if let Some(color) = color(emphasis) {
        content = content.with(text_color(color));
    }
    content.to_string()
}

/// A line outside any table that should recede, such as a subtitle or a note.
pub(super) fn dimmed_line(text: &str, styled: bool) -> String {
    if styled {
        text.dim().to_string()
    } else {
        text.to_owned()
    }
}

/// comfy-table mirrors crossterm's colors rather than re-exporting them, so
/// the two names have to be bridged for text written outside a table.
fn text_color(color: Color) -> crossterm::style::Color {
    match color {
        Color::Green => crossterm::style::Color::Green,
        Color::Cyan => crossterm::style::Color::Cyan,
        Color::Yellow => crossterm::style::Color::Yellow,
        _ => crossterm::style::Color::Red,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_emphasis_has_a_decided_appearance() {
        assert_eq!(color(Emphasis::None), None);
        assert_eq!(color(Emphasis::Info), Some(Color::Green));
        assert_eq!(color(Emphasis::Notice), Some(Color::Cyan));
        assert_eq!(color(Emphasis::Warning), Some(Color::Yellow));
        assert_eq!(color(Emphasis::Danger), Some(Color::Red));

        for emphasis in [
            Emphasis::None,
            Emphasis::Info,
            Emphasis::Notice,
            Emphasis::Warning,
        ] {
            assert!(!bold(emphasis), "{emphasis:?} should not be bold");
        }
        assert!(bold(Emphasis::Danger));
    }

    #[test]
    fn unstyled_lines_carry_no_escapes() {
        assert_eq!(heading_line("Title", Emphasis::Danger, false), "Title");
        assert_eq!(dimmed_line("Subtitle", false), "Subtitle");
    }
}
