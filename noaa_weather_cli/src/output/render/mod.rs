//! Terminal rendering: the one place that decides how output looks.
//!
//! [`noaa_weather_summary`] already renders a [`Summary`] as markdown and as
//! plain text; neither can draw a box-drawing table or write a color escape,
//! so this module implements exactly that and nothing else.
//!
//! [`RenderOptions`] is built once from the global flags and then applies to
//! everything the CLI writes: summaries rendered here, and the tables the
//! eleven un-ported presenters still build by hand.

use std::env;

use clap::ValueEnum;
use comfy_table::{ContentArrangement, Table};
use jiff::tz::TimeZone;
use noaa_weather_summary::Summary;
use noaa_weather_summary::render::RenderOptions as ValueOptions;

mod style;
mod table;

/// When styling escapes are written.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub(crate) enum ColorMode {
    /// Style when the destination is a terminal and `NO_COLOR` is unset or
    /// empty.
    #[default]
    Auto,
    /// Always style, even into a file or a pipe, and even under `NO_COLOR`.
    Always,
    /// Never style.
    Never,
}

/// Which zone timestamps are shown in.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum TimeZoneChoice {
    /// The zone this machine is set to.
    #[default]
    System,
    /// The UTC offset NOAA wrote the timestamp in.
    Source,
    /// A named IANA zone such as `America/Detroit`.
    Named(TimeZone),
}

/// How wide a rendered line may be.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Width {
    /// No wrapping at all, for piping into a pager such as `less -S`.
    Unlimited,
    /// Wrap to this many columns.
    Columns(u16),
}

/// The narrowest width worth rendering: below this a table is unreadable
/// whatever it does with the space.
const MIN_WIDTH: u16 = 40;

/// The width assumed when nothing else says: no `--width`, no `COLUMNS`, and
/// no terminal to measure.
pub(super) const FALLBACK_WIDTH: u16 = 100;

/// Every appearance decision the CLI makes, resolved once at startup.
#[derive(Clone, Debug)]
pub(crate) struct RenderOptions {
    /// Whether cells and headings carry styling escapes. [`ColorMode::Auto`]
    /// resolved this against the destination and `NO_COLOR` at startup, so
    /// nothing downstream has to ask again.
    styled: bool,
    /// Maximum line width.
    width: Width,
    /// Zone for timestamps; `None` keeps the offset NOAA sent.
    time_zone: Option<TimeZone>,
}

impl RenderOptions {
    /// Resolves the global flags against the environment and the destination.
    ///
    /// `is_terminal` describes the destination the output is going to, which
    /// is not the same question as whether this process's stdout is a
    /// terminal: `--output report.txt` writes to a file from a terminal
    /// session.
    pub(crate) fn new(
        color: ColorMode,
        width: Option<u16>,
        time_zone: &TimeZoneChoice,
        is_terminal: bool,
    ) -> Self {
        if color == ColorMode::Always {
            // crossterm drops color escapes under `NO_COLOR` no matter what
            // the caller asked for. `--color always` is an explicit override,
            // which is what the NO_COLOR convention says wins.
            crossterm::style::force_color_output(true);
        }

        Self {
            styled: match color {
                ColorMode::Always => true,
                ColorMode::Never => false,
                ColorMode::Auto => is_terminal && !no_color_requested(),
            },
            width: resolve_width(width, is_terminal),
            time_zone: match time_zone {
                TimeZoneChoice::Source => None,
                TimeZoneChoice::System => Some(TimeZone::try_system().unwrap_or(TimeZone::UTC)),
                TimeZoneChoice::Named(zone) => Some(zone.clone()),
            },
        }
    }

    /// The zone the un-ported presenters format their timestamps in.
    ///
    /// Those presenters cannot express "keep the source offset", so
    /// `--time-zone source` leaves them on this machine's zone.
    pub(crate) fn presenter_time_zone(&self) -> TimeZone {
        self.time_zone
            .clone()
            .unwrap_or_else(|| TimeZone::try_system().unwrap_or(TimeZone::UTC))
    }

    /// Renders a summary for a terminal.
    pub(crate) fn render(&self, summary: &Summary) -> String {
        table::render(summary, self)
    }

    /// Applies color and width policy to a table built anywhere in the CLI.
    ///
    /// comfy-table otherwise decides styling from this process's stdout,
    /// which is the wrong question, and crossterm keeps attributes such as
    /// bold under `NO_COLOR` even while it drops colors. Deciding here means
    /// one policy covers the ported families and the un-ported ones alike.
    pub(crate) fn apply(&self, table: &mut Table) {
        table.force_no_tty();
        if self.styled {
            table.enforce_styling();
        }
        match self.width {
            Width::Unlimited => {
                table.set_content_arrangement(ContentArrangement::Disabled);
            }
            Width::Columns(columns) => {
                table.set_content_arrangement(ContentArrangement::Dynamic);
                table.set_width(columns);
            }
        }
    }

    /// The summary crate's own appearance options, so both renderers format a
    /// value the same way.
    fn value_options(&self) -> ValueOptions {
        ValueOptions {
            time_zone: self.time_zone.clone(),
        }
    }

    fn width(&self) -> Width {
        self.width
    }

    fn styled(&self) -> bool {
        self.styled
    }
}

/// Parses `--time-zone`: `auto`, `source`, or an IANA zone name.
pub(crate) fn parse_time_zone(text: &str) -> Result<TimeZoneChoice, String> {
    match text {
        "auto" => Ok(TimeZoneChoice::System),
        "source" => Ok(TimeZoneChoice::Source),
        name => TimeZone::get(name)
            .map(TimeZoneChoice::Named)
            .map_err(|error| format!("unknown time zone {name:?}: {error}")),
    }
}

/// Whether the `NO_COLOR` convention applies: set and not empty.
fn no_color_requested() -> bool {
    env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty())
}

fn resolve_width(requested: Option<u16>, is_terminal: bool) -> Width {
    match requested {
        Some(0) => Width::Unlimited,
        Some(columns) => Width::Columns(columns.max(MIN_WIDTH)),
        None => Width::Columns(detected_width(is_terminal).max(MIN_WIDTH)),
    }
}

/// `COLUMNS`, then the terminal, then a readable default.
fn detected_width(is_terminal: bool) -> u16 {
    if let Some(columns) = env::var("COLUMNS")
        .ok()
        .and_then(|value| value.trim().parse::<u16>().ok())
    {
        return columns;
    }
    if is_terminal && let Ok((columns, _)) = crossterm::terminal::size() {
        return columns;
    }
    FALLBACK_WIDTH
}

#[cfg(test)]
mod tests;
