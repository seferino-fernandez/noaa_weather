//! Geographic values for NOAA requests and the GeoJSON envelopes NOAA wraps
//! responses in.
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
//!
//! # Envelopes
//!
//! NOAA returns most resources as GeoJSON (RFC 7946). A single resource is a
//! [`Feature<T>`] whose `properties` is the model from [`crate::models`]; a
//! list is a [`FeatureCollection<T>`] of such features, optionally decorated
//! with `title`, `updated`, and [`Pagination`]. Both envelopes are generic so
//! one pair of types serves every operation, and `Feature<T>` dereferences to
//! `T` so property access reads naturally. Geometry is a [`Geometry`] enum of
//! [`Position`] (`[longitude, latitude]`) arrays, or `None` when NOAA sent
//! `null`.
//!
//! The JSON-LD `@context` member and the `observationStations` list that
//! duplicates `features[].id` are the only response members not represented.

mod coordinates;
mod feature;
mod feature_collection;
mod geometry;
mod position;

pub use coordinates::Coordinates;
pub use feature::Feature;
pub use feature_collection::{FeatureCollection, Pagination};
pub use geometry::Geometry;
pub use position::Position;
