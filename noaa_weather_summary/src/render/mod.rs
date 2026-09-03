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

const MISSING: &str = "N/A";
const INVALID: &str = "Invalid";
const TIMESTAMP_FORMAT: &str = "%Y-%m-%d %H:%M %:z";

/// Formats a value as text, without any markup.
///
/// [`Value::Lines`] joins with newlines, so a caller that puts the result in a
/// cell must decide what a newline means there: the markdown renderer writes
/// `<br>`, the plain renderer writes `; `, and a terminal table can take the
/// newline as it stands.
pub fn format_value(value: &Value, options: &RenderOptions) -> String {
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
            match unit {
                Some(unit) => format!("{value:.precision$} {unit}"),
                None => format!("{value:.precision$}"),
            }
        }
        Value::Percent(percent) => format!("{percent:.0}%"),
        Value::Count(count) => count.to_string(),
        Value::Bytes(bytes) => format_bytes(*bytes),
        Value::YesNo(true) => "Yes".to_owned(),
        Value::YesNo(false) => "No".to_owned(),
        Value::Coordinates { lat, lon } => format!("{lat:.4}, {lon:.4}"),
        Value::List(values) => join(values, ", ", options),
        Value::Lines(values) => join(values, "\n", options),
    }
}

fn join(values: &[Value], separator: &str, options: &RenderOptions) -> String {
    values
        .iter()
        .map(|value| format_value(value, options))
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
            format_value(&Value::timestamp(at()), &source),
            "2026-09-02 03:48 -04:00"
        );
        let utc = RenderOptions {
            time_zone: Some(TimeZone::UTC),
        };
        assert_eq!(
            format_value(&Value::timestamp(at()), &utc),
            "2026-09-02 07:48 +00:00"
        );
        let central = RenderOptions {
            time_zone: Some(TimeZone::fixed(jiff::tz::offset(-5))),
        };
        assert_eq!(
            format_value(&Value::timestamp(at()), &central),
            "2026-09-02 02:48 -05:00"
        );
    }

    #[test]
    fn interval_uses_en_dash_and_ongoing() {
        let options = RenderOptions::default();
        assert_eq!(
            format_value(&Value::interval(at(), None), &options),
            "2026-09-02 03:48 -04:00 \u{2013} ongoing"
        );
        let end: OffsetDateTime = "2026-09-02T09:00:00+00:00".parse().unwrap();
        assert_eq!(
            format_value(&Value::interval(at(), Some(end)), &options),
            "2026-09-02 03:48 -04:00 \u{2013} 2026-09-02 09:00 +00:00"
        );
    }

    #[test]
    fn quantity_precision_and_unit() {
        let options = RenderOptions::default();
        assert_eq!(
            format_value(&Value::number(Some(12.345), 1, Some("mph")), &options),
            "12.3 mph"
        );
        assert_eq!(
            format_value(&Value::number(Some(12.345), 0, None), &options),
            "12"
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
        assert_eq!(format_value(&Value::Missing, &options), "N/A");
        assert_eq!(format_value(&Value::Invalid, &options), "Invalid");
        assert_eq!(format_value(&Value::percent(Some(39.6)), &options), "40%");
        assert_eq!(format_value(&Value::count(7), &options), "7");
        assert_eq!(format_value(&Value::yes_no(Some(true)), &options), "Yes");
        assert_eq!(
            format_value(&Value::coordinates(42.331_427, -83.045_754), &options),
            "42.3314, -83.0458"
        );
        assert_eq!(
            format_value(
                &Value::list(vec![
                    Value::identifier("MIZ044"),
                    Value::identifier("MIZ045")
                ]),
                &options
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
                &options
            ),
            "NWS Detroit MI\nN/A"
        );
    }
}
