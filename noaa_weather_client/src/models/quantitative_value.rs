use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::QualityControl;

/// QuantitativeValue : A structured value representing a measurement and its unit of measure. This object is a slighly modified version of the schema.org definition at https://schema.org/QuantitativeValue
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuantitativeValue {
    /// A measured value
    #[serde(
        rename = "value",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Option<f64>>,
    /// The maximum value of a range of measured values
    #[serde(rename = "maxValue", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// The minimum value of a range of measured values
    #[serde(rename = "minValue", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// A string denoting a unit of measure, expressed in the format \"{unit}\" or \"{namespace}:{unit}\". Units with the namespace \"wmo\" or \"wmoUnit\" are defined in the World Meteorological Organization Codes Registry at http://codes.wmo.int/common/unit and should be canonically resolvable to http://codes.wmo.int/common/unit/{unit}. Units with the namespace \"nwsUnit\" are currently custom and do not align to any standard. Units with no namespace or the namespace \"uc\" are compliant with the Unified Code for Units of Measure syntax defined at https://unitsofmeasure.org/. This also aligns with recent versions of the Geographic Markup Language (GML) standard, the IWXXM standard, and OGC Observations and Measurements v2.0 (ISO/DIS 19156). Namespaced units are considered deprecated. We will be aligning API to use the same standards as GML/IWXXM in the future.
    #[serde(rename = "unitCode", skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<String>,
    /// For values in observation records, the quality control flag from the MADIS system.
    #[serde(rename = "qualityControl", skip_serializing_if = "Option::is_none")]
    pub quality_control: Option<QualityControl>,
}

impl QuantitativeValue {
    /// A structured value representing a measurement and its unit of measure. This object is a slighly modified version of the schema.org definition at https://schema.org/QuantitativeValue
    pub fn new() -> QuantitativeValue {
        QuantitativeValue {
            value: None,
            max_value: None,
            min_value: None,
            unit_code: None,
            quality_control: None,
        }
    }
}

impl Display for QuantitativeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?}",
            self.value.unwrap_or_default(),
            self.unit_code.clone().unwrap_or("".to_string())
        )
    }
}
