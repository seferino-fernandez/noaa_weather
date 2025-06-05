use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationCloudLayersInner {
    #[serde(rename = "base")]
    pub base: Box<models::ValueUnit>,
    #[serde(rename = "amount")]
    pub amount: models::MetarSkyCoverage,
}

impl ObservationCloudLayersInner {
    pub fn new(
        base: models::ValueUnit,
        amount: models::MetarSkyCoverage,
    ) -> ObservationCloudLayersInner {
        ObservationCloudLayersInner {
            base: Box::new(base),
            amount,
        }
    }
}
