use serde::{Deserialize, Serialize};

use super::{JsonLdContext, ValueUnit};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarStationFeature {
    /// Unique identifier for the radar station feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the GeoJSON object, typically "Feature".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Geometric data for the feature (location of the radar station).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<PointGeometry>,
    /// Detailed properties of the radar station.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub radar_station: Option<RadarStation>,
}

/// Contains all properties of a radar station, including identity, location, and operational details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarStation {
    /// JSON-LD context, can be a string, an object, or an array of strings and objects.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<JsonLdContext>>,
    /// Unique identifier (URL) for the radar station.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// Type identifier for the radar station, (e.g., "wx:RadarStation").
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// Short identifier for the radar station (e.g., "KFSD").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Human-readable name of the radar station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of the radar station (e.g., "WSR-88D").
    #[serde(rename = "stationType", skip_serializing_if = "Option::is_none")]
    pub station_type: Option<String>,
    /// Elevation of the radar station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<ValueUnit>,
    /// Time zone of the radar station.
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Latency information for the station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<LatencyInfo>,
    /// RDA (Radar Data Acquisition) unit information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rda: Option<RdaInfo>,
    /// Performance metrics for the station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance: Option<PerformanceInfo>,
    /// Adaptation parameters for the station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptation: Option<AdaptationInfo>,
}

/// Represents a GeoJSON Point geometry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointGeometry {
    /// The type of geometry, typically "Point".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The coordinates of the point, usually [longitude, latitude].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Vec<f64>>,
}

/// Contains latency information for a radar station.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LatencyInfo {
    /// Current latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<ValueUnit>,
    /// Average latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<ValueUnit>,
    /// Maximum latency observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<ValueUnit>,
    /// Timestamp of the last Level II data received.
    #[serde(
        rename = "levelTwoLastReceivedTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub level_two_last_received_time: Option<String>,
    /// Timestamp associated with the maximum latency.
    #[serde(rename = "maxLatencyTime", skip_serializing_if = "Option::is_none")]
    pub max_latency_time: Option<String>,
    /// The host that reported this latency information.
    #[serde(rename = "reportingHost", skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
    /// The specific host associated with the latency data (e.g., LDM server).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

/// Properties related to the RDA (Radar Data Acquisition) unit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RdaProperties {
    /// Version of the resolution. Can be null.
    #[serde(rename = "resolutionVersion", skip_serializing_if = "Option::is_none")]
    pub resolution_version: Option<String>, // Assuming string or null
    /// Path for NL2 data.
    #[serde(rename = "nl2Path", skip_serializing_if = "Option::is_none")]
    pub nl2_path: Option<String>,
    /// Volume coverage pattern (e.g., "R35", "R212").
    #[serde(
        rename = "volumeCoveragePattern",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_coverage_pattern: Option<String>,
    /// Control status of the RDA.
    #[serde(rename = "controlStatus", skip_serializing_if = "Option::is_none")]
    pub control_status: Option<String>,
    /// Build number of the RDA software/firmware.
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<f64>, // e.g., 23.1 or 23
    /// Summary of current alarms.
    #[serde(rename = "alarmSummary", skip_serializing_if = "Option::is_none")]
    pub alarm_summary: Option<String>,
    /// Operational mode of the RDA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// State of the power generator.
    #[serde(rename = "generatorState", skip_serializing_if = "Option::is_none")]
    pub generator_state: Option<String>,
    /// Status of super resolution capabilities.
    #[serde(
        rename = "superResolutionStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub super_resolution_status: Option<String>,
    /// Overall operability status of the RDA.
    #[serde(rename = "operabilityStatus", skip_serializing_if = "Option::is_none")]
    pub operability_status: Option<String>,
    /// General status of the RDA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Average transmitter power.
    #[serde(
        rename = "averageTransmitterPower",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_transmitter_power: Option<ValueUnit>,
    /// Reflectivity calibration correction value.
    #[serde(
        rename = "reflectivityCalibrationCorrection",
        skip_serializing_if = "Option::is_none"
    )]
    pub reflectivity_calibration_correction: Option<ValueUnit>,
}

/// Information about the RDA (Radar Data Acquisition) unit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RdaInfo {
    /// Timestamp of the RDA information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The host that reported this RDA information.
    #[serde(rename = "reportingHost", skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
    /// Detailed properties of the RDA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RdaProperties>,
}

/// Properties of a radar station feature.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarStationProperties {
    /// Unique identifier (URL) for the radar station.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// Type identifier for the radar station, often from a vocabulary (e.g., "wx:RadarStation").
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// Short identifier for the radar station (e.g., "KFSD").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Human-readable name of the radar station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of the radar station (e.g., "WSR-88D").
    #[serde(rename = "stationType", skip_serializing_if = "Option::is_none")]
    pub station_type: Option<String>,
    /// Elevation of the radar station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<ValueUnit>,
    /// Time zone of the radar station.
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Latency information for the station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<LatencyInfo>,
    /// RDA (Radar Data Acquisition) unit information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rda: Option<RdaInfo>,
}

/// Represents a single feature in a GeoJSON FeatureCollection.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feature {
    /// Unique identifier for the feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the GeoJSON object, typically "Feature".
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Geometric data for the feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<PointGeometry>,
    /// Properties associated with the feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RadarStationProperties>,
}

/// Detailed performance metrics of the radar station.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceProperties {
    #[serde(rename = "ntp_status", skip_serializing_if = "Option::is_none")]
    pub ntp_status: Option<i32>,
    #[serde(rename = "commandChannel", skip_serializing_if = "Option::is_none")]
    pub command_channel: Option<String>,
    #[serde(
        rename = "radomeAirTemperature",
        skip_serializing_if = "Option::is_none"
    )]
    pub radome_air_temperature: Option<ValueUnit>,
    #[serde(
        rename = "transitionalPowerSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub transitional_power_source: Option<String>,
    #[serde(
        rename = "horizontalShortPulseNoise",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_short_pulse_noise: Option<ValueUnit>,
    #[serde(
        rename = "elevationEncoderLight",
        skip_serializing_if = "Option::is_none"
    )]
    pub elevation_encoder_light: Option<String>,
    #[serde(
        rename = "horizontalLongPulseNoise",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_long_pulse_noise: Option<ValueUnit>,
    #[serde(
        rename = "azimuthEncoderLight",
        skip_serializing_if = "Option::is_none"
    )]
    pub azimuth_encoder_light: Option<String>,
    #[serde(
        rename = "horizontalNoiseTemperature",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_noise_temperature: Option<ValueUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linearity: Option<f64>,
    #[serde(
        rename = "transmitterPeakPower",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter_peak_power: Option<ValueUnit>,
    #[serde(
        rename = "horizontalDeltadBZ0",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_deltad_bz0: Option<ValueUnit>,
    #[serde(
        rename = "transmitterRecycleCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter_recycle_count: Option<i32>,
    #[serde(rename = "verticalDeltadBZ0", skip_serializing_if = "Option::is_none")]
    pub vertical_deltad_bz0: Option<ValueUnit>,
    #[serde(rename = "receiverBias", skip_serializing_if = "Option::is_none")]
    pub receiver_bias: Option<ValueUnit>,
    #[serde(
        rename = "shortPulseHorizontaldBZ0",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_pulse_horizontal_dbz0: Option<ValueUnit>,
    #[serde(
        rename = "transmitterImbalance",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter_imbalance: Option<ValueUnit>,
    #[serde(
        rename = "longPulseHorizontaldBZ0",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_pulse_horizontal_dbz0: Option<ValueUnit>,
    #[serde(
        rename = "performanceCheckTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub performance_check_time: Option<String>,
    #[serde(
        rename = "transmitterLeavingAirTemperature",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter_leaving_air_temperature: Option<ValueUnit>,
    #[serde(rename = "shelterTemperature", skip_serializing_if = "Option::is_none")]
    pub shelter_temperature: Option<ValueUnit>,
    #[serde(rename = "powerSource", skip_serializing_if = "Option::is_none")]
    pub power_source: Option<String>,
    #[serde(rename = "dynamicRange", skip_serializing_if = "Option::is_none")]
    pub dynamic_range: Option<ValueUnit>,
    #[serde(rename = "fuelLevel", skip_serializing_if = "Option::is_none")]
    pub fuel_level: Option<ValueUnit>,
}

/// Encapsulates performance data along with its reporting context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "reportingHost", skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PerformanceProperties>,
}

/// Detailed adaptation parameters of the radar station.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdaptationProperties {
    #[serde(
        rename = "transmitterFrequency",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter_frequency: Option<ValueUnit>,
    #[serde(
        rename = "pathLossWG04Circulator",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg04_circulator: Option<ValueUnit>,
    #[serde(
        rename = "antennaGainIncludingRadome",
        skip_serializing_if = "Option::is_none"
    )]
    pub antenna_gain_including_radome: Option<ValueUnit>,
    #[serde(
        rename = "pathLossA6ArcDetector",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_a6_arc_detector: Option<ValueUnit>,
    #[serde(rename = "cohoPowerAtA1J4", skip_serializing_if = "Option::is_none")]
    pub coho_power_at_a1j4: Option<ValueUnit>,
    #[serde(
        rename = "ameHorzizontalTestSignalPower",
        skip_serializing_if = "Option::is_none"
    )]
    pub ame_horizontal_test_signal_power: Option<ValueUnit>,
    #[serde(
        rename = "pathLossTransmitterCouplerCoupling",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_transmitter_coupler_coupling: Option<ValueUnit>,
    #[serde(rename = "staloPowerAtA1J2", skip_serializing_if = "Option::is_none")]
    pub stalo_power_at_a1j2: Option<ValueUnit>,
    #[serde(
        rename = "ameNoiseSourceHorizontalExcessNoiseRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub ame_noise_source_horizontal_excess_noise_ratio: Option<ValueUnit>,
    #[serde(
        rename = "pathLossVerticalIFHeliaxTo4AT16",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_vertical_if_heliax_to_4at16: Option<ValueUnit>,
    #[serde(
        rename = "pathLossAT4Attenuator",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_at4_attenuator: Option<ValueUnit>,
    #[serde(
        rename = "pathLossHorzontalIFHeliaxTo4AT17",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_horizontal_if_heliax_to_4at17: Option<ValueUnit>,
    #[serde(
        rename = "pathLossIFDRIFAntiAliasFilter",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_ifdrif_anti_alias_filter: Option<ValueUnit>,
    #[serde(
        rename = "pathLossIFDBurstAntiAliasFilter",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_ifd_burst_anti_alias_filter: Option<ValueUnit>,
    #[serde(
        rename = "pathLossWG02HarmonicFilter",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg02_harmonic_filter: Option<ValueUnit>,
    #[serde(
        rename = "transmitterPowerDataWattsFactor",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter_power_data_watts_factor: Option<ValueUnit>,
    #[serde(
        rename = "pathLossWaveguideKlystronToSwitch",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_waveguide_klystron_to_switch: Option<ValueUnit>,
    #[serde(
        rename = "pulseWidthTransmitterOutputShortPulse",
        skip_serializing_if = "Option::is_none"
    )]
    pub pulse_width_transmitter_output_short_pulse: Option<ValueUnit>,
    #[serde(
        rename = "pulseWidthTransmitterOutputLongPulse",
        skip_serializing_if = "Option::is_none"
    )]
    pub pulse_width_transmitter_output_long_pulse: Option<ValueUnit>,
    #[serde(
        rename = "pathLossWG06SpectrumFilter",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_loss_wg06_spectrum_filter: Option<ValueUnit>,
    #[serde(
        rename = "horizontalReceiverNoiseShortPulse",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_receiver_noise_short_pulse: Option<ValueUnit>,
    #[serde(
        rename = "horizontalReceiverNoiseLongPulse",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_receiver_noise_long_pulse: Option<ValueUnit>,
    #[serde(
        rename = "transmitterSpectrumFilterInstalled",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmitter_spectrum_filter_installed: Option<String>,
}

/// Encapsulates adaptation data along with its reporting context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdaptationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "reportingHost", skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdaptationProperties>,
}
