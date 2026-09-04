//! The properties of one Center Weather Advisory (CWA) feature.
//!
//! # Requiredness
//!
//! The 2026-09-03 live census covers 411 CWAs. `id`, `issueTime`, `cwsu`,
//! `sequence`, `start`, `end`, and `text` were present and non-null in all
//! 411. `observedProperty` was keyed in all 411 but was null sometimes, so it
//! remains an `Option` that serializes as `null`.

use serde::{Deserialize, Serialize};

use crate::ids::CwsuId;
use crate::time::OffsetDateTime;

/// A Center Weather Advisory returned in a GeoJSON feature's `properties`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CenterWeatherAdvisory {
    /// The advisory's canonical API URL. Present and non-null in all 411 live
    /// CWAs in the 2026-09-03 census.
    pub id: String,
    /// When NOAA issued the advisory. Present and non-null in all 411 live
    /// CWAs in the 2026-09-03 census.
    pub issue_time: OffsetDateTime,
    /// The issuing Center Weather Service Unit. Present and non-null in all
    /// 411 live CWAs in the 2026-09-03 census; every observed value parses as
    /// a [`CwsuId`].
    pub cwsu: CwsuId,
    /// NOAA's numeric advisory sequence. Present and non-null in all 411 live
    /// CWAs in the 2026-09-03 census.
    pub sequence: u32,
    /// When the advisory becomes valid. Present and non-null in all 411 live
    /// CWAs in the 2026-09-03 census.
    pub start: OffsetDateTime,
    /// When the advisory ceases to be valid. Present and non-null in all 411
    /// live CWAs in the 2026-09-03 census.
    pub end: OffsetDateTime,
    /// NOAA's basis-for-issuance URI. The key was present in all 411 live
    /// CWAs in the 2026-09-03 census but was sometimes null, so `None`
    /// serializes as `null`.
    pub observed_property: Option<String>,
    /// The advisory text. Present and non-null in all 411 live CWAs in the
    /// 2026-09-03 census.
    pub text: String,
}
