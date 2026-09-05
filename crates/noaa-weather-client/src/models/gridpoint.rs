//! Models for the `/gridpoints` family: the raw layers of a [`Gridpoint`]
//! and the textual [`Forecast`] generated from them.
//!
//! A gridpoint is one 2.5 km cell of a forecast office's grid.
//! `/gridpoints/{office}/{x},{y}` returns [`Gridpoint`]: an envelope plus 59
//! forecast layers, each a series of values over time intervals.
//! `/gridpoints/{office}/{x},{y}/forecast` and `.../forecast/hourly` return
//! [`Forecast`], the same envelope of metadata wrapped around a list of
//! [`ForecastPeriod`]s written for people to read.
//!
//! # One forecast type, two endpoints
//!
//! The twelve-hour and hourly endpoints return structurally identical
//! responses. They differ in [`Forecast::forecast_generator`], in how long a
//! period lasts, and in two fields NOAA sends only on hourly periods
//! ([`ForecastPeriod::dewpoint`] and
//! [`ForecastPeriod::relative_humidity`]). That is data, not shape, so both
//! [`Gridpoints::forecast`](crate::apis::gridpoints::Gridpoints::forecast)
//! and
//! [`Gridpoints::forecast_hourly`](crate::apis::gridpoints::Gridpoints::forecast_hourly)
//! return `Feature<Forecast>`.
//!
//! # Requiredness
//!
//! A live probe of 63 grids across 62 forecast offices (CONUS, Alaska,
//! Hawaii, Puerto Rico, and Guam) found all nine envelope keys and all 59
//! layers present on every grid — marine layers included on inland grids —
//! so every layer is a plain field with a possibly-empty
//! [`GridpointLayer::values`]. Ten `potentialOf…` layers were empty on all
//! 63; several others were empty on most. A caller must expect no values,
//! never a missing layer.
//!
//! [`GridpointLayer::unit`] is the one genuinely optional part: NOAA omits
//! `uom` whenever `values` is empty and on every dimensionless index layer
//! even when populated.
//!
//! Across 4,760 forecast periods from both endpoints, every period key was
//! present. [`ForecastPeriod::name`],
//! [`ForecastPeriod::detailed_forecast`], and
//! [`ForecastPeriod::wind_direction`] are `Option` because NOAA writes an
//! empty string rather than omitting them — hourly periods have no name or
//! narrative at all. They serialize back as `""`, so a re-serialized
//! forecast reproduces NOAA's key set.
//!
//! # Unmodeled layers
//!
//! [`Gridpoint::other`] collects any layer NOAA adds beyond the 59 named
//! here, typed as a [`GridpointLayer`] rather than raw JSON, so a new layer
//! is usable without a release.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_with::{NoneAsEmptyString, serde_as};

use super::{NwsForecastOfficeId, Quantity, Unit};
use crate::time::{Interval, OffsetDateTime};

/// Raw forecast layers for one 2.5 km grid cell.
///
/// Every layer is always there, so reading one is a field access rather
/// than a lookup — but a layer this office does not publish has no values.
///
/// ```no_run
/// use noaa_weather_client::{Client, GridpointId};
///
/// # async fn run() -> Result<(), noaa_weather_client::Error> {
/// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
/// let grid: GridpointId = "TOP/31,80".parse()?;
/// let gridpoint = client.gridpoints().get(&grid).await?;
///
/// let temperature = &gridpoint.properties.temperature;
/// println!("{}", temperature.unit.as_ref().map_or("index", |unit| unit.code()));
/// for reading in temperature.values.iter().take(3) {
///     println!("{}: {:?}", reading.valid_time, reading.value);
/// }
///
/// // A marine layer inland, or a layer this office does not publish, is
/// // empty rather than absent.
/// assert!(gridpoint.properties.potential_of_15mph_winds.values.is_empty());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Gridpoint {
    /// The canonical API URL for this grid cell.
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// The JSON-LD type assigned to this grid cell (`wx:Gridpoint`).
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// When the office last updated the data these layers come from.
    pub update_time: OffsetDateTime,
    /// The period the whole grid covers.
    pub valid_times: Interval,
    /// The elevation of the grid cell.
    pub elevation: Quantity,
    /// The API URL of the office that issues this grid.
    pub forecast_office: String,
    /// The office that issues this grid.
    pub grid_id: NwsForecastOfficeId,
    /// The column of the grid.
    pub grid_x: u32,
    /// The row of the grid.
    pub grid_y: u32,
    /// Surface air temperature.
    pub temperature: GridpointLayer,
    /// Dewpoint temperature.
    pub dewpoint: GridpointLayer,
    /// Daily maximum temperature.
    pub max_temperature: GridpointLayer,
    /// Daily minimum temperature.
    pub min_temperature: GridpointLayer,
    /// Relative humidity.
    pub relative_humidity: GridpointLayer,
    /// Apparent ("feels like") temperature.
    pub apparent_temperature: GridpointLayer,
    /// Wet bulb globe temperature, a heat stress index.
    pub wet_bulb_globe_temperature: GridpointLayer,
    /// Heat index.
    pub heat_index: GridpointLayer,
    /// Wind chill.
    pub wind_chill: GridpointLayer,
    /// Fraction of the sky covered by cloud.
    pub sky_cover: GridpointLayer,
    /// Direction the wind blows from.
    pub wind_direction: GridpointLayer,
    /// Sustained wind speed.
    pub wind_speed: GridpointLayer,
    /// Peak wind gust.
    pub wind_gust: GridpointLayer,
    /// Expected weather phenomena.
    pub weather: WeatherLayer,
    /// Watch and advisory products in effect.
    pub hazards: HazardsLayer,
    /// The NWS HeatRisk index, from 0 (little risk) to 4 (extreme).
    pub heat_risk: GridpointLayer,
    /// Probability of measurable precipitation.
    pub probability_of_precipitation: GridpointLayer,
    /// Liquid precipitation amount.
    pub quantitative_precipitation: GridpointLayer,
    /// Ice accumulation.
    pub ice_accumulation: GridpointLayer,
    /// Snowfall amount.
    pub snowfall_amount: GridpointLayer,
    /// Elevation of the snow level.
    pub snow_level: GridpointLayer,
    /// Height of the cloud ceiling.
    pub ceiling_height: GridpointLayer,
    /// Horizontal visibility.
    pub visibility: GridpointLayer,
    /// Wind speed in the mixed layer, used for smoke dispersion.
    pub transport_wind_speed: GridpointLayer,
    /// Wind direction in the mixed layer.
    pub transport_wind_direction: GridpointLayer,
    /// Height of the mixing layer.
    pub mixing_height: GridpointLayer,
    /// Haines index, a fire weather stability index.
    pub haines_index: GridpointLayer,
    /// Lightning activity level.
    pub lightning_activity_level: GridpointLayer,
    /// Wind speed twenty feet above the ground, used for fire weather.
    pub twenty_foot_wind_speed: GridpointLayer,
    /// Wind direction twenty feet above the ground.
    pub twenty_foot_wind_direction: GridpointLayer,
    /// Significant wave height.
    pub wave_height: GridpointLayer,
    /// Dominant wave period.
    pub wave_period: GridpointLayer,
    /// Dominant wave direction.
    pub wave_direction: GridpointLayer,
    /// Primary swell height.
    pub primary_swell_height: GridpointLayer,
    /// Primary swell direction.
    pub primary_swell_direction: GridpointLayer,
    /// Secondary swell height.
    pub secondary_swell_height: GridpointLayer,
    /// Secondary swell direction.
    pub secondary_swell_direction: GridpointLayer,
    /// Secondary wave period.
    #[serde(rename = "wavePeriod2")]
    pub wave_period_2: GridpointLayer,
    /// Wind wave height.
    pub wind_wave_height: GridpointLayer,
    /// Atmospheric dispersion index for smoke management.
    pub dispersion_index: GridpointLayer,
    /// Barometric pressure.
    pub pressure: GridpointLayer,
    /// Probability of tropical storm force winds.
    pub probability_of_tropical_storm_winds: GridpointLayer,
    /// Probability of hurricane force winds.
    pub probability_of_hurricane_winds: GridpointLayer,
    /// Probability of sustained winds reaching 15 mph.
    #[serde(rename = "potentialOf15mphWinds")]
    pub potential_of_15mph_winds: GridpointLayer,
    /// Probability of sustained winds reaching 25 mph.
    #[serde(rename = "potentialOf25mphWinds")]
    pub potential_of_25mph_winds: GridpointLayer,
    /// Probability of sustained winds reaching 35 mph.
    #[serde(rename = "potentialOf35mphWinds")]
    pub potential_of_35mph_winds: GridpointLayer,
    /// Probability of sustained winds reaching 45 mph.
    #[serde(rename = "potentialOf45mphWinds")]
    pub potential_of_45mph_winds: GridpointLayer,
    /// Probability of wind gusts reaching 20 mph.
    #[serde(rename = "potentialOf20mphWindGusts")]
    pub potential_of_20mph_wind_gusts: GridpointLayer,
    /// Probability of wind gusts reaching 30 mph.
    #[serde(rename = "potentialOf30mphWindGusts")]
    pub potential_of_30mph_wind_gusts: GridpointLayer,
    /// Probability of wind gusts reaching 40 mph.
    #[serde(rename = "potentialOf40mphWindGusts")]
    pub potential_of_40mph_wind_gusts: GridpointLayer,
    /// Probability of wind gusts reaching 50 mph.
    #[serde(rename = "potentialOf50mphWindGusts")]
    pub potential_of_50mph_wind_gusts: GridpointLayer,
    /// Probability of wind gusts reaching 60 mph.
    #[serde(rename = "potentialOf60mphWindGusts")]
    pub potential_of_60mph_wind_gusts: GridpointLayer,
    /// Grassland fire danger index.
    pub grassland_fire_danger_index: GridpointLayer,
    /// Probability of thunder.
    pub probability_of_thunder: GridpointLayer,
    /// Davis stability index.
    pub davis_stability_index: GridpointLayer,
    /// Atmospheric dispersion index.
    pub atmospheric_dispersion_index: GridpointLayer,
    /// Low visibility occurrence risk index.
    pub low_visibility_occurrence_risk_index: GridpointLayer,
    /// Atmospheric stability class.
    pub stability: GridpointLayer,
    /// Red flag threat index for fire weather.
    pub red_flag_threat_index: GridpointLayer,
    /// Layers NOAA returned that this crate does not name, keyed by their
    /// NOAA name.
    #[serde(flatten)]
    pub other: BTreeMap<String, GridpointLayer>,
}

/// One numerical forecast layer: a series of values over time.
///
/// ```
/// use noaa_weather_client::models::GridpointLayer;
///
/// let temperature: GridpointLayer = serde_json::from_str(r#"{
///   "uom": "wmoUnit:degC",
///   "values": [{"validTime": "2026-09-02T00:00:00+00:00/PT1H", "value": 33.3}]
/// }"#)?;
/// assert_eq!(temperature.unit.unwrap().code(), "wmoUnit:degC");
///
/// // A dimensionless index has values but no unit.
/// let heat_risk: GridpointLayer = serde_json::from_str(r#"{
///   "values": [{"validTime": "2026-09-02T00:00:00+00:00/P3DT6H", "value": 4}]
/// }"#)?;
/// assert_eq!(heat_risk.unit, None);
/// assert_eq!(heat_risk.values[0].value, Some(4.0));
/// assert_eq!(
///     heat_risk.values[0].valid_time.to_string(),
///     "2026-09-02T00:00:00+00:00/P3DT6H"
/// );
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct GridpointLayer {
    /// The unit every value in the layer is expressed in. `None` on a
    /// dimensionless index and on an empty layer, where NOAA omits `uom`.
    #[serde(rename = "uom", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// The values, in time order. Empty when the office publishes nothing
    /// for this layer.
    #[serde(default)]
    pub values: Vec<LayerValue>,
}

/// One value of a numerical layer and the interval it covers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LayerValue {
    /// The interval this value covers.
    pub valid_time: Interval,
    /// The value, in the layer's unit.
    #[serde(default)]
    pub value: Option<f64>,
}

/// The `weather` layer: expected phenomena over time.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct WeatherLayer {
    /// The phenomena, in time order.
    #[serde(default)]
    pub values: Vec<WeatherPeriod>,
}

/// The phenomena expected over one interval.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WeatherPeriod {
    /// The interval these phenomena cover.
    pub valid_time: Interval,
    /// The phenomena expected. NOAA sends one entry with every field null
    /// to mean "nothing expected".
    #[serde(default)]
    pub value: Vec<WeatherCondition>,
}

/// One expected weather phenomenon.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct WeatherCondition {
    /// How much of the area or period the phenomenon covers.
    #[serde(default)]
    pub coverage: Option<WeatherCoverage>,
    /// The phenomenon itself.
    #[serde(default)]
    pub weather: Option<WeatherPhenomenon>,
    /// How intense the phenomenon is.
    #[serde(default)]
    pub intensity: Option<WeatherIntensity>,
    /// Visibility during the phenomenon.
    pub visibility: Quantity,
    /// Additional hazards accompanying the phenomenon.
    #[serde(default)]
    pub attributes: Vec<WeatherAttribute>,
}

/// The `hazards` layer: watch and advisory products in effect.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct HazardsLayer {
    /// The hazards, in time order.
    #[serde(default)]
    pub values: Vec<HazardPeriod>,
}

/// The hazards in effect over one interval.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct HazardPeriod {
    /// The interval these hazards cover.
    pub valid_time: Interval,
    /// The hazards in effect.
    #[serde(default)]
    pub value: Vec<Hazard>,
}

/// One hazard in effect.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct Hazard {
    /// The P-VTEC phenomenon code from NWS Directive 10-1703, such as `HT`
    /// for heat.
    pub phenomenon: String,
    /// The P-VTEC significance code, most often `A` for a watch or `Y` for
    /// an advisory.
    pub significance: String,
    /// The sequence number of the national or regional center product this
    /// hazard refers to, when it refers to one.
    #[serde(rename = "event_number", default)]
    pub event_number: Option<u32>,
}

/// A textual forecast for one grid cell, from either forecast endpoint.
///
/// ```
/// use noaa_weather_client::models::{Forecast, ForecastGenerator, ForecastUnits};
///
/// let forecast: Forecast = serde_json::from_str(r#"{
///   "units": "us",
///   "forecastGenerator": "BaselineForecastGenerator",
///   "generatedAt": "2026-09-02T07:50:51+00:00",
///   "updateTime": "2026-09-02T06:26:13+00:00",
///   "validTimes": "2026-09-02T00:00:00+00:00/P8DT1H",
///   "elevation": {"unitCode": "wmoUnit:m", "value": 456.8952},
///   "periods": [{
///     "number": 1, "name": "Overnight",
///     "startTime": "2026-09-02T02:00:00-05:00",
///     "endTime": "2026-09-02T06:00:00-05:00",
///     "isDaytime": false,
///     "temperature": {"unitCode": "wmoUnit:degC", "value": 23.88888888888889},
///     "temperatureTrend": null,
///     "probabilityOfPrecipitation": {"unitCode": "wmoUnit:percent", "value": 3},
///     "windSpeed": {"unitCode": "wmoUnit:km_h-1", "value": 16.09344},
///     "windGust": null,
///     "windDirection": "S",
///     "icon": "https://api.weather.gov/icons/land/night/sct?size=medium",
///     "shortForecast": "Partly Cloudy",
///     "detailedForecast": "Partly cloudy, with a low around 75."
///   }]
/// }"#).unwrap();
///
/// assert_eq!(forecast.units, ForecastUnits::Us);
/// assert_eq!(forecast.forecast_generator, ForecastGenerator::Baseline);
/// let period = &forecast.periods[0];
/// assert_eq!(period.name.as_deref(), Some("Overnight"));
/// assert_eq!(period.start_time.to_string(), "2026-09-02T02:00:00-05:00");
/// assert_eq!(period.wind_gust, None);
/// assert_eq!(period.dewpoint, None);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Forecast {
    /// The units NOAA rendered the narrative text in. With the
    /// quantitative-value feature flags this crate always sends, the
    /// measured fields are metric either way and only this echo changes.
    pub units: ForecastUnits,
    /// Which generator produced the forecast, which is also what
    /// distinguishes the twelve-hour endpoint from the hourly one.
    pub forecast_generator: ForecastGenerator,
    /// When this forecast text was generated.
    pub generated_at: OffsetDateTime,
    /// When the office last updated the grid data behind it.
    pub update_time: OffsetDateTime,
    /// The period the underlying grid covers.
    pub valid_times: Interval,
    /// The elevation of the grid cell.
    pub elevation: Quantity,
    /// The forecast periods, in time order.
    #[serde(default)]
    pub periods: Vec<ForecastPeriod>,
}

/// One period of a textual forecast: twelve hours, or one hour.
#[serde_as]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ForecastPeriod {
    /// The period's position in the forecast, counting from 1.
    pub number: u32,
    /// The period's name, such as `Tonight` or `Thursday`. `None` on hourly
    /// periods, which NOAA names with an empty string.
    #[serde_as(as = "NoneAsEmptyString")]
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    pub name: Option<String>,
    /// When the period begins, in the grid's local time.
    pub start_time: OffsetDateTime,
    /// When the period ends, in the grid's local time.
    pub end_time: OffsetDateTime,
    /// Whether the period is daytime.
    pub is_daytime: bool,
    /// The forecast temperature.
    pub temperature: Quantity,
    /// A non-diurnal temperature trend: rising overnight, or falling during
    /// the day. `None` for the ordinary case.
    #[serde(default)]
    pub temperature_trend: Option<TemperatureTrend>,
    /// The probability of measurable precipitation.
    pub probability_of_precipitation: Quantity,
    /// The dewpoint. Hourly periods only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dewpoint: Option<Quantity>,
    /// The relative humidity. Hourly periods only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative_humidity: Option<Quantity>,
    /// The sustained wind speed, or a range of speeds on twelve-hour
    /// periods, where NOAA sends `minValue` and `maxValue` instead of a
    /// single value.
    pub wind_speed: Quantity,
    /// The peak wind gust, when the office forecasts one.
    #[serde(default)]
    pub wind_gust: Option<Quantity>,
    /// The prevailing wind direction on a 16-point compass. `None` when
    /// there is no prevailing direction, which NOAA writes as an empty
    /// string.
    #[serde_as(as = "NoneAsEmptyString")]
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    pub wind_direction: Option<WindDirection>,
    /// The URL of an icon depicting the period's conditions.
    pub icon: String,
    /// A few words summarizing the period, such as `Partly Cloudy`.
    pub short_forecast: String,
    /// The narrative forecast for the period. `None` on hourly periods,
    /// which NOAA writes as an empty string.
    #[serde_as(as = "NoneAsEmptyString")]
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    pub detailed_forecast: Option<String>,
}

/// The unit system NOAA renders forecast narrative text in.
///
/// `us` is US customary and the NOAA default; `si` is metric. With the
/// quantitative-value feature flags this crate always sends, every measured
/// field comes back in the same units either way — only the narrative text
/// and the echoed [`Forecast::units`] change.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum ForecastUnits {
    /// The United States Customary System.
    #[serde(rename = "us")]
    Us,
    /// The International System of Units.
    #[serde(rename = "si")]
    Si,
}

impl std::fmt::Display for ForecastUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Us => write!(f, "us"),
            Self::Si => write!(f, "si"),
        }
    }
}

impl std::str::FromStr for ForecastUnits {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "us" => Ok(Self::Us),
            "si" => Ok(Self::Si),
            _ => Err(format!("Invalid gridpoint forecast units: {string}")),
        }
    }
}

/// The internal NWS class that produced a forecast.
///
/// This is the field that tells the two forecast endpoints apart:
/// `/forecast` is generated by [`ForecastGenerator::Baseline`] and
/// `/forecast/hourly` by [`ForecastGenerator::Hourly`]. Any other generator
/// NOAA introduces is kept verbatim.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
#[non_exhaustive]
pub enum ForecastGenerator {
    /// `BaselineForecastGenerator`, behind `/forecast`.
    Baseline,
    /// `HourlyForecastGenerator`, behind `/forecast/hourly`.
    Hourly,
    /// A generator this crate does not name.
    Other(Box<str>),
}

impl ForecastGenerator {
    /// Returns the class name NOAA writes on the wire.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Baseline => "BaselineForecastGenerator",
            Self::Hourly => "HourlyForecastGenerator",
            Self::Other(name) => name,
        }
    }
}

impl From<String> for ForecastGenerator {
    fn from(name: String) -> Self {
        match name.as_str() {
            "BaselineForecastGenerator" => Self::Baseline,
            "HourlyForecastGenerator" => Self::Hourly,
            _ => Self::Other(name.into_boxed_str()),
        }
    }
}

impl From<ForecastGenerator> for String {
    fn from(generator: ForecastGenerator) -> Self {
        generator.as_str().to_owned()
    }
}

impl std::fmt::Display for ForecastGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl_string_schema!(
    ForecastGenerator,
    "The internal NWS generator class that produced the forecast, for example \
     BaselineForecastGenerator."
);

/// A non-diurnal temperature trend over a forecast period.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum TemperatureTrend {
    /// Temperature rising overnight.
    #[serde(rename = "rising")]
    Rising,
    /// Temperature falling during the day.
    #[serde(rename = "falling")]
    Falling,
}

impl std::fmt::Display for TemperatureTrend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rising => f.write_str("rising"),
            Self::Falling => f.write_str("falling"),
        }
    }
}

impl std::str::FromStr for TemperatureTrend {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_lowercase().as_str() {
            "rising" => Ok(Self::Rising),
            "falling" => Ok(Self::Falling),
            _ => Err(format!("Invalid temperature trend: {string}")),
        }
    }
}

/// A direction on the 16-point compass, as NOAA labels wind direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum WindDirection {
    /// North.
    #[serde(rename = "N")]
    N,
    /// North-northeast.
    #[serde(rename = "NNE")]
    Nne,
    /// Northeast.
    #[serde(rename = "NE")]
    Ne,
    /// East-northeast.
    #[serde(rename = "ENE")]
    Ene,
    /// East.
    #[serde(rename = "E")]
    E,
    /// East-southeast.
    #[serde(rename = "ESE")]
    Ese,
    /// Southeast.
    #[serde(rename = "SE")]
    Se,
    /// South-southeast.
    #[serde(rename = "SSE")]
    Sse,
    /// South.
    #[serde(rename = "S")]
    S,
    /// South-southwest.
    #[serde(rename = "SSW")]
    Ssw,
    /// Southwest.
    #[serde(rename = "SW")]
    Sw,
    /// West-southwest.
    #[serde(rename = "WSW")]
    Wsw,
    /// West.
    #[serde(rename = "W")]
    W,
    /// West-northwest.
    #[serde(rename = "WNW")]
    Wnw,
    /// Northwest.
    #[serde(rename = "NW")]
    Nw,
    /// North-northwest.
    #[serde(rename = "NNW")]
    Nnw,
}

impl WindDirection {
    /// Returns the compass abbreviation, such as `NNE`.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::N => "N",
            Self::Nne => "NNE",
            Self::Ne => "NE",
            Self::Ene => "ENE",
            Self::E => "E",
            Self::Ese => "ESE",
            Self::Se => "SE",
            Self::Sse => "SSE",
            Self::S => "S",
            Self::Ssw => "SSW",
            Self::Sw => "SW",
            Self::Wsw => "WSW",
            Self::W => "W",
            Self::Wnw => "WNW",
            Self::Nw => "NW",
            Self::Nnw => "NNW",
        }
    }
}

impl std::fmt::Display for WindDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for WindDirection {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_uppercase().as_str() {
            "N" => Ok(Self::N),
            "NNE" => Ok(Self::Nne),
            "NE" => Ok(Self::Ne),
            "ENE" => Ok(Self::Ene),
            "E" => Ok(Self::E),
            "ESE" => Ok(Self::Ese),
            "SE" => Ok(Self::Se),
            "SSE" => Ok(Self::Sse),
            "S" => Ok(Self::S),
            "SSW" => Ok(Self::Ssw),
            "SW" => Ok(Self::Sw),
            "WSW" => Ok(Self::Wsw),
            "W" => Ok(Self::W),
            "WNW" => Ok(Self::Wnw),
            "NW" => Ok(Self::Nw),
            "NNW" => Ok(Self::Nnw),
            _ => Err(format!("Invalid wind direction: {string}")),
        }
    }
}

/// How much of an area or period a weather phenomenon covers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[serde(rename_all = "snake_case")]
pub enum WeatherCoverage {
    /// Areas of the phenomenon.
    Areas,
    /// Brief occurrences.
    Brief,
    /// A chance of the phenomenon.
    Chance,
    /// Definite.
    Definite,
    /// A few occurrences.
    Few,
    /// Frequent occurrences.
    Frequent,
    /// Intermittent occurrences.
    Intermittent,
    /// Isolated occurrences.
    Isolated,
    /// Likely.
    Likely,
    /// Numerous occurrences.
    Numerous,
    /// Occasional occurrences.
    Occasional,
    /// Patchy.
    Patchy,
    /// Periods of the phenomenon.
    Periods,
    /// Scattered occurrences.
    Scattered,
    /// A slight chance.
    SlightChance,
    /// Widespread.
    Widespread,
}

/// A forecast weather phenomenon.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[serde(rename_all = "snake_case")]
pub enum WeatherPhenomenon {
    /// Blowing dust.
    BlowingDust,
    /// Blowing sand.
    BlowingSand,
    /// Blowing snow.
    BlowingSnow,
    /// Drizzle.
    Drizzle,
    /// Fog.
    Fog,
    /// Freezing fog.
    FreezingFog,
    /// Freezing drizzle.
    FreezingDrizzle,
    /// Freezing rain.
    FreezingRain,
    /// Freezing spray.
    FreezingSpray,
    /// Frost.
    Frost,
    /// Hail.
    Hail,
    /// Haze.
    Haze,
    /// Ice crystals.
    IceCrystals,
    /// Ice fog.
    IceFog,
    /// Rain.
    Rain,
    /// Rain showers.
    RainShowers,
    /// Sleet.
    Sleet,
    /// Smoke.
    Smoke,
    /// Snow.
    Snow,
    /// Snow showers.
    SnowShowers,
    /// Thunderstorms.
    Thunderstorms,
    /// Volcanic ash.
    VolcanicAsh,
    /// Water spouts.
    WaterSpouts,
}

/// How intense a weather phenomenon is.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[serde(rename_all = "snake_case")]
pub enum WeatherIntensity {
    /// Very light.
    VeryLight,
    /// Light.
    Light,
    /// Moderate.
    Moderate,
    /// Heavy.
    Heavy,
}

/// An additional hazard accompanying a weather phenomenon.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[serde(rename_all = "snake_case")]
pub enum WeatherAttribute {
    /// Damaging wind.
    DamagingWind,
    /// Dry thunderstorms.
    DryThunderstorms,
    /// Flooding.
    Flooding,
    /// Gusty wind.
    GustyWind,
    /// Heavy rain.
    HeavyRain,
    /// Large hail.
    LargeHail,
    /// Small hail.
    SmallHail,
    /// Tornadoes.
    Tornadoes,
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    /// The Kansas grid, trimmed to one value per layer.
    fn gridpoint_json() -> Value {
        let mut properties = json!({
            "@id": "https://api.weather.gov/gridpoints/TOP/31,80",
            "@type": "wx:Gridpoint",
            "updateTime": "2026-09-02T06:26:13+00:00",
            "validTimes": "2026-09-02T00:00:00+00:00/P8DT1H",
            "elevation": {"unitCode": "wmoUnit:m", "value": 456.8952},
            "forecastOffice": "https://api.weather.gov/offices/TOP",
            "gridId": "TOP",
            "gridX": 31,
            "gridY": 80,
            "temperature": {
                "uom": "wmoUnit:degC",
                "values": [{"validTime": "2026-09-02T00:00:00+00:00/PT1H", "value": 33.3}]
            },
            "weather": {"values": [{
                "validTime": "2026-09-02T00:00:00+00:00/P4DT12H",
                "value": [{
                    "coverage": "slight_chance",
                    "weather": "rain_showers",
                    "intensity": "light",
                    "visibility": {"unitCode": "wmoUnit:km", "value": null},
                    "attributes": ["gusty_wind"]
                }]
            }]},
            "hazards": {"values": [{
                "validTime": "2026-09-02T18:00:00+00:00/P2DT7H",
                "value": [{"phenomenon": "HT", "significance": "Y", "event_number": null}]
            }]},
        });
        let object = properties.as_object_mut().unwrap();
        for layer in LAYERS {
            object
                .entry((*layer).to_owned())
                .or_insert_with(|| json!({"values": []}));
        }
        properties
    }

    /// Every quantitative layer NOAA sends, in wire order.
    const LAYERS: &[&str] = &[
        "temperature",
        "dewpoint",
        "maxTemperature",
        "minTemperature",
        "relativeHumidity",
        "apparentTemperature",
        "wetBulbGlobeTemperature",
        "heatIndex",
        "windChill",
        "skyCover",
        "windDirection",
        "windSpeed",
        "windGust",
        "heatRisk",
        "probabilityOfPrecipitation",
        "quantitativePrecipitation",
        "iceAccumulation",
        "snowfallAmount",
        "snowLevel",
        "ceilingHeight",
        "visibility",
        "transportWindSpeed",
        "transportWindDirection",
        "mixingHeight",
        "hainesIndex",
        "lightningActivityLevel",
        "twentyFootWindSpeed",
        "twentyFootWindDirection",
        "waveHeight",
        "wavePeriod",
        "waveDirection",
        "primarySwellHeight",
        "primarySwellDirection",
        "secondarySwellHeight",
        "secondarySwellDirection",
        "wavePeriod2",
        "windWaveHeight",
        "dispersionIndex",
        "pressure",
        "probabilityOfTropicalStormWinds",
        "probabilityOfHurricaneWinds",
        "potentialOf15mphWinds",
        "potentialOf25mphWinds",
        "potentialOf35mphWinds",
        "potentialOf45mphWinds",
        "potentialOf20mphWindGusts",
        "potentialOf30mphWindGusts",
        "potentialOf40mphWindGusts",
        "potentialOf50mphWindGusts",
        "potentialOf60mphWindGusts",
        "grasslandFireDangerIndex",
        "probabilityOfThunder",
        "davisStabilityIndex",
        "atmosphericDispersionIndex",
        "lowVisibilityOccurrenceRiskIndex",
        "stability",
        "redFlagThreatIndex",
    ];

    #[test]
    fn the_layer_list_is_the_fifty_seven_noaa_sends() {
        assert_eq!(LAYERS.len(), 57);
    }

    /// Applies the one normalization a gridpoint round trip performs: every
    /// JSON number becomes a float, since measured values are `f64`. Interval
    /// and timestamp text is reproduced exactly, so a comparison shows any
    /// other difference.
    fn as_floats(value: &Value) -> Value {
        match value {
            Value::Number(number) => serde_json::Number::from_f64(number.as_f64().unwrap())
                .map_or(Value::Null, Value::Number),
            Value::Array(items) => Value::Array(items.iter().map(as_floats).collect()),
            Value::Object(members) => {
                let mapped = members
                    .iter()
                    .map(|(key, child)| (key.clone(), as_floats(child)));
                Value::Object(mapped.collect())
            }
            other => other.clone(),
        }
    }

    #[test]
    fn a_full_gridpoint_round_trips_to_the_same_json() {
        let raw = gridpoint_json();
        let gridpoint: Gridpoint = serde_json::from_value(raw.clone()).unwrap();
        let round_tripped = serde_json::to_value(&gridpoint).unwrap();
        assert_eq!(as_floats(&round_tripped), as_floats(&raw));
        assert!(gridpoint.other.is_empty());
    }

    #[test]
    fn intervals_keep_the_numeric_offset_noaa_writes() {
        let gridpoint: Gridpoint = serde_json::from_value(gridpoint_json()).unwrap();
        let value = serde_json::to_value(&gridpoint).unwrap();
        assert_eq!(
            value["validTimes"],
            json!("2026-09-02T00:00:00+00:00/P8DT1H")
        );
        assert_eq!(
            value["temperature"]["values"][0]["validTime"],
            json!("2026-09-02T00:00:00+00:00/PT1H")
        );
        // The same convention the envelope's timestamps already use.
        assert_eq!(value["updateTime"], json!("2026-09-02T06:26:13+00:00"));
    }

    #[test]
    fn every_named_layer_is_present_and_empty_layers_are_empty_not_missing() {
        let gridpoint: Gridpoint = serde_json::from_value(gridpoint_json()).unwrap();
        assert_eq!(gridpoint.grid_id, NwsForecastOfficeId::Top);
        assert_eq!((gridpoint.grid_x, gridpoint.grid_y), (31, 80));
        assert_eq!(gridpoint.elevation.value, Some(456.8952));
        assert_eq!(gridpoint.temperature.values.len(), 1);
        assert!(gridpoint.potential_of_60mph_wind_gusts.values.is_empty());
        assert_eq!(gridpoint.potential_of_60mph_wind_gusts.unit, None);
        assert_eq!(gridpoint.wave_period_2.values, []);
    }

    /// Every one of the 59 layers, not a sample: the probe found all of them
    /// on all 63 grids, and that finding is what makes each a plain field
    /// rather than an `Option`. A layer that quietly became optional would
    /// hand callers a `None` NOAA never sends.
    #[test]
    fn a_missing_layer_is_a_decode_error() {
        let layers: Vec<&str> = LAYERS
            .iter()
            .copied()
            .chain(["weather", "hazards"])
            .collect();
        assert_eq!(layers.len(), 59);

        for layer in layers {
            let mut raw = gridpoint_json();
            assert!(
                raw.as_object_mut().unwrap().remove(layer).is_some(),
                "{layer} is missing from the test document, so its \
                 requiredness is untested"
            );
            assert!(
                serde_json::from_value::<Gridpoint>(raw).is_err(),
                "{layer} should be required"
            );
        }
    }

    /// The same proof for the seven envelope keys the probe found on every
    /// grid. `@id` and `@type` are deliberately optional: they are JSON-LD
    /// decoration, not weather data.
    #[test]
    fn a_missing_envelope_key_is_a_decode_error() {
        for key in [
            "updateTime",
            "validTimes",
            "elevation",
            "forecastOffice",
            "gridId",
            "gridX",
            "gridY",
        ] {
            let mut raw = gridpoint_json();
            assert!(raw.as_object_mut().unwrap().remove(key).is_some(), "{key}");
            assert!(
                serde_json::from_value::<Gridpoint>(raw).is_err(),
                "{key} should be required"
            );
        }

        for key in ["@id", "@type"] {
            let mut raw = gridpoint_json();
            raw.as_object_mut().unwrap().remove(key);
            let gridpoint: Gridpoint = serde_json::from_value(raw)
                .unwrap_or_else(|error| panic!("{key} should be optional: {error}"));
            assert!(gridpoint.other.is_empty());
        }
    }

    #[test]
    fn an_unnamed_layer_lands_in_other_and_survives_the_round_trip() {
        let mut raw = gridpoint_json();
        raw.as_object_mut().unwrap().insert(
            "seaSurfaceTemperature".to_owned(),
            json!({"uom": "wmoUnit:degC", "values": [
                {"validTime": "2026-09-02T00:00:00+00:00/PT1H", "value": 18.0}
            ]}),
        );
        let gridpoint: Gridpoint = serde_json::from_value(raw.clone()).unwrap();
        let extra = &gridpoint.other["seaSurfaceTemperature"];
        assert_eq!(extra.unit.as_ref().unwrap().code(), "wmoUnit:degC");
        assert_eq!(extra.values[0].value, Some(18.0));
        assert_eq!(
            as_floats(&serde_json::to_value(&gridpoint).unwrap()),
            as_floats(&raw)
        );
    }

    #[test]
    fn weather_and_hazards_keep_their_own_shapes() {
        let gridpoint: Gridpoint = serde_json::from_value(gridpoint_json()).unwrap();
        let condition = &gridpoint.weather.values[0].value[0];
        assert_eq!(condition.coverage, Some(WeatherCoverage::SlightChance));
        assert_eq!(condition.weather, Some(WeatherPhenomenon::RainShowers));
        assert_eq!(condition.intensity, Some(WeatherIntensity::Light));
        assert_eq!(condition.visibility.value, None);
        assert_eq!(condition.attributes, [WeatherAttribute::GustyWind]);

        let hazard = &gridpoint.hazards.values[0].value[0];
        assert_eq!(hazard.phenomenon, "HT");
        assert_eq!(hazard.significance, "Y");
        assert_eq!(hazard.event_number, None);
    }

    const TWELVE_HOUR: &str = r#"{
  "units": "us",
  "forecastGenerator": "BaselineForecastGenerator",
  "generatedAt": "2026-09-02T07:50:51+00:00",
  "updateTime": "2026-09-02T06:26:13+00:00",
  "validTimes": "2026-09-02T00:00:00+00:00/P8DT1H",
  "elevation": {"unitCode": "wmoUnit:m", "value": 456.8952},
  "periods": [
    {
      "number": 1,
      "name": "Overnight",
      "startTime": "2026-09-02T02:00:00-05:00",
      "endTime": "2026-09-02T06:00:00-05:00",
      "isDaytime": false,
      "temperature": {"unitCode": "wmoUnit:degC", "value": 23.88888888888889},
      "temperatureTrend": null,
      "probabilityOfPrecipitation": {"unitCode": "wmoUnit:percent", "value": 3},
      "windSpeed": {"unitCode": "wmoUnit:km_h-1", "maxValue": 24.14016, "minValue": 16.09344},
      "windGust": null,
      "windDirection": "S",
      "icon": "https://api.weather.gov/icons/land/night/sct?size=medium",
      "shortForecast": "Partly Cloudy",
      "detailedForecast": "Partly cloudy, with a low around 75."
    }
  ]
}"#;

    const HOURLY: &str = r#"{
  "units": "us",
  "forecastGenerator": "HourlyForecastGenerator",
  "generatedAt": "2026-09-02T07:50:51+00:00",
  "updateTime": "2026-09-02T06:26:13+00:00",
  "validTimes": "2026-09-02T00:00:00+00:00/P8DT1H",
  "elevation": {"unitCode": "wmoUnit:m", "value": 456.8952},
  "periods": [
    {
      "number": 1,
      "name": "",
      "startTime": "2026-09-02T02:00:00-05:00",
      "endTime": "2026-09-02T03:00:00-05:00",
      "isDaytime": false,
      "temperature": {"unitCode": "wmoUnit:degC", "value": 26.11111111111111},
      "temperatureTrend": null,
      "probabilityOfPrecipitation": {"unitCode": "wmoUnit:percent", "value": 0},
      "dewpoint": {"unitCode": "wmoUnit:degC", "value": 15},
      "relativeHumidity": {"unitCode": "wmoUnit:percent", "value": 50},
      "windSpeed": {"unitCode": "wmoUnit:km_h-1", "value": 16.09344},
      "windGust": {"unitCode": "wmoUnit:km_h-1", "value": 32.18688},
      "windDirection": "",
      "icon": "https://api.weather.gov/icons/land/night/sct?size=small",
      "shortForecast": "Partly Cloudy",
      "detailedForecast": ""
    }
  ]
}"#;

    #[test]
    fn both_endpoints_decode_into_one_forecast_and_round_trip() {
        for raw in [TWELVE_HOUR, HOURLY] {
            let mut original: Value = serde_json::from_str(raw).unwrap();
            // A range object carries no `value` key; the single value is
            // `Option`, so the round trip writes it back as null.
            let wind_speed = original["periods"][0]["windSpeed"].as_object_mut().unwrap();
            wind_speed.entry("value").or_insert(Value::Null);

            let forecast: Forecast = serde_json::from_str(raw).unwrap();
            let round_tripped = serde_json::to_value(&forecast).unwrap();
            assert_eq!(as_floats(&round_tripped), as_floats(&original));
        }
    }

    #[test]
    fn the_generator_is_what_tells_the_two_endpoints_apart() {
        let twelve: Forecast = serde_json::from_str(TWELVE_HOUR).unwrap();
        let hourly: Forecast = serde_json::from_str(HOURLY).unwrap();
        assert_eq!(twelve.forecast_generator, ForecastGenerator::Baseline);
        assert_eq!(hourly.forecast_generator, ForecastGenerator::Hourly);
        assert_eq!(
            hourly.forecast_generator.as_str(),
            "HourlyForecastGenerator"
        );

        let unknown = ForecastGenerator::from("SomethingNewGenerator".to_owned());
        assert_eq!(unknown.as_str(), "SomethingNewGenerator");
        assert_eq!(
            serde_json::to_value(&unknown).unwrap(),
            json!("SomethingNewGenerator")
        );
    }

    #[test]
    fn empty_strings_read_as_none_and_serialize_back_as_empty_strings() {
        let hourly: Forecast = serde_json::from_str(HOURLY).unwrap();
        let period = &hourly.periods[0];
        assert_eq!(period.name, None);
        assert_eq!(period.detailed_forecast, None);
        assert_eq!(period.wind_direction, None);

        let value = serde_json::to_value(&hourly).unwrap();
        let period = &value["periods"][0];
        assert_eq!(period["name"], json!(""));
        assert_eq!(period["detailedForecast"], json!(""));
        assert_eq!(period["windDirection"], json!(""));
    }

    #[test]
    fn nulls_stay_null_and_hourly_only_fields_are_skipped_on_twelve_hour_periods() {
        let twelve: Forecast = serde_json::from_str(TWELVE_HOUR).unwrap();
        let period = &twelve.periods[0];
        assert_eq!(period.temperature_trend, None);
        assert_eq!(period.wind_gust, None);
        assert_eq!(period.dewpoint, None);
        assert_eq!(period.relative_humidity, None);
        assert_eq!(period.wind_speed.value, None);
        assert_eq!(period.wind_speed.min_value, Some(16.09344));

        let value = serde_json::to_value(&twelve).unwrap();
        let period = value["periods"][0].as_object().unwrap();
        assert_eq!(period["temperatureTrend"], Value::Null);
        assert_eq!(period["windGust"], Value::Null);
        assert!(!period.contains_key("dewpoint"));
        assert!(!period.contains_key("relativeHumidity"));
    }

    #[test]
    fn a_missing_required_period_key_is_a_decode_error() {
        for key in ["number", "startTime", "isDaytime", "temperature", "icon"] {
            let mut value: Value = serde_json::from_str(TWELVE_HOUR).unwrap();
            value["periods"][0].as_object_mut().unwrap().remove(key);
            assert!(
                serde_json::from_value::<Forecast>(value).is_err(),
                "{key} should be required"
            );
        }
    }

    #[test]
    fn wind_directions_round_trip_through_their_compass_labels() {
        for direction in [
            WindDirection::N,
            WindDirection::Nne,
            WindDirection::Ese,
            WindDirection::Wsw,
            WindDirection::Nnw,
        ] {
            let label = direction.to_string();
            assert_eq!(label.parse::<WindDirection>().unwrap(), direction);
            assert_eq!(serde_json::to_value(direction).unwrap(), json!(label));
        }
    }
}
