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

use super::{QualityControl, Quantity, Unit};
use crate::geo::Geometry;
use crate::ids::RadarStationId;
use crate::time::OffsetDateTime;

/// A radar measurement whose unit may be absent.
///
/// Most NOAA measurements use [`Quantity`], but radar telemetry has been
/// observed to send a numeric latency without `unitCode`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarMeasurement {
    #[serde(default)]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(rename = "unitCode", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
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
    #[serde(rename = "Single")]
    Single,
}

/// Human-oriented geographic state for a radar station.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum RadarPosition {
    Missing,
    Invalid,
    Coordinates { longitude: f64, latitude: f64 },
}

/// One radar station GeoJSON feature, normalized for callers.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarStationTelemetry {
    pub feature_id: Option<String>,
    pub geometry: Option<Geometry>,
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
    #[serde(rename = "@id")]
    pub at_id: String,
    #[serde(rename = "@type")]
    pub at_type: String,
    pub id: RadarStationId,
    pub name: String,
    pub station_type: String,
    pub elevation: RadarMeasurement,
    #[serde(with = "jiff::fmt::serde::tz::required")]
    #[cfg_attr(feature = "schemars", schemars(with = "String"))]
    pub time_zone: TimeZone,
    pub latency: RadarStationLatency,
    pub rda: Option<RadarDataAcquisitionTelemetry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance: Option<RadarPerformanceTelemetry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adaptation: Option<RadarAdaptationTelemetry>,
}

/// Delivery latency for one radar station.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarStationLatency {
    pub current: Option<RadarMeasurement>,
    pub average: Option<RadarMeasurement>,
    #[serde(rename = "max")]
    pub maximum: Option<RadarMeasurement>,
    pub level_two_last_received_time: Option<OffsetDateTime>,
    pub max_latency_time: Option<OffsetDateTime>,
    pub reporting_host: Option<String>,
    pub host: Option<String>,
}

/// Radar Data Acquisition telemetry and provenance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarDataAcquisitionTelemetry {
    pub timestamp: OffsetDateTime,
    pub reporting_host: String,
    pub properties: RadarDataAcquisitionProperties,
}

/// Operational properties reported by a radar data acquisition system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarDataAcquisitionProperties {
    pub resolution_version: Option<i32>,
    pub nl2_path: String,
    pub volume_coverage_pattern: String,
    pub control_status: String,
    pub build_number: f64,
    pub alarm_summary: String,
    pub mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub super_resolution_status: Option<String>,
    pub operability_status: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_transmitter_power: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reflectivity_calibration_correction: Option<RadarMeasurement>,
}

/// Radar performance telemetry and provenance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarPerformanceTelemetry {
    pub timestamp: Option<OffsetDateTime>,
    pub reporting_host: String,
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
    #[serde(
        rename = "ntp_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ntp_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_channel: Option<CommandChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radome_air_temperature: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitional_power_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_short_pulse_noise: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elevation_encoder_light: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_long_pulse_noise: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub azimuth_encoder_light: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_noise_temperature: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linearity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_peak_power: Option<RadarMeasurement>,
    #[serde(
        rename = "horizontalDeltadBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_delta_dbz0: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_recycle_count: Option<i32>,
    #[serde(
        rename = "verticalDeltadBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_delta_dbz0: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receiver_bias: Option<RadarMeasurement>,
    #[serde(
        rename = "shortPulseHorizontaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_pulse_horizontal_dbz0: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_imbalance: Option<RadarMeasurement>,
    #[serde(
        rename = "longPulseHorizontaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_pulse_horizontal_dbz0: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance_check_time: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_leaving_air_temperature: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shelter_temperature: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub power_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_range: Option<RadarMeasurement>,
    #[serde(
        rename = "shortPulseVerticaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub short_pulse_vertical_dbz0: Option<RadarMeasurement>,
    #[serde(
        rename = "longPulseVerticaldBZ0",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub long_pulse_vertical_dbz0: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuel_level: Option<RadarMeasurement>,
}

/// Radar adaptation telemetry and provenance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarAdaptationTelemetry {
    pub timestamp: Option<OffsetDateTime>,
    pub reporting_host: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_frequency: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossWG04Circulator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg04_circulator: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub antenna_gain_including_radome: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_loss_a6_arc_detector: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coho_power_at_a1_j4: Option<RadarMeasurement>,
    #[serde(
        rename = "ameHorzizontalTestSignalPower",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ame_horizontal_test_signal_power: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_loss_transmitter_coupler_coupling: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stalo_power_at_a1_j2: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ame_noise_source_horizontal_excess_noise_ratio: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossVerticalIFHeliaxTo4AT16",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_vertical_if_heliax_to_4at16: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossAT4Attenuator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_at4_attenuator: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossHorzontalIFHeliaxTo4AT17",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_horizontal_if_heliax_to_4at17: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossIFDRIFAntiAliasFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_ifdrif_anti_alias_filter: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossIFDBurstAntiAliasFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_ifd_burst_anti_alias_filter: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossWG02HarmonicFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg02_harmonic_filter: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_power_data_watts_factor: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_loss_waveguide_klystron_to_switch: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulse_width_transmitter_output_short_pulse: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulse_width_transmitter_output_long_pulse: Option<RadarMeasurement>,
    #[serde(
        rename = "pathLossWG06SpectrumFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg06_spectrum_filter: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_receiver_noise_short_pulse: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_receiver_noise_long_pulse: Option<RadarMeasurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transmitter_spectrum_filter_installed: Option<String>,
}

/// A collection of radar station features.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RadarStationsResponse {
    #[serde(rename = "features", default)]
    pub stations: Vec<RadarStationTelemetry>,
}

impl RadarStationsResponse {
    #[must_use]
    pub fn len(&self) -> usize {
        self.stations.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stations.is_empty()
    }
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
    #[serde(rename = "@id")]
    pub at_id: String,
    #[serde(rename = "@type")]
    pub at_type: String,
    pub id: String,
    #[serde(rename = "type")]
    pub server_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radar_network_up: Option<bool>,
    pub collection_time: OffsetDateTime,
    pub reporting_host: String,
    pub ingest_host: String,
    pub ping: RadarPingTelemetry,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<RadarCommandTelemetry>,
    pub hardware: RadarHardwareTelemetry,
    pub ldm: RadarLdmTelemetry,
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
    pub targets: RadarPingTargets,
    pub timestamp: OffsetDateTime,
}

/// Ping reachability by target category.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarPingTargets {
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub client: BTreeMap<String, bool>,
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub ldm: BTreeMap<String, bool>,
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub radar: BTreeMap<String, bool>,
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub server: BTreeMap<String, bool>,
    #[serde(default, deserialize_with = "deserialize_map_or_empty_array")]
    pub misc: BTreeMap<String, bool>,
}

/// Count of reachable targets in one ping category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RadarPingSummary {
    pub up: usize,
    pub total: usize,
}

impl RadarPingTargets {
    #[must_use]
    pub fn client_summary(&self) -> RadarPingSummary {
        ping_summary(&self.client)
    }
    #[must_use]
    pub fn ldm_summary(&self) -> RadarPingSummary {
        ping_summary(&self.ldm)
    }
    #[must_use]
    pub fn radar_summary(&self) -> RadarPingSummary {
        ping_summary(&self.radar)
    }
    #[must_use]
    pub fn server_summary(&self) -> RadarPingSummary {
        ping_summary(&self.server)
    }
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
    pub last_executed: String,
    pub last_executed_time: OffsetDateTime,
    pub last_nexrad_data_time: OffsetDateTime,
    pub last_received: String,
    pub last_received_time: OffsetDateTime,
    pub timestamp: OffsetDateTime,
}

/// Hardware utilization on one radar server.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarHardwareTelemetry {
    pub timestamp: OffsetDateTime,
    pub cpu_idle: f64,
    pub io_utilization: f64,
    pub disk: i32,
    pub load1: f64,
    pub load5: f64,
    pub load15: f64,
    pub memory: f64,
    pub uptime: OffsetDateTime,
}

/// Local Data Manager telemetry on one radar server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarLdmTelemetry {
    pub timestamp: OffsetDateTime,
    pub latest_product: OffsetDateTime,
    pub oldest_product: OffsetDateTime,
    pub storage_size: u64,
    pub count: u64,
    pub active: bool,
}

/// Network telemetry on one radar server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarNetworkTelemetry {
    pub timestamp: OffsetDateTime,
    pub eth0: RadarNetworkInterfaceTelemetry,
    pub eth1: RadarNetworkInterfaceTelemetry,
}

/// Counters and link state for one network interface.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarNetworkInterfaceTelemetry {
    pub interface: String,
    pub active: bool,
    pub trans_no_error: u64,
    pub trans_error: u64,
    pub trans_dropped: u64,
    pub trans_overrun: u64,
    pub recv_no_error: u64,
    pub recv_error: u64,
    pub recv_dropped: u64,
    pub recv_overrun: u64,
}

/// A collection of radar servers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarServersResponse {
    #[serde(rename = "@graph", default)]
    pub servers: Vec<RadarServerTelemetry>,
}

impl RadarServersResponse {
    #[must_use]
    pub fn len(&self) -> usize {
        self.servers.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.servers.is_empty()
    }
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
    #[serde(rename = "@type")]
    pub at_type: String,
    pub host: String,
    pub arrival_time: OffsetDateTime,
    pub creation_time: OffsetDateTime,
    pub station_id: RadarStationId,
    #[serde(rename = "type")]
    pub data_type: String,
    pub feed: String,
    pub resolution_version: i32,
    pub sequence_number: String,
    pub size: u64,
}

/// A radar distribution queue response.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarQueuesResponse {
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@graph", default)]
    pub entries: Vec<RadarQueue>,
}

impl RadarQueuesResponse {
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
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
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub station_id: Option<RadarStationId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_channel: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Radar alarms for one station.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarStationAlarmsResponse {
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@graph", default)]
    pub alarms: Vec<RadarStationAlarm>,
}

impl RadarStationAlarmsResponse {
    #[must_use]
    pub fn len(&self) -> usize {
        self.alarms.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.alarms.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = &RadarStationAlarm> {
        self.alarms.iter()
    }
}

/// SPGDS telemetry for all reporting hosts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarSpgdsResponse {
    #[serde(rename = "@graph", default)]
    pub spgds: Vec<RadarSpgdsEntry>,
}

impl RadarSpgdsResponse {
    #[must_use]
    pub fn len(&self) -> usize {
        self.spgds.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.spgds.is_empty()
    }
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
    #[serde(rename = "@type")]
    pub at_type: String,
    pub id: String,
    pub timestamp: OffsetDateTime,
    pub dataflow: RadarSpgdsStatus,
    pub connect_q: RadarSpgdsStatus,
    pub app_running: RadarSpgdsStatus,
    pub ldm: RadarSpgdsLdmStatus,
    #[serde(rename = "secondHD")]
    pub second_hd: RadarSpgdsDiskStatus,
    #[serde(rename = "spgdsUpSince")]
    pub uptime: RadarSpgdsUptime,
    pub throughput: RadarSpgdsThroughput,
    #[serde(default)]
    pub spg: BTreeMap<String, RadarSpgdsGatewayStatus>,
}

/// SPGDS state plus epoch-second transition and validation values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsStatus {
    pub state: String,
    pub state_since: String,
    pub state_valid: String,
}

/// SPGDS Local Data Manager connection telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsLdmStatus {
    pub conns: String,
    pub conns_valid: String,
}

/// SPGDS secondary-disk telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsDiskStatus {
    pub state: String,
    pub state_since: String,
    pub state_valid: String,
    #[serde(rename = "pctUsed")]
    pub percent_used: String,
    #[serde(rename = "pctUsedValid")]
    pub percent_used_valid: String,
}

/// SPGDS host uptime telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsUptime {
    pub up_since: String,
    pub up_since_valid: String,
}

/// SPGDS inbound and outbound throughput telemetry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct RadarSpgdsThroughput {
    #[serde(rename = "in")]
    pub inbound: String,
    #[serde(rename = "inDateTime")]
    pub inbound_date_time: String,
    #[serde(rename = "inValid")]
    pub inbound_valid: String,
    #[serde(rename = "out")]
    pub outbound: String,
    #[serde(rename = "outDateTime")]
    pub outbound_date_time: String,
    #[serde(rename = "outValid")]
    pub outbound_valid: String,
}

/// Telemetry for one dynamically named SPGDS gateway.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct RadarSpgdsGatewayStatus {
    pub swim_data_state: String,
    pub swim_data_state_since: String,
    pub swim_data_state_valid: String,
    pub ldm_ping_state: String,
    pub ldm_ping_state_since: String,
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
