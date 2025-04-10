use serde::{Deserialize, Serialize};

/// MetarPhenomenon : An object representing a decoded METAR phenomenon string.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetarPhenomenon {
    #[serde(rename = "intensity", deserialize_with = "Option::deserialize")]
    pub intensity: Option<Intensity>,
    #[serde(rename = "modifier", deserialize_with = "Option::deserialize")]
    pub modifier: Option<Modifier>,
    #[serde(rename = "weather")]
    pub weather: Weather,
    #[serde(rename = "rawString")]
    pub raw_string: String,
    #[serde(rename = "inVicinity", skip_serializing_if = "Option::is_none")]
    pub in_vicinity: Option<bool>,
}

impl MetarPhenomenon {
    /// An object representing a decoded METAR phenomenon string.
    pub fn new(
        intensity: Option<Intensity>,
        modifier: Option<Modifier>,
        weather: Weather,
        raw_string: String,
    ) -> MetarPhenomenon {
        MetarPhenomenon {
            intensity,
            modifier,
            weather,
            raw_string,
            in_vicinity: None,
        }
    }
}

/// Intensity of the phenomenon
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Intensity {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "heavy")]
    Heavy,
}

impl Default for Intensity {
    fn default() -> Intensity {
        Self::Light
    }
}

/// Modifier of the phenomenon
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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

impl Default for Modifier {
    fn default() -> Modifier {
        Self::Patches
    }
}

/// Weather of the phenomenon
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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

impl Default for Weather {
    fn default() -> Weather {
        Self::FogMist
    }
}
