use std::{fmt, str::FromStr};

use jiff::{Span, Timestamp, fmt::temporal::SpanParser};
use serde::{Deserialize, Serialize};

use crate::ids::{InvalidValue, ValueKind};

const SHAPE: &str = "must be start/end, start/duration, duration/end, or duration \
                     using RFC 3339 timestamps and ISO 8601 durations";
const ORDER: &str = "end must not be before start";
const NEGATIVE: &str = "duration must not be negative";
const ANCHOR: &str = "two durations have no anchor; one side must be a timestamp";
const BARE: &str = "a bare timestamp is not an interval; add /end or /duration";

/// Parses ISO 8601 durations only, not jiff's "friendly" format.
const SPAN_PARSER: SpanParser = SpanParser::new();

#[derive(Clone, Copy)]
enum Bounds {
    Between { start: Timestamp, end: Timestamp },
    Starting { start: Timestamp, span: Span },
    Ending { span: Span, end: Timestamp },
    Lasting { span: Span },
}

/// An ISO 8601 time interval in one of its four forms.
///
/// | Form             | Constructor            | Text                                       |
/// |------------------|------------------------|--------------------------------------------|
/// | start and end    | [`Interval::between`]  | `2024-01-01T00:00:00Z/2024-01-02T00:00:00Z` |
/// | start and length | [`Interval::starting`] | `2024-01-01T00:00:00Z/PT6H`                |
/// | length and end   | [`Interval::ending`]   | `PT6H/2024-01-02T00:00:00Z`                |
/// | length only      | [`Interval::lasting`]  | `PT6H`                                     |
///
/// Timestamps are truncated to whole seconds at construction (never
/// rounded, because NOAA rejects fractional seconds), so equality, the text
/// form, and serde all describe the same instant. They are written as
/// RFC 3339 in UTC and durations in ISO 8601 form, exactly as NOAA's
/// `interval`, `arrived`, `created`, and `published` query parameters
/// expect.
///
/// ```
/// use noaa_weather_client::Interval;
///
/// let six_hours: jiff::Span = "PT6H".parse().unwrap();
/// let start: jiff::Timestamp = "2024-01-01T00:00:00Z".parse().unwrap();
///
/// let recent = Interval::lasting(six_hours)?;
/// assert_eq!(recent.to_string(), "PT6H");
///
/// let from_start = Interval::starting(start, six_hours)?;
/// assert_eq!(from_start.to_string(), "2024-01-01T00:00:00Z/PT6H");
/// assert_eq!(from_start, "2024-01-01T00:00:00Z/PT6H".parse()?);
/// # Ok::<(), noaa_weather_client::InvalidValue>(())
/// ```
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Interval(Bounds);

impl Interval {
    /// Creates an interval from a start and an end instant.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `end` is before `start`.
    pub fn between(start: Timestamp, end: Timestamp) -> Result<Self, InvalidValue> {
        let (start, end) = (whole_seconds(start), whole_seconds(end));
        if end < start {
            return Err(InvalidValue::new(
                ValueKind::Interval,
                format!("{start}/{end}"),
                ORDER,
            ));
        }
        Ok(Self(Bounds::Between { start, end }))
    }

    /// Creates an interval that begins at `start` and lasts `span`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `span` is negative.
    pub fn starting(start: Timestamp, span: Span) -> Result<Self, InvalidValue> {
        let start = whole_seconds(start);
        check_span(span, || format!("{start}/{span}"))?;
        Ok(Self(Bounds::Starting { start, span }))
    }

    /// Creates an interval that lasts `span` and ends at `end`.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `span` is negative.
    pub fn ending(span: Span, end: Timestamp) -> Result<Self, InvalidValue> {
        let end = whole_seconds(end);
        check_span(span, || format!("{span}/{end}"))?;
        Ok(Self(Bounds::Ending { span, end }))
    }

    /// Creates an interval of a given length with no fixed start or end.
    ///
    /// NOAA anchors it to the current time, so `PT6H` means the last six
    /// hours.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when `span` is negative.
    pub fn lasting(span: Span) -> Result<Self, InvalidValue> {
        check_span(span, || span.to_string())?;
        Ok(Self(Bounds::Lasting { span }))
    }

    /// Returns the explicit start instant, truncated to whole seconds, if
    /// this form has one.
    #[must_use]
    pub const fn start(&self) -> Option<Timestamp> {
        match self.0 {
            Bounds::Between { start, .. } | Bounds::Starting { start, .. } => Some(start),
            Bounds::Ending { .. } | Bounds::Lasting { .. } => None,
        }
    }

    /// Returns the explicit end instant, truncated to whole seconds, if this
    /// form has one.
    #[must_use]
    pub const fn end(&self) -> Option<Timestamp> {
        match self.0 {
            Bounds::Between { end, .. } | Bounds::Ending { end, .. } => Some(end),
            Bounds::Starting { .. } | Bounds::Lasting { .. } => None,
        }
    }

    /// Returns the explicit duration, if this form has one.
    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        match self.0 {
            Bounds::Starting { span, .. }
            | Bounds::Ending { span, .. }
            | Bounds::Lasting { span } => Some(span),
            Bounds::Between { .. } => None,
        }
    }
}

/// Drops sub-second precision without rounding, the only timestamp form
/// NOAA accepts.
fn whole_seconds(instant: Timestamp) -> Timestamp {
    Timestamp::from_second(instant.as_second()).unwrap_or(instant)
}

fn check_span(span: Span, input: impl FnOnce() -> String) -> Result<(), InvalidValue> {
    if span.is_negative() {
        return Err(InvalidValue::new(ValueKind::Interval, input(), NEGATIVE));
    }
    Ok(())
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (
                Bounds::Between { start, end },
                Bounds::Between {
                    start: other_start,
                    end: other_end,
                },
            ) => start == other_start && end == other_end,
            (
                Bounds::Starting { start, span },
                Bounds::Starting {
                    start: other_start,
                    span: other_span,
                },
            ) => start == other_start && span.fieldwise() == other_span.fieldwise(),
            (
                Bounds::Ending { span, end },
                Bounds::Ending {
                    span: other_span,
                    end: other_end,
                },
            ) => end == other_end && span.fieldwise() == other_span.fieldwise(),
            (Bounds::Lasting { span }, Bounds::Lasting { span: other_span }) => {
                span.fieldwise() == other_span.fieldwise()
            }
            _ => false,
        }
    }
}

impl Eq for Interval {}

impl fmt::Debug for Interval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Interval({self})")
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seconds = |instant: Timestamp| instant.strftime(super::RFC3339_SECONDS);
        match self.0 {
            Bounds::Between { start, end } => {
                write!(formatter, "{}/{}", seconds(start), seconds(end))
            }
            Bounds::Starting { start, span } => write!(formatter, "{}/{span}", seconds(start)),
            Bounds::Ending { span, end } => write!(formatter, "{span}/{}", seconds(end)),
            Bounds::Lasting { span } => write!(formatter, "{span}"),
        }
    }
}

/// One side of an interval's text form.
enum Part {
    Instant(Timestamp),
    Duration(Span),
}

fn parse_part(text: &str) -> Result<Part, &'static str> {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (None, _) => Err(SHAPE),
        (Some('P' | 'p'), _) | (Some('-' | '+'), Some('P' | 'p')) => {
            let span = SPAN_PARSER.parse_span(text).map_err(|_| SHAPE)?;
            if span.is_negative() {
                return Err(NEGATIVE);
            }
            Ok(Part::Duration(span))
        }
        _ => text.parse().map(Part::Instant).map_err(|_| SHAPE),
    }
}

impl FromStr for Interval {
    type Err = InvalidValue;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reject = |reason| InvalidValue::new(ValueKind::Interval, input, reason);
        let (first, second) = match input.split_once('/') {
            None => (input, None),
            Some((_, second)) if second.contains('/') => {
                return Err(reject(SHAPE));
            }
            Some((first, second)) => (first, Some(second)),
        };
        let first = parse_part(first).map_err(reject)?;
        let Some(second) = second else {
            return match first {
                Part::Duration(span) => Ok(Self(Bounds::Lasting { span })),
                Part::Instant(_) => Err(reject(BARE)),
            };
        };
        let second = parse_part(second).map_err(reject)?;
        match (first, second) {
            (Part::Instant(start), Part::Instant(end)) => {
                Self::between(start, end).map_err(|error| reject(error.reason()))
            }
            (Part::Instant(start), Part::Duration(span)) => {
                Self::starting(start, span).map_err(|error| reject(error.reason()))
            }
            (Part::Duration(span), Part::Instant(end)) => {
                Self::ending(span, end).map_err(|error| reject(error.reason()))
            }
            (Part::Duration(_), Part::Duration(_)) => Err(reject(ANCHOR)),
        }
    }
}

impl From<Interval> for String {
    fn from(value: Interval) -> Self {
        value.to_string()
    }
}

impl_try_from_str!(Interval);
impl_string_schema!(
    Interval,
    "ISO 8601 time interval: start/end, start/duration, duration/end, or duration, \
     with RFC 3339 timestamps and ISO 8601 durations (for example \
     2024-01-01T00:00:00Z/PT6H or PT6H)."
);

#[cfg(test)]
mod tests {
    use super::*;

    fn at(text: &str) -> Timestamp {
        text.parse().unwrap()
    }

    fn span(text: &str) -> Span {
        text.parse().unwrap()
    }

    #[test]
    fn between_renders_and_parses() {
        let interval =
            Interval::between(at("2024-01-01T00:00:00Z"), at("2024-01-02T00:00:00Z")).unwrap();
        assert_eq!(
            interval.to_string(),
            "2024-01-01T00:00:00Z/2024-01-02T00:00:00Z"
        );
        assert_eq!(interval.to_string().parse::<Interval>().unwrap(), interval);
        assert_eq!(interval.start(), Some(at("2024-01-01T00:00:00Z")));
        assert_eq!(interval.end(), Some(at("2024-01-02T00:00:00Z")));
        assert!(interval.span().is_none());
    }

    #[test]
    fn between_accepts_equal_bounds_and_rejects_reversed() {
        let instant = at("2024-01-01T00:00:00Z");
        assert!(Interval::between(instant, instant).is_ok());
        let error = Interval::between(at("2024-01-02T00:00:00Z"), instant).unwrap_err();
        assert_eq!(error.kind(), ValueKind::Interval);
        assert_eq!(error.reason(), ORDER);
        assert_eq!(error.input(), "2024-01-02T00:00:00Z/2024-01-01T00:00:00Z");
    }

    #[test]
    fn starting_renders_and_parses() {
        let interval = Interval::starting(at("2024-01-01T00:00:00Z"), span("PT1H")).unwrap();
        assert_eq!(interval.to_string(), "2024-01-01T00:00:00Z/PT1H");
        assert_eq!(interval.to_string().parse::<Interval>().unwrap(), interval);
        assert_eq!(interval.start(), Some(at("2024-01-01T00:00:00Z")));
        assert!(interval.end().is_none());
        assert_eq!(
            interval.span().unwrap().fieldwise(),
            span("PT1H").fieldwise()
        );
    }

    #[test]
    fn ending_renders_and_parses() {
        let interval = Interval::ending(span("PT1H"), at("2024-01-01T00:00:00Z")).unwrap();
        assert_eq!(interval.to_string(), "PT1H/2024-01-01T00:00:00Z");
        assert_eq!(interval.to_string().parse::<Interval>().unwrap(), interval);
        assert!(interval.start().is_none());
        assert_eq!(interval.end(), Some(at("2024-01-01T00:00:00Z")));
    }

    #[test]
    fn lasting_renders_and_parses() {
        let interval = Interval::lasting(span("PT1H")).unwrap();
        assert_eq!(interval.to_string(), "PT1H");
        assert_eq!("PT1H".parse::<Interval>().unwrap(), interval);
        assert_eq!("pt1h".parse::<Interval>().unwrap(), interval);
        assert_eq!(
            "P1DT2H30M".parse::<Interval>().unwrap().to_string(),
            "P1DT2H30M"
        );
        assert!(interval.start().is_none());
        assert!(interval.end().is_none());
        assert_eq!(format!("{interval:?}"), "Interval(PT1H)");
    }

    #[test]
    fn negative_spans_are_rejected_everywhere() {
        let negative = span("-PT1H");
        let start = at("2024-01-01T00:00:00Z");
        assert_eq!(Interval::lasting(negative).unwrap_err().reason(), NEGATIVE);
        assert_eq!(
            Interval::starting(start, negative).unwrap_err().reason(),
            NEGATIVE
        );
        assert_eq!(
            Interval::ending(negative, start).unwrap_err().reason(),
            NEGATIVE
        );
        assert_eq!("-PT1H".parse::<Interval>().unwrap_err().reason(), NEGATIVE);
        assert_eq!(
            "-P1D/2024-01-01T00:00:00Z"
                .parse::<Interval>()
                .unwrap_err()
                .reason(),
            NEGATIVE
        );
        assert_eq!(
            "2024-01-01T00:00:00Z/-PT1H"
                .parse::<Interval>()
                .unwrap_err()
                .reason(),
            NEGATIVE
        );
    }

    #[test]
    fn explicitly_positive_spans_are_accepted() {
        let hour = Interval::lasting(span("PT1H")).unwrap();
        assert_eq!("+PT1H".parse::<Interval>().unwrap(), hour);
        assert_eq!("+PT1H".parse::<Interval>().unwrap().to_string(), "PT1H");
        let end = at("2024-01-01T00:00:00Z");
        assert_eq!(
            "+P1D/2024-01-01T00:00:00Z".parse::<Interval>().unwrap(),
            Interval::ending(span("P1D"), end).unwrap()
        );
        assert_eq!(
            "2024-01-01T00:00:00Z/+P1D".parse::<Interval>().unwrap(),
            Interval::starting(end, span("P1D")).unwrap()
        );
        assert_eq!("+1h".parse::<Interval>().unwrap_err().reason(), SHAPE);
    }

    #[test]
    fn timestamps_normalize_to_utc() {
        let interval: Interval = "2024-01-01T05:00:00+05:00/PT1H".parse().unwrap();
        assert_eq!(interval.to_string(), "2024-01-01T00:00:00Z/PT1H");
    }

    #[test]
    fn rejects_malformed_forms() {
        let cases = [
            ("", SHAPE),
            ("/", SHAPE),
            ("PT1H/", SHAPE),
            ("/PT1H", SHAPE),
            ("PT1H/PT2H", ANCHOR),
            ("2024-01-01T00:00:00Z", BARE),
            ("2024-01-01T00:00:00Z/2024-01-02T00:00:00Z/PT1H", SHAPE),
            ("2024-01-02T00:00:00Z/2024-01-01T00:00:00Z", ORDER),
            ("1h", SHAPE),
            ("2024-01-01/PT1H", SHAPE),
            ("2024-01-01T00:00:00/PT1H", SHAPE),
            ("yesterday/PT1H", SHAPE),
            ("PTX", SHAPE),
        ];
        for (input, reason) in cases {
            let error = input.parse::<Interval>().unwrap_err();
            assert_eq!(error.kind(), ValueKind::Interval, "{input:?}");
            assert_eq!(error.input(), input);
            assert_eq!(error.reason(), reason, "{input:?}");
        }
    }

    #[test]
    fn equality_is_structural() {
        let start = at("2024-01-01T00:00:00Z");
        assert_ne!(
            Interval::lasting(span("PT1H")).unwrap(),
            Interval::lasting(span("PT60M")).unwrap()
        );
        assert_ne!(
            Interval::starting(start, span("PT1H")).unwrap(),
            Interval::ending(span("PT1H"), start).unwrap()
        );
        assert_eq!(
            Interval::lasting(span("PT1H")).unwrap(),
            Interval::lasting(span("PT1H")).unwrap()
        );
    }

    #[test]
    fn serde_round_trip() {
        let interval = Interval::starting(at("2024-01-01T00:00:00Z"), span("PT6H")).unwrap();
        let json = serde_json::to_string(&interval).unwrap();
        assert_eq!(json, "\"2024-01-01T00:00:00Z/PT6H\"");
        assert_eq!(serde_json::from_str::<Interval>(&json).unwrap(), interval);
        assert_eq!(Interval::try_from("PT6H").unwrap().to_string(), "PT6H");
        assert_eq!(
            Interval::try_from(String::from("PT6H"))
                .unwrap()
                .to_string(),
            "PT6H"
        );
        assert_eq!(String::from(interval), "2024-01-01T00:00:00Z/PT6H");
        let error = serde_json::from_str::<Interval>("\"PT1H/PT2H\"").unwrap_err();
        assert!(error.to_string().contains("invalid interval"), "{error}");
    }

    #[test]
    fn display_truncates_sub_second_precision() {
        let start = at("2024-01-01T00:00:00.999999999Z");
        let end = at("2024-01-01T06:00:00.5Z");
        assert_eq!(
            Interval::between(start, end).unwrap().to_string(),
            "2024-01-01T00:00:00Z/2024-01-01T06:00:00Z"
        );
        assert_eq!(
            Interval::starting(start, span("PT6H")).unwrap().to_string(),
            "2024-01-01T00:00:00Z/PT6H"
        );
        assert_eq!(
            Interval::ending(span("PT6H"), end).unwrap().to_string(),
            "PT6H/2024-01-01T06:00:00Z"
        );
    }

    #[test]
    fn fractional_input_round_trips_to_the_truncated_form() {
        let parsed: Interval = "2024-01-01T00:00:00.25Z/PT6H".parse().unwrap();
        assert_eq!(parsed.to_string(), "2024-01-01T00:00:00Z/PT6H");
        let again: Interval = parsed.to_string().parse().unwrap();
        assert_eq!(again, parsed);
        assert_eq!(String::from(parsed), "2024-01-01T00:00:00Z/PT6H");
        assert_eq!(parsed.start(), Some(at("2024-01-01T00:00:00Z")));

        let json = serde_json::to_string(&parsed).unwrap();
        assert_eq!(json, "\"2024-01-01T00:00:00Z/PT6H\"");
        assert_eq!(serde_json::from_str::<Interval>(&json).unwrap(), parsed);
        assert_eq!(
            Interval::starting(at("2024-01-01T00:00:00.999Z"), span("PT6H")).unwrap(),
            parsed
        );

        let between: Interval = "2024-01-01T00:00:00.25Z/2024-01-01T06:00:00.75Z"
            .parse()
            .unwrap();
        let built = Interval::between(
            at("2024-01-01T00:00:00.123456789Z"),
            at("2024-01-01T06:00:00.999999999Z"),
        )
        .unwrap();
        assert_eq!(between, built);
        assert_eq!(built.start(), Some(at("2024-01-01T00:00:00Z")));
        assert_eq!(built.end(), Some(at("2024-01-01T06:00:00Z")));
        assert_eq!(
            serde_json::from_str::<Interval>(&serde_json::to_string(&built).unwrap()).unwrap(),
            built
        );
        let ending = Interval::ending(span("PT6H"), at("2024-01-01T06:00:00.5Z")).unwrap();
        assert_eq!(ending.end(), Some(at("2024-01-01T06:00:00Z")));
        assert_eq!(ending, "PT6H/2024-01-01T06:00:00Z".parse().unwrap());
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_is_a_string() {
        let schema = schemars::schema_for!(Interval);
        let value = schema.as_value();
        assert_eq!(value["type"], "string");
        assert!(value.get("description").is_some());
    }
}
