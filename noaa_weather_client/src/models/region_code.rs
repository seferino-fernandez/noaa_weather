use std::fmt::{self, Display};

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegionCode {
    LandRegionCode(models::LandRegionCode),
    MarineRegionCode(models::MarineRegionCode),
}

impl Display for RegionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegionCode::LandRegionCode(code) => write!(f, "{code}"),
            RegionCode::MarineRegionCode(code) => write!(f, "{code}"),
        }
    }
}
