use crate::models;
use serde::{Deserialize, Serialize};

/// GridpointWeatherValuesInnerValueInner : A value object representing expected weather phenomena.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GridpointWeatherValuesInnerValueInner {
    #[serde(rename = "coverage", deserialize_with = "Option::deserialize")]
    pub coverage: Option<Coverage>,
    #[serde(rename = "weather", deserialize_with = "Option::deserialize")]
    pub weather: Option<Weather>,
    #[serde(rename = "intensity", deserialize_with = "Option::deserialize")]
    pub intensity: Option<Intensity>,
    #[serde(rename = "visibility")]
    pub visibility: Box<models::QuantitativeValue>,
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attributes>,
}

impl GridpointWeatherValuesInnerValueInner {
    /// A value object representing expected weather phenomena.
    pub fn new(
        coverage: Option<Coverage>,
        weather: Option<Weather>,
        intensity: Option<Intensity>,
        visibility: models::QuantitativeValue,
        attributes: Vec<Attributes>,
    ) -> GridpointWeatherValuesInnerValueInner {
        GridpointWeatherValuesInnerValueInner {
            coverage,
            weather,
            intensity,
            visibility: Box::new(visibility),
            attributes,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Coverage {
    #[serde(rename = "areas")]
    Areas,
    #[serde(rename = "brief")]
    Brief,
    #[serde(rename = "chance")]
    Chance,
    #[serde(rename = "definite")]
    Definite,
    #[serde(rename = "few")]
    Few,
    #[serde(rename = "frequent")]
    Frequent,
    #[serde(rename = "intermittent")]
    Intermittent,
    #[serde(rename = "isolated")]
    Isolated,
    #[serde(rename = "likely")]
    Likely,
    #[serde(rename = "numerous")]
    Numerous,
    #[serde(rename = "occasional")]
    Occasional,
    #[serde(rename = "patchy")]
    Patchy,
    #[serde(rename = "periods")]
    Periods,
    #[serde(rename = "scattered")]
    Scattered,
    #[serde(rename = "slight_chance")]
    SlightChance,
    #[serde(rename = "widespread")]
    Widespread,
}

impl Default for Coverage {
    fn default() -> Coverage {
        Self::Areas
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Weather {
    #[serde(rename = "blowing_dust")]
    BlowingDust,
    #[serde(rename = "blowing_sand")]
    BlowingSand,
    #[serde(rename = "blowing_snow")]
    BlowingSnow,
    #[serde(rename = "drizzle")]
    Drizzle,
    #[serde(rename = "fog")]
    Fog,
    #[serde(rename = "freezing_fog")]
    FreezingFog,
    #[serde(rename = "freezing_drizzle")]
    FreezingDrizzle,
    #[serde(rename = "freezing_rain")]
    FreezingRain,
    #[serde(rename = "freezing_spray")]
    FreezingSpray,
    #[serde(rename = "frost")]
    Frost,
    #[serde(rename = "hail")]
    Hail,
    #[serde(rename = "haze")]
    Haze,
    #[serde(rename = "ice_crystals")]
    IceCrystals,
    #[serde(rename = "ice_fog")]
    IceFog,
    #[serde(rename = "rain")]
    Rain,
    #[serde(rename = "rain_showers")]
    RainShowers,
    #[serde(rename = "sleet")]
    Sleet,
    #[serde(rename = "smoke")]
    Smoke,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "snow_showers")]
    SnowShowers,
    #[serde(rename = "thunderstorms")]
    Thunderstorms,
    #[serde(rename = "volcanic_ash")]
    VolcanicAsh,
    #[serde(rename = "water_spouts")]
    WaterSpouts,
}

impl Default for Weather {
    fn default() -> Weather {
        Self::BlowingDust
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Intensity {
    #[serde(rename = "very_light")]
    VeryLight,
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "heavy")]
    Heavy,
}

impl Default for Intensity {
    fn default() -> Intensity {
        Self::VeryLight
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Attributes {
    #[serde(rename = "damaging_wind")]
    DamagingWind,
    #[serde(rename = "dry_thunderstorms")]
    DryThunderstorms,
    #[serde(rename = "flooding")]
    Flooding,
    #[serde(rename = "gusty_wind")]
    GustyWind,
    #[serde(rename = "heavy_rain")]
    HeavyRain,
    #[serde(rename = "large_hail")]
    LargeHail,
    #[serde(rename = "small_hail")]
    SmallHail,
    #[serde(rename = "tornadoes")]
    Tornadoes,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Self::DamagingWind
    }
}
