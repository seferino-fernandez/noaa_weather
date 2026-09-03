//! Text renderers: appearance only.
//!
//! Both renderers walk a [`crate::Summary`] the same way and share one set of value
//! formatting rules; they differ only in markup.

use jiff::tz::TimeZone;
use noaa_weather_client::OffsetDateTime;

use crate::Value;

pub mod markdown;
pub mod plain;

/// Appearance choices a caller may make.
#[derive(Clone, Debug, Default)]
pub struct RenderOptions {
    /// Zone in which timestamps are shown. `None` keeps the offset the source
    /// sent, so an Eastern office's alert reads `-04:00`.
    pub time_zone: Option<TimeZone>,
}

/// How the two bounds of a [`Value::Range`] are joined.
///
/// The only appearance decision a caller has to make per renderer rather than
/// once per run, which is why it is an argument to [`format_value`] instead of
/// a field of [`RenderOptions`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RangeStyle {
    /// `10 to 20 mph`. A word cannot be misread as a minus sign, and negative
    /// bounds are ordinary wherever wind chill is.
    #[default]
    Words,
    /// `10–20 mph`, with an en dash, for markup that reads as prose.
    Dash,
}

const MISSING: &str = "N/A";
const INVALID: &str = "Invalid";
const TIMESTAMP_FORMAT: &str = "%Y-%m-%d %H:%M %:z";

/// Formats a value as text, without any markup.
///
/// [`Value::Lines`] joins with newlines, so a caller that puts the result in a
/// cell must decide what a newline means there: the markdown renderer writes
/// `<br>`, the plain renderer writes `; `, and a terminal table can take the
/// newline as it stands.
pub fn format_value(value: &Value, options: &RenderOptions, range: RangeStyle) -> String {
    match value {
        Value::Text(text) | Value::Identifier(text) => text.clone(),
        Value::Missing => MISSING.to_owned(),
        Value::Invalid => INVALID.to_owned(),
        Value::Timestamp(timestamp) => format_timestamp(timestamp, options),
        Value::Interval { start, end } => {
            let start = format_timestamp(start, options);
            let end = end.as_ref().map_or_else(
                || "ongoing".to_owned(),
                |end| format_timestamp(end, options),
            );
            format!("{start} \u{2013} {end}")
        }
        Value::Quantity {
            value,
            unit,
            precision,
        } => {
            let precision = usize::from(*precision);
            with_unit(format!("{value:.precision$}"), unit.as_deref())
        }
        Value::Range {
            min,
            max,
            unit,
            precision,
        } => {
            let precision = usize::from(*precision);
            let bounds = match range {
                RangeStyle::Words => format!("{min:.precision$} to {max:.precision$}"),
                RangeStyle::Dash => format!("{min:.precision$}\u{2013}{max:.precision$}"),
            };
            with_unit(bounds, unit.as_deref())
        }
        Value::Percent(percent) => format!("{percent:.0}%"),
        Value::Count(count) => count.to_string(),
        Value::Bytes(bytes) => format_bytes(*bytes),
        Value::YesNo(true) => "Yes".to_owned(),
        Value::YesNo(false) => "No".to_owned(),
        Value::Coordinates { lat, lon } => format!("{lat:.4}, {lon:.4}"),
        Value::List(values) => join(values, ", ", options, range),
        Value::Lines(values) => join(values, "\n", options, range),
    }
}

/// Writes a number next to its unit label.
///
/// `%` sits tight against the number, because [`Value::Percent`] already
/// writes `40%` and one crate should not spell one unit two ways. Everything
/// else takes a space, including `°C` and `°F`, where NIST specifies one.
fn with_unit(number: String, unit: Option<&str>) -> String {
    match unit {
        None => number,
        Some("%") => format!("{number}%"),
        Some(unit) => format!("{number} {unit}"),
    }
}

fn join(values: &[Value], separator: &str, options: &RenderOptions, range: RangeStyle) -> String {
    values
        .iter()
        .map(|value| format_value(value, options, range))
        .collect::<Vec<_>>()
        .join(separator)
}

fn format_timestamp(timestamp: &OffsetDateTime, options: &RenderOptions) -> String {
    let zoned = match &options.time_zone {
        None => timestamp.to_zoned(),
        Some(time_zone) => timestamp.in_tz(time_zone),
    };
    zoned.strftime(TIMESTAMP_FORMAT).to_string()
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        return format!("{bytes} B");
    }
    let kib = bytes as f64 / 1024.0;
    if kib < 1024.0 {
        return format!("{kib:.2} KiB");
    }
    let mib = kib / 1024.0;
    if mib < 1024.0 {
        return format!("{mib:.2} MiB");
    }
    format!("{:.2} GiB", mib / 1024.0)
}

/// A heading worth printing: present and not empty.
fn heading_or_empty(heading: Option<&String>) -> Option<&str> {
    heading
        .map(String::as_str)
        .filter(|heading| !heading.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at() -> OffsetDateTime {
        "2026-09-02T03:48:00-04:00".parse().unwrap()
    }

    #[test]
    fn timestamps_keep_the_source_offset_and_honor_time_zone() {
        let source = RenderOptions::default();
        assert_eq!(
            format_value(&Value::timestamp(at()), &source, RangeStyle::Words),
            "2026-09-02 03:48 -04:00"
        );
        let utc = RenderOptions {
            time_zone: Some(TimeZone::UTC),
        };
        assert_eq!(
            format_value(&Value::timestamp(at()), &utc, RangeStyle::Words),
            "2026-09-02 07:48 +00:00"
        );
        let central = RenderOptions {
            time_zone: Some(TimeZone::fixed(jiff::tz::offset(-5))),
        };
        assert_eq!(
            format_value(&Value::timestamp(at()), &central, RangeStyle::Words),
            "2026-09-02 02:48 -05:00"
        );
    }

    #[test]
    fn interval_uses_en_dash_and_ongoing() {
        let options = RenderOptions::default();
        assert_eq!(
            format_value(&Value::interval(at(), None), &options, RangeStyle::Words),
            "2026-09-02 03:48 -04:00 \u{2013} ongoing"
        );
        let end: OffsetDateTime = "2026-09-02T09:00:00+00:00".parse().unwrap();
        assert_eq!(
            format_value(
                &Value::interval(at(), Some(end)),
                &options,
                RangeStyle::Words
            ),
            "2026-09-02 03:48 -04:00 \u{2013} 2026-09-02 09:00 +00:00"
        );
    }

    #[test]
    fn quantity_precision_and_unit() {
        let options = RenderOptions::default();
        assert_eq!(
            format_value(
                &Value::number(Some(12.345), 1, Some("mph")),
                &options,
                RangeStyle::Words
            ),
            "12.3 mph"
        );
        assert_eq!(
            format_value(
                &Value::number(Some(12.345), 0, None),
                &options,
                RangeStyle::Words
            ),
            "12"
        );
    }

    #[test]
    fn range_joins_with_a_word_or_an_en_dash() {
        let options = RenderOptions::default();
        let wind = Value::range(Some(16.093), Some(24.140), 0, Some("km/h"));
        assert_eq!(
            format_value(&wind, &options, RangeStyle::Words),
            "16 to 24 km/h"
        );
        assert_eq!(
            format_value(&wind, &options, RangeStyle::Dash),
            "16\u{2013}24 km/h"
        );
        assert_eq!(
            format_value(
                &Value::range(Some(0.5), Some(1.25), 2, None),
                &options,
                RangeStyle::Words
            ),
            "0.50 to 1.25"
        );
    }

    /// The reason the terminal spells the join out: a dash between a negative
    /// bound and a positive one reads as arithmetic.
    #[test]
    fn a_negative_bound_stays_legible_in_words() {
        let chill = Value::range(Some(-5.0), Some(3.0), 0, Some("\u{b0}C"));
        assert_eq!(
            format_value(&chill, &RenderOptions::default(), RangeStyle::Words),
            "-5 to 3 \u{b0}C"
        );
    }

    /// `%` is the one unit that joins tight, so a bounded probability and a
    /// [`Value::Percent`] spell it the same way. A degree sign keeps its
    /// space.
    #[test]
    fn percent_joins_tight_and_other_units_keep_their_space() {
        let options = RenderOptions::default();
        assert_eq!(
            format_value(
                &Value::number(Some(39.6), 0, Some("%")),
                &options,
                RangeStyle::Words
            ),
            "40%"
        );
        assert_eq!(
            format_value(
                &Value::range(Some(20.0), Some(60.0), 0, Some("%")),
                &options,
                RangeStyle::Words
            ),
            "20 to 60%"
        );
        assert_eq!(
            format_value(
                &Value::range(Some(20.0), Some(60.0), 0, Some("%")),
                &options,
                RangeStyle::Dash
            ),
            "20\u{2013}60%"
        );
        assert_eq!(
            format_value(
                &Value::number(Some(23.5), 0, Some("\u{b0}F")),
                &options,
                RangeStyle::Words
            ),
            "24 \u{b0}F"
        );
    }

    #[test]
    fn bytes_humanize() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.00 KiB");
        assert_eq!(format_bytes(1_536_000), "1.46 MiB");
        assert_eq!(format_bytes(5 * 1024 * 1024 * 1024), "5.00 GiB");
    }

    #[test]
    fn simple_variants() {
        let options = RenderOptions::default();
        assert_eq!(
            format_value(&Value::Missing, &options, RangeStyle::Words),
            "N/A"
        );
        assert_eq!(
            format_value(&Value::Invalid, &options, RangeStyle::Words),
            "Invalid"
        );
        assert_eq!(
            format_value(&Value::percent(Some(39.6)), &options, RangeStyle::Words),
            "40%"
        );
        assert_eq!(
            format_value(&Value::count(7), &options, RangeStyle::Words),
            "7"
        );
        assert_eq!(
            format_value(&Value::yes_no(Some(true)), &options, RangeStyle::Words),
            "Yes"
        );
        assert_eq!(
            format_value(
                &Value::coordinates(42.331_427, -83.045_754),
                &options,
                RangeStyle::Words
            ),
            "42.3314, -83.0458"
        );
        assert_eq!(
            format_value(
                &Value::list(vec![
                    Value::identifier("MIZ044"),
                    Value::identifier("MIZ045")
                ]),
                &options,
                RangeStyle::Words
            ),
            "MIZ044, MIZ045"
        );
    }

    #[test]
    fn lines_join_with_newlines() {
        let options = RenderOptions::default();
        assert_eq!(
            format_value(
                &Value::lines(vec![Value::text(Some("NWS Detroit MI")), Value::text(None)]),
                &options,
                RangeStyle::Words
            ),
            "NWS Detroit MI\nN/A"
        );
    }
}
