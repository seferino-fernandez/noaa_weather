use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneState {
    StateTerritoryCode(models::StateTerritoryCode),
    String(String),
}
