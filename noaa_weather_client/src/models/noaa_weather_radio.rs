//! NOAA Weather Radio data returned in a [`Point`](super::Point).
//!
//! This is the `nwr` object of a `/points` response, not a model in the
//! `/radio` endpoint family.
//!
//! # Requiredness
//!
//! A live census of 21 points across 18 forecast offices (CONUS, Alaska,
//! Hawaii, Puerto Rico, the U.S. Virgin Islands, Guam, and American Samoa) on
//! 2026-09-03 found this object and all four of its keys on every response.
//! Ponce, Puerto Rico returned `null` for `transmitter` and `areaBroadcast`,
//! so those fields are nullable. The SAME code and point broadcast URL were
//! non-null in all 21 responses and are core identifiers for this coverage.

use serde::{Deserialize, Serialize};

/// NOAA Weather Radio coverage metadata for a point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct NoaaWeatherRadio {
    /// Transmitter callsign. NOAA sent this key in all 21 points sampled on
    /// 2026-09-03; Ponce returned `null`.
    pub transmitter: Option<String>,
    /// The SAME code of this point's county. NOAA sent this key with a value
    /// in all 21 points sampled on 2026-09-03.
    pub same_code: String,
    /// A link to the area NWR broadcast from this transmitter. NOAA sent this
    /// key in all 21 points sampled on 2026-09-03; Ponce returned `null`.
    pub area_broadcast: Option<String>,
    /// A link to the local NWR broadcast for this point. NOAA sent this key
    /// with a value in all 21 points sampled on 2026-09-03.
    pub point_broadcast: String,
}
