use serde::{Deserialize, Serialize};

use super::{NwsUnitCode, WmoUnitCode};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnitCodeType {
    /// Represents a World Meteorological Organization (WMO) unit code.
    Wmo(WmoUnitCode),
    /// Represents a National Weather Service (NWS) unit code.
    Nws(NwsUnitCode),
}

/// Represents a value with an associated unit code.
/// This is used for fields like elevation, latency, power, etc.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueUnit {
    /// The unit code, which can be either a WMO unit code or an NWS unit code.
    /// Examples: "wmoUnit:m", "nwsUnit:s".
    #[serde(rename = "unitCode", skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<UnitCodeType>,
    /// The numerical value. Using f64 to accommodate both integers and floating-point numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "qualityControl", skip_serializing_if = "Option::is_none")]
    pub quality_control: Option<String>,
}
