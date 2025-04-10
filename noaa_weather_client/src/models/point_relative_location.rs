use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointRelativeLocation {
    RelativeLocationGeoJson(Box<models::RelativeLocationGeoJson>),
    RelativeLocationJsonLd(Box<models::RelativeLocationJsonLd>),
}

impl Default for PointRelativeLocation {
    fn default() -> Self {
        Self::RelativeLocationGeoJson(Default::default())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Feature")]
    Feature,
}

impl Default for Type {
    fn default() -> Type {
        Self::Feature
    }
}
