use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

use super::JsonLdContext;

/// A radar SPGDS telemetry response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsResponse {
    /// JSON-LD context supplied with the response.
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Box<JsonLdContext>>,
    /// SPGDS entries mapped from the JSON-LD graph.
    #[serde(rename = "@graph", default)]
    pub spgds: Vec<RadarSpgdsEntry>,
}

/// Telemetry for one radar SPGDS host.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsEntry {
    /// JSON-LD type.
    #[serde(rename = "@type", default, deserialize_with = "scalar_string")]
    pub r#type: Option<String>,
    /// SPGDS identifier.
    #[serde(default, deserialize_with = "scalar_string")]
    pub id: Option<String>,
    /// Telemetry timestamp.
    #[serde(default, deserialize_with = "scalar_string")]
    pub timestamp: Option<String>,
    /// Data-flow status.
    pub dataflow: Option<RadarSpgdsStatus>,
    /// Connection-queue status.
    #[serde(rename = "connectQ")]
    pub connect_q: Option<RadarSpgdsStatus>,
    /// Application process status.
    #[serde(rename = "appRunning")]
    pub app_running: Option<RadarSpgdsStatus>,
    /// Local Data Manager connection telemetry.
    pub ldm: Option<RadarSpgdsLdmStatus>,
    /// Secondary-disk telemetry.
    #[serde(rename = "secondHD")]
    pub second_hd: Option<RadarSpgdsDiskStatus>,
    /// Host uptime telemetry.
    #[serde(rename = "spgdsUpSince")]
    pub uptime: Option<RadarSpgdsUptime>,
    /// Inbound and outbound throughput telemetry.
    pub throughput: Option<RadarSpgdsThroughput>,
    /// Per-gateway telemetry keyed by the dynamic gateway identifier.
    #[serde(default)]
    pub spg: HashMap<String, RadarSpgdsGatewayStatus>,
}

/// State telemetry with its transition and validity times.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsStatus {
    /// State value.
    #[serde(default, deserialize_with = "scalar_string")]
    pub state: Option<String>,
    /// Time at which the current state began.
    #[serde(rename = "stateSince", default, deserialize_with = "scalar_string")]
    pub state_since: Option<String>,
    /// Time through which the state was validated.
    #[serde(rename = "stateValid", default, deserialize_with = "scalar_string")]
    pub state_valid: Option<String>,
}

/// Local Data Manager connection telemetry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsLdmStatus {
    /// Connection count.
    #[serde(default, deserialize_with = "scalar_string")]
    pub conns: Option<String>,
    /// Time through which the connection count was validated.
    #[serde(rename = "connsValid", default, deserialize_with = "scalar_string")]
    pub conns_valid: Option<String>,
}

/// Secondary-disk state and utilization telemetry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsDiskStatus {
    /// Disk state.
    #[serde(default, deserialize_with = "scalar_string")]
    pub state: Option<String>,
    /// Time at which the current state began.
    #[serde(rename = "stateSince", default, deserialize_with = "scalar_string")]
    pub state_since: Option<String>,
    /// Time through which the state was validated.
    #[serde(rename = "stateValid", default, deserialize_with = "scalar_string")]
    pub state_valid: Option<String>,
    /// Percentage of disk space used.
    #[serde(rename = "pctUsed", default, deserialize_with = "scalar_string")]
    pub percent_used: Option<String>,
    /// Time through which utilization was validated.
    #[serde(rename = "pctUsedValid", default, deserialize_with = "scalar_string")]
    pub percent_used_valid: Option<String>,
}

/// SPGDS uptime telemetry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsUptime {
    /// Time at which the host came up.
    #[serde(rename = "upSince", default, deserialize_with = "scalar_string")]
    pub up_since: Option<String>,
    /// Time through which uptime was validated.
    #[serde(rename = "upSinceValid", default, deserialize_with = "scalar_string")]
    pub up_since_valid: Option<String>,
}

/// Inbound and outbound SPGDS throughput telemetry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsThroughput {
    /// Inbound throughput value.
    #[serde(rename = "in", default, deserialize_with = "scalar_string")]
    pub inbound: Option<String>,
    /// Time associated with the inbound sample.
    #[serde(rename = "inDateTime", default, deserialize_with = "scalar_string")]
    pub inbound_date_time: Option<String>,
    /// Time through which the inbound sample was validated.
    #[serde(rename = "inValid", default, deserialize_with = "scalar_string")]
    pub inbound_valid: Option<String>,
    /// Outbound throughput value.
    #[serde(rename = "out", default, deserialize_with = "scalar_string")]
    pub outbound: Option<String>,
    /// Time associated with the outbound sample.
    #[serde(rename = "outDateTime", default, deserialize_with = "scalar_string")]
    pub outbound_date_time: Option<String>,
    /// Time through which the outbound sample was validated.
    #[serde(rename = "outValid", default, deserialize_with = "scalar_string")]
    pub outbound_valid: Option<String>,
}

/// Telemetry for one dynamically named radar gateway.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RadarSpgdsGatewayStatus {
    /// SWIM data state.
    #[serde(rename = "swimDataState", default, deserialize_with = "scalar_string")]
    pub swim_data_state: Option<String>,
    /// Time at which the SWIM data state began.
    #[serde(
        rename = "swimDataStateSince",
        default,
        deserialize_with = "scalar_string"
    )]
    pub swim_data_state_since: Option<String>,
    /// Time through which the SWIM data state was validated.
    #[serde(
        rename = "swimDataStateValid",
        default,
        deserialize_with = "scalar_string"
    )]
    pub swim_data_state_valid: Option<String>,
    /// LDM ping state.
    #[serde(rename = "ldmPingState", default, deserialize_with = "scalar_string")]
    pub ldm_ping_state: Option<String>,
    /// Time at which the LDM ping state began.
    #[serde(
        rename = "ldmPingStateSince",
        default,
        deserialize_with = "scalar_string"
    )]
    pub ldm_ping_state_since: Option<String>,
    /// Time through which the LDM ping state was validated.
    #[serde(
        rename = "ldmPingStateValid",
        default,
        deserialize_with = "scalar_string"
    )]
    pub ldm_ping_state_valid: Option<String>,
}

fn scalar_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Scalar {
        String(String),
        Number(serde_json::Number),
        Bool(bool),
    }

    Ok(
        Option::<Scalar>::deserialize(deserializer)?.map(|value| match value {
            Scalar::String(value) => value,
            Scalar::Number(value) => value.to_string(),
            Scalar::Bool(value) => value.to_string(),
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::RadarSpgdsResponse;

    #[test]
    fn accepts_sparse_unknown_and_scalar_flexible_telemetry() {
        let response: RadarSpgdsResponse = serde_json::from_str(
            r#"{
                "@graph": [{
                    "id": 7,
                    "timestamp": true,
                    "dataflow": {"state": 1, "stateSince": null, "unknown": []},
                    "ldm": {"conns": 47.5},
                    "throughput": {"in": false, "out": "42"},
                    "spg": {"TXYZ": {"swimDataState": 0, "ldmPingState": true}},
                    "unknown": {"nested": "ignored"}
                }]
            }"#,
        )
        .unwrap();

        let entry = &response.spgds[0];
        assert_eq!(entry.id.as_deref(), Some("7"));
        assert_eq!(entry.timestamp.as_deref(), Some("true"));
        assert_eq!(entry.dataflow.as_ref().unwrap().state.as_deref(), Some("1"));
        assert_eq!(entry.dataflow.as_ref().unwrap().state_since, None);
        assert_eq!(entry.ldm.as_ref().unwrap().conns.as_deref(), Some("47.5"));
        assert_eq!(
            entry.throughput.as_ref().unwrap().inbound.as_deref(),
            Some("false")
        );
        assert_eq!(entry.spg["TXYZ"].ldm_ping_state.as_deref(), Some("true"));
    }
}
