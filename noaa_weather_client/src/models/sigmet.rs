//! The properties of one aviation SIGMET or AIRMET feature.
//!
//! # Requiredness
//!
//! The 2026-09-03 live census covers 1,093 SIGMETs. `id`, `issueTime`,
//! `atsu`, `start`, and `end` were present and non-null in all 1,093. `fir`,
//! `sequence`, and `phenomenon` were also keyed in every response, but each
//! was null sometimes, so each is an `Option` that serializes as `null`.
//! This is a genuinely sparse family: none of the nullable fields are made
//! required merely because the key is always present.

use serde::{Deserialize, Serialize};

use crate::ids::AtsuId;
use crate::time::OffsetDateTime;

/// A SIGMET or AIRMET product returned in a GeoJSON feature's `properties`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Sigmet {
    /// The product's canonical API URL. Present and non-null in all 1,093
    /// live SIGMETs in the 2026-09-03 census.
    pub id: String,
    /// When NOAA issued the product. Present and non-null in all 1,093 live
    /// SIGMETs in the 2026-09-03 census.
    pub issue_time: OffsetDateTime,
    /// The flight information region, such as `KZAB`. The key was present in
    /// all 1,093 live SIGMETs in the 2026-09-03 census but was sometimes
    /// null, so `None` serializes as `null`.
    pub fir: Option<String>,
    /// The issuing Air Traffic Service Unit. Present and non-null in all
    /// 1,093 live SIGMETs in the 2026-09-03 census; every observed value
    /// follows [`AtsuId`]'s 3-to-4-character rule, including `ANC`, `FAI`,
    /// `HNL`, and `JNU`.
    pub atsu: AtsuId,
    /// NOAA's product sequence, such as `27C`. The key was present in all
    /// 1,093 live SIGMETs in the 2026-09-03 census but was sometimes null,
    /// so `None` serializes as `null`.
    pub sequence: Option<String>,
    /// The phenomenon URI, if NOAA assigned one. The key was present in all
    /// 1,093 live SIGMETs in the 2026-09-03 census but was sometimes null,
    /// so `None` serializes as `null`.
    pub phenomenon: Option<String>,
    /// When the product becomes valid. Present and non-null in all 1,093
    /// live SIGMETs in the 2026-09-03 census.
    pub start: OffsetDateTime,
    /// When the product ceases to be valid. Present and non-null in all 1,093
    /// live SIGMETs in the 2026-09-03 census.
    pub end: OffsetDateTime,
}
