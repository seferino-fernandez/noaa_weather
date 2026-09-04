//! Sunrise, sunset, and twilight data returned in a [`Point`](super::Point).
//!
//! # Requiredness
//!
//! A live census of 21 points across 18 forecast offices (CONUS, Alaska,
//! Hawaii, Puerto Rico, the U.S. Virgin Islands, Guam, and American Samoa) on
//! 2026-09-03 found every key in this object present on every response.
//! `astronomicalTwilightBegin` and `astronomicalTwilightEnd` were `null` for
//! Fairbanks, Alaska, so those two endpoints are nullable. The other seven
//! timestamps were non-null on every response and are required. `None`
//! serializes as `null`, preserving NOAA's always-present key shape.

use serde::{Deserialize, Serialize};

use crate::time::OffsetDateTime;

/// Sunrise, sunset, and twilight information for a point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AstronomicalData {
    /// Sunrise. NOAA sent this key with a timestamp in all 21 points sampled
    /// on 2026-09-03.
    pub sunrise: OffsetDateTime,
    /// Sunset. NOAA sent this key with a timestamp in all 21 points sampled
    /// on 2026-09-03.
    pub sunset: OffsetDateTime,
    /// Solar transit. NOAA sent this key with a timestamp in all 21 points
    /// sampled on 2026-09-03.
    pub transit: OffsetDateTime,
    /// Beginning of civil twilight. NOAA sent this key with a timestamp in
    /// all 21 points sampled on 2026-09-03.
    pub civil_twilight_begin: OffsetDateTime,
    /// End of civil twilight. NOAA sent this key with a timestamp in all 21
    /// points sampled on 2026-09-03.
    pub civil_twilight_end: OffsetDateTime,
    /// Beginning of nautical twilight. NOAA sent this key with a timestamp
    /// in all 21 points sampled on 2026-09-03.
    pub nautical_twilight_begin: OffsetDateTime,
    /// End of nautical twilight. NOAA sent this key with a timestamp in all
    /// 21 points sampled on 2026-09-03.
    pub nautical_twilight_end: OffsetDateTime,
    /// Beginning of astronomical twilight. NOAA sent this key in all 21
    /// points sampled on 2026-09-03; Fairbanks returned `null`.
    pub astronomical_twilight_begin: Option<OffsetDateTime>,
    /// End of astronomical twilight. NOAA sent this key in all 21 points
    /// sampled on 2026-09-03; Fairbanks returned `null`.
    pub astronomical_twilight_end: Option<OffsetDateTime>,
}
