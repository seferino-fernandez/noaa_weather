use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Three-letter identifier for a Center Weather Service Unit (CWSU).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NwsCenterWeatherServiceUnitId {
    #[serde(rename = "ZAB")]
    Zab,
    #[serde(rename = "ZAN")]
    Zan,
    #[serde(rename = "ZAU")]
    Zau,
    #[serde(rename = "ZBW")]
    Zbw,
    #[serde(rename = "ZDC")]
    Zdc,
    #[serde(rename = "ZDV")]
    Zdv,
    #[serde(rename = "ZFA")]
    Zfa,
    #[serde(rename = "ZFW")]
    Zfw,
    #[serde(rename = "ZHU")]
    Zhu,
    #[serde(rename = "ZID")]
    Zid,
    #[serde(rename = "ZJX")]
    Zjx,
    #[serde(rename = "ZKC")]
    Zkc,
    #[serde(rename = "ZLA")]
    Zla,
    #[serde(rename = "ZLC")]
    Zlc,
    #[serde(rename = "ZMA")]
    Zma,
    #[serde(rename = "ZME")]
    Zme,
    #[serde(rename = "ZMP")]
    Zmp,
    #[serde(rename = "ZNY")]
    Zny,
    #[serde(rename = "ZOA")]
    Zoa,
    #[serde(rename = "ZOB")]
    Zob,
    #[serde(rename = "ZSE")]
    Zse,
    #[serde(rename = "ZTL")]
    Ztl,
}

impl std::fmt::Display for NwsCenterWeatherServiceUnitId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Zab => write!(f, "ZAB"),
            Self::Zan => write!(f, "ZAN"),
            Self::Zau => write!(f, "ZAU"),
            Self::Zbw => write!(f, "ZBW"),
            Self::Zdc => write!(f, "ZDC"),
            Self::Zdv => write!(f, "ZDV"),
            Self::Zfa => write!(f, "ZFA"),
            Self::Zfw => write!(f, "ZFW"),
            Self::Zhu => write!(f, "ZHU"),
            Self::Zid => write!(f, "ZID"),
            Self::Zjx => write!(f, "ZJX"),
            Self::Zkc => write!(f, "ZKC"),
            Self::Zla => write!(f, "ZLA"),
            Self::Zlc => write!(f, "ZLC"),
            Self::Zma => write!(f, "ZMA"),
            Self::Zme => write!(f, "ZME"),
            Self::Zmp => write!(f, "ZMP"),
            Self::Zny => write!(f, "ZNY"),
            Self::Zoa => write!(f, "ZOA"),
            Self::Zob => write!(f, "ZOB"),
            Self::Zse => write!(f, "ZSE"),
            Self::Ztl => write!(f, "ZTL"),
        }
    }
}

impl FromStr for NwsCenterWeatherServiceUnitId {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lower_string = string.to_lowercase();
        match lower_string.as_str() {
            "zab" => Ok(Self::Zab),
            "zan" => Ok(Self::Zan),
            "zau" => Ok(Self::Zau),
            "zbw" => Ok(Self::Zbw),
            "zdc" => Ok(Self::Zdc),
            "zdv" => Ok(Self::Zdv),
            "zfa" => Ok(Self::Zfa),
            "zfw" => Ok(Self::Zfw),
            "zhu" => Ok(Self::Zhu),
            "zid" => Ok(Self::Zid),
            "zjx" => Ok(Self::Zjx),
            "zkc" => Ok(Self::Zkc),
            "zla" => Ok(Self::Zla),
            "zlc" => Ok(Self::Zlc),
            "zma" => Ok(Self::Zma),
            "zme" => Ok(Self::Zme),
            "zmp" => Ok(Self::Zmp),
            "zny" => Ok(Self::Zny),
            "zoa" => Ok(Self::Zoa),
            "zob" => Ok(Self::Zob),
            "zse" => Ok(Self::Zse),
            "ztl" => Ok(Self::Ztl),
            _ => Err(format!(
                "Invalid NWS Center Weather Service Unit ID: {string}"
            )),
        }
    }
}

impl Default for NwsCenterWeatherServiceUnitId {
    fn default() -> NwsCenterWeatherServiceUnitId {
        Self::Zab
    }
}
