use serde::{Deserialize, Serialize};

/// GridpointHazardsValuesInnerValueInner : A value object representing an expected hazard.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointHazardsValuesInnerValueInner {
    /// Hazard code. This value will correspond to a P-VTEC phenomenon code as defined in NWS Directive 10-1703.
    #[serde(rename = "phenomenon")]
    pub phenomenon: String,
    /// Significance code. This value will correspond to a P-VTEC significance code as defined in NWS Directive 10-1703. This will most frequently be \"A\" for a watch or \"Y\" for an advisory.
    #[serde(rename = "significance")]
    pub significance: String,
    /// Event number. If this hazard refers to a national or regional center product (such as a Storm Prediction Center convective watch), this value will be the sequence number of that product.
    #[serde(rename = "event_number", deserialize_with = "Option::deserialize")]
    pub event_number: Option<i32>,
}

impl GridpointHazardsValuesInnerValueInner {
    /// A value object representing an expected hazard.
    pub fn new(
        phenomenon: String,
        significance: String,
        event_number: Option<i32>,
    ) -> GridpointHazardsValuesInnerValueInner {
        GridpointHazardsValuesInnerValueInner {
            phenomenon,
            significance,
            event_number,
        }
    }
}
