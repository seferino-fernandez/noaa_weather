use crate::models;
use serde::{Deserialize, Serialize};

/// GridpointQuantitativeValueLayer : A gridpoint layer consisting of quantitative values (numeric values with associated units of measure).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointQuantitativeValueLayer {
    /// A string denoting a unit of measure, expressed in the format \"{unit}\" or \"{namespace}:{unit}\". Units with the namespace \"wmo\" or \"wmoUnit\" are defined in the World Meteorological Organization Codes Registry at http://codes.wmo.int/common/unit and should be canonically resolvable to http://codes.wmo.int/common/unit/{unit}. Units with the namespace \"nwsUnit\" are currently custom and do not align to any standard. Units with no namespace or the namespace \"uc\" are compliant with the Unified Code for Units of Measure syntax defined at https://unitsofmeasure.org/. This also aligns with recent versions of the Geographic Markup Language (GML) standard, the IWXXM standard, and OGC Observations and Measurements v2.0 (ISO/DIS 19156). Namespaced units are considered deprecated. We will be aligning API to use the same standards as GML/IWXXM in the future.
    #[serde(rename = "uom", skip_serializing_if = "Option::is_none")]
    pub uom: Option<String>,
    #[serde(rename = "values")]
    pub values: Vec<models::GridpointQuantitativeValueLayerValuesInner>,
}

impl GridpointQuantitativeValueLayer {
    /// A gridpoint layer consisting of quantitative values (numeric values with associated units of measure).
    pub fn new(
        values: Vec<models::GridpointQuantitativeValueLayerValuesInner>,
    ) -> GridpointQuantitativeValueLayer {
        GridpointQuantitativeValueLayer { uom: None, values }
    }
}
