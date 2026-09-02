//! Geographic values for NOAA requests.
//!
//! [`Coordinates`] is the validated `latitude,longitude` pair used by
//! `/points/{point}` and `/points/{point}/radio`. It is range-checked and
//! rounded to four decimals at construction, which is the precision NOAA
//! accepts, so two coordinates that name the same NOAA point compare equal.
//!
//! ```
//! use noaa_weather_client::Coordinates;
//!
//! let here = Coordinates::new(39.74561, -97.08919)?;
//! assert_eq!(here.to_string(), "39.7456,-97.0892");
//! assert_eq!(here, "39.7456, -97.0892".parse()?);
//! # Ok::<(), noaa_weather_client::InvalidValue>(())
//! ```

mod coordinates;

pub use coordinates::Coordinates;
