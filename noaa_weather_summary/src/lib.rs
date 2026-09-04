#![doc = include_str!("../README.md")]

use std::borrow::Cow;
use std::collections::BTreeSet;

use noaa_weather_client::OffsetDateTime;
use serde::Serialize;

pub mod alerts;
pub mod audit;
pub mod aviation;
pub mod glossary;
pub mod gridpoints;
pub mod points;
pub mod products;
pub mod render;
pub mod stations;
pub mod units;
mod value;
pub mod vtec;
pub mod zones;

pub use audit::coverage_gaps;
pub use units::QuantityKind;

/// A NOAA response that knows how to describe itself to a person.
///
/// Implementations decide *meaning*: which properties matter, what they are
/// called, and how bad they are. They never decide appearance; that belongs to
/// the renderers in [`render`].
pub trait Summarize: Serialize {
    /// Builds the human summary of this value.
    fn summarize(&self, options: &SummaryOptions) -> Summary;

    /// NOAA property keys deliberately absent from the summary, each with the
    /// reason. [`coverage_gaps`] treats these keys as accounted for, so every
    /// property is either shown, listed here, or reported as a gap.
    const OMITTED: &'static [(&'static str, &'static str)] = &[];
}

/// Meaning choices a caller may make.
///
/// The counterpart of [`RenderOptions`](render::RenderOptions), and the split
/// between them is what each choice changes: a time zone changes how an
/// instant is *printed*, a unit system changes *which number exists*. So a
/// unit choice shows up in the [`Summary`] itself and a zone choice does not.
#[derive(Clone, Debug, Default)]
pub struct SummaryOptions {
    /// The units measurements are converted to before they are shown.
    pub units: UnitSystem,
}

/// The system a measurement is shown in.
///
/// NOAA's own wire units are not a system — it sends `degC` alongside
/// `km_h-1` — so there is no third "as sent" choice here; `--json` is where
/// the untouched numbers live.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UnitSystem {
    /// Customary US units: °F, mph, feet, miles, inches, inches of mercury.
    #[default]
    Us,
    /// Metric units: °C, km/h, metres, kilometres, millimetres, hectopascals.
    Si,
}

/// The complete human view of one response.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Summary {
    /// Headline of the whole summary, for example the alert event name.
    pub title: String,
    /// One-line qualifier under the title, for example the alert headline.
    pub subtitle: Option<String>,
    /// How urgently the whole summary should read.
    pub emphasis: Emphasis,
    /// Body, in display order.
    pub sections: Vec<Section>,
    /// Trailing remarks such as "More alerts available".
    pub notes: Vec<String>,
}

impl Summary {
    /// Starts a summary with only a title.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            emphasis: Emphasis::None,
            sections: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Sets the subtitle.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Sets the summary-wide emphasis.
    #[must_use]
    pub fn emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphasis = emphasis;
        self
    }

    /// Appends a section.
    #[must_use]
    pub fn push(mut self, section: Section) -> Self {
        self.sections.push(section);
        self
    }

    /// Appends a trailing note.
    #[must_use]
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Every NOAA property key rendered by a [`Fact`], a [`Column`], a
    /// [`Section::Prose`] or a [`Section::Empty`], including the extra keys
    /// each fact or column lists in its `also` field.
    pub fn keys(&self) -> BTreeSet<&'static str> {
        self.sections
            .iter()
            .flat_map(|section| match section {
                Section::Facts { facts, .. } => facts
                    .iter()
                    .flat_map(|fact| fact.key.into_iter().chain(fact.also.iter().copied()))
                    .collect::<Vec<_>>(),
                Section::Table { columns, .. } => columns
                    .iter()
                    .flat_map(|column| column.key.into_iter().chain(column.also.iter().copied()))
                    .collect(),
                Section::Prose { key, .. } | Section::Empty { key, .. } => {
                    key.iter().copied().collect()
                }
            })
            .collect()
    }
}

/// One block of a [`Summary`].
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Section {
    /// Label and value pairs.
    Facts {
        /// Optional heading above the facts.
        heading: Option<String>,
        /// The pairs, in display order.
        facts: Vec<Fact>,
    },
    /// Rows of values under titled columns.
    Table {
        /// Optional heading above the table.
        heading: Option<String>,
        /// Column titles, keys and alignment.
        columns: Vec<Column>,
        /// One entry per row; each row has one cell per column.
        rows: Vec<Vec<Cell>>,
    },
    /// Free text such as an alert description.
    Prose {
        /// Optional heading above the text.
        heading: Option<String>,
        /// The NOAA property key this text renders, if it renders one.
        key: Option<&'static str>,
        /// The paragraph text.
        text: String,
    },
    /// Nothing to show, with an explanation such as "No alerts".
    Empty {
        /// The NOAA property key that turned out to be empty, if any. An
        /// absent description still counts as shown.
        key: Option<&'static str>,
        /// Why the summary has no content.
        message: String,
    },
}

/// One cell of a [`Section::Table`] row.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Cell {
    /// The value.
    pub value: Value,
    /// How urgently this cell should read.
    pub emphasis: Emphasis,
}

impl Cell {
    /// A cell with the given emphasis.
    pub fn new(value: Value, emphasis: Emphasis) -> Self {
        Self { value, emphasis }
    }
}

impl From<Value> for Cell {
    /// A cell with [`Emphasis::None`].
    fn from(value: Value) -> Self {
        Self::new(value, Emphasis::None)
    }
}

/// One labelled value.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Fact {
    /// Human label, for example "Areas affected".
    pub label: Cow<'static, str>,
    /// The NOAA property key this fact renders, if it renders one.
    pub key: Option<&'static str>,
    /// Further NOAA property keys folded into this fact's value, for example
    /// `expires` when the value is an interval from `effective` to `expires`.
    pub also: &'static [&'static str],
    /// The value.
    pub value: Value,
    /// How urgently this fact should read.
    pub emphasis: Emphasis,
}

impl Fact {
    /// A fact with [`Emphasis::None`] and no extra keys.
    pub fn new(
        label: impl Into<Cow<'static, str>>,
        key: Option<&'static str>,
        value: Value,
    ) -> Self {
        Self {
            label: label.into(),
            key,
            also: &[],
            value,
            emphasis: Emphasis::None,
        }
    }

    /// Sets the emphasis.
    #[must_use]
    pub fn with_emphasis(mut self, emphasis: Emphasis) -> Self {
        self.emphasis = emphasis;
        self
    }

    /// Records further NOAA property keys folded into this fact's value.
    #[must_use]
    pub fn also(mut self, keys: &'static [&'static str]) -> Self {
        self.also = keys;
        self
    }
}

/// One column of a [`Section::Table`].
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Column {
    /// Column title.
    pub title: Cow<'static, str>,
    /// The NOAA property key this column renders, if it renders one.
    pub key: Option<&'static str>,
    /// Further NOAA property keys folded into this column's cells, for
    /// example `headline` when a cell shows the sender name and the headline.
    pub also: &'static [&'static str],
    /// Horizontal alignment of the cells.
    pub align: Align,
}

impl Column {
    /// A left-aligned column with no extra keys.
    pub fn new(title: impl Into<Cow<'static, str>>, key: Option<&'static str>) -> Self {
        Self {
            title: title.into(),
            key,
            also: &[],
            align: Align::default(),
        }
    }

    /// Sets the alignment.
    #[must_use]
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Records further NOAA property keys folded into this column's cells.
    #[must_use]
    pub fn also(mut self, keys: &'static [&'static str]) -> Self {
        self.also = keys;
        self
    }
}

/// Horizontal alignment of table cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum Align {
    /// Flush left (the default).
    #[default]
    Left,
    /// Flush right, for numbers.
    Right,
    /// Centered.
    Center,
}

/// A displayable value with its meaning already decided.
///
/// Build values through the constructors in this crate (`Value::text`,
/// `Value::number`, ...) so that missing, blank and non-finite inputs are
/// classified consistently. Renderers only choose how each variant looks.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Value {
    /// Free text, already trimmed.
    Text(String),
    /// An identifier the reader may need for the next command.
    Identifier(String),
    /// The source had nothing to say.
    Missing,
    /// The source said something unusable, such as a non-finite number.
    Invalid,
    /// A single instant, carrying the UTC offset NOAA wrote it in.
    Timestamp(OffsetDateTime),
    /// A span with an optional end; `None` means ongoing.
    Interval {
        /// When the span starts.
        start: OffsetDateTime,
        /// When the span ends, or `None` while it is open.
        end: Option<OffsetDateTime>,
    },
    /// A measurement.
    Quantity {
        /// The magnitude.
        value: f64,
        /// Unit label to print after the magnitude, if any.
        unit: Option<String>,
        /// Decimal places to show.
        precision: u8,
    },
    /// A measurement NOAA gave as bounds rather than a single number, such as
    /// the twelve-hour wind speed.
    Range {
        /// The low bound.
        min: f64,
        /// The high bound.
        max: f64,
        /// Unit label to print after the bounds, if any.
        unit: Option<String>,
        /// Decimal places to show on each bound.
        precision: u8,
    },
    /// A percentage in the range 0 to 100.
    Percent(f64),
    /// A whole number of things.
    Count(u64),
    /// A size in bytes.
    Bytes(u64),
    /// A boolean the reader sees as yes or no.
    YesNo(bool),
    /// A geographic point.
    Coordinates {
        /// Latitude in decimal degrees.
        lat: f64,
        /// Longitude in decimal degrees.
        lon: f64,
    },
    /// Several values shown together as one enumeration.
    List(Vec<Value>),
    /// Several values shown one per line, each independently emphasizable.
    ///
    /// Unlike [`Value::List`], which reads as a comma-joined enumeration,
    /// these are separate statements that happen to share a cell.
    Lines(Vec<Value>),
}

/// How urgently a summary or fact should read, from calm to alarming.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum Emphasis {
    /// Ordinary content.
    #[default]
    None,
    /// Worth noticing, not worrying.
    Info,
    /// Deserves attention.
    Notice,
    /// Potentially harmful.
    Warning,
    /// Immediately dangerous.
    Danger,
}
