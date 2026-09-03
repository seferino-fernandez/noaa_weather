//! Constructors that turn raw NOAA fields into [`Value`]s.
//!
//! Each constructor carries a meaning decision: when is a field missing, when
//! is it invalid, what counts as an identifier. Nothing here decides how a
//! value looks; see [`crate::render`] for that.

use noaa_weather_client::OffsetDateTime;

use crate::Value;

impl Value {
    /// Free text. `None` or text that is blank after trimming is [`Value::Missing`].
    pub fn text(text: Option<&str>) -> Self {
        match text.map(str::trim) {
            None | Some("") => Self::Missing,
            Some(text) => Self::Text(text.to_owned()),
        }
    }

    /// A measurement shown with `precision` decimal places and an optional
    /// unit label. `None` is [`Value::Missing`]; a non-finite number is
    /// [`Value::Invalid`].
    pub fn number(value: Option<f64>, precision: u8, unit: Option<&str>) -> Self {
        match value {
            None => Self::Missing,
            Some(value) if !value.is_finite() => Self::Invalid,
            Some(value) => Self::Quantity {
                value,
                unit: unit.map(str::to_owned),
                precision,
            },
        }
    }

    /// A percentage. `None` is [`Value::Missing`]; a non-finite number is
    /// [`Value::Invalid`].
    pub fn percent(value: Option<f64>) -> Self {
        match value {
            None => Self::Missing,
            Some(value) if !value.is_finite() => Self::Invalid,
            Some(value) => Self::Percent(value),
        }
    }

    /// A whole number of things.
    pub fn count(count: u64) -> Self {
        Self::Count(count)
    }

    /// A size in bytes.
    pub fn bytes(bytes: u64) -> Self {
        Self::Bytes(bytes)
    }

    /// A yes-or-no flag. `None` is [`Value::Missing`].
    pub fn yes_no(flag: Option<bool>) -> Self {
        flag.map_or(Self::Missing, Self::YesNo)
    }

    /// A single instant.
    pub fn timestamp(timestamp: OffsetDateTime) -> Self {
        Self::Timestamp(timestamp)
    }

    /// A span; `None` for `end` means it is still ongoing.
    pub fn interval(start: OffsetDateTime, end: Option<OffsetDateTime>) -> Self {
        Self::Interval { start, end }
    }

    /// An identifier the reader may need for the next command.
    pub fn identifier(id: impl Into<String>) -> Self {
        Self::Identifier(id.into())
    }

    /// The identifier at the end of a NOAA URL: its last non-empty path
    /// segment. A URL without one is [`Value::Missing`].
    pub fn identifier_from_url(url: &str) -> Self {
        url.rsplit('/')
            .find(|segment| !segment.is_empty())
            .map_or(Self::Missing, |segment| {
                Self::Identifier(segment.to_owned())
            })
    }

    /// A geographic point in decimal degrees.
    pub fn coordinates(lat: f64, lon: f64) -> Self {
        Self::Coordinates { lat, lon }
    }

    /// Several values shown together. An empty list is [`Value::Missing`].
    pub fn list(values: Vec<Value>) -> Self {
        if values.is_empty() {
            Self::Missing
        } else {
            Self::List(values)
        }
    }

    /// Several values shown one per line. An empty list is [`Value::Missing`].
    pub fn lines(values: Vec<Value>) -> Self {
        if values.is_empty() {
            Self::Missing
        } else {
            Self::Lines(values)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_trims_and_treats_blank_as_missing() {
        assert_eq!(Value::text(None), Value::Missing);
        assert_eq!(Value::text(Some("   ")), Value::Missing);
        assert_eq!(Value::text(Some("")), Value::Missing);
        assert_eq!(
            Value::text(Some("  Wayne  ")),
            Value::Text("Wayne".to_owned())
        );
    }

    #[test]
    fn number_classifies_missing_and_invalid() {
        assert_eq!(Value::number(None, 1, Some("mph")), Value::Missing);
        assert_eq!(Value::number(Some(f64::NAN), 1, None), Value::Invalid);
        assert_eq!(Value::number(Some(f64::INFINITY), 1, None), Value::Invalid);
        assert_eq!(
            Value::number(Some(12.5), 1, Some("mph")),
            Value::Quantity {
                value: 12.5,
                unit: Some("mph".to_owned()),
                precision: 1,
            }
        );
    }

    #[test]
    fn percent_classifies_missing_and_invalid() {
        assert_eq!(Value::percent(None), Value::Missing);
        assert_eq!(Value::percent(Some(f64::NEG_INFINITY)), Value::Invalid);
        assert_eq!(Value::percent(Some(40.0)), Value::Percent(40.0));
    }

    #[test]
    fn count_and_bytes_wrap_plainly() {
        assert_eq!(Value::count(3), Value::Count(3));
        assert_eq!(Value::bytes(2048), Value::Bytes(2048));
    }

    #[test]
    fn yes_no_treats_none_as_missing() {
        assert_eq!(Value::yes_no(None), Value::Missing);
        assert_eq!(Value::yes_no(Some(true)), Value::YesNo(true));
    }

    #[test]
    fn timestamp_and_interval_wrap_instants() {
        let start: OffsetDateTime = "2026-09-02T03:48:00-04:00".parse().unwrap();
        let end: OffsetDateTime = "2026-09-02T05:00:00-04:00".parse().unwrap();
        assert_eq!(Value::timestamp(start), Value::Timestamp(start));
        assert_eq!(
            Value::interval(start, Some(end)),
            Value::Interval {
                start,
                end: Some(end)
            }
        );
        assert_eq!(
            Value::interval(start, None),
            Value::Interval { start, end: None }
        );
    }

    #[test]
    fn identifier_from_url_takes_last_non_empty_segment() {
        assert_eq!(
            Value::identifier_from_url("https://api.weather.gov/zones/forecast/MIZ044"),
            Value::Identifier("MIZ044".to_owned())
        );
        assert_eq!(
            Value::identifier_from_url("https://api.weather.gov/zones/forecast/MIZ044/"),
            Value::Identifier("MIZ044".to_owned())
        );
        assert_eq!(Value::identifier_from_url(""), Value::Missing);
        assert_eq!(Value::identifier_from_url("///"), Value::Missing);
        assert_eq!(
            Value::identifier("KDTX"),
            Value::Identifier("KDTX".to_owned())
        );
    }

    #[test]
    fn coordinates_wrap_plainly() {
        assert_eq!(
            Value::coordinates(42.33, -83.05),
            Value::Coordinates {
                lat: 42.33,
                lon: -83.05
            }
        );
    }

    #[test]
    fn empty_list_is_missing() {
        assert_eq!(Value::list(Vec::new()), Value::Missing);
        assert_eq!(
            Value::list(vec![Value::count(1)]),
            Value::List(vec![Value::Count(1)])
        );
    }

    #[test]
    fn empty_lines_are_missing() {
        assert_eq!(Value::lines(Vec::new()), Value::Missing);
        assert_eq!(
            Value::lines(vec![Value::text(Some("NWS Detroit")), Value::text(None)]),
            Value::Lines(vec![Value::Text("NWS Detroit".to_owned()), Value::Missing])
        );
    }
}
