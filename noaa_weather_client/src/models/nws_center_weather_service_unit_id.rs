use serde::{Deserialize, Serialize};

/// NwsCenterWeatherServiceUnitId : Three-letter identifier for a Center Weather Service Unit (CWSU).
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

impl Default for NwsCenterWeatherServiceUnitId {
    fn default() -> NwsCenterWeatherServiceUnitId {
        Self::Zab
    }
}
