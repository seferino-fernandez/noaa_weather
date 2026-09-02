//! Time values for NOAA requests.
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
//! ```
//! use noaa_weather_client::Interval;
//!
//! let last_hour = Interval::lasting("PT1H".parse::<jiff::Span>().unwrap())?;
//! assert_eq!(last_hour.to_string(), "PT1H");
//!
//! let interval: Interval = "2024-01-01T00:00:00Z/PT6H".parse()?;
//! assert_eq!(interval.start().unwrap().to_string(), "2024-01-01T00:00:00Z");
//! # Ok::<(), noaa_weather_client::InvalidValue>(())
//! ```

mod interval;

pub use interval::Interval;

/// RFC 3339 in UTC with whole seconds, the only timestamp form NOAA accepts
/// in query parameters and path segments. Formatting with it truncates any
/// sub-second precision.
pub(crate) const RFC3339_SECONDS: &str = "%Y-%m-%dT%H:%M:%SZ";
