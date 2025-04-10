use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObservationCloudLayersInner {
    #[serde(rename = "base")]
    pub base: Box<models::QuantitativeValue>,
    #[serde(rename = "amount")]
    pub amount: models::MetarSkyCoverage,
}

impl ObservationCloudLayersInner {
    pub fn new(
        base: models::QuantitativeValue,
        amount: models::MetarSkyCoverage,
    ) -> ObservationCloudLayersInner {
        ObservationCloudLayersInner {
            base: Box::new(base),
            amount,
        }
    }
}
