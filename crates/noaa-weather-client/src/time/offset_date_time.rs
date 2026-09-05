use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use jiff::{
    Timestamp, Zoned,
    fmt::{
        StdFmtWrite,
        temporal::{DateTimePrinter, Pieces},
    },
    tz::{Offset, TimeZone},
};
use serde::{Deserialize, Serialize};

use crate::ids::{InvalidValue, ValueKind};

const REASON: &str = "must be an RFC 3339 timestamp with a date, a time, and a Z or \
                      ±HH:MM offset (for example 2026-09-02T03:48:00-04:00)";

/// Prints whole seconds, fractional seconds only when they are non-zero, and
/// the offset as `±HH:MM` (`+00:00` for UTC).
const PRINTER: DateTimePrinter = DateTimePrinter::new();

/// An instant paired with the UTC offset it was written in.
///
/// NOAA reports every timestamp in a response (`sent`, `effective`,
/// `updated`, ...) as RFC 3339 text carrying the issuing office's local
/// offset, for example `2026-09-02T03:48:00-04:00`. A bare
/// [`jiff::Timestamp`] would keep the instant and lose the offset; a
/// [`jiff::Zoned`] cannot be built from an offset alone. `OffsetDateTime`
/// keeps both, so the text NOAA sent is the text this type prints.
///
/// # Equality is by instant
///
/// `PartialEq`, `Eq`, `Ord`, `PartialOrd`, and `Hash` consider only the
/// instant: `2026-09-02T03:48:00-04:00` equals `2026-09-02T07:48:00+00:00`.
/// The offset is presentation, reachable through [`OffsetDateTime::offset`]
/// and visible in `Display`, `Debug`, and serde output.
///
/// # Round trip
///
/// Parsing then printing reproduces the input for the text NOAA sends:
/// uppercase `T`, whole or fractional seconds, and a numeric offset.
/// Accepted variants normalize on output: `Z` prints as `+00:00` (the form
/// NOAA itself uses for UTC), a missing seconds field prints as `:00`, and a
/// lowercase `t`/`z` or a space separator prints in the canonical form.
/// Fractional seconds are kept and printed only when non-zero.
///
/// ```
/// use noaa_weather_client::OffsetDateTime;
///
/// let sent: OffsetDateTime = "2026-09-02T03:48:00-04:00".parse()?;
/// assert_eq!(sent.to_string(), "2026-09-02T03:48:00-04:00");
/// assert_eq!(sent.offset().seconds(), -4 * 60 * 60);
///
/// let same_instant: OffsetDateTime = "2026-09-02T07:48:00Z".parse()?;
/// assert_eq!(sent, same_instant);
/// assert_eq!(same_instant.to_string(), "2026-09-02T07:48:00+00:00");
/// # Ok::<(), noaa_weather_client::InvalidValue>(())
/// ```
#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct OffsetDateTime {
    timestamp: Timestamp,
    offset: Offset,
}

impl OffsetDateTime {
    /// Pairs an instant with the offset it should be displayed in.
    #[must_use]
    pub const fn new(timestamp: Timestamp, offset: Offset) -> Self {
        Self { timestamp, offset }
    }

    /// Returns the instant, independent of offset.
    #[must_use]
    pub const fn timestamp(self) -> Timestamp {
        self.timestamp
    }

    /// Returns the UTC offset the instant was written in.
    #[must_use]
    pub const fn offset(self) -> Offset {
        self.offset
    }

    /// Returns the instant as a [`Zoned`] in a fixed-offset time zone.
    ///
    /// The result prints as `…-04:00[-04:00]`, jiff's form for a zone that
    /// is only an offset. Use [`OffsetDateTime::in_tz`] to view it in a
    /// named zone instead.
    #[must_use]
    pub fn to_zoned(self) -> Zoned {
        self.timestamp.to_zoned(self.offset.to_time_zone())
    }

    /// Returns the instant as a [`Zoned`] in `time_zone`, discarding the
    /// original offset.
    #[must_use]
    pub fn in_tz(self, time_zone: &TimeZone) -> Zoned {
        self.timestamp.to_zoned(time_zone.clone())
    }
}

impl PartialEq for OffsetDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for OffsetDateTime {}

impl PartialEq<Timestamp> for OffsetDateTime {
    fn eq(&self, other: &Timestamp) -> bool {
        self.timestamp == *other
    }
}

impl PartialOrd for OffsetDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OffsetDateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl Hash for OffsetDateTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.timestamp.hash(state);
    }
}

impl From<Timestamp> for OffsetDateTime {
    /// Pairs the instant with the UTC offset.
    fn from(timestamp: Timestamp) -> Self {
        Self::new(timestamp, Offset::UTC)
    }
}

impl From<OffsetDateTime> for Timestamp {
    fn from(value: OffsetDateTime) -> Self {
        value.timestamp
    }
}

impl fmt::Debug for OffsetDateTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "OffsetDateTime({self})")
    }
}

impl fmt::Display for OffsetDateTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        PRINTER
            .print_timestamp_with_offset(&self.timestamp, self.offset, StdFmtWrite(formatter))
            .map_err(|_| fmt::Error)
    }
}

impl FromStr for OffsetDateTime {
    type Err = InvalidValue;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reject = || InvalidValue::new(ValueKind::Timestamp, input, REASON);
        let pieces = Pieces::parse(input).map_err(|_| reject())?;
        if pieces.time_zone_annotation().is_some() {
            return Err(reject());
        }
        let time = pieces.time().ok_or_else(reject)?;
        let offset = pieces.to_numeric_offset().ok_or_else(reject)?;
        // jiff accepts offsets up to ±25:59:59; RFC 3339 stops at 23:59.
        if offset.seconds().abs() >= 24 * 60 * 60 {
            return Err(reject());
        }
        let timestamp = offset
            .to_timestamp(pieces.date().to_datetime(time))
            .map_err(|_| reject())?;
        Ok(Self { timestamp, offset })
    }
}

impl From<OffsetDateTime> for String {
    fn from(value: OffsetDateTime) -> Self {
        value.to_string()
    }
}

impl_try_from_str!(OffsetDateTime);
impl_string_schema!(
    OffsetDateTime,
    "RFC 3339 timestamp with a numeric UTC offset, as NOAA sends it (for example \
     2026-09-02T03:48:00-04:00). Equality is by instant; the offset is kept for display.",
    "^[0-9]{4}-[0-9]{2}-[0-9]{2}[Tt ][0-9]{2}:[0-9]{2}(:[0-9]{2}([.,][0-9]{1,9})?)?\
     ([Zz]|[+-][0-9]{2}(:?[0-9]{2}(:?[0-9]{2})?)?)$"
);

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn at(text: &str) -> OffsetDateTime {
        text.parse().unwrap()
    }

    #[test]
    fn round_trips_noaa_offsets_and_normalizes_z() {
        for text in [
            "2026-09-02T03:48:00-04:00",
            "2026-09-02T07:48:22+00:00",
            "2026-09-02T02:46:00-05:00",
            "2026-09-02T13:18:00+05:30",
            "2026-09-02T03:48:00.5-04:00",
            "2026-09-02T03:48:00.123456789-04:00",
        ] {
            let parsed = at(text);
            assert_eq!(parsed.to_string(), text);
            assert_eq!(
                parsed.to_string().parse::<OffsetDateTime>().unwrap(),
                parsed
            );
            assert_eq!(String::from(parsed), text);
        }
        let zulu = at("2026-09-02T07:48:00Z");
        assert_eq!(zulu.to_string(), "2026-09-02T07:48:00+00:00");
        assert_eq!(zulu.offset(), Offset::UTC);
        assert_eq!(
            at("2026-09-02T07:48:00.000Z").to_string(),
            "2026-09-02T07:48:00+00:00"
        );
        // Seconds may be omitted on input; the text form always has them.
        assert_eq!(
            at("2026-09-02T03:48-04:00").to_string(),
            "2026-09-02T03:48:00-04:00"
        );
    }

    #[test]
    fn keeps_the_offset_and_the_instant_separately() {
        let sent = at("2026-09-02T03:48:00-04:00");
        assert_eq!(sent.offset().seconds(), -4 * 3600);
        assert_eq!(
            sent.timestamp(),
            "2026-09-02T07:48:00Z".parse::<Timestamp>().unwrap()
        );
        assert_eq!(sent, "2026-09-02T07:48:00Z".parse::<Timestamp>().unwrap());
        assert_eq!(Timestamp::from(sent), sent.timestamp());
        assert_eq!(
            OffsetDateTime::new(sent.timestamp(), sent.offset()).to_string(),
            "2026-09-02T03:48:00-04:00"
        );
        assert_eq!(
            format!("{sent:?}"),
            "OffsetDateTime(2026-09-02T03:48:00-04:00)"
        );
    }

    #[test]
    fn from_timestamp_uses_utc() {
        let instant: Timestamp = "2026-09-02T07:48:00Z".parse().unwrap();
        let value = OffsetDateTime::from(instant);
        assert_eq!(value.offset(), Offset::UTC);
        assert_eq!(value.to_string(), "2026-09-02T07:48:00+00:00");
    }

    #[test]
    fn equality_ordering_and_hashing_are_by_instant() {
        let eastern = at("2026-09-02T03:48:00-04:00");
        let utc = at("2026-09-02T07:48:00+00:00");
        let central = at("2026-09-02T02:46:00-05:00");
        assert_eq!(eastern, utc);
        assert_ne!(eastern.offset(), utc.offset());
        assert!(central < eastern, "{central} should precede {eastern}");
        assert_eq!(eastern.cmp(&utc), Ordering::Equal);
        assert_eq!(eastern.partial_cmp(&central), Some(Ordering::Greater));
        let mut sorted = [eastern, central, utc];
        sorted.sort();
        assert_eq!(sorted[0], central);
        let set: HashSet<OffsetDateTime> = [eastern, utc, central].into_iter().collect();
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn zoned_views_keep_the_instant() {
        let sent = at("2026-09-02T03:48:00-04:00");
        let fixed = sent.to_zoned();
        assert_eq!(fixed.timestamp(), sent.timestamp());
        assert_eq!(fixed.offset(), sent.offset());
        assert_eq!(fixed.hour(), 3);
        let utc = sent.in_tz(&TimeZone::UTC);
        assert_eq!(utc.timestamp(), sent.timestamp());
        assert_eq!(utc.hour(), 7);
    }

    #[test]
    fn rejects_missing_offsets_annotations_and_garbage() {
        for input in [
            "",
            "2026-09-02",
            "2026-09-02T03:48:00",
            "2026-09-02T03:48",
            "2026-09-02T03:48:00-04:00[America/New_York]",
            "2026-09-02T03:48:00[America/New_York]",
            "03:48:00-04:00",
            "yesterday",
            "2026-13-02T03:48:00-04:00",
            "2026-09-02T25:48:00-04:00",
            "2026-09-02T03:48:00-24:00",
            "2026-09-02T03:48:00 -04:00",
            "1756799280",
            " 2026-09-02T03:48:00-04:00",
            "2026-09-02T03:48:00-04:00\n",
        ] {
            let error = input.parse::<OffsetDateTime>().unwrap_err();
            assert_eq!(error.kind(), ValueKind::Timestamp, "{input:?}");
            assert_eq!(error.input(), input);
            assert_eq!(error.reason(), REASON);
        }
        assert_eq!(
            "nope".parse::<OffsetDateTime>().unwrap_err().to_string(),
            format!("invalid timestamp \"nope\": {REASON}")
        );
    }

    #[test]
    fn serde_uses_the_text_form() {
        let sent = at("2026-09-02T03:48:00-04:00");
        let json = serde_json::to_string(&sent).unwrap();
        assert_eq!(json, "\"2026-09-02T03:48:00-04:00\"");
        let parsed: OffsetDateTime = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, sent);
        assert_eq!(parsed.offset(), sent.offset());
        assert_eq!(
            OffsetDateTime::try_from("2026-09-02T07:48:00Z").unwrap(),
            sent
        );
        assert_eq!(
            OffsetDateTime::try_from(String::from("2026-09-02T07:48:00Z")).unwrap(),
            sent
        );
        let error = serde_json::from_str::<OffsetDateTime>("\"2026-09-02T03:48:00\"").unwrap_err();
        assert!(error.to_string().contains("invalid timestamp"), "{error}");
        let error = serde_json::from_str::<OffsetDateTime>("1756799280").unwrap_err();
        assert!(error.to_string().contains("expected a string"), "{error}");
    }

    #[test]
    fn stays_copy_and_compact() {
        fn assert_copy<T: Copy + Send + Sync + 'static>() {}
        assert_copy::<OffsetDateTime>();
        assert!(std::mem::size_of::<OffsetDateTime>() <= 24);
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_pattern_agrees_with_the_parser() {
        crate::ids::schema_tests::assert_pattern_matches_parser::<OffsetDateTime>(
            &[
                "2026-09-02T03:48:00-04:00",
                "2026-09-02T07:48:22+00:00",
                "2026-09-02T07:48:00Z",
                "2026-09-02t07:48:00z",
                "2026-09-02 07:48:00Z",
                "2026-09-02T03:48:00.5-04:00",
                "2026-09-02T03:48:00.123456789-04:00",
                "2026-09-02T13:18:00+05:30",
                "2026-09-02T03:48-04:00",
            ],
            &[
                "",
                "2026-09-02",
                "2026-09-02T03:48:00",
                "2026-09-02T03:48",
                "2026-09-02T03:48:00-04:00[America/New_York]",
                "2026-09-02T03:48:00 -04:00",
                "2026-09-02T03:48:00-04:00\n",
                "1756799280",
                "yesterday",
            ],
        );
        let schema = schemars::schema_for!(OffsetDateTime);
        let value = schema.as_value();
        assert_eq!(value["type"], "string");
        assert!(value.get("description").is_some());
    }
}
