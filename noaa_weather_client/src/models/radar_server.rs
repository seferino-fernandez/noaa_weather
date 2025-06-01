use super::JsonLdContext;
use crate::utils::serde::deserialize_optional_map_or_empty_array;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the status and configuration of a radar server.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServer {
    /// JSON-LD context.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<JsonLdContext>,
    /// Unique identifier (URL) for the radar server.
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// Type identifier for the radar server, often from a vocabulary (e.g., "wx:RadarServer").
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// Short identifier for the radar server (e.g., "ldm4").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the server (e.g., "ldm").
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Indicates if the server is currently active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Indicates if this is the primary server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    /// Indicates if this server is an aggregate server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<bool>,
    /// Indicates if the server configuration is locked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Indicates if the radar network is up.
    #[serde(rename = "radarNetworkUp", skip_serializing_if = "Option::is_none")]
    pub radar_network_up: Option<bool>,
    /// Timestamp of the last data collection time.
    #[serde(rename = "collectionTime", skip_serializing_if = "Option::is_none")]
    pub collection_time: Option<String>,
    /// The host that reported this status.
    #[serde(rename = "reportingHost", skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
    /// Ping status information for various components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping: Option<RadarServerPingStatus>,
    /// Command execution and reception status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<RadarServerCommandStatus>,
    /// Hardware status of the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<RadarServerHardwareStatus>,
    /// LDM (Local Data Manager) status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldm: Option<RadarServerLdmStatus>,
    /// Network interface status and statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<RadarServerNetworkStatus>,
}

/// Represents the target systems for ping status, categorized by type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServerPingTargets {
    /// Status of client targets, where keys are client identifiers and values are their ping status.
    #[serde(
        deserialize_with = "deserialize_optional_map_or_empty_array",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub client: Option<HashMap<String, bool>>,
    /// Status of LDM targets, where keys are LDM identifiers and values are their ping status (boolean).
    #[serde(
        deserialize_with = "deserialize_optional_map_or_empty_array",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ldm: Option<HashMap<String, bool>>,

    /// Status of radar targets, where keys are radar identifiers and values are their ping status.
    #[serde(
        deserialize_with = "deserialize_optional_map_or_empty_array",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub radar: Option<HashMap<String, bool>>,

    /// Status of server targets, where keys are server identifiers and values are their ping status.
    #[serde(
        deserialize_with = "deserialize_optional_map_or_empty_array",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub server: Option<HashMap<String, bool>>,

    /// Status of miscellaneous targets, where keys are target identifiers and values are their ping status.
    #[serde(
        deserialize_with = "deserialize_optional_map_or_empty_array",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub misc: Option<HashMap<String, bool>>,
}

/// Represents the overall ping status of various server components and targets.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServerPingStatus {
    /// Detailed status of different target categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<RadarServerPingTargets>,
    /// Timestamp of when the ping status was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Represents the status of commands executed or received by the radar server.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServerCommandStatus {
    /// The last command that was executed.
    #[serde(rename = "lastExecuted", skip_serializing_if = "Option::is_none")]
    pub last_executed: Option<String>,
    /// Timestamp of when the last command was executed.
    #[serde(rename = "lastExecutedTime", skip_serializing_if = "Option::is_none")]
    pub last_executed_time: Option<String>,
    /// Timestamp of the last NEXRAD data time relevant to commands.
    #[serde(rename = "lastNexradDataTime", skip_serializing_if = "Option::is_none")]
    pub last_nexrad_data_time: Option<String>,
    /// The last command that was received.
    #[serde(rename = "lastReceived", skip_serializing_if = "Option::is_none")]
    pub last_received: Option<String>,
    /// Timestamp of when the last command was received.
    #[serde(rename = "lastReceivedTime", skip_serializing_if = "Option::is_none")]
    pub last_received_time: Option<String>,
    /// Timestamp of when this command status was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// Represents the hardware status of the radar server.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServerHardwareStatus {
    /// Timestamp of when the hardware status was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// CPU idle percentage.
    #[serde(rename = "cpuIdle", skip_serializing_if = "Option::is_none")]
    pub cpu_idle: Option<f64>,
    /// I/O utilization percentage.
    #[serde(rename = "ioUtilization", skip_serializing_if = "Option::is_none")]
    pub io_utilization: Option<f64>,
    /// Disk usage or status indicator (e.g., percentage or specific value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<i32>, // JSON shows 2
    /// System load average over 1 minute.
    #[serde(rename = "load1", skip_serializing_if = "Option::is_none")]
    pub load1: Option<f64>,
    /// System load average over 5 minutes.
    #[serde(rename = "load5", skip_serializing_if = "Option::is_none")]
    pub load5: Option<f64>,
    /// System load average over 15 minutes.
    #[serde(rename = "load15", skip_serializing_if = "Option::is_none")]
    pub load15: Option<f64>,
    /// Memory usage percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
    /// System uptime timestamp, indicating when the system started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
}

/// Represents the LDM (Local Data Manager) status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServerLdmStatus {
    /// Timestamp of when the LDM status was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Timestamp of the latest product processed by LDM.
    #[serde(rename = "latestProduct", skip_serializing_if = "Option::is_none")]
    pub latest_product: Option<String>,
    /// Timestamp of the oldest product currently in LDM storage.
    #[serde(rename = "oldestProduct", skip_serializing_if = "Option::is_none")]
    pub oldest_product: Option<String>,
    /// Current size of LDM storage in bytes.
    #[serde(rename = "storageSize", skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<i64>,
    /// Count of products currently in LDM storage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>, // JSON shows 31548, fits in i32
    /// Indicates if the LDM is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// Represents statistics for a network interface.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServerNetworkInterfaceStats {
    /// Name of the network interface (e.g., "eth0", "eth1").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    /// Indicates if the network interface is active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Number of successfully transmitted packets.
    #[serde(rename = "transNoError", skip_serializing_if = "Option::is_none")]
    pub trans_no_error: Option<i64>,
    /// Number of transmission errors.
    #[serde(rename = "transError", skip_serializing_if = "Option::is_none")]
    pub trans_error: Option<i64>,
    /// Number of dropped transmitted packets.
    #[serde(rename = "transDropped", skip_serializing_if = "Option::is_none")]
    pub trans_dropped: Option<i64>,
    /// Number of transmission overruns.
    #[serde(rename = "transOverrun", skip_serializing_if = "Option::is_none")]
    pub trans_overrun: Option<i64>,
    /// Number of successfully received packets.
    #[serde(rename = "recvNoError", skip_serializing_if = "Option::is_none")]
    pub recv_no_error: Option<i64>,
    /// Number of reception errors.
    #[serde(rename = "recvError", skip_serializing_if = "Option::is_none")]
    pub recv_error: Option<i64>,
    /// Number of dropped received packets.
    #[serde(rename = "recvDropped", skip_serializing_if = "Option::is_none")]
    pub recv_dropped: Option<i64>,
    /// Number of reception overruns.
    #[serde(rename = "recvOverrun", skip_serializing_if = "Option::is_none")]
    pub recv_overrun: Option<i64>,
}

/// Represents the overall network status, including statistics for specific interfaces.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RadarServerNetworkStatus {
    /// Timestamp of when the network status was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Statistics for the 'eth1' network interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth1: Option<RadarServerNetworkInterfaceStats>,
    /// Statistics for the 'eth0' network interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eth0: Option<RadarServerNetworkInterfaceStats>,
}
