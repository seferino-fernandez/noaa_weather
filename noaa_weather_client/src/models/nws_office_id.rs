use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NwsOfficeId {
    NwsForecastOfficeId(models::NwsForecastOfficeId),
    NwsRegionalHqid(models::NwsRegionalHqid),
    NwsNationalHqid(models::NwsNationalHqid),
}
