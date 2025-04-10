use serde::{Deserialize, Serialize};

/// Iso8601Interval : A time interval in ISO 8601 format. This can be one of:      1. Start and end time     2. Start time and duration     3. Duration and end time The string \"NOW\" can also be used in place of a start/end time.
/// A time interval in ISO 8601 format. This can be one of:      1. Start and end time     2. Start time and duration     3. Duration and end time The string \"NOW\" can also be used in place of a start/end time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Iso8601Interval {
    String(String),
}

impl Default for Iso8601Interval {
    fn default() -> Self {
        Self::String(Default::default())
    }
}
