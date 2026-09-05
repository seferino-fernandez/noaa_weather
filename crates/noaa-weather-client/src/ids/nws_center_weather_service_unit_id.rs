use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Identifies a Center Weather Service Unit accepted by NOAA aviation endpoints.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum NwsCenterWeatherServiceUnitId {
    /// The `ZAB` identifier for the Albuquerque CWSU in Albuquerque, New Mexico.
    #[serde(rename = "ZAB")]
    #[default]
    Zab,
    /// The `ZAN` identifier for the Anchorage CWSU in Anchorage, Alaska.
    #[serde(rename = "ZAN")]
    Zan,
    /// The `ZAU` identifier for the Chicago CWSU in Aurora, Illinois.
    #[serde(rename = "ZAU")]
    Zau,
    /// The `ZBW` identifier for the Boston CWSU.
    #[serde(rename = "ZBW")]
    Zbw,
    /// The `ZDC` identifier for the Washington CWSU in Leesburg, Virginia.
    #[serde(rename = "ZDC")]
    Zdc,
    /// The `ZDV` identifier for the Denver CWSU in Longmont, Colorado.
    #[serde(rename = "ZDV")]
    Zdv,
    /// The `ZFA` identifier for the FAA Academy in Oklahoma City, Oklahoma.
    #[serde(rename = "ZFA")]
    Zfa,
    /// The `ZFW` identifier for the Fort Worth CWSU in Fort Worth, Texas.
    #[serde(rename = "ZFW")]
    Zfw,
    /// The `ZHU` identifier for the Houston CWSU in Houston, Texas.
    #[serde(rename = "ZHU")]
    Zhu,
    /// The `ZID` identifier for the Indianapolis CWSU in Indianapolis, Indiana.
    #[serde(rename = "ZID")]
    Zid,
    /// The `ZJX` identifier for the Jacksonville CWSU in Hilliard, Florida.
    #[serde(rename = "ZJX")]
    Zjx,
    /// The `ZKC` identifier for the Kansas City CWSU in Olathe, Kansas.
    #[serde(rename = "ZKC")]
    Zkc,
    /// The `ZLA` identifier for the Los Angeles CWSU in Palmdale, California.
    #[serde(rename = "ZLA")]
    Zla,
    /// The `ZLC` identifier for the Salt Lake City CWSU in Salt Lake City, Utah.
    #[serde(rename = "ZLC")]
    Zlc,
    /// The `ZMA` identifier for the Miami CWSU in Miami, Florida.
    #[serde(rename = "ZMA")]
    Zma,
    /// The `ZME` identifier for the Memphis CWSU in Memphis, Tennessee.
    #[serde(rename = "ZME")]
    Zme,
    /// The `ZMP` identifier for the Minneapolis CWSU in Farmington, Minnesota.
    #[serde(rename = "ZMP")]
    Zmp,
    /// The `ZNY` identifier for the New York CWSU.
    #[serde(rename = "ZNY")]
    Zny,
    /// The `ZOA` identifier for the Oakland CWSU in Fremont, California.
    #[serde(rename = "ZOA")]
    Zoa,
    /// The `ZOB` identifier for the Cleveland CWSU in Oberlin, Ohio.
    #[serde(rename = "ZOB")]
    Zob,
    /// The `ZSE` identifier for the Seattle CWSU in Auburn, Washington.
    #[serde(rename = "ZSE")]
    Zse,
    /// The `ZTL` identifier for the Atlanta CWSU in Hampton, Georgia.
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
