//! Semantic Terminal Aerodrome Forecast (TAF) meaning.
//!
//! IWXXM wire structure is decoded privately. Callers receive typed report
//! state and one ordered sequence of semantic forecast groups, independent of
//! the different XML shapes used for base and change forecasts. Measurements
//! are normalized to meters, knots, feet, and degrees Celsius; explicit
//! CAVOK, unchanged, unavailable, cancellation, and missing states remain
//! distinguishable.

mod decode;
mod error;
mod wire;

use jiff::Timestamp;
use serde::Serialize;

pub use error::{TafDecodeError, TafDecodeErrorKind};

/// A decoded Terminal Aerodrome Forecast.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TerminalAerodromeForecast {
    bulletin_identifier: Box<str>,
    report_metadata: ReportMetadata,
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    issued_at: Timestamp,
    aerodrome: Aerodrome,
    report: ForecastReport,
}

impl TerminalAerodromeForecast {
    /// Decodes one WMO IWXXM meteorological bulletin into forecast meaning.
    ///
    /// The XML wire tree remains private; callers receive the same normalized
    /// model as [`crate::apis::stations::Stations::taf`].
    ///
    /// # Errors
    ///
    /// Returns a contextual [`TafDecodeError`] when the XML is malformed or
    /// required forecast meaning cannot be normalized.
    pub fn from_iwxxm(bytes: &[u8]) -> Result<Self, TafDecodeError> {
        decode::decode_iwxxm(bytes)
    }

    /// Aerodrome described by this forecast.
    #[must_use]
    pub const fn aerodrome(&self) -> &Aerodrome {
        &self.aerodrome
    }

    /// WMO bulletin identifier that carried this forecast.
    #[must_use]
    pub const fn bulletin_identifier(&self) -> &str {
        &self.bulletin_identifier
    }

    /// IWXXM report status and usage metadata.
    #[must_use]
    pub const fn report_metadata(&self) -> &ReportMetadata {
        &self.report_metadata
    }

    /// Time at which the forecast was issued.
    #[must_use]
    pub const fn issued_at(&self) -> Timestamp {
        self.issued_at
    }

    /// Forecast, cancellation, or missing-report content.
    #[must_use]
    pub const fn report(&self) -> &ForecastReport {
        &self.report
    }

    /// Forecast groups in IWXXM document order, with the base group first.
    #[must_use]
    pub const fn groups(&self) -> &[ForecastGroup] {
        match &self.report {
            ForecastReport::Forecast { groups, .. } => groups,
            ForecastReport::Cancellation { .. } | ForecastReport::Missing { .. } => &[],
        }
    }

    /// The base forecast, when this report contains forecast conditions.
    #[must_use]
    pub fn base_forecast(&self) -> Option<&ForecastGroup> {
        self.groups()
            .first()
            .filter(|group| group.kind == ForecastGroupKind::Base)
    }

    /// Change forecasts in IWXXM document order.
    #[must_use]
    pub fn change_forecasts(&self) -> &[ForecastGroup] {
        match self.base_forecast() {
            Some(_) => &self.groups()[1..],
            None => self.groups(),
        }
    }
}

/// Report-level IWXXM metadata.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReportMetadata {
    status: ReportStatus,
    permissible_usage: PermissibleUsage,
    #[serde(skip_serializing_if = "Option::is_none")]
    translation: Option<TranslationMetadata>,
}

impl ReportMetadata {
    /// Issuance status of this report.
    #[must_use]
    pub const fn status(&self) -> &ReportStatus {
        &self.status
    }

    /// Permitted operational use of this report.
    #[must_use]
    pub const fn permissible_usage(&self) -> &PermissibleUsage {
        &self.permissible_usage
    }

    /// TAC-to-IWXXM translation provenance, when the report was translated.
    #[must_use]
    pub const fn translation(&self) -> Option<&TranslationMetadata> {
        self.translation.as_ref()
    }
}

/// Provenance supplied when a TAC bulletin was translated to IWXXM.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TranslationMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_bulletin_identifier: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    source_bulletin_received_at: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    centre_designator: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    centre_name: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    translated_at: Option<Timestamp>,
}

impl TranslationMetadata {
    /// Identifier of the TAC bulletin that was translated.
    #[must_use]
    pub fn source_bulletin_identifier(&self) -> Option<&str> {
        self.source_bulletin_identifier.as_deref()
    }

    /// Time the source TAC bulletin was received by the translation centre.
    #[must_use]
    pub const fn source_bulletin_received_at(&self) -> Option<Timestamp> {
        self.source_bulletin_received_at
    }

    /// ICAO designator of the translation centre.
    #[must_use]
    pub fn centre_designator(&self) -> Option<&str> {
        self.centre_designator.as_deref()
    }

    /// Human-readable name of the translation centre.
    #[must_use]
    pub fn centre_name(&self) -> Option<&str> {
        self.centre_name.as_deref()
    }

    /// Time at which TAC-to-IWXXM translation completed.
    #[must_use]
    pub const fn translated_at(&self) -> Option<Timestamp> {
        self.translated_at
    }
}

/// IWXXM report issuance status.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ReportStatus {
    /// Routine forecast issuance.
    Normal,
    /// Amendment to an earlier forecast.
    Amendment,
    /// Correction to an earlier forecast.
    Correction,
    /// A future status not yet known to this client.
    Other {
        /// Original IWXXM status code.
        code: Box<str>,
    },
}

/// Permitted use of an IWXXM report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum PermissibleUsage {
    /// Report may be used operationally.
    Operational,
    /// Report is restricted to a non-operational purpose.
    NonOperational {
        /// Structured reason supplied by IWXXM.
        reason: Option<PermissibleUsageReason>,
        /// Additional human-readable usage restriction.
        supplementary: Option<Box<str>>,
    },
    /// A future usage code not yet known to this client.
    Other {
        /// Original IWXXM usage code.
        code: Box<str>,
        /// Structured reason supplied with the future usage code.
        reason: Option<PermissibleUsageReason>,
        /// Additional human-readable usage restriction.
        supplementary: Option<Box<str>>,
    },
}

/// Why an IWXXM report is non-operational.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum PermissibleUsageReason {
    /// Test data.
    Test,
    /// Exercise data.
    Exercise,
    /// A future reason not yet known to this client.
    Other {
        /// Original IWXXM reason code.
        code: Box<str>,
    },
}

/// Content state of a TAF report.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
#[non_exhaustive]
pub enum ForecastReport {
    /// Ordinary forecast conditions.
    Forecast {
        /// Overall period for which the TAF is valid.
        valid_period: TimeRange,
        /// Base group followed by change groups in document order.
        groups: Box<[ForecastGroup]>,
    },
    /// Cancellation of an earlier report.
    Cancellation {
        /// Valid period of the report being cancelled.
        cancelled_period: TimeRange,
    },
    /// Report whose forecast content is missing.
    Missing {
        /// Why no forecast content is available.
        reason: MissingForecastReason,
    },
}

/// Why a TAF contains no forecast groups.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum MissingForecastReason {
    /// The source report did not provide forecast content.
    NotProvided,
    /// Translation of the source TAC report failed.
    TranslationFailed {
        /// Complete source TAC retained by IWXXM.
        tac: Box<str>,
    },
}

/// Inclusive time range associated with forecast meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TimeRange {
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    start: Timestamp,
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    end: Timestamp,
}

impl TimeRange {
    /// Beginning of the range.
    #[must_use]
    pub const fn start(self) -> Timestamp {
        self.start
    }

    /// End of the range.
    #[must_use]
    pub const fn end(self) -> Timestamp {
        self.end
    }
}

/// Aerodrome identity carried by a TAF.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Aerodrome {
    designator: Box<str>,
    icao_identifier: Box<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<GeoPosition>,
}

impl Aerodrome {
    /// Aerodrome designator.
    #[must_use]
    pub const fn designator(&self) -> &str {
        &self.designator
    }

    /// ICAO location indicator.
    #[must_use]
    pub const fn icao_identifier(&self) -> &str {
        &self.icao_identifier
    }

    /// Aerodrome reference point, when supplied inline by NOAA.
    #[must_use]
    pub const fn position(&self) -> Option<GeoPosition> {
        self.position
    }
}

/// Geographic point in latitude/longitude order.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GeoPosition {
    latitude: f64,
    longitude: f64,
}

impl GeoPosition {
    /// Latitude in decimal degrees.
    #[must_use]
    pub const fn latitude(self) -> f64 {
        self.latitude
    }

    /// Longitude in decimal degrees.
    #[must_use]
    pub const fn longitude(self) -> f64 {
        self.longitude
    }
}

/// Relational operator attached to a forecast measurement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum Comparison {
    /// The reported value is exact.
    Exact,
    /// Actual conditions are above the reported value.
    Above,
    /// Actual conditions are below the reported value.
    Below,
    /// A future IWXXM operator not yet known to this client.
    Other {
        /// Original IWXXM operator code.
        code: Box<str>,
    },
}

/// Prevailing horizontal visibility.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Visibility {
    meters: f64,
    comparison: Comparison,
}

impl Visibility {
    /// Visibility in meters.
    #[must_use]
    pub const fn meters(&self) -> f64 {
        self.meters
    }

    /// Comparison applied to the reported distance.
    #[must_use]
    pub const fn comparison(&self) -> &Comparison {
        &self.comparison
    }
}

/// Forecast wind direction.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", content = "degrees", rename_all = "camelCase")]
#[non_exhaustive]
pub enum WindDirection {
    /// Wind direction varies.
    Variable,
    /// Direction from which wind blows, in degrees true.
    Degrees(f64),
}

/// Wind speed normalized to knots.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WindSpeed {
    knots: f64,
    comparison: Comparison,
}

impl WindSpeed {
    /// Speed in knots.
    #[must_use]
    pub const fn knots(&self) -> f64 {
        self.knots
    }

    /// Comparison applied to the reported speed.
    #[must_use]
    pub const fn comparison(&self) -> &Comparison {
        &self.comparison
    }
}

/// Surface-wind forecast.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SurfaceWind {
    direction: WindDirection,
    speed: WindSpeed,
    #[serde(skip_serializing_if = "Option::is_none")]
    gust: Option<WindSpeed>,
}

/// Reported, omitted, or explicitly unavailable forecast meaning.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "state", content = "value", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ForecastElement<T> {
    /// This group does not report a change to the value.
    NotReported,
    /// Concrete forecast meaning.
    Value(T),
    /// Meaning is explicitly unavailable.
    Unavailable {
        /// IWXXM nil reason.
        reason: MissingReason,
    },
}

impl<T> ForecastElement<T> {
    /// Concrete meaning, if reported.
    #[must_use]
    pub const fn value(&self) -> Option<&T> {
        match self {
            Self::Value(value) => Some(value),
            Self::NotReported | Self::Unavailable { .. } => None,
        }
    }

    /// Unavailability reason, when the source explicitly supplied one.
    #[must_use]
    pub const fn unavailable_reason(&self) -> Option<&MissingReason> {
        match self {
            Self::Unavailable { reason } => Some(reason),
            Self::NotReported | Self::Value(_) => None,
        }
    }
}

/// Surface-wind state for one forecast group.
pub type ForecastWind = ForecastElement<SurfaceWind>;

/// Prevailing-visibility state for one forecast group.
pub type ForecastVisibility = ForecastElement<Visibility>;

/// A reported value or an explicit IWXXM unavailable value.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "state", content = "value", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ForecastValue<T> {
    /// Concrete forecast value.
    Value(T),
    /// Value explicitly unavailable for the supplied reason.
    Unavailable {
        /// IWXXM nil reason.
        reason: MissingReason,
    },
}

impl<T> ForecastValue<T> {
    /// Concrete value, if one was reported.
    #[must_use]
    pub const fn value(&self) -> Option<&T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Unavailable { .. } => None,
        }
    }

    /// Unavailability reason, if the value was explicitly unavailable.
    #[must_use]
    pub const fn unavailable_reason(&self) -> Option<&MissingReason> {
        match self {
            Self::Value(_) => None,
            Self::Unavailable { reason } => Some(reason),
        }
    }
}

/// Why IWXXM forecast meaning is unavailable.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum MissingReason {
    /// Nothing operationally significant is forecast.
    NoSignificant,
    /// Value cannot be observed or forecast.
    NotObservable,
    /// Required source data is missing.
    Missing,
    /// Value was withheld.
    Withheld,
    /// A future IWXXM nil reason not yet known to this client.
    Other {
        /// Original nil-reason code or URI.
        code: Box<str>,
    },
}

/// Significant-weather state for one forecast group.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "state", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ForecastWeather {
    /// This group does not report a weather change.
    NotReported,
    /// The group explicitly forecasts no significant weather.
    NoSignificant,
    /// Significant weather phenomena reported by the group.
    Phenomena {
        /// Phenomena in document order.
        items: Box<[Weather]>,
    },
    /// Weather meaning is explicitly unavailable.
    Unavailable {
        /// IWXXM nil reason.
        reason: MissingReason,
    },
}

/// One exact WMO 4678 weather code and its parsed meaning.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Weather {
    code: Box<str>,
    intensity: WeatherIntensity,
    in_vicinity: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    descriptor: Option<WeatherDescriptor>,
    phenomena: Box<[WeatherPhenomenon]>,
}

impl Weather {
    /// Exact WMO 4678 code supplied by NOAA.
    #[must_use]
    pub const fn code(&self) -> &str {
        &self.code
    }

    /// Intensity encoded by the weather code.
    #[must_use]
    pub const fn intensity(&self) -> WeatherIntensity {
        self.intensity
    }

    /// Whether the phenomenon is forecast in the vicinity.
    #[must_use]
    pub const fn is_in_vicinity(&self) -> bool {
        self.in_vicinity
    }

    /// Weather descriptor, when present.
    #[must_use]
    pub const fn descriptor(&self) -> Option<&WeatherDescriptor> {
        self.descriptor.as_ref()
    }

    /// Individual phenomena encoded by the combined weather code.
    #[must_use]
    pub const fn phenomena(&self) -> &[WeatherPhenomenon] {
        &self.phenomena
    }
}

/// Intensity of forecast weather.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum WeatherIntensity {
    /// Light intensity (`-`).
    Light,
    /// Moderate/default intensity (no prefix).
    Moderate,
    /// Heavy intensity (`+`).
    Heavy,
}

/// Descriptor modifying one or more weather phenomena.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum WeatherDescriptor {
    /// Shallow.
    Shallow,
    /// Partial.
    Partial,
    /// Patches.
    Patches,
    /// Low drifting.
    LowDrifting,
    /// Blowing.
    Blowing,
    /// Showers.
    Showers,
    /// Thunderstorm.
    Thunderstorm,
    /// Freezing.
    Freezing,
    /// A future descriptor not yet known to this client.
    Other {
        /// Original two-character code.
        code: Box<str>,
    },
}

/// Atomic phenomenon encoded by a WMO 4678 weather code.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum WeatherPhenomenon {
    /// Drizzle.
    Drizzle,
    /// Rain.
    Rain,
    /// Snow.
    Snow,
    /// Snow grains.
    SnowGrains,
    /// Ice crystals.
    IceCrystals,
    /// Ice pellets.
    IcePellets,
    /// Hail.
    Hail,
    /// Small hail or snow pellets.
    SmallHail,
    /// Unknown precipitation.
    UnknownPrecipitation,
    /// Mist.
    Mist,
    /// Fog.
    Fog,
    /// Smoke.
    Smoke,
    /// Volcanic ash.
    VolcanicAsh,
    /// Widespread dust.
    Dust,
    /// Sand.
    Sand,
    /// Haze.
    Haze,
    /// Spray.
    Spray,
    /// Dust or sand whirls.
    DustWhirls,
    /// Squalls.
    Squalls,
    /// Funnel cloud or tornado/waterspout.
    FunnelCloud,
    /// Sandstorm.
    Sandstorm,
    /// Duststorm.
    Duststorm,
    /// A future phenomenon not yet known to this client.
    Other {
        /// Original two-character code.
        code: Box<str>,
    },
}

/// Cloud state for one forecast group.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "state", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ForecastClouds {
    /// This group does not report a cloud change.
    NotReported,
    /// The group explicitly forecasts no significant cloud.
    NoSignificant,
    /// Vertical visibility into an obscuring medium.
    VerticalVisibility {
        /// Vertical visibility normalized to feet.
        feet: ForecastValue<f64>,
    },
    /// Forecast cloud layers.
    Layers {
        /// Layers in document order.
        layers: Box<[CloudLayer]>,
    },
    /// Cloud meaning is explicitly unavailable.
    Unavailable {
        /// IWXXM nil reason.
        reason: MissingReason,
    },
}

/// One forecast cloud layer.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CloudLayer {
    amount: ForecastValue<CloudAmount>,
    base_feet: ForecastValue<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_type: Option<ForecastValue<CloudType>>,
}

impl CloudLayer {
    /// Reported cloud amount or its unavailable state.
    #[must_use]
    pub const fn amount(&self) -> &ForecastValue<CloudAmount> {
        &self.amount
    }

    /// Cloud-base height in feet AGL or its unavailable state.
    #[must_use]
    pub const fn base_feet(&self) -> &ForecastValue<f64> {
        &self.base_feet
    }

    /// Significant convective cloud type, when reported.
    #[must_use]
    pub const fn cloud_type(&self) -> Option<&ForecastValue<CloudType>> {
        self.cloud_type.as_ref()
    }
}

/// Amount of sky covered by one cloud layer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum CloudAmount {
    /// Few clouds.
    Few,
    /// Scattered clouds.
    Scattered,
    /// Broken cloud layer.
    Broken,
    /// Overcast cloud layer.
    Overcast,
    /// No significant cloud.
    NoSignificant,
    /// Sky clear.
    SkyClear,
    /// A future cloud amount not yet known to this client.
    Other {
        /// Original WMO registry code.
        code: Box<str>,
    },
}

/// Significant convective cloud type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum CloudType {
    /// Cumulonimbus.
    Cumulonimbus,
    /// Towering cumulus.
    ToweringCumulus,
    /// A future cloud type not yet known to this client.
    Other {
        /// Original WMO registry code.
        code: Box<str>,
    },
}

/// Paired maximum and minimum temperature forecast for one validity period.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TemperatureForecast {
    maximum: TemperatureExtreme,
    minimum: TemperatureExtreme,
}

impl TemperatureForecast {
    /// Forecast maximum temperature and its occurrence time.
    #[must_use]
    pub const fn maximum(&self) -> &TemperatureExtreme {
        &self.maximum
    }

    /// Forecast minimum temperature and its occurrence time.
    #[must_use]
    pub const fn minimum(&self) -> &TemperatureExtreme {
        &self.minimum
    }
}

/// One forecast temperature extremum.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TemperatureExtreme {
    celsius: f64,
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    occurs_at: Timestamp,
}

impl TemperatureExtreme {
    /// Temperature in degrees Celsius.
    #[must_use]
    pub const fn celsius(self) -> f64 {
        self.celsius
    }

    /// Time at which this temperature is forecast to occur.
    #[must_use]
    pub const fn occurs_at(self) -> Timestamp {
        self.occurs_at
    }
}

impl SurfaceWind {
    /// Forecast wind direction.
    #[must_use]
    pub const fn direction(&self) -> WindDirection {
        self.direction
    }

    /// Forecast mean wind speed.
    #[must_use]
    pub const fn speed(&self) -> &WindSpeed {
        &self.speed
    }

    /// Forecast gust speed, when reported.
    #[must_use]
    pub const fn gust(&self) -> Option<&WindSpeed> {
        self.gust.as_ref()
    }
}

/// Meteorological conditions reported by one forecast group.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ForecastConditions {
    cavok: bool,
    visibility: ForecastVisibility,
    wind: ForecastWind,
    weather: ForecastWeather,
    clouds: ForecastClouds,
    temperatures: Box<[TemperatureForecast]>,
}

impl ForecastConditions {
    /// Whether ceiling and visibility are forecast as OK (CAVOK).
    #[must_use]
    pub const fn is_cavok(&self) -> bool {
        self.cavok
    }

    /// Prevailing visibility state for this group.
    #[must_use]
    pub const fn visibility(&self) -> &ForecastVisibility {
        &self.visibility
    }

    /// Surface wind, when reported by this group.
    #[must_use]
    pub const fn wind(&self) -> &ForecastWind {
        &self.wind
    }

    /// Significant-weather state for this group.
    #[must_use]
    pub const fn weather(&self) -> &ForecastWeather {
        &self.weather
    }

    /// Cloud state for this group.
    #[must_use]
    pub const fn clouds(&self) -> &ForecastClouds {
        &self.clouds
    }

    /// Maximum/minimum temperature forecasts carried by this group.
    ///
    /// IWXXM permits temperatures only on the base forecast.
    #[must_use]
    pub const fn temperatures(&self) -> &[TemperatureForecast] {
        &self.temperatures
    }
}

/// One base or change group in a TAF.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ForecastGroup {
    kind: ForecastGroupKind,
    valid_period: TimeRange,
    conditions: ForecastConditions,
}

impl ForecastGroup {
    /// Semantic kind of this forecast group.
    #[must_use]
    pub const fn kind(&self) -> &ForecastGroupKind {
        &self.kind
    }

    /// Period during which this group's conditions occur.
    #[must_use]
    pub const fn valid_period(&self) -> TimeRange {
        self.valid_period
    }

    /// Meteorological conditions carried by this group.
    #[must_use]
    pub const fn conditions(&self) -> &ForecastConditions {
        &self.conditions
    }
}

/// How a forecast group modifies the prevailing conditions.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ForecastGroupKind {
    /// Initial prevailing forecast conditions.
    Base,
    /// Conditions prevailing from a specified instant.
    From,
    /// Conditions becoming established during a period.
    Becoming,
    /// Temporary fluctuations during a period.
    Temporary,
    /// Alternative conditions with an indicated probability.
    Probability {
        /// Probability percentage supplied by IWXXM.
        percent: u8,
        /// Whether the probability applies to temporary fluctuations.
        temporary: bool,
    },
    /// A future IWXXM change indicator not yet known to this client.
    Other {
        /// Original IWXXM change-indicator code.
        code: Box<str>,
    },
}
