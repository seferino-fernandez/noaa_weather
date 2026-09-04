//! Normalized meaning for NOAA radar station and server telemetry.
//!
//! The raw radar models remain the serialization interface. These types provide
//! validated, owned meaning for callers that need to interpret radar telemetry.

use std::{collections::HashMap, error, fmt};

use jiff::Timestamp;

use super::radar_server::{
    RadarServer, RadarServerCommandStatus, RadarServerHardwareStatus, RadarServerLdmStatus,
    RadarServerNetworkInterfaceStats, RadarServerNetworkStatus, RadarServerPingStatus,
    RadarServerPingTargets,
};
use super::radar_station::{
    AdaptationInfo, AdaptationProperties, CommandChannel, LatencyInfo, PerformanceInfo,
    PerformanceProperties, RadarStation, RadarStationFeature, RdaInfo, RdaProperties,
};
use super::{UnitCodeType, ValueUnit};

macro_rules! string_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub fn $method(&self) -> Option<&str> {
            self.$field.as_deref()
        }
    )+};
}

macro_rules! measurement_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub const fn $method(&self) -> Option<&RadarMeasurement> {
            self.$field.as_ref()
        }
    )+};
}

macro_rules! timestamp_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub const fn $method(&self) -> Option<Timestamp> {
            self.$field
        }
    )+};
}

macro_rules! bool_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub const fn $method(&self) -> Option<bool> {
            self.$field
        }
    )+};
}

macro_rules! f64_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub const fn $method(&self) -> Option<f64> {
            self.$field
        }
    )+};
}

macro_rules! i64_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub const fn $method(&self) -> Option<i64> {
            self.$field
        }
    )+};
}

macro_rules! i32_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub const fn $method(&self) -> Option<i32> {
            self.$field
        }
    )+};
}

macro_rules! ping_accessors {
    ($($method:ident => $field:ident),+ $(,)?) => {$(
        #[must_use]
        pub const fn $method(&self) -> Option<RadarPingSummary> {
            self.$field
        }
    )+};
}

/// Identifies the kind of telemetry being normalized.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum RadarTelemetryKind {
    /// Radar station telemetry.
    Station,
    /// Radar server telemetry.
    Server,
}

impl fmt::Display for RadarTelemetryKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Station => "radar station telemetry",
            Self::Server => "radar server telemetry",
        })
    }
}

/// A radar fact could not be normalized into its promised semantic type.
#[derive(Debug)]
#[non_exhaustive]
pub struct RadarNormalizationError {
    telemetry: RadarTelemetryKind,
    field: &'static str,
    source: jiff::Error,
}

impl RadarNormalizationError {
    /// Kind of radar telemetry that failed normalization.
    #[must_use]
    pub const fn telemetry(&self) -> RadarTelemetryKind {
        self.telemetry
    }

    /// Semantic field that failed normalization.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl fmt::Display for RadarNormalizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid {} field {}: {}",
            self.telemetry, self.field, self.source
        )
    }
}

impl error::Error for RadarNormalizationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.source)
    }
}

/// A radar measurement whose value and unit presence remain distinct.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarMeasurement {
    value: Option<f64>,
    unit: Option<UnitCodeType>,
    maximum: Option<f64>,
    minimum: Option<Option<f64>>,
    quality_control: Option<Box<str>>,
}

impl RadarMeasurement {
    /// Numeric value, when NOAA supplied one.
    #[must_use]
    pub const fn value(&self) -> Option<f64> {
        self.value
    }

    /// Unit meaning, when NOAA supplied one.
    #[must_use]
    pub const fn unit(&self) -> Option<&UnitCodeType> {
        self.unit.as_ref()
    }

    /// Maximum value of a reported range, when present.
    #[must_use]
    pub const fn maximum(&self) -> Option<f64> {
        self.maximum
    }

    /// Minimum range state. `Some(None)` retains an explicit NOAA null.
    #[must_use]
    pub const fn minimum(&self) -> Option<Option<f64>> {
        self.minimum
    }

    /// Quality-control fact, when present.
    #[must_use]
    pub fn quality_control(&self) -> Option<&str> {
        self.quality_control.as_deref()
    }
}

impl From<&ValueUnit> for RadarMeasurement {
    fn from(value: &ValueUnit) -> Self {
        Self {
            value: value.value,
            unit: value.unit_code.clone(),
            maximum: value.max_value,
            minimum: value.min_value,
            quality_control: value.quality_control.as_deref().map(Into::into),
        }
    }
}

/// Geographic state carried by radar station telemetry.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum RadarPosition {
    /// NOAA omitted the geometry or its coordinates.
    Missing,
    /// NOAA supplied fewer than two coordinates.
    Invalid,
    /// Longitude and latitude from the first two coordinates.
    Coordinates {
        /// Longitude in degrees.
        longitude: f64,
        /// Latitude in degrees.
        latitude: f64,
    },
}

/// Normalized meaning for one NOAA radar station record.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarStationTelemetry {
    feature_id: Option<Box<str>>,
    position: RadarPosition,
    station: Option<RadarStationDetails>,
}

impl RadarStationTelemetry {
    /// Feature identifier supplied by NOAA.
    #[must_use]
    pub fn feature_id(&self) -> Option<&str> {
        self.feature_id.as_deref()
    }

    /// Station position state.
    #[must_use]
    pub const fn position(&self) -> RadarPosition {
        self.position
    }

    /// Detailed station meaning, when present.
    #[must_use]
    pub const fn station(&self) -> Option<&RadarStationDetails> {
        self.station.as_ref()
    }
}

/// General and operational meaning for a radar station.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarStationDetails {
    resource_id: Option<Box<str>>,
    type_identifier: Option<Box<str>>,
    id: Option<Box<str>>,
    name: Option<Box<str>>,
    station_type: Option<Box<str>>,
    elevation: Option<RadarMeasurement>,
    time_zone: Option<Box<str>>,
    latency: Option<RadarStationLatency>,
    rda: Option<RadarDataAcquisitionTelemetry>,
    performance: Option<RadarPerformanceTelemetry>,
    adaptation: Option<RadarAdaptationTelemetry>,
}

impl RadarStationDetails {
    string_accessors! {
        resource_id => resource_id,
        type_identifier => type_identifier,
        id => id,
        name => name,
        station_type => station_type,
        time_zone => time_zone,
    }

    /// Station elevation.
    #[must_use]
    pub const fn elevation(&self) -> Option<&RadarMeasurement> {
        self.elevation.as_ref()
    }

    /// Latency telemetry, when present.
    #[must_use]
    pub const fn latency(&self) -> Option<&RadarStationLatency> {
        self.latency.as_ref()
    }

    /// Radar Data Acquisition telemetry, when present.
    #[must_use]
    pub const fn rda(&self) -> Option<&RadarDataAcquisitionTelemetry> {
        self.rda.as_ref()
    }

    /// Performance telemetry, when present.
    #[must_use]
    pub const fn performance(&self) -> Option<&RadarPerformanceTelemetry> {
        self.performance.as_ref()
    }

    /// Adaptation telemetry, when present.
    #[must_use]
    pub const fn adaptation(&self) -> Option<&RadarAdaptationTelemetry> {
        self.adaptation.as_ref()
    }
}

/// Radar station latency meaning.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarStationLatency {
    current: Option<RadarMeasurement>,
    average: Option<RadarMeasurement>,
    maximum: Option<RadarMeasurement>,
    level_two_last_received: Option<Timestamp>,
    maximum_at: Option<Timestamp>,
    reporting_host: Option<Box<str>>,
    data_host: Option<Box<str>>,
}

impl RadarStationLatency {
    measurement_accessors! {
        current => current,
        average => average,
        maximum => maximum,
    }
    timestamp_accessors! {
        level_two_last_received => level_two_last_received,
        maximum_at => maximum_at,
    }
    string_accessors! {
        reporting_host => reporting_host,
        data_host => data_host,
    }
}

/// Radar Data Acquisition telemetry.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarDataAcquisitionTelemetry {
    timestamp: Option<Timestamp>,
    reporting_host: Option<Box<str>>,
    properties: Option<RadarDataAcquisitionProperties>,
}

impl RadarDataAcquisitionTelemetry {
    timestamp_accessors! { timestamp => timestamp }
    string_accessors! { reporting_host => reporting_host }

    /// RDA properties, when present.
    #[must_use]
    pub const fn properties(&self) -> Option<&RadarDataAcquisitionProperties> {
        self.properties.as_ref()
    }
}

/// Operational properties reported by a radar data acquisition system.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarDataAcquisitionProperties {
    resolution_version: Option<i32>,
    nl2_path: Option<Box<str>>,
    volume_coverage_pattern: Option<Box<str>>,
    control_status: Option<Box<str>>,
    build_number: Option<f64>,
    alarm_summary: Option<Box<str>>,
    mode: Option<Box<str>>,
    generator_state: Option<Box<str>>,
    super_resolution_status: Option<Box<str>>,
    operability_status: Option<Box<str>>,
    status: Option<Box<str>>,
    average_transmitter_power: Option<RadarMeasurement>,
    reflectivity_calibration_correction: Option<RadarMeasurement>,
}

impl RadarDataAcquisitionProperties {
    string_accessors! {
        nl2_path => nl2_path,
        volume_coverage_pattern => volume_coverage_pattern,
        control_status => control_status,
        alarm_summary => alarm_summary,
        mode => mode,
        generator_state => generator_state,
        super_resolution_status => super_resolution_status,
        operability_status => operability_status,
        status => status,
    }
    i32_accessors! { resolution_version => resolution_version }
    measurement_accessors! {
        average_transmitter_power => average_transmitter_power,
        reflectivity_calibration_correction => reflectivity_calibration_correction,
    }

    /// RDA build number.
    #[must_use]
    pub const fn build_number(&self) -> Option<f64> {
        self.build_number
    }
}

/// Radar station performance telemetry.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarPerformanceTelemetry {
    timestamp: Option<Timestamp>,
    reporting_host: Option<Box<str>>,
    properties: Option<RadarPerformanceProperties>,
}

impl RadarPerformanceTelemetry {
    timestamp_accessors! { timestamp => timestamp }
    string_accessors! { reporting_host => reporting_host }

    /// Performance properties, when present.
    #[must_use]
    pub const fn properties(&self) -> Option<&RadarPerformanceProperties> {
        self.properties.as_ref()
    }
}

/// Performance facts used by the default radar presentation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarPerformanceProperties {
    ntp_status: Option<i32>,
    command_channel: Option<CommandChannel>,
    transitional_power_source: Option<Box<str>>,
    horizontal_short_pulse_noise: Option<RadarMeasurement>,
    elevation_encoder_light: Option<Box<str>>,
    horizontal_long_pulse_noise: Option<RadarMeasurement>,
    azimuth_encoder_light: Option<Box<str>>,
    horizontal_noise_temperature: Option<RadarMeasurement>,
    linearity: Option<f64>,
    transmitter_peak_power: Option<RadarMeasurement>,
    horizontal_deltad_bz0: Option<RadarMeasurement>,
    transmitter_recycle_count: Option<i32>,
    vertical_deltad_bz0: Option<RadarMeasurement>,
    receiver_bias: Option<RadarMeasurement>,
    short_pulse_horizontal_dbz0: Option<RadarMeasurement>,
    transmitter_imbalance: Option<RadarMeasurement>,
    long_pulse_horizontal_dbz0: Option<RadarMeasurement>,
    performance_check_time: Option<Timestamp>,
    transmitter_leaving_air_temperature: Option<RadarMeasurement>,
    shelter_temperature: Option<RadarMeasurement>,
    radome_air_temperature: Option<RadarMeasurement>,
    power_source: Option<Box<str>>,
    dynamic_range: Option<RadarMeasurement>,
    fuel_level: Option<RadarMeasurement>,
}

impl RadarPerformanceProperties {
    string_accessors! {
        transitional_power_source => transitional_power_source,
        elevation_encoder_light => elevation_encoder_light,
        azimuth_encoder_light => azimuth_encoder_light,
        power_source => power_source,
    }
    measurement_accessors! {
        horizontal_short_pulse_noise => horizontal_short_pulse_noise,
        horizontal_long_pulse_noise => horizontal_long_pulse_noise,
        horizontal_noise_temperature => horizontal_noise_temperature,
        horizontal_deltad_bz0 => horizontal_deltad_bz0,
        vertical_deltad_bz0 => vertical_deltad_bz0,
        receiver_bias => receiver_bias,
        short_pulse_horizontal_dbz0 => short_pulse_horizontal_dbz0,
        transmitter_imbalance => transmitter_imbalance,
        long_pulse_horizontal_dbz0 => long_pulse_horizontal_dbz0,
        transmitter_leaving_air_temperature => transmitter_leaving_air_temperature,
        fuel_level => fuel_level,
        shelter_temperature => shelter_temperature,
        radome_air_temperature => radome_air_temperature,
        transmitter_peak_power => transmitter_peak_power,
        dynamic_range => dynamic_range,
    }
    timestamp_accessors! { performance_check_time => performance_check_time }

    /// Command-channel mode or number; NOAA sent `"Single"` for 137 and `1` or `2`
    /// for 21 of 159 WSR-88D stations measured on 2026-09-03.
    #[must_use]
    pub const fn command_channel(&self) -> Option<&CommandChannel> {
        self.command_channel.as_ref()
    }

    /// NTP status code.
    #[must_use]
    pub const fn ntp_status(&self) -> Option<i32> {
        self.ntp_status
    }

    /// Transmitter recycle count.
    #[must_use]
    pub const fn transmitter_recycle_count(&self) -> Option<i32> {
        self.transmitter_recycle_count
    }

    /// Reported linearity.
    #[must_use]
    pub const fn linearity(&self) -> Option<f64> {
        self.linearity
    }
}

/// Radar station adaptation telemetry.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarAdaptationTelemetry {
    timestamp: Option<Timestamp>,
    reporting_host: Option<Box<str>>,
    properties: Option<RadarAdaptationProperties>,
}

impl RadarAdaptationTelemetry {
    timestamp_accessors! { timestamp => timestamp }
    string_accessors! { reporting_host => reporting_host }

    /// Adaptation properties, when present.
    #[must_use]
    pub const fn properties(&self) -> Option<&RadarAdaptationProperties> {
        self.properties.as_ref()
    }
}

/// Adaptation facts used by the default radar presentation.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarAdaptationProperties {
    transmitter_frequency: Option<RadarMeasurement>,
    path_loss_wg04_circulator: Option<RadarMeasurement>,
    antenna_gain_including_radome: Option<RadarMeasurement>,
    path_loss_a6_arc_detector: Option<RadarMeasurement>,
    coho_power_at_a1j4: Option<RadarMeasurement>,
    ame_horizontal_test_signal_power: Option<RadarMeasurement>,
    path_loss_transmitter_coupler_coupling: Option<RadarMeasurement>,
    stalo_power_at_a1j2: Option<RadarMeasurement>,
    ame_noise_source_horizontal_excess_noise_ratio: Option<RadarMeasurement>,
    path_loss_vertical_if_heliax_to_4at16: Option<RadarMeasurement>,
    path_loss_at4_attenuator: Option<RadarMeasurement>,
    path_loss_horizontal_if_heliax_to_4at17: Option<RadarMeasurement>,
    path_loss_ifdrif_anti_alias_filter: Option<RadarMeasurement>,
    path_loss_ifd_burst_anti_alias_filter: Option<RadarMeasurement>,
    path_loss_wg02_harmonic_filter: Option<RadarMeasurement>,
    transmitter_power_data_watts_factor: Option<RadarMeasurement>,
    path_loss_waveguide_klystron_to_switch: Option<RadarMeasurement>,
    pulse_width_transmitter_output_short_pulse: Option<RadarMeasurement>,
    pulse_width_transmitter_output_long_pulse: Option<RadarMeasurement>,
    path_loss_wg06_spectrum_filter: Option<RadarMeasurement>,
    horizontal_receiver_noise_short_pulse: Option<RadarMeasurement>,
    horizontal_receiver_noise_long_pulse: Option<RadarMeasurement>,
    transmitter_spectrum_filter_installed: Option<Box<str>>,
}

impl RadarAdaptationProperties {
    measurement_accessors! {
        transmitter_frequency => transmitter_frequency,
        path_loss_wg04_circulator => path_loss_wg04_circulator,
        antenna_gain_including_radome => antenna_gain_including_radome,
        path_loss_a6_arc_detector => path_loss_a6_arc_detector,
        coho_power_at_a1j4 => coho_power_at_a1j4,
        ame_horizontal_test_signal_power => ame_horizontal_test_signal_power,
        path_loss_transmitter_coupler_coupling => path_loss_transmitter_coupler_coupling,
        stalo_power_at_a1j2 => stalo_power_at_a1j2,
        ame_noise_source_horizontal_excess_noise_ratio => ame_noise_source_horizontal_excess_noise_ratio,
        path_loss_vertical_if_heliax_to_4at16 => path_loss_vertical_if_heliax_to_4at16,
        path_loss_at4_attenuator => path_loss_at4_attenuator,
        path_loss_horizontal_if_heliax_to_4at17 => path_loss_horizontal_if_heliax_to_4at17,
        path_loss_ifdrif_anti_alias_filter => path_loss_ifdrif_anti_alias_filter,
        path_loss_ifd_burst_anti_alias_filter => path_loss_ifd_burst_anti_alias_filter,
        path_loss_wg02_harmonic_filter => path_loss_wg02_harmonic_filter,
        transmitter_power_data_watts_factor => transmitter_power_data_watts_factor,
        path_loss_waveguide_klystron_to_switch => path_loss_waveguide_klystron_to_switch,
        pulse_width_transmitter_output_short_pulse => pulse_width_transmitter_output_short_pulse,
        pulse_width_transmitter_output_long_pulse => pulse_width_transmitter_output_long_pulse,
        path_loss_wg06_spectrum_filter => path_loss_wg06_spectrum_filter,
        horizontal_receiver_noise_short_pulse => horizontal_receiver_noise_short_pulse,
        horizontal_receiver_noise_long_pulse => horizontal_receiver_noise_long_pulse,
    }
    string_accessors! {
        transmitter_spectrum_filter_installed => transmitter_spectrum_filter_installed,
    }
}

/// Normalized meaning for one NOAA radar server record.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarServerTelemetry {
    resource_id: Option<Box<str>>,
    type_identifier: Option<Box<str>>,
    id: Option<Box<str>>,
    server_type: Option<Box<str>>,
    active: Option<bool>,
    primary: Option<bool>,
    aggregate: Option<bool>,
    locked: Option<bool>,
    radar_network_up: Option<bool>,
    collection_time: Option<Timestamp>,
    reporting_host: Option<Box<str>>,
    ping: Option<RadarPingTelemetry>,
    command: Option<RadarCommandTelemetry>,
    hardware: Option<RadarHardwareTelemetry>,
    ldm: Option<RadarLdmTelemetry>,
    network: Option<RadarNetworkTelemetry>,
}

impl RadarServerTelemetry {
    string_accessors! {
        resource_id => resource_id,
        type_identifier => type_identifier,
        id => id,
        server_type => server_type,
        reporting_host => reporting_host,
    }
    bool_accessors! {
        active => active,
        primary => primary,
        aggregate => aggregate,
        locked => locked,
        radar_network_up => radar_network_up,
    }
    timestamp_accessors! { collection_time => collection_time }

    /// Ping telemetry, when present.
    #[must_use]
    pub const fn ping(&self) -> Option<&RadarPingTelemetry> {
        self.ping.as_ref()
    }

    /// Command telemetry, when present.
    #[must_use]
    pub const fn command(&self) -> Option<&RadarCommandTelemetry> {
        self.command.as_ref()
    }

    /// Hardware telemetry, when present.
    #[must_use]
    pub const fn hardware(&self) -> Option<&RadarHardwareTelemetry> {
        self.hardware.as_ref()
    }

    /// Local Data Manager telemetry, when present.
    #[must_use]
    pub const fn ldm(&self) -> Option<&RadarLdmTelemetry> {
        self.ldm.as_ref()
    }

    /// Network telemetry, when present.
    #[must_use]
    pub const fn network(&self) -> Option<&RadarNetworkTelemetry> {
        self.network.as_ref()
    }
}

/// Per-category radar server ping meaning.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarPingTelemetry {
    timestamp: Option<Timestamp>,
    targets: Option<RadarPingTargets>,
}

impl RadarPingTelemetry {
    timestamp_accessors! { timestamp => timestamp }

    /// Target-category summaries, when present.
    #[must_use]
    pub const fn targets(&self) -> Option<&RadarPingTargets> {
        self.targets.as_ref()
    }
}

/// Ping summaries for each NOAA radar target category.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarPingTargets {
    client: Option<RadarPingSummary>,
    ldm: Option<RadarPingSummary>,
    radar: Option<RadarPingSummary>,
    server: Option<RadarPingSummary>,
    misc: Option<RadarPingSummary>,
}

impl RadarPingTargets {
    ping_accessors! {
        client => client,
        ldm => ldm,
        radar => radar,
        server => server,
        misc => misc,
    }
}

/// Count of reachable targets within one radar ping category.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct RadarPingSummary {
    up: usize,
    total: usize,
}

impl RadarPingSummary {
    /// Number of reachable targets.
    #[must_use]
    pub const fn up(self) -> usize {
        self.up
    }

    /// Total number of targets.
    #[must_use]
    pub const fn total(self) -> usize {
        self.total
    }
}

/// Radar server command telemetry.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarCommandTelemetry {
    timestamp: Option<Timestamp>,
    last_executed: Option<Box<str>>,
    last_executed_time: Option<Timestamp>,
    last_nexrad_data_time: Option<Timestamp>,
    last_received: Option<Box<str>>,
    last_received_time: Option<Timestamp>,
}

impl RadarCommandTelemetry {
    timestamp_accessors! {
        timestamp => timestamp,
        last_executed_time => last_executed_time,
        last_nexrad_data_time => last_nexrad_data_time,
        last_received_time => last_received_time,
    }
    string_accessors! {
        last_executed => last_executed,
        last_received => last_received,
    }
}

/// Radar server hardware telemetry.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarHardwareTelemetry {
    timestamp: Option<Timestamp>,
    cpu_idle: Option<f64>,
    io_utilization: Option<f64>,
    disk: Option<i32>,
    load1: Option<f64>,
    load5: Option<f64>,
    load15: Option<f64>,
    memory: Option<f64>,
    uptime: Option<Timestamp>,
}

impl RadarHardwareTelemetry {
    timestamp_accessors! { timestamp => timestamp, uptime => uptime }
    f64_accessors! {
        cpu_idle => cpu_idle,
        io_utilization => io_utilization,
        load1 => load1,
        load5 => load5,
        load15 => load15,
        memory => memory,
    }

    /// Disk status/value.
    #[must_use]
    pub const fn disk(&self) -> Option<i32> {
        self.disk
    }
}

/// Radar server Local Data Manager telemetry.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarLdmTelemetry {
    timestamp: Option<Timestamp>,
    active: Option<bool>,
    latest_product: Option<Timestamp>,
    oldest_product: Option<Timestamp>,
    storage_size: Option<i64>,
    count: Option<i32>,
}

impl RadarLdmTelemetry {
    timestamp_accessors! {
        timestamp => timestamp,
        latest_product => latest_product,
        oldest_product => oldest_product,
    }
    bool_accessors! { active => active }

    /// Storage size in bytes.
    #[must_use]
    pub const fn storage_size(&self) -> Option<i64> {
        self.storage_size
    }

    /// Product count.
    #[must_use]
    pub const fn count(&self) -> Option<i32> {
        self.count
    }
}

/// Radar server network telemetry.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarNetworkTelemetry {
    timestamp: Option<Timestamp>,
    eth0: Option<RadarNetworkInterfaceTelemetry>,
    eth1: Option<RadarNetworkInterfaceTelemetry>,
}

impl RadarNetworkTelemetry {
    timestamp_accessors! { timestamp => timestamp }

    /// `eth0` telemetry, when present.
    #[must_use]
    pub const fn eth0(&self) -> Option<&RadarNetworkInterfaceTelemetry> {
        self.eth0.as_ref()
    }

    /// `eth1` telemetry, when present.
    #[must_use]
    pub const fn eth1(&self) -> Option<&RadarNetworkInterfaceTelemetry> {
        self.eth1.as_ref()
    }
}

/// Packet counters and state for one radar server network interface.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RadarNetworkInterfaceTelemetry {
    interface: Option<Box<str>>,
    active: Option<bool>,
    transmitted_ok: Option<i64>,
    transmitted_errors: Option<i64>,
    transmitted_dropped: Option<i64>,
    transmitted_overruns: Option<i64>,
    received_ok: Option<i64>,
    received_errors: Option<i64>,
    received_dropped: Option<i64>,
    received_overruns: Option<i64>,
}

impl RadarNetworkInterfaceTelemetry {
    string_accessors! { interface => interface }
    bool_accessors! { active => active }
    i64_accessors! {
        transmitted_ok => transmitted_ok,
        transmitted_errors => transmitted_errors,
        transmitted_dropped => transmitted_dropped,
        transmitted_overruns => transmitted_overruns,
        received_ok => received_ok,
        received_errors => received_errors,
        received_dropped => received_dropped,
        received_overruns => received_overruns,
    }
}

impl TryFrom<&RadarStationFeature> for RadarStationTelemetry {
    type Error = RadarNormalizationError;

    fn try_from(raw: &RadarStationFeature) -> Result<Self, Self::Error> {
        Ok(Self {
            feature_id: boxed(&raw.id),
            position: normalize_position(raw),
            station: raw
                .radar_station
                .as_ref()
                .map(normalize_station)
                .transpose()?,
        })
    }
}

impl TryFrom<&RadarServer> for RadarServerTelemetry {
    type Error = RadarNormalizationError;

    fn try_from(raw: &RadarServer) -> Result<Self, Self::Error> {
        Ok(Self {
            resource_id: boxed(&raw.at_id),
            type_identifier: boxed(&raw.at_type),
            id: boxed(&raw.id),
            server_type: boxed(&raw.r#type),
            active: raw.active,
            primary: raw.primary,
            aggregate: raw.aggregate,
            locked: raw.locked,
            radar_network_up: raw.radar_network_up,
            collection_time: timestamp(
                RadarTelemetryKind::Server,
                "collection_time",
                raw.collection_time.as_deref(),
            )?,
            reporting_host: boxed(&raw.reporting_host),
            ping: raw.ping.as_ref().map(normalize_ping).transpose()?,
            command: raw.command.as_ref().map(normalize_command).transpose()?,
            hardware: raw.hardware.as_ref().map(normalize_hardware).transpose()?,
            ldm: raw.ldm.as_ref().map(normalize_ldm).transpose()?,
            network: raw.network.as_ref().map(normalize_network).transpose()?,
        })
    }
}

fn normalize_position(raw: &RadarStationFeature) -> RadarPosition {
    let Some(coordinates) = raw
        .geometry
        .as_ref()
        .and_then(|geometry| geometry.coordinates.as_ref())
    else {
        return RadarPosition::Missing;
    };
    match coordinates.as_slice() {
        [longitude, latitude, ..] => RadarPosition::Coordinates {
            longitude: *longitude,
            latitude: *latitude,
        },
        _ => RadarPosition::Invalid,
    }
}

fn normalize_station(raw: &RadarStation) -> Result<RadarStationDetails, RadarNormalizationError> {
    Ok(RadarStationDetails {
        resource_id: boxed(&raw.at_id),
        type_identifier: boxed(&raw.at_type),
        id: boxed(&raw.id),
        name: boxed(&raw.name),
        station_type: boxed(&raw.station_type),
        elevation: measurement(&raw.elevation),
        time_zone: boxed(&raw.time_zone),
        latency: raw.latency.as_ref().map(normalize_latency).transpose()?,
        rda: raw.rda.as_ref().map(normalize_rda).transpose()?,
        performance: raw
            .performance
            .as_ref()
            .map(normalize_performance)
            .transpose()?,
        adaptation: raw
            .adaptation
            .as_ref()
            .map(normalize_adaptation)
            .transpose()?,
    })
}

fn normalize_latency(raw: &LatencyInfo) -> Result<RadarStationLatency, RadarNormalizationError> {
    Ok(RadarStationLatency {
        current: measurement(&raw.current),
        average: measurement(&raw.average),
        maximum: measurement(&raw.max),
        level_two_last_received: timestamp(
            RadarTelemetryKind::Station,
            "latency.level_two_last_received",
            raw.level_two_last_received_time.as_deref(),
        )?,
        maximum_at: timestamp(
            RadarTelemetryKind::Station,
            "latency.maximum_at",
            raw.max_latency_time.as_deref(),
        )?,
        reporting_host: boxed(&raw.reporting_host),
        data_host: boxed(&raw.host),
    })
}

fn normalize_rda(raw: &RdaInfo) -> Result<RadarDataAcquisitionTelemetry, RadarNormalizationError> {
    Ok(RadarDataAcquisitionTelemetry {
        timestamp: timestamp(
            RadarTelemetryKind::Station,
            "rda.timestamp",
            raw.timestamp.as_deref(),
        )?,
        reporting_host: boxed(&raw.reporting_host),
        properties: raw.properties.as_ref().map(normalize_rda_properties),
    })
}

fn normalize_rda_properties(raw: &RdaProperties) -> RadarDataAcquisitionProperties {
    RadarDataAcquisitionProperties {
        resolution_version: raw.resolution_version,
        nl2_path: boxed(&raw.nl2_path),
        volume_coverage_pattern: boxed(&raw.volume_coverage_pattern),
        control_status: boxed(&raw.control_status),
        build_number: raw.build_number,
        alarm_summary: boxed(&raw.alarm_summary),
        mode: boxed(&raw.mode),
        generator_state: boxed(&raw.generator_state),
        super_resolution_status: boxed(&raw.super_resolution_status),
        operability_status: boxed(&raw.operability_status),
        status: boxed(&raw.status),
        average_transmitter_power: measurement(&raw.average_transmitter_power),
        reflectivity_calibration_correction: measurement(&raw.reflectivity_calibration_correction),
    }
}

fn normalize_performance(
    raw: &PerformanceInfo,
) -> Result<RadarPerformanceTelemetry, RadarNormalizationError> {
    Ok(RadarPerformanceTelemetry {
        timestamp: timestamp(
            RadarTelemetryKind::Station,
            "performance.timestamp",
            raw.timestamp.as_deref(),
        )?,
        reporting_host: boxed(&raw.reporting_host),
        properties: raw
            .properties
            .as_ref()
            .map(normalize_performance_properties)
            .transpose()?,
    })
}

fn normalize_performance_properties(
    raw: &PerformanceProperties,
) -> Result<RadarPerformanceProperties, RadarNormalizationError> {
    Ok(RadarPerformanceProperties {
        ntp_status: raw.ntp_status,
        command_channel: raw.command_channel.clone(),
        transitional_power_source: boxed(&raw.transitional_power_source),
        horizontal_short_pulse_noise: measurement(&raw.horizontal_short_pulse_noise),
        elevation_encoder_light: boxed(&raw.elevation_encoder_light),
        horizontal_long_pulse_noise: measurement(&raw.horizontal_long_pulse_noise),
        azimuth_encoder_light: boxed(&raw.azimuth_encoder_light),
        horizontal_noise_temperature: measurement(&raw.horizontal_noise_temperature),
        linearity: raw.linearity,
        transmitter_peak_power: measurement(&raw.transmitter_peak_power),
        horizontal_deltad_bz0: measurement(&raw.horizontal_deltad_bz0),
        transmitter_recycle_count: raw.transmitter_recycle_count,
        vertical_deltad_bz0: measurement(&raw.vertical_deltad_bz0),
        receiver_bias: measurement(&raw.receiver_bias),
        short_pulse_horizontal_dbz0: measurement(&raw.short_pulse_horizontal_dbz0),
        transmitter_imbalance: measurement(&raw.transmitter_imbalance),
        long_pulse_horizontal_dbz0: measurement(&raw.long_pulse_horizontal_dbz0),
        performance_check_time: timestamp(
            RadarTelemetryKind::Station,
            "performance.check_time",
            raw.performance_check_time.as_deref(),
        )?,
        transmitter_leaving_air_temperature: measurement(&raw.transmitter_leaving_air_temperature),
        shelter_temperature: measurement(&raw.shelter_temperature),
        radome_air_temperature: measurement(&raw.radome_air_temperature),
        power_source: boxed(&raw.power_source),
        dynamic_range: measurement(&raw.dynamic_range),
        fuel_level: measurement(&raw.fuel_level),
    })
}

fn normalize_adaptation(
    raw: &AdaptationInfo,
) -> Result<RadarAdaptationTelemetry, RadarNormalizationError> {
    Ok(RadarAdaptationTelemetry {
        timestamp: timestamp(
            RadarTelemetryKind::Station,
            "adaptation.timestamp",
            raw.timestamp.as_deref(),
        )?,
        reporting_host: boxed(&raw.reporting_host),
        properties: raw.properties.as_ref().map(normalize_adaptation_properties),
    })
}

fn normalize_adaptation_properties(raw: &AdaptationProperties) -> RadarAdaptationProperties {
    RadarAdaptationProperties {
        transmitter_frequency: measurement(&raw.transmitter_frequency),
        path_loss_wg04_circulator: measurement(&raw.path_loss_wg04_circulator),
        antenna_gain_including_radome: measurement(&raw.antenna_gain_including_radome),
        path_loss_a6_arc_detector: measurement(&raw.path_loss_a6_arc_detector),
        coho_power_at_a1j4: measurement(&raw.coho_power_at_a1j4),
        ame_horizontal_test_signal_power: measurement(&raw.ame_horizontal_test_signal_power),
        path_loss_transmitter_coupler_coupling: measurement(
            &raw.path_loss_transmitter_coupler_coupling,
        ),
        stalo_power_at_a1j2: measurement(&raw.stalo_power_at_a1j2),
        ame_noise_source_horizontal_excess_noise_ratio: measurement(
            &raw.ame_noise_source_horizontal_excess_noise_ratio,
        ),
        path_loss_vertical_if_heliax_to_4at16: measurement(
            &raw.path_loss_vertical_if_heliax_to_4at16,
        ),
        path_loss_at4_attenuator: measurement(&raw.path_loss_at4_attenuator),
        path_loss_horizontal_if_heliax_to_4at17: measurement(
            &raw.path_loss_horizontal_if_heliax_to_4at17,
        ),
        path_loss_ifdrif_anti_alias_filter: measurement(&raw.path_loss_ifdrif_anti_alias_filter),
        path_loss_ifd_burst_anti_alias_filter: measurement(
            &raw.path_loss_ifd_burst_anti_alias_filter,
        ),
        path_loss_wg02_harmonic_filter: measurement(&raw.path_loss_wg02_harmonic_filter),
        transmitter_power_data_watts_factor: measurement(&raw.transmitter_power_data_watts_factor),
        path_loss_waveguide_klystron_to_switch: measurement(
            &raw.path_loss_waveguide_klystron_to_switch,
        ),
        pulse_width_transmitter_output_short_pulse: measurement(
            &raw.pulse_width_transmitter_output_short_pulse,
        ),
        pulse_width_transmitter_output_long_pulse: measurement(
            &raw.pulse_width_transmitter_output_long_pulse,
        ),
        path_loss_wg06_spectrum_filter: measurement(&raw.path_loss_wg06_spectrum_filter),
        horizontal_receiver_noise_short_pulse: measurement(
            &raw.horizontal_receiver_noise_short_pulse,
        ),
        horizontal_receiver_noise_long_pulse: measurement(
            &raw.horizontal_receiver_noise_long_pulse,
        ),
        transmitter_spectrum_filter_installed: boxed(&raw.transmitter_spectrum_filter_installed),
    }
}

fn normalize_ping(
    raw: &RadarServerPingStatus,
) -> Result<RadarPingTelemetry, RadarNormalizationError> {
    Ok(RadarPingTelemetry {
        timestamp: timestamp(
            RadarTelemetryKind::Server,
            "ping.timestamp",
            raw.timestamp.as_deref(),
        )?,
        targets: raw.targets.as_ref().map(normalize_ping_targets),
    })
}

fn normalize_ping_targets(raw: &RadarServerPingTargets) -> RadarPingTargets {
    RadarPingTargets {
        client: ping_summary(&raw.client),
        ldm: ping_summary(&raw.ldm),
        radar: ping_summary(&raw.radar),
        server: ping_summary(&raw.server),
        misc: ping_summary(&raw.misc),
    }
}

fn normalize_command(
    raw: &RadarServerCommandStatus,
) -> Result<RadarCommandTelemetry, RadarNormalizationError> {
    Ok(RadarCommandTelemetry {
        timestamp: server_timestamp("command.timestamp", raw.timestamp.as_deref())?,
        last_executed: boxed(&raw.last_executed),
        last_executed_time: server_timestamp(
            "command.last_executed_time",
            raw.last_executed_time.as_deref(),
        )?,
        last_nexrad_data_time: server_timestamp(
            "command.last_nexrad_data_time",
            raw.last_nexrad_data_time.as_deref(),
        )?,
        last_received: boxed(&raw.last_received),
        last_received_time: server_timestamp(
            "command.last_received_time",
            raw.last_received_time.as_deref(),
        )?,
    })
}

fn normalize_hardware(
    raw: &RadarServerHardwareStatus,
) -> Result<RadarHardwareTelemetry, RadarNormalizationError> {
    Ok(RadarHardwareTelemetry {
        timestamp: server_timestamp("hardware.timestamp", raw.timestamp.as_deref())?,
        cpu_idle: raw.cpu_idle,
        io_utilization: raw.io_utilization,
        disk: raw.disk,
        load1: raw.load1,
        load5: raw.load5,
        load15: raw.load15,
        memory: raw.memory,
        uptime: server_timestamp("hardware.uptime", raw.uptime.as_deref())?,
    })
}

fn normalize_ldm(raw: &RadarServerLdmStatus) -> Result<RadarLdmTelemetry, RadarNormalizationError> {
    Ok(RadarLdmTelemetry {
        timestamp: server_timestamp("ldm.timestamp", raw.timestamp.as_deref())?,
        active: raw.active,
        latest_product: server_timestamp("ldm.latest_product", raw.latest_product.as_deref())?,
        oldest_product: server_timestamp("ldm.oldest_product", raw.oldest_product.as_deref())?,
        storage_size: raw.storage_size,
        count: raw.count,
    })
}

fn normalize_network(
    raw: &RadarServerNetworkStatus,
) -> Result<RadarNetworkTelemetry, RadarNormalizationError> {
    Ok(RadarNetworkTelemetry {
        timestamp: server_timestamp("network.timestamp", raw.timestamp.as_deref())?,
        eth0: raw.eth0.as_ref().map(normalize_network_interface),
        eth1: raw.eth1.as_ref().map(normalize_network_interface),
    })
}

fn normalize_network_interface(
    raw: &RadarServerNetworkInterfaceStats,
) -> RadarNetworkInterfaceTelemetry {
    RadarNetworkInterfaceTelemetry {
        interface: boxed(&raw.interface),
        active: raw.active,
        transmitted_ok: raw.trans_no_error,
        transmitted_errors: raw.trans_error,
        transmitted_dropped: raw.trans_dropped,
        transmitted_overruns: raw.trans_overrun,
        received_ok: raw.recv_no_error,
        received_errors: raw.recv_error,
        received_dropped: raw.recv_dropped,
        received_overruns: raw.recv_overrun,
    }
}

fn timestamp(
    telemetry: RadarTelemetryKind,
    field: &'static str,
    raw: Option<&str>,
) -> Result<Option<Timestamp>, RadarNormalizationError> {
    raw.map(|value| {
        value.parse().map_err(|source| RadarNormalizationError {
            telemetry,
            field,
            source,
        })
    })
    .transpose()
}

fn server_timestamp(
    field: &'static str,
    raw: Option<&str>,
) -> Result<Option<Timestamp>, RadarNormalizationError> {
    timestamp(RadarTelemetryKind::Server, field, raw)
}

fn boxed(value: &Option<String>) -> Option<Box<str>> {
    value.as_deref().map(Into::into)
}

fn measurement(value: &Option<ValueUnit>) -> Option<RadarMeasurement> {
    value.as_ref().map(Into::into)
}

fn ping_summary(value: &Option<HashMap<String, bool>>) -> Option<RadarPingSummary> {
    value.as_ref().map(|targets| RadarPingSummary {
        up: targets.values().filter(|&&is_up| is_up).count(),
        total: targets.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::radar_station::{PointGeometry, RadarStationFeature};

    const STATION_FIXTURE: &str = include_str!("../../tests/fixtures/radar/station.json");
    const SERVER_FIXTURE: &str = include_str!("../../tests/fixtures/radar/server.json");

    #[test]
    fn station_fixture_normalizes_typed_meaning() {
        let raw: RadarStationFeature = serde_json::from_str(STATION_FIXTURE).unwrap();
        let telemetry = RadarStationTelemetry::try_from(&raw).unwrap();

        assert_eq!(
            telemetry.feature_id(),
            Some("https://api.weather.gov/radar/stations/KXYZ")
        );
        assert_eq!(
            telemetry.position(),
            RadarPosition::Coordinates {
                longitude: -112.1469,
                latitude: 33.2903,
            }
        );
        let station = telemetry.station().unwrap();
        assert_eq!(station.id(), Some("KXYZ"));
        assert_eq!(station.name(), Some("Example Radar"));
        let elevation = station.elevation().unwrap();
        assert_eq!(elevation.maximum(), Some(421.0));
        assert_eq!(elevation.minimum(), None);
        assert_eq!(elevation.quality_control(), Some("checked"));
        let latency = station.latency().unwrap();
        assert_eq!(latency.current().unwrap().value(), Some(1.25));
        assert!(latency.average().unwrap().unit().is_none());
        assert_eq!(latency.maximum().unwrap().value(), Some(9.75));
        assert_eq!(
            latency.maximum_at().unwrap().to_string(),
            "2026-08-31T15:59:00Z"
        );
        let rda = station.rda().unwrap().properties().unwrap();
        assert_eq!(rda.resolution_version(), Some(4));
        assert_eq!(rda.nl2_path(), Some("/data/level2"));
        let adaptation = station.adaptation().unwrap();
        assert_eq!(adaptation.reporting_host(), Some("adapt.example"));
        assert_eq!(
            adaptation
                .properties()
                .unwrap()
                .path_loss_wg04_circulator()
                .unwrap()
                .value(),
            Some(0.4)
        );
    }

    #[test]
    fn station_position_preserves_missing_and_invalid_states() {
        let missing = RadarStationTelemetry::try_from(&RadarStationFeature::default()).unwrap();
        assert_eq!(missing.position(), RadarPosition::Missing);

        let invalid_raw = RadarStationFeature {
            geometry: Some(PointGeometry {
                coordinates: Some(vec![1.0]),
                ..PointGeometry::default()
            }),
            ..RadarStationFeature::default()
        };
        let invalid = RadarStationTelemetry::try_from(&invalid_raw).unwrap();
        assert_eq!(invalid.position(), RadarPosition::Invalid);
    }

    #[test]
    fn server_fixture_normalizes_ping_and_network_meaning() {
        let raw: RadarServer = serde_json::from_str(SERVER_FIXTURE).unwrap();
        let telemetry = RadarServerTelemetry::try_from(&raw).unwrap();

        assert_eq!(telemetry.id(), Some("ldm1"));
        assert_eq!(
            telemetry.resource_id(),
            Some("https://api.weather.gov/radar/servers/ldm1")
        );
        assert_eq!(telemetry.type_identifier(), Some("wx:RadarServer"));
        let targets = telemetry.ping().unwrap().targets().unwrap();
        assert_eq!(targets.client().unwrap().up(), 2);
        assert_eq!(targets.client().unwrap().total(), 3);
        assert_eq!(targets.ldm().unwrap().total(), 0);
        assert!(targets.radar().is_none());
        assert_eq!(targets.server().unwrap().up(), 0);

        let network = telemetry.network().unwrap();
        assert_eq!(network.eth0().unwrap().interface(), Some("eno1"));
        assert_eq!(network.eth0().unwrap().transmitted_errors(), Some(2));
        assert!(network.eth1().is_some());
        assert!(network.eth1().unwrap().interface().is_none());
    }

    #[test]
    fn malformed_timestamp_reports_telemetry_and_field() {
        let raw = RadarServer {
            collection_time: Some("not-a-timestamp".to_owned()),
            ..RadarServer::default()
        };

        let error = RadarServerTelemetry::try_from(&raw).unwrap_err();
        assert_eq!(error.telemetry(), RadarTelemetryKind::Server);
        assert_eq!(error.field(), "collection_time");
        assert!(error.to_string().contains("radar server telemetry"));
    }

    #[test]
    fn normalization_does_not_change_raw_serialization() {
        let station: RadarStationFeature = serde_json::from_str(STATION_FIXTURE).unwrap();
        let server: RadarServer = serde_json::from_str(SERVER_FIXTURE).unwrap();
        let station_before = serde_json::to_value(&station).unwrap();
        let server_before = serde_json::to_value(&server).unwrap();

        RadarStationTelemetry::try_from(&station).unwrap();
        RadarServerTelemetry::try_from(&server).unwrap();

        assert_eq!(serde_json::to_value(&station).unwrap(), station_before);
        assert_eq!(serde_json::to_value(&server).unwrap(), server_before);
    }
}
