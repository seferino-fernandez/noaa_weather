use serde::{Deserialize, Serialize};

/// PaginationInfo : Links for retrieving more data from paged data sets
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginationInfo {
    /// A link to the next page of records
    #[serde(rename = "next")]
    pub next: String,
}

impl PaginationInfo {
    /// Links for retrieving more data from paged data sets
    pub fn new(next: String) -> PaginationInfo {
        PaginationInfo { next }
    }
}
