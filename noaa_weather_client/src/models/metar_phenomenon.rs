use serde::{Deserialize, Serialize};

/// One decoded METAR present-weather phenomenon.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MetarPhenomenon {
    /// Light or heavy intensity; absence means moderate.
    pub intensity: Option<Intensity>,
    /// Phenomenon modifier such as showers or freezing.
    pub modifier: Option<Modifier>,
    /// Basic weather phenomenon.
    pub weather: Weather,
    /// Original METAR token.
    pub raw_string: String,
    /// Whether the phenomenon is nearby rather than at the station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_vicinity: Option<bool>,
}

/// Intensity of the phenomenon
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum Intensity {
    /// Light intensity.
    #[serde(rename = "light")]
    Light,
    /// Heavy intensity.
    #[serde(rename = "heavy")]
    Heavy,
}

/// Modifier of the phenomenon
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum Modifier {
    #[serde(rename = "patches")]
    Patches,
    #[serde(rename = "blowing")]
    Blowing,
    #[serde(rename = "low_drifting")]
    LowDrifting,
    #[serde(rename = "freezing")]
    Freezing,
    #[serde(rename = "shallow")]
    Shallow,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "showers")]
    Showers,
}

/// Weather of the phenomenon
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum Weather {
    #[serde(rename = "fog_mist")]
    FogMist,
    #[serde(rename = "dust_storm")]
    DustStorm,
    #[serde(rename = "dust")]
    Dust,
    #[serde(rename = "drizzle")]
    Drizzle,
    #[serde(rename = "funnel_cloud")]
    FunnelCloud,
    #[serde(rename = "fog")]
    Fog,
    #[serde(rename = "smoke")]
    Smoke,
    #[serde(rename = "hail")]
    Hail,
    #[serde(rename = "snow_pellets")]
    SnowPellets,
    #[serde(rename = "haze")]
    Haze,
    #[serde(rename = "ice_crystals")]
    IceCrystals,
    #[serde(rename = "ice_pellets")]
    IcePellets,
    #[serde(rename = "dust_whirls")]
    DustWhirls,
    #[serde(rename = "spray")]
    Spray,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "sand")]
    Sand,
    #[serde(rename = "snow_grains")]
    SnowGrains,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "squalls")]
    Squalls,
    #[serde(rename = "sand_storm")]
    SandStorm,
    #[serde(rename = "thunderstorms")]
    Thunderstorms,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "volcanic_ash")]
    VolcanicAsh,
}
