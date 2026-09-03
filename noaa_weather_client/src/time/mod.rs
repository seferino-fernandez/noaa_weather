//! Time values for NOAA requests and responses.
//!
//! NOAA query parameters that name one instant (`start`, `end`, `time`) take
//! an RFC 3339 timestamp, and the type for them throughout this crate is
//! `Option<jiff::Timestamp>`. There is no relative-time enum: a caller who
//! wants "six hours ago" resolves it to a [`jiff::Timestamp`] first, and
//! the CLI does the same before it calls the client.
//!
//! Parameters that name a period (`interval`, `arrived`, `created`,
//! `published`) take an [`Interval`], an ISO 8601 time interval in any of
//! its four forms: `start/end`, `start/duration`, `duration/end`, or a bare
//! `duration`.
//!
//! Timestamps inside responses are [`OffsetDateTime`]: the instant plus the
//! UTC offset NOAA wrote it in, so `2026-09-02T03:48:00-04:00` reads back
//! and prints as exactly that text while comparing equal to the same
//! instant in any other offset.
//!
//! ```
//! use noaa_weather_client::Interval;
//!
//! let last_hour = Interval::lasting("PT1H".parse::<jiff::Span>().unwrap())?;
//! assert_eq!(last_hour.to_string(), "PT1H");
//!
//! let interval: Interval = "2024-01-01T00:00:00Z/PT6H".parse()?;
//! assert_eq!(interval.start().unwrap().to_string(), "2024-01-01T00:00:00Z");
//! // Either offset form parses; both print with a numeric offset.
//! assert_eq!(interval.to_string(), "2024-01-01T00:00:00+00:00/PT6H");
//! # Ok::<(), noaa_weather_client::InvalidValue>(())
//! ```

mod interval;
mod offset_date_time;

pub use interval::Interval;
pub use offset_date_time::OffsetDateTime;

/// RFC 3339 in UTC with whole seconds, the only timestamp form NOAA accepts
/// in query parameters and path segments. Formatting with it truncates any
/// sub-second precision.
pub(crate) const RFC3339_SECONDS: &str = "%Y-%m-%dT%H:%M:%SZ";

/// The same instant with a numeric `+00:00` suffix instead of `Z`.
///
/// [`Interval`] writes its endpoints this way so that one document never
/// mixes offset conventions: [`OffsetDateTime`] prints every response
/// timestamp with a numeric offset, and NOAA writes `validTime` and
/// `validTimes` as `+00:00` too. Both forms denote the same instant and
/// NOAA accepts either in its `interval`, `arrived`, `created`, `published`,
/// and `time` query parameters.
pub(crate) const RFC3339_OFFSET: &str = "%Y-%m-%dT%H:%M:%S%:z";
