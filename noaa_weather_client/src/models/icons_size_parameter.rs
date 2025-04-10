use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconsSizeParameter {}

impl IconsSizeParameter {
    pub fn new() -> IconsSizeParameter {
        IconsSizeParameter {}
    }
}
