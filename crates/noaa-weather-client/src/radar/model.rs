//! Curated models for radar stations, servers, queues, alarms, and SPGDS telemetry.
//!
//! A live census on 2026-09-04 covered all 208 radar stations, all six radar
//! servers, 500 queue entries, and all eight SPGDS hosts. Queue and SPGDS
//! records each shared one fully populated keyset. Every server carried its
//! identity plus ping, hardware, LDM, and network telemetry; the two
//! distribution servers omit command and role flags. Every station carried
//! identity, location, elevation, time zone, and latency; only the five
//! profilers lacked RDA telemetry. Detailed station responses add performance
//! and adaptation telemetry, whose properties are empty arrays for TDWR sites.
//!
//! JSON-LD context is vocabulary metadata and is deliberately excluded. The
//! private wire structs normalize GeoJSON and server envelopes into the public
//! semantic telemetry types.

use std::collections::BTreeMap;

use jiff::tz::TimeZone;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct as _};

use crate::geo::Geometry;
use crate::ids::RadarStationId;
use crate::stations::QualityControl;
use crate::time::OffsetDateTime;
use crate::units::{Quantity, Unit};

/// A radar measurement whose unit may be absent.
///
/// Most NOAA measurements use [`Quantity`], but radar telemetry has been
/// observed to send a numeric latency without `unitCode`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarMeasurement {
    /// The observed numeric value, or `None` when NOAA reports no value.
    #[serde(default)]
    pub value: Option<f64>,
    /// The lower bound NOAA supplied for the measurement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// The upper bound NOAA supplied for the measurement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// The unit associated with the value and bounds.
    #[serde(rename = "unitCode", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    /// The quality-control flag assigned to the measurement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_control: Option<QualityControl>,
}

impl RadarMeasurement {
    /// Returns the standard quantity form when NOAA supplied a unit.
    #[must_use]
    pub fn quantity(&self) -> Option<Quantity> {
        Some(Quantity {
            value: self.value,
            min_value: self.min_value,
            max_value: self.max_value,
            unit: self.unit.clone()?,
            quality_control: self.quality_control,
        })
    }
}

/// The two meanings NOAA puts in `commandChannel`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(untagged)]
#[non_exhaustive]
pub enum CommandChannel {
    /// A redundant command-channel number.
    Channel(u8),
    /// A named command-channel mode.
    Mode(CommandChannelMode),
    /// A future named mode kept verbatim.
    Other(String),
}

/// A named radar command-channel mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
#[non_exhaustive]
pub enum CommandChannelMode {
    /// A single, non-redundant command channel.
    #[serde(rename = "Single")]
    Single,
}

/// Human-oriented geographic state for a radar station.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum RadarPosition {
    /// The station response contains no geometry.
    Missing,
    /// The station geometry is present but is not a point.
    Invalid,
    /// The station is located at the given longitude and latitude.
    Coordinates {
        /// The station longitude in degrees.
        longitude: f64,
        /// The station latitude in degrees.
        latitude: f64,
    },
}

/// One radar station GeoJSON feature, normalized for callers.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarStationTelemetry {
    /// The GeoJSON feature identifier, when NOAA supplies one.
    pub feature_id: Option<String>,
    /// The station geometry from the GeoJSON feature.
    pub geometry: Option<Geometry>,
    /// The station identity and operational telemetry.
    pub station: RadarStationDetails,
}

impl RadarStationTelemetry {
    /// Returns the point location, or why one is unavailable.
    #[must_use]
    pub const fn position(&self) -> RadarPosition {
        match self.geometry.as_ref() {
            None => RadarPosition::Missing,
            Some(Geometry::Point(position)) => RadarPosition::Coordinates {
                longitude: position.lon(),
                latitude: position.lat(),
            },
            Some(_) => RadarPosition::Invalid,
        }
    }
}

impl Serialize for RadarStationTelemetry {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct(
            "RadarStationTelemetry",
            3 + usize::from(self.feature_id.is_some()),
        )?;
        state.serialize_field("type", "Feature")?;
        if let Some(id) = &self.feature_id {
            state.serialize_field("id", id)?;
        }
        state.serialize_field("geometry", &self.geometry)?;
        state.serialize_field("properties", &self.station)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for RadarStationTelemetry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RadarStationWire::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for RadarStationTelemetry {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RadarStationTelemetry".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "description": "GeoJSON feature containing radar station telemetry.",
            "properties": {
                "type": {"type": "string", "const": "Feature"},
                "id": {"type": "string"},
                "geometry": generator.subschema_for::<Option<Geometry>>(),
                "properties": generator.subschema_for::<RadarStationDetails>(),
            },
            "required": ["type", "geometry", "properties"],
        })
    }
}

/// Identity and operational telemetry for one radar station.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarStationDetails {
    /// The JSON-LD resource identifier from the `@id` property.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// The JSON-LD resource type from the `@type` property.
    #[serde(rename = "@type")]
    pub at_type: String,
    /// The radar station identifier.
    pub id: RadarStationId,
    /// The human-readable station name.
    pub name: String,
    /// The radar installation type, such as `WSR-88D` or `TDWR`.
    pub station_type: String,
    /// The station elevation and its reported unit.
    pub elevation: RadarMeasurement,
    /// The station's civil time zone.
    #[serde(with = "jiff::fmt::serde::tz::required")]
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    pub time_zone: TimeZone,
    /// Product-delivery latency observed for the station.
    pub latency: RadarStationLatency,
    /// Radar Data Acquisition telemetry, when available for the station.
    pub rda: Option<RadarDataAcquisitionTelemetry>,
    /// Detailed radar performance telemetry, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<RadarPerformanceTelemetry>,
    /// Radar adaptation and calibration values, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adaptation: Option<RadarAdaptationTelemetry>,
}

/// Delivery latency for one radar station.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarStationLatency {
    /// The most recently measured product-delivery latency.
    pub current: Option<RadarMeasurement>,
    /// The average product-delivery latency reported by NOAA.
    pub average: Option<RadarMeasurement>,
    /// The maximum product-delivery latency reported by NOAA.
    #[serde(rename = "max")]
    pub maximum: Option<RadarMeasurement>,
    /// The time the latest Level II product was received.
    pub level_two_last_received_time: Option<OffsetDateTime>,
    /// The time at which the reported maximum latency occurred.
    pub max_latency_time: Option<OffsetDateTime>,
    /// The host that reported the latency telemetry.
    pub reporting_host: Option<String>,
    /// The Local Data Manager host serving the station products.
    pub host: Option<String>,
}

/// Radar Data Acquisition telemetry and provenance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarDataAcquisitionTelemetry {
    /// The time associated with the acquisition-system report.
    pub timestamp: OffsetDateTime,
    /// The host that reported the acquisition-system telemetry.
    pub reporting_host: String,
    /// The reported acquisition-system operating properties.
    pub properties: RadarDataAcquisitionProperties,
}

/// Operational properties reported by a radar data acquisition system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarDataAcquisitionProperties {
    /// The Level II data resolution version.
    pub resolution_version: Option<i32>,
    /// The internal path used for Level II radar products.
    pub nl2_path: String,
    /// The active volume coverage pattern identifier.
    pub volume_coverage_pattern: String,
    /// The radar control authority and connection status.
    pub control_status: String,
    /// The installed radar software build number.
    pub build_number: f64,
    /// The acquisition system's alarm summary.
    pub alarm_summary: String,
    /// The acquisition system's operating mode.
    pub mode: String,
    /// The radar product generator state, when reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator_state: Option<String>,
    /// The super-resolution processing status, when reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub super_resolution_status: Option<String>,
    /// The acquisition system's operability status.
    pub operability_status: String,
    /// The acquisition system's current operating status.
    pub status: String,
    /// The average transmitter output power, when reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_transmitter_power: Option<RadarMeasurement>,
    /// The reflectivity calibration correction, when reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reflectivity_calibration_correction: Option<RadarMeasurement>,
}

/// Radar performance telemetry and provenance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarPerformanceTelemetry {
    /// The time associated with the performance report.
    pub timestamp: Option<OffsetDateTime>,
    /// The host that reported the performance telemetry.
    pub reporting_host: String,
    /// The detailed performance measurements, when the response contains an object.
    #[serde(
        default,
        deserialize_with = "deserialize_object_or_empty_array",
        skip_serializing_if = "Option::is_none"
    )]
    pub properties: Option<RadarPerformanceProperties>,
}

/// Detailed WSR-88D performance metrics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarPerformanceProperties {
    /// The reported Network Time Protocol synchronization status code.
    #[serde(
        rename = "ntp_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ntp_status: Option<i32>,
    /// The command channel number or named operating mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_channel: Option<CommandChannel>,
    /// The air temperature within the antenna radome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radome_air_temperature: Option<RadarMeasurement>,
    /// The transitional electrical power source status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitional_power_source: Option<String>,
    /// The horizontal receiver noise measured with short pulses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_short_pulse_noise: Option<RadarMeasurement>,
    /// The elevation encoder indicator state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elevation_encoder_light: Option<String>,
    /// The horizontal receiver noise measured with long pulses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_long_pulse_noise: Option<RadarMeasurement>,
    /// The azimuth encoder indicator state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azimuth_encoder_light: Option<String>,
    /// The equivalent noise temperature of the horizontal receiver channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_noise_temperature: Option<RadarMeasurement>,
    /// The receiver linearity value as reported by NOAA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linearity: Option<f64>,
    /// The transmitter's measured peak output power.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_peak_power: Option<RadarMeasurement>,
    /// The horizontal-channel reflectivity calibration offset at zero dBZ.
    #[serde(
        rename = "horizontalDeltadBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_delta_dbz0: Option<RadarMeasurement>,
    /// The transmitter recycle event count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_recycle_count: Option<i32>,
    /// The vertical-channel reflectivity calibration offset at zero dBZ.
    #[serde(
        rename = "verticalDeltadBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_delta_dbz0: Option<RadarMeasurement>,
    /// The measured receiver bias.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_bias: Option<RadarMeasurement>,
    /// The short-pulse horizontal-channel reflectivity baseline at zero dBZ.
    #[serde(
        rename = "shortPulseHorizontaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_pulse_horizontal_dbz0: Option<RadarMeasurement>,
    /// The measured imbalance in the transmitter signal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_imbalance: Option<RadarMeasurement>,
    /// The long-pulse horizontal-channel reflectivity baseline at zero dBZ.
    #[serde(
        rename = "longPulseHorizontaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_pulse_horizontal_dbz0: Option<RadarMeasurement>,
    /// The time the performance check was performed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance_check_time: Option<OffsetDateTime>,
    /// The temperature of air leaving the transmitter enclosure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_leaving_air_temperature: Option<RadarMeasurement>,
    /// The temperature inside the radar equipment shelter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shelter_temperature: Option<RadarMeasurement>,
    /// The electrical power source supplying the radar equipment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_source: Option<String>,
    /// The receiver dynamic range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_range: Option<RadarMeasurement>,
    /// The short-pulse vertical-channel reflectivity baseline at zero dBZ.
    #[serde(
        rename = "shortPulseVerticaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_pulse_vertical_dbz0: Option<RadarMeasurement>,
    /// The long-pulse vertical-channel reflectivity baseline at zero dBZ.
    #[serde(
        rename = "longPulseVerticaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_pulse_vertical_dbz0: Option<RadarMeasurement>,
    /// The backup generator fuel level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuel_level: Option<RadarMeasurement>,
}

/// Radar adaptation telemetry and provenance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarAdaptationTelemetry {
    /// The time associated with the adaptation report.
    pub timestamp: Option<OffsetDateTime>,
    /// The host that reported the adaptation telemetry.
    pub reporting_host: String,
    /// The detailed adaptation values, when the response contains an object.
    #[serde(
        default,
        deserialize_with = "deserialize_object_or_empty_array",
        skip_serializing_if = "Option::is_none"
    )]
    pub properties: Option<RadarAdaptationProperties>,
}

/// Detailed WSR-88D adaptation values.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarAdaptationProperties {
    /// The configured radar transmitter frequency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_frequency: Option<RadarMeasurement>,
    /// The signal loss through the WG04 circulator path.
    #[serde(
        rename = "pathLossWG04Circulator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg04_circulator: Option<RadarMeasurement>,
    /// The antenna gain including attenuation from the radome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub antenna_gain_including_radome: Option<RadarMeasurement>,
    /// The signal loss through the A6 arc-detector path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_loss_a6_arc_detector: Option<RadarMeasurement>,
    /// The coherent oscillator power measured at A1 J4.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coho_power_at_a1_j4: Option<RadarMeasurement>,
    /// The horizontal test-signal power reported by the antenna measurement equipment.
    #[serde(
        rename = "ameHorzizontalTestSignalPower",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ame_horizontal_test_signal_power: Option<RadarMeasurement>,
    /// The path-loss correction for transmitter coupler coupling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_loss_transmitter_coupler_coupling: Option<RadarMeasurement>,
    /// The stable local oscillator power measured at A1 J2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stalo_power_at_a1_j2: Option<RadarMeasurement>,
    /// The horizontal noise source's excess-noise ratio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ame_noise_source_horizontal_excess_noise_ratio: Option<RadarMeasurement>,
    /// The signal loss from the vertical IF Heliax path to 4AT16.
    #[serde(
        rename = "pathLossVerticalIFHeliaxTo4AT16",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_vertical_if_heliax_to_4at16: Option<RadarMeasurement>,
    /// The signal loss through the AT4 attenuator.
    #[serde(
        rename = "pathLossAT4Attenuator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_at4_attenuator: Option<RadarMeasurement>,
    /// The signal loss from the horizontal IF Heliax path to 4AT17.
    #[serde(
        rename = "pathLossHorzontalIFHeliaxTo4AT17",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_horizontal_if_heliax_to_4at17: Option<RadarMeasurement>,
    /// The signal loss through the IFDR IF anti-alias filter.
    #[serde(
        rename = "pathLossIFDRIFAntiAliasFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_ifdrif_anti_alias_filter: Option<RadarMeasurement>,
    /// The signal loss through the IFD burst anti-alias filter.
    #[serde(
        rename = "pathLossIFDBurstAntiAliasFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_ifd_burst_anti_alias_filter: Option<RadarMeasurement>,
    /// The signal loss through the WG02 harmonic filter.
    #[serde(
        rename = "pathLossWG02HarmonicFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg02_harmonic_filter: Option<RadarMeasurement>,
    /// The factor that converts transmitter power data to watts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_power_data_watts_factor: Option<RadarMeasurement>,
    /// The waveguide path loss from the klystron to the switch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_loss_waveguide_klystron_to_switch: Option<RadarMeasurement>,
    /// The configured transmitter output width for short pulses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulse_width_transmitter_output_short_pulse: Option<RadarMeasurement>,
    /// The configured transmitter output width for long pulses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulse_width_transmitter_output_long_pulse: Option<RadarMeasurement>,
    /// The signal loss through the WG06 spectrum filter.
    #[serde(
        rename = "pathLossWG06SpectrumFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg06_spectrum_filter: Option<RadarMeasurement>,
    /// The horizontal receiver noise measured with short pulses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_receiver_noise_short_pulse: Option<RadarMeasurement>,
    /// The horizontal receiver noise measured with long pulses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_receiver_noise_long_pulse: Option<RadarMeasurement>,
    /// Whether the transmitter spectrum filter is installed, as reported by NOAA.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_spectrum_filter_installed: Option<String>,
}

/// A collection of radar station features.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RadarStationsResponse {
    /// The radar station features returned by NOAA.
    #[serde(rename = "features", default)]
    pub stations: Vec<RadarStationTelemetry>,
}

impl RadarStationsResponse {
    /// Returns the number of radar stations in the response.
    #[must_use]
    pub fn len(&self) -> usize {
        self.stations.len()
    }
    /// Returns `true` when the response contains no radar stations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stations.is_empty()
    }
    /// Returns an iterator over the radar stations in response order.
    pub fn iter(&self) -> impl Iterator<Item = &RadarStationTelemetry> {
        self.stations.iter()
    }
}

impl Serialize for RadarStationsResponse {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("RadarStationsResponse", 2)?;
        state.serialize_field("type", "FeatureCollection")?;
        state.serialize_field("features", &self.stations)?;
        state.end()
    }
}

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for RadarStationsResponse {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RadarStationsResponse".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "object",
            "properties": {
                "type": {"type": "string", "const": "FeatureCollection"},
                "features": {"type": "array", "items": generator.subschema_for::<RadarStationTelemetry>()},
            },
            "required": ["type", "features"],
        })
    }
}

/// One radar server and its latest telemetry.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarServerTelemetry {
    /// The JSON-LD resource identifier from the `@id` property.
    #[serde(rename = "@id")]
    pub at_id: String,
    /// The JSON-LD resource type from the `@type` property.
    #[serde(rename = "@type")]
    pub at_type: String,
    /// The radar server identifier.
    pub id: String,
    /// The server's role or service type.
    #[serde(rename = "type")]
    pub server_type: String,
    /// Whether the server is active, when NOAA reports the role flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Whether the server is primary, when NOAA reports the role flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// Whether the server aggregates radar products, when NOAA reports the role flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<bool>,
    /// Whether the server is administratively locked, when NOAA reports the flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Whether the radar network is reachable, when NOAA reports the flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radar_network_up: Option<bool>,
    /// The time NOAA collected this server telemetry.
    pub collection_time: OffsetDateTime,
    /// The host that reported the server telemetry.
    pub reporting_host: String,
    /// The host from which the server ingests radar products.
    pub ingest_host: String,
    /// Reachability measurements for the server's ping targets.
    pub ping: RadarPingTelemetry,
    /// Command processing telemetry, when the server reports it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<RadarCommandTelemetry>,
    /// Host hardware and utilization telemetry.
    pub hardware: RadarHardwareTelemetry,
    /// Local Data Manager storage and product telemetry.
    pub ldm: RadarLdmTelemetry,
    /// Network interface counters and link state.
    pub network: RadarNetworkTelemetry,
}

impl<'de> Deserialize<'de> for RadarServerTelemetry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RadarServerWire::deserialize(deserializer).map(Into::into)
    }
}

/// Radar server ping telemetry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarPingTelemetry {
    /// The ping results grouped by target category.
    pub targets: RadarPingTargets,
    /// The time associated with the ping report.
    pub timestamp: OffsetDateTime,
}

/// Ping reachability by target category.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarPingTargets {
    /// Reachability keyed by client target name.
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub client: BTreeMap<String, bool>,
    /// Reachability keyed by Local Data Manager target name.
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub ldm: BTreeMap<String, bool>,
    /// Reachability keyed by radar target name.
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub radar: BTreeMap<String, bool>,
    /// Reachability keyed by server target name.
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub server: BTreeMap<String, bool>,
    /// Reachability keyed by miscellaneous target name.
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub misc: BTreeMap<String, bool>,
}

/// Count of reachable targets in one ping category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadarPingSummary {
    /// The number of targets reported as reachable.
    pub up: usize,
    /// The total number of targets in the category.
    pub total: usize,
}

impl RadarPingTargets {
    /// Returns reachability counts for client targets.
    #[must_use]
    pub fn client_summary(&self) -> RadarPingSummary {
        ping_summary(&self.client)
    }
    /// Returns reachability counts for Local Data Manager targets.
    #[must_use]
    pub fn ldm_summary(&self) -> RadarPingSummary {
        ping_summary(&self.ldm)
    }
    /// Returns reachability counts for radar targets.
    #[must_use]
    pub fn radar_summary(&self) -> RadarPingSummary {
        ping_summary(&self.radar)
    }
    /// Returns reachability counts for server targets.
    #[must_use]
    pub fn server_summary(&self) -> RadarPingSummary {
        ping_summary(&self.server)
    }
    /// Returns reachability counts for miscellaneous targets.
    #[must_use]
    pub fn misc_summary(&self) -> RadarPingSummary {
        ping_summary(&self.misc)
    }
}

/// Command activity on one radar server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarCommandTelemetry {
    /// The most recent command executed by the server.
    pub last_executed: String,
    /// The time the most recent command was executed.
    pub last_executed_time: OffsetDateTime,
    /// The timestamp of the latest NEXRAD data known to the command service.
    pub last_nexrad_data_time: OffsetDateTime,
    /// The most recent command received by the server.
    pub last_received: String,
    /// The time the most recent command was received.
    pub last_received_time: OffsetDateTime,
    /// The time associated with the command telemetry report.
    pub timestamp: OffsetDateTime,
}

/// Hardware utilization on one radar server.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarHardwareTelemetry {
    /// The time associated with the hardware report.
    pub timestamp: OffsetDateTime,
    /// The percentage of processor capacity that was idle.
    pub cpu_idle: f64,
    /// The reported input/output utilization.
    pub io_utilization: f64,
    /// The reported disk utilization value.
    pub disk: i32,
    /// The one-minute system load average.
    pub load1: f64,
    /// The five-minute system load average.
    pub load5: f64,
    /// The fifteen-minute system load average.
    pub load15: f64,
    /// The reported memory utilization.
    pub memory: f64,
    /// The time at which the server last started.
    pub uptime: OffsetDateTime,
}

/// Local Data Manager telemetry on one radar server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarLdmTelemetry {
    /// The time associated with the Local Data Manager report.
    pub timestamp: OffsetDateTime,
    /// The creation time of the newest stored radar product.
    pub latest_product: OffsetDateTime,
    /// The creation time of the oldest stored radar product.
    pub oldest_product: OffsetDateTime,
    /// The total storage occupied by radar products, in bytes.
    pub storage_size: u64,
    /// The number of radar products in storage.
    pub count: u64,
    /// Whether the Local Data Manager is active.
    pub active: bool,
}

/// Network telemetry on one radar server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarNetworkTelemetry {
    /// The time associated with the network report.
    pub timestamp: OffsetDateTime,
    /// Telemetry for the server's first reported Ethernet interface.
    pub eth0: RadarNetworkInterfaceTelemetry,
    /// Telemetry for the server's second reported Ethernet interface.
    pub eth1: RadarNetworkInterfaceTelemetry,
}

/// Counters and link state for one network interface.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarNetworkInterfaceTelemetry {
    /// The operating-system name of the network interface.
    pub interface: String,
    /// Whether the network interface is active.
    pub active: bool,
    /// The number of transmitted packets without an error.
    pub trans_no_error: u64,
    /// The number of transmission errors.
    pub trans_error: u64,
    /// The number of transmitted packets dropped.
    pub trans_dropped: u64,
    /// The number of transmission overruns.
    pub trans_overrun: u64,
    /// The number of received packets without an error.
    pub recv_no_error: u64,
    /// The number of receive errors.
    pub recv_error: u64,
    /// The number of received packets dropped.
    pub recv_dropped: u64,
    /// The number of receive overruns.
    pub recv_overrun: u64,
}

/// A collection of radar servers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarServersResponse {
    /// The radar servers returned by NOAA.
    #[serde(rename = "@graph", default)]
    pub servers: Vec<RadarServerTelemetry>,
}

impl RadarServersResponse {
    /// Returns the number of radar servers in the response.
    #[must_use]
    pub fn len(&self) -> usize {
        self.servers.len()
    }
    /// Returns `true` when the response contains no radar servers.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.servers.is_empty()
    }
    /// Returns an iterator over the radar servers in response order.
    pub fn iter(&self) -> impl Iterator<Item = &RadarServerTelemetry> {
        self.servers.iter()
    }
}

/// One product waiting in a radar distribution queue.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarQueue {
    /// The JSON-LD resource type from the `@type` property.
    #[serde(rename = "@type")]
    pub at_type: String,
    /// The Local Data Manager host holding the queued product.
    pub host: String,
    /// The time the product arrived at the queue host.
    pub arrival_time: OffsetDateTime,
    /// The time the radar product was created.
    pub creation_time: OffsetDateTime,
    /// The station that produced the radar product.
    pub station_id: RadarStationId,
    /// The radar product type from the wire `type` property.
    #[serde(rename = "type")]
    pub data_type: String,
    /// The product's originating data feed.
    pub feed: String,
    /// The Level II data resolution version.
    pub resolution_version: i32,
    /// The product sequence identifier reported by the queue.
    pub sequence_number: String,
    /// The product size in bytes.
    pub size: u64,
}

/// A radar distribution queue response.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarQueuesResponse {
    /// The JSON-LD resource identifier from the `@id` property.
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// The queued radar products returned by NOAA.
    #[serde(rename = "@graph", default)]
    pub entries: Vec<RadarQueue>,
}

impl RadarQueuesResponse {
    /// Returns the number of queued radar products in the response.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` when the response contains no queued radar products.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Returns an iterator over queued products in response order.
    pub fn iter(&self) -> impl Iterator<Item = &RadarQueue> {
        self.entries.iter()
    }
}

/// One radar-station alarm.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarStationAlarm {
    /// The JSON-LD resource type from the `@type` property.
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// The station associated with the alarm.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station_id: Option<RadarStationId>,
    /// The alarm status reported by the station.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The time associated with the alarm report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<OffsetDateTime>,
    /// The command channel that was active when the alarm was reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_channel: Option<i32>,
    /// The human-readable alarm message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Radar alarms for one station.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarStationAlarmsResponse {
    /// The JSON-LD resource identifier from the `@id` property.
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// The station alarms returned by NOAA.
    #[serde(rename = "@graph", default)]
    pub alarms: Vec<RadarStationAlarm>,
}

impl RadarStationAlarmsResponse {
    /// Returns the number of station alarms in the response.
    #[must_use]
    pub fn len(&self) -> usize {
        self.alarms.len()
    }
    /// Returns `true` when the response contains no station alarms.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.alarms.is_empty()
    }
    /// Returns an iterator over station alarms in response order.
    pub fn iter(&self) -> impl Iterator<Item = &RadarStationAlarm> {
        self.alarms.iter()
    }
}

/// SPGDS telemetry for all reporting hosts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarSpgdsResponse {
    /// The SPGDS host reports returned by NOAA.
    #[serde(rename = "@graph", default)]
    pub spgds: Vec<RadarSpgdsEntry>,
}

impl RadarSpgdsResponse {
    /// Returns the number of SPGDS host reports in the response.
    #[must_use]
    pub fn len(&self) -> usize {
        self.spgds.len()
    }
    /// Returns `true` when the response contains no SPGDS host reports.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.spgds.is_empty()
    }
    /// Returns an iterator over SPGDS host reports in response order.
    pub fn iter(&self) -> impl Iterator<Item = &RadarSpgdsEntry> {
        self.spgds.iter()
    }
}

/// Telemetry for one SPGDS host.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsEntry {
    /// The JSON-LD resource type from the `@type` property.
    #[serde(rename = "@type")]
    pub at_type: String,
    /// The SPGDS host identifier.
    pub id: String,
    /// The time associated with the host report.
    pub timestamp: OffsetDateTime,
    /// The radar product data-flow state and its source timestamps.
    pub dataflow: RadarSpgdsStatus,
    /// The connection-queue state and its source timestamps.
    pub connect_q: RadarSpgdsStatus,
    /// The SPGDS application state and its source timestamps.
    pub app_running: RadarSpgdsStatus,
    /// The Local Data Manager connection count and validation time.
    pub ldm: RadarSpgdsLdmStatus,
    /// The secondary disk's state and utilization.
    #[serde(rename = "secondHD")]
    pub second_hd: RadarSpgdsDiskStatus,
    /// The SPGDS host's startup time and validation time.
    #[serde(rename = "spgdsUpSince")]
    pub uptime: RadarSpgdsUptime,
    /// The host's inbound and outbound throughput telemetry.
    pub throughput: RadarSpgdsThroughput,
    /// Gateway telemetry keyed by gateway identifier.
    #[serde(default)]
    pub spg: BTreeMap<String, RadarSpgdsGatewayStatus>,
}

/// SPGDS state plus epoch-second transition and validation values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsStatus {
    /// The state value reported by the SPGDS host.
    pub state: String,
    /// The source-provided epoch-second time at which the state began.
    pub state_since: String,
    /// The source-provided epoch-second time at which the state was validated.
    pub state_valid: String,
}

/// SPGDS Local Data Manager connection telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsLdmStatus {
    /// The Local Data Manager connection count as source text.
    pub conns: String,
    /// The source-provided epoch-second time at which the count was validated.
    pub conns_valid: String,
}

/// SPGDS secondary-disk telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsDiskStatus {
    /// The secondary disk state reported by the SPGDS host.
    pub state: String,
    /// The source-provided epoch-second time at which the disk state began.
    pub state_since: String,
    /// The source-provided epoch-second time at which the disk state was validated.
    pub state_valid: String,
    /// The secondary disk utilization percentage as source text.
    #[serde(rename = "pctUsed")]
    pub percent_used: String,
    /// The source-provided epoch-second time at which utilization was validated.
    #[serde(rename = "pctUsedValid")]
    pub percent_used_valid: String,
}

/// SPGDS host uptime telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsUptime {
    /// The source-provided epoch-second time at which the host started.
    pub up_since: String,
    /// The source-provided epoch-second time at which the startup time was validated.
    pub up_since_valid: String,
}

/// SPGDS inbound and outbound throughput telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarSpgdsThroughput {
    /// The inbound throughput value as source text.
    #[serde(rename = "in")]
    pub inbound: String,
    /// The source-provided epoch-second time associated with inbound throughput.
    #[serde(rename = "inDateTime")]
    pub inbound_date_time: String,
    /// The source-provided epoch-second time at which inbound throughput was validated.
    #[serde(rename = "inValid")]
    pub inbound_valid: String,
    /// The outbound throughput value as source text.
    #[serde(rename = "out")]
    pub outbound: String,
    /// The source-provided epoch-second time associated with outbound throughput.
    #[serde(rename = "outDateTime")]
    pub outbound_date_time: String,
    /// The source-provided epoch-second time at which outbound throughput was validated.
    #[serde(rename = "outValid")]
    pub outbound_valid: String,
}

/// Telemetry for one dynamically named SPGDS gateway.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsGatewayStatus {
    /// The gateway's SWIM data state.
    pub swim_data_state: String,
    /// The source-provided epoch-second time at which the SWIM data state began.
    pub swim_data_state_since: String,
    /// The source-provided epoch-second time at which the SWIM data state was validated.
    pub swim_data_state_valid: String,
    /// The gateway's Local Data Manager ping state.
    pub ldm_ping_state: String,
    /// The source-provided epoch-second time at which the ping state began.
    pub ldm_ping_state_since: String,
    /// The source-provided epoch-second time at which the ping state was validated.
    pub ldm_ping_state_valid: String,
}

#[derive(Deserialize)]
struct RadarStationWire {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    geometry: Option<Geometry>,
    properties: RadarStationDetails,
}

impl From<RadarStationWire> for RadarStationTelemetry {
    fn from(wire: RadarStationWire) -> Self {
        Self {
            feature_id: wire.id,
            geometry: wire.geometry,
            station: wire.properties,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RadarServerWire {
    #[serde(rename = "@id")]
    at_id: String,
    #[serde(rename = "@type")]
    at_type: String,
    id: String,
    #[serde(rename = "type")]
    server_type: String,
    #[serde(default)]
    active: Option<bool>,
    #[serde(default)]
    primary: Option<bool>,
    #[serde(default)]
    aggregate: Option<bool>,
    #[serde(default)]
    locked: Option<bool>,
    #[serde(default)]
    radar_network_up: Option<bool>,
    collection_time: OffsetDateTime,
    reporting_host: String,
    ingest_host: String,
    ping: RadarPingTelemetry,
    #[serde(default)]
    command: Option<RadarCommandTelemetry>,
    hardware: RadarHardwareTelemetry,
    ldm: RadarLdmTelemetry,
    network: RadarNetworkTelemetry,
}

impl From<RadarServerWire> for RadarServerTelemetry {
    fn from(wire: RadarServerWire) -> Self {
        Self {
            at_id: wire.at_id,
            at_type: wire.at_type,
            id: wire.id,
            server_type: wire.server_type,
            active: wire.active,
            primary: wire.primary,
            aggregate: wire.aggregate,
            locked: wire.locked,
            radar_network_up: wire.radar_network_up,
            collection_time: wire.collection_time,
            reporting_host: wire.reporting_host,
            ingest_host: wire.ingest_host,
            ping: wire.ping,
            command: wire.command,
            hardware: wire.hardware,
            ldm: wire.ldm,
            network: wire.network,
        }
    }
}

fn ping_summary(targets: &BTreeMap<String, bool>) -> RadarPingSummary {
    RadarPingSummary {
        up: targets.values().filter(|up| **up).count(),
        total: targets.len(),
    }
}

fn deserialize_map_or_empty_array<'de, D>(
    deserializer: D,
) -> Result<BTreeMap<String, bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MapOrArray {
        Map(BTreeMap<String, bool>),
        Array(Vec<serde::de::IgnoredAny>),
    }

    match MapOrArray::deserialize(deserializer)? {
        MapOrArray::Map(map) => Ok(map),
        MapOrArray::Array(array) if array.is_empty() => Ok(BTreeMap::new()),
        MapOrArray::Array(_) => Err(serde::de::Error::custom(
            "expected an empty array or a target map",
        )),
    }
}

fn deserialize_object_or_empty_array<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ObjectOrArray<T> {
        Array(Vec<serde::de::IgnoredAny>),
        Object(T),
    }

    match Option::<ObjectOrArray<T>>::deserialize(deserializer)? {
        None => Ok(None),
        Some(ObjectOrArray::Object(object)) => Ok(Some(object)),
        Some(ObjectOrArray::Array(array)) if array.is_empty() => Ok(None),
        Some(ObjectOrArray::Array(_)) => Err(serde::de::Error::custom(
            "expected an object or an empty array",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_channel_keeps_numeric_and_named_shapes() {
        let numeric = serde_json::from_str::<CommandChannel>("2").unwrap();
        let named = serde_json::from_str::<CommandChannel>(r#""Single""#).unwrap();
        assert_eq!(serde_json::to_string(&numeric).unwrap(), "2");
        assert_eq!(serde_json::to_string(&named).unwrap(), r#""Single""#);
    }

    #[test]
    fn station_position_comes_from_geojson_geometry() {
        let telemetry: RadarStationTelemetry =
            serde_json::from_str(include_str!("../../tests/fixtures/radar/KFSX.json")).unwrap();
        assert!(matches!(
            telemetry.position(),
            RadarPosition::Coordinates { .. }
        ));
        assert_eq!(telemetry.station.id.as_str(), "KFSX");
    }

    #[test]
    fn empty_tdwr_properties_arrays_become_absent() {
        let telemetry: RadarStationTelemetry =
            serde_json::from_str(include_str!("../../tests/fixtures/radar/TSLC.json")).unwrap();
        assert!(telemetry.station.performance.unwrap().properties.is_none());
        assert!(telemetry.station.adaptation.unwrap().properties.is_none());
    }

    #[test]
    fn empty_ping_array_becomes_an_empty_map() {
        let response: RadarServersResponse =
            serde_json::from_str(include_str!("../../tests/fixtures/radar/servers.json")).unwrap();
        assert!(response.servers[0].ping.targets.radar.is_empty());
    }
}
