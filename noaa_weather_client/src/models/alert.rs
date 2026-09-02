//! Models for the `/alerts` family: [`Alert`] and its parts,
//! [`ActiveAlertCounts`], and [`AlertEventTypes`].
//!
//! [`Alert`] is the `properties` object of every alert feature returned by
//! `/alerts`, `/alerts/active`, and `/alerts/{id}`. Its fields follow the
//! National Weather Service CAP v1.2 profile, which extends the OASIS Common
//! Alerting Protocol v1.2 and the IPAWS Profile v1.0; see
//! <https://alerts.weather.gov/#technical-notes-v12>.
//!
//! # Requiredness
//!
//! A field is not `Option` when CAP requires it or NOAA sends it non-null on
//! every alert. Lists and maps are never `Option`: a missing or `null` list
//! decodes as empty. Everything else is `Option`, and a missing required
//! scalar is a decode error rather than a silent `None`.
//!
//! # Null versus absent
//!
//! `Option` fields whose key NOAA always sends (`onset`, `ends`, `headline`,
//! `description`, `instruction`, `note`, `response`, `code`, `language`,
//! `web`) serialize as `null` when `None`, so re-serializing an alert
//! reproduces NOAA's key set. Fields NOAA omits when absent (`@id`, `@type`,
//! `replacedBy`, `replacedAt`) are skipped when `None`. Both `null` and an
//! absent key deserialize to `None`.
//!
//! # Timestamps and identifiers
//!
//! Every timestamp is an [`OffsetDateTime`], which keeps the UTC offset NOAA
//! wrote it in (`-04:00` for an Eastern office) so JSON output reproduces the
//! original text. `id` and `references[].identifier` are [`AlertId`] and
//! `geocode.UGC` entries are [`ZoneId`], so they feed straight back into
//! [`crate::apis::alerts::Alerts::get`] and
//! [`crate::apis::alerts::Alerts::active_for_zone`].

use std::{collections::BTreeMap, fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::ids::{AlertId, ZoneId};
use crate::time::OffsetDateTime;

/// A public alert message: the `properties` of one alert feature.
///
/// ```
/// use noaa_weather_client::models::{Alert, AlertSeverity};
///
/// let alert: Alert = serde_json::from_str(r#"{
///   "id": "urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1",
///   "areaDesc": "Newaygo; Montcalm; Kent",
///   "geocode": {"SAME": ["026123"], "UGC": ["MIZ044"]},
///   "affectedZones": ["https://api.weather.gov/zones/forecast/MIZ044"],
///   "sent": "2026-09-02T03:48:00-04:00",
///   "effective": "2026-09-02T03:48:00-04:00",
///   "onset": "2026-09-02T03:48:00-04:00",
///   "expires": "2026-09-02T04:45:00-04:00",
///   "ends": null,
///   "status": "Actual", "messageType": "Alert", "category": "Met",
///   "severity": "Moderate", "certainty": "Observed", "urgency": "Expected",
///   "event": "Special Weather Statement",
///   "sender": "w-nws.webmaster@noaa.gov",
///   "senderName": "NWS Grand Rapids MI",
///   "headline": null, "description": null, "instruction": null, "response": null,
///   "parameters": {"maxWindGust": ["40 MPH"]},
///   "scope": "Public", "code": "IPAWSv1.0", "language": "en-US", "web": null,
///   "note": null, "eventCode": {"SAME": ["SPS"]}
/// }"#).unwrap();
///
/// assert_eq!(alert.event, "Special Weather Statement");
/// assert_eq!(alert.sent.to_string(), "2026-09-02T03:48:00-04:00");
/// assert_eq!(alert.affected_zone_ids().next().unwrap().as_str(), "MIZ044");
/// assert_eq!(alert.parameter("maxWindGust"), ["40 MPH"]);
/// assert!(alert.severity <= AlertSeverity::Moderate);
/// assert_eq!(alert.ends, None);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Alert {
    /// The canonical API URL for this alert.
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// The JSON-LD type assigned to this alert (`wx:Alert`).
    #[serde(rename = "@type", default, skip_serializing_if = "Option::is_none")]
    pub at_type: Option<String>,
    /// The identifier of the alert message, accepted by `/alerts/{id}`.
    pub id: AlertId,
    /// A textual description of the area affected by the alert.
    pub area_desc: String,
    /// SAME and UGC codes for the affected counties and zones.
    #[serde(default)]
    pub geocode: AlertGeocode,
    /// API URLs of the zones affected by the alert. An API-specific
    /// extension, not part of CAP. See [`Alert::affected_zone_ids`].
    #[serde(default)]
    pub affected_zones: Vec<String>,
    /// Prior alerts that this alert updates or replaces.
    #[serde(default)]
    pub references: Vec<AlertReference>,
    /// When the alert message was issued.
    pub sent: OffsetDateTime,
    /// When the information in the alert message takes effect.
    pub effective: OffsetDateTime,
    /// The expected beginning of the subject event.
    #[serde(default)]
    pub onset: Option<OffsetDateTime>,
    /// When the information in the alert message expires.
    pub expires: OffsetDateTime,
    /// The expected end of the subject event.
    #[serde(default)]
    pub ends: Option<OffsetDateTime>,
    /// Whether the alert is real, an exercise, a system message, a test, or
    /// a draft.
    pub status: AlertStatus,
    /// Whether the message is a new alert, an update, or a cancellation.
    pub message_type: AlertMessageType,
    /// The category of the subject event.
    pub category: AlertCategory,
    /// The severity of the subject event.
    pub severity: AlertSeverity,
    /// The certainty of the subject event.
    pub certainty: AlertCertainty,
    /// The urgency of the subject event.
    pub urgency: AlertUrgency,
    /// The type of the subject event, such as `Tornado Warning`.
    pub event: String,
    /// Email address of the NWS webmaster.
    pub sender: String,
    /// The name of the originating office, such as `NWS Grand Rapids MI`.
    pub sender_name: String,
    /// The headline of the alert message.
    #[serde(default)]
    pub headline: Option<String>,
    /// The text describing the subject event.
    #[serde(default)]
    pub description: Option<String>,
    /// The recommended action for recipients of the alert.
    #[serde(default)]
    pub instruction: Option<String>,
    /// The recommended response type (`responseType` in CAP).
    #[serde(default)]
    pub response: Option<AlertResponse>,
    /// System-specific parameters, keyed by the parameter names in the NWS
    /// CAP specification (`NWSheadline`, `VTEC`, `maxWindGust`, ...). See
    /// [`Alert::parameter`].
    #[serde(default)]
    pub parameters: BTreeMap<String, Vec<String>>,
    /// The intended distribution of the alert message.
    pub scope: AlertScope,
    /// The code denoting special handling, always `IPAWSv1.0` today.
    #[serde(default)]
    pub code: Option<String>,
    /// The language of the alert text, always `en-US` today.
    #[serde(default)]
    pub language: Option<String>,
    /// A hyperlink to additional information.
    #[serde(default)]
    pub web: Option<String>,
    /// Additional narrative information about the alert.
    #[serde(default)]
    pub note: Option<String>,
    /// System-specific codes identifying the event type, keyed by coding
    /// system (`SAME`, `NationalWeatherService`).
    #[serde(default)]
    pub event_code: BTreeMap<String, Vec<String>>,
    /// The API URL of the alert that superseded this one. NOAA omits the
    /// key while the alert is current.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<String>,
    /// When this alert was superseded. NOAA omits the key while the alert
    /// is current.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_at: Option<OffsetDateTime>,
}

impl Alert {
    /// Returns the zone identifiers named by [`Alert::affected_zones`].
    ///
    /// Each URL ends in a zone id (`https://api.weather.gov/zones/forecast/MIZ044`);
    /// URLs whose last segment is not a valid [`ZoneId`] are skipped.
    pub fn affected_zone_ids(&self) -> impl Iterator<Item = ZoneId> + '_ {
        self.affected_zones.iter().filter_map(|url| {
            url.trim_end_matches('/')
                .rsplit('/')
                .next()
                .and_then(|segment| segment.parse().ok())
        })
    }

    /// Returns the values of one CAP parameter, or an empty slice when the
    /// alert does not carry it.
    #[must_use]
    pub fn parameter(&self, name: &str) -> &[String] {
        self.parameters.get(name).map_or(&[], Vec::as_slice)
    }
}

/// Codes for the counties and zones an alert affects.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct AlertGeocode {
    /// SAME (Specific Area Message Encoding) codes, six digits each, for
    /// the affected counties.
    #[serde(rename = "SAME", default)]
    pub same: Vec<String>,
    /// UGC codes: the affected NWS public zones or counties.
    #[serde(rename = "UGC", default)]
    pub ugc: Vec<ZoneId>,
}

/// A prior alert that the containing alert updates or replaces.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[non_exhaustive]
pub struct AlertReference {
    /// The API URL of the prior alert.
    #[serde(rename = "@id", default, skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    /// The identifier of the prior alert.
    pub identifier: AlertId,
    /// The sender of the prior alert.
    pub sender: String,
    /// When the prior alert was sent.
    pub sent: OffsetDateTime,
}

/// Whether an alert is real, an exercise, a system message, a test, or a
/// draft.
///
/// `Display` is lowercase (`actual`), the form NOAA's `status` query
/// parameter takes; serde uses CAP's capitalized form (`Actual`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertStatus {
    /// A real alert.
    #[serde(rename = "Actual")]
    Actual,
    /// An exercise.
    #[serde(rename = "Exercise")]
    Exercise,
    /// A system message, such as a keep-alive.
    #[serde(rename = "System")]
    System,
    /// A test message.
    #[serde(rename = "Test")]
    Test,
    /// A draft.
    #[serde(rename = "Draft")]
    Draft,
}

impl fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Actual => write!(f, "actual"),
            Self::Exercise => write!(f, "exercise"),
            Self::System => write!(f, "system"),
            Self::Test => write!(f, "test"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

impl FromStr for AlertStatus {
    type Err = String;

    /// Parse a string into an [`AlertStatus`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use noaa_weather_client::models::AlertStatus;
    ///
    /// let status = AlertStatus::from_str("actual").unwrap();
    /// assert_eq!(status, AlertStatus::Actual);
    /// ```
    ///
    /// ```
    /// use std::str::FromStr;
    /// use noaa_weather_client::models::AlertStatus;
    ///
    /// let status = AlertStatus::from_str("ACTUAL").unwrap();
    /// assert_eq!(status, AlertStatus::Actual);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid alert status.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "actual" => Ok(Self::Actual),
            "exercise" => Ok(Self::Exercise),
            "system" => Ok(Self::System),
            "test" => Ok(Self::Test),
            "draft" => Ok(Self::Draft),
            _ => Err(format!("Invalid alert status: {s}")),
        }
    }
}

/// Whether a message is a new alert, an update, or a cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertMessageType {
    /// Initial information about an event.
    #[serde(rename = "Alert")]
    Alert,
    /// Updates and supersedes an earlier message.
    #[serde(rename = "Update")]
    Update,
    /// Cancels an earlier message.
    #[serde(rename = "Cancel")]
    Cancel,
    /// Acknowledges receipt of an earlier message.
    #[serde(rename = "Ack")]
    Ack,
    /// Reports rejection of an earlier message.
    #[serde(rename = "Error")]
    Error,
}

impl fmt::Display for AlertMessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Alert => write!(f, "Alert"),
            Self::Update => write!(f, "Update"),
            Self::Cancel => write!(f, "Cancel"),
            Self::Ack => write!(f, "Ack"),
            Self::Error => write!(f, "Error"),
        }
    }
}

impl FromStr for AlertMessageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "alert" => Ok(Self::Alert),
            "update" => Ok(Self::Update),
            "cancel" => Ok(Self::Cancel),
            "ack" => Ok(Self::Ack),
            "error" => Ok(Self::Error),
            _ => Err(format!("Invalid alert message type: {s}")),
        }
    }
}

/// The category of an alert's subject event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertCategory {
    /// Meteorological, including flood.
    #[serde(rename = "Met")]
    Met,
    /// Geophysical, including landslide.
    #[serde(rename = "Geo")]
    Geo,
    /// General emergency and public safety.
    #[serde(rename = "Safety")]
    Safety,
    /// Law enforcement, military, homeland and local or private security.
    #[serde(rename = "Security")]
    Security,
    /// Rescue and recovery.
    #[serde(rename = "Rescue")]
    Rescue,
    /// Fire suppression and rescue.
    #[serde(rename = "Fire")]
    Fire,
    /// Medical and public health.
    #[serde(rename = "Health")]
    Health,
    /// Pollution and other environmental.
    #[serde(rename = "Env")]
    Env,
    /// Public and private transportation.
    #[serde(rename = "Transport")]
    Transport,
    /// Utility, telecommunication, and other infrastructure.
    #[serde(rename = "Infra")]
    Infra,
    /// Chemical, biological, radiological, nuclear, or high-yield explosive.
    #[serde(rename = "CBRNE")]
    Cbrne,
    /// Other events.
    #[serde(rename = "Other")]
    Other,
}

impl fmt::Display for AlertCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Met => write!(f, "Met"),
            Self::Geo => write!(f, "Geo"),
            Self::Safety => write!(f, "Safety"),
            Self::Security => write!(f, "Security"),
            Self::Rescue => write!(f, "Rescue"),
            Self::Fire => write!(f, "Fire"),
            Self::Health => write!(f, "Health"),
            Self::Env => write!(f, "Env"),
            Self::Transport => write!(f, "Transport"),
            Self::Infra => write!(f, "Infra"),
            Self::Cbrne => write!(f, "CBRNE"),
            Self::Other => write!(f, "Other"),
        }
    }
}

/// The severity of an alert's subject event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertSeverity {
    /// Extraordinary threat to life or property.
    #[serde(rename = "Extreme")]
    Extreme,
    /// Significant threat to life or property.
    #[serde(rename = "Severe")]
    Severe,
    /// Possible threat to life or property.
    #[serde(rename = "Moderate")]
    Moderate,
    /// Minimal to no known threat to life or property.
    #[serde(rename = "Minor")]
    Minor,
    /// Severity unknown.
    #[serde(rename = "Unknown")]
    Unknown,
}

impl fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Extreme => write!(f, "Extreme"),
            Self::Severe => write!(f, "Severe"),
            Self::Moderate => write!(f, "Moderate"),
            Self::Minor => write!(f, "Minor"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for AlertSeverity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Extreme" => Ok(AlertSeverity::Extreme),
            "Severe" => Ok(AlertSeverity::Severe),
            "Moderate" => Ok(AlertSeverity::Moderate),
            "Minor" => Ok(AlertSeverity::Minor),
            "Unknown" => Ok(AlertSeverity::Unknown),
            _ => Err(format!("Invalid alert severity: {s}")),
        }
    }
}

/// The certainty of an alert's subject event.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertCertainty {
    /// Determined to have occurred or to be ongoing.
    #[serde(rename = "Observed")]
    #[default]
    Observed,
    /// Likely (probability greater than roughly 50%).
    #[serde(rename = "Likely")]
    Likely,
    /// Possible but not likely (probability roughly 50% or less).
    #[serde(rename = "Possible")]
    Possible,
    /// Not expected to occur (probability near 0).
    #[serde(rename = "Unlikely")]
    Unlikely,
    /// Certainty unknown.
    #[serde(rename = "Unknown")]
    Unknown,
}

impl fmt::Display for AlertCertainty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Observed => write!(f, "Observed"),
            Self::Likely => write!(f, "Likely"),
            Self::Possible => write!(f, "Possible"),
            Self::Unlikely => write!(f, "Unlikely"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for AlertCertainty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Observed" => Ok(AlertCertainty::Observed),
            "Likely" => Ok(AlertCertainty::Likely),
            "Possible" => Ok(AlertCertainty::Possible),
            "Unlikely" => Ok(AlertCertainty::Unlikely),
            "Unknown" => Ok(AlertCertainty::Unknown),
            _ => Err(format!("Invalid alert certainty: {s}")),
        }
    }
}

/// The urgency of an alert's subject event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertUrgency {
    /// Responsive action should be taken immediately.
    #[serde(rename = "Immediate")]
    Immediate,
    /// Responsive action should be taken soon (within the next hour).
    #[serde(rename = "Expected")]
    Expected,
    /// Responsive action should be taken in the near future.
    #[serde(rename = "Future")]
    Future,
    /// Responsive action is no longer required.
    #[serde(rename = "Past")]
    Past,
    /// Urgency unknown.
    #[serde(rename = "Unknown")]
    Unknown,
}

impl fmt::Display for AlertUrgency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Immediate => write!(f, "Immediate"),
            Self::Expected => write!(f, "Expected"),
            Self::Future => write!(f, "Future"),
            Self::Past => write!(f, "Past"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for AlertUrgency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "immediate" => Ok(AlertUrgency::Immediate),
            "expected" => Ok(AlertUrgency::Expected),
            "future" => Ok(AlertUrgency::Future),
            "past" => Ok(AlertUrgency::Past),
            "unknown" => Ok(AlertUrgency::Unknown),
            _ => Err(format!("Invalid alert urgency: {s}")),
        }
    }
}

/// The recommended response to an alert (`responseType` in CAP).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertResponse {
    /// Take shelter in place or per instruction.
    #[serde(rename = "Shelter")]
    Shelter,
    /// Relocate as instructed.
    #[serde(rename = "Evacuate")]
    Evacuate,
    /// Make preparations per instruction.
    #[serde(rename = "Prepare")]
    Prepare,
    /// Execute a pre-planned activity identified in the instruction.
    #[serde(rename = "Execute")]
    Execute,
    /// Avoid the subject event as per instruction.
    #[serde(rename = "Avoid")]
    Avoid,
    /// Attend to information sources as described in the instruction.
    #[serde(rename = "Monitor")]
    Monitor,
    /// Evaluate the information in this message (not for public warnings).
    #[serde(rename = "Assess")]
    Assess,
    /// The subject event no longer poses a threat or concern.
    #[serde(rename = "AllClear")]
    AllClear,
    /// No action recommended.
    #[serde(rename = "None")]
    None,
}

impl fmt::Display for AlertResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Shelter => write!(f, "Shelter"),
            Self::Evacuate => write!(f, "Evacuate"),
            Self::Prepare => write!(f, "Prepare"),
            Self::Execute => write!(f, "Execute"),
            Self::Avoid => write!(f, "Avoid"),
            Self::Monitor => write!(f, "Monitor"),
            Self::Assess => write!(f, "Assess"),
            Self::AllClear => write!(f, "AllClear"),
            Self::None => write!(f, "None"),
        }
    }
}

/// The intended distribution of an alert message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum AlertScope {
    /// For general dissemination to unrestricted audiences.
    #[serde(rename = "Public")]
    Public,
    /// For dissemination only to users with a known operational requirement.
    #[serde(rename = "Restricted")]
    Restricted,
    /// For dissemination only to specified addresses.
    #[serde(rename = "Private")]
    Private,
}

impl fmt::Display for AlertScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Public => write!(f, "Public"),
            Self::Restricted => write!(f, "Restricted"),
            Self::Private => write!(f, "Private"),
        }
    }
}

/// Counts of active alerts, from `/alerts/active/count`.
///
/// The three totals are always present; the maps are keyed by marine
/// region code, state or territory code, and zone or county id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ActiveAlertCounts {
    /// The total number of active alerts.
    pub total: u32,
    /// The number of active alerts affecting land zones.
    pub land: u32,
    /// The number of active alerts affecting marine zones.
    pub marine: u32,
    /// Active alerts by marine region code (`AL`, `AT`, `GL`, `GM`, `PA`,
    /// `PI`).
    #[serde(default)]
    pub regions: BTreeMap<String, u32>,
    /// Active alerts by state or territory code.
    #[serde(default)]
    pub areas: BTreeMap<String, u32>,
    /// Active alerts by NWS public zone or county id.
    #[serde(default)]
    pub zones: BTreeMap<String, u32>,
}

/// The event types the alert system recognizes, from `/alerts/types`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AlertEventTypes {
    /// Recognized event names, such as `Tornado Warning`.
    #[serde(default)]
    pub event_types: Vec<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    /// One alert from `tests/fixtures/alerts/list.json` (2026-09-02) with
    /// the `replacedBy`/`replacedAt` pair NOAA adds once an alert is
    /// superseded.
    const ALERT: &str = r##"{
  "@id": "https://api.weather.gov/alerts/urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1",
  "@type": "wx:Alert",
  "id": "urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1",
  "areaDesc": "Newaygo; Montcalm; Kent",
  "geocode": {
    "SAME": ["026123", "026117", "026081"],
    "UGC": ["MIZ044", "MIZ051", "MIZ057"]
  },
  "affectedZones": [
    "https://api.weather.gov/zones/forecast/MIZ044",
    "https://api.weather.gov/zones/forecast/MIZ051",
    "https://api.weather.gov/zones/forecast/MIZ057"
  ],
  "references": [],
  "sent": "2026-09-02T03:48:00-04:00",
  "effective": "2026-09-02T03:48:00-04:00",
  "onset": "2026-09-02T03:48:00-04:00",
  "expires": "2026-09-02T04:45:00-04:00",
  "ends": null,
  "status": "Actual",
  "messageType": "Alert",
  "category": "Met",
  "severity": "Moderate",
  "certainty": "Observed",
  "urgency": "Expected",
  "event": "Special Weather Statement",
  "sender": "w-nws.webmaster@noaa.gov",
  "senderName": "NWS Grand Rapids MI",
  "headline": "Special Weather Statement issued September 2 at 3:48AM EDT by NWS Grand Rapids MI",
  "description": "At 347 AM EDT, Doppler radar was tracking strong thunderstorms along\na line extending from near Howard City to 9 miles northeast of\nSparta. Movement was east at 30 mph.\n\nHAZARD...Winds in excess of 40 mph and penny size hail.\n\nSOURCE...Radar indicated.\n\nIMPACT...Gusty winds could knock down tree limbs and blow around\nunsecured objects. Minor damage to outdoor objects is\npossible.\n\nLocations impacted include...\nStanton...             Howard City...         Edmore...\nLakeview...            Cedar Springs...       Crystal...\nSidney...              Sand Lake...           Casnovia...\nMcBride...             Pierson...             Westville...\nGowen...               Cedar Lake...          Entrican...\nAmble...               Indian Lake...         Vestaburg...\nWyman...               Langston...",
  "instruction": "If outdoors, consider seeking shelter inside a building.",
  "response": "Execute",
  "note": null,
  "parameters": {
    "AWIPSidentifier": ["SPSGRR"],
    "WMOidentifier": ["WWUS83 KGRR 020748"],
    "NWSheadline": ["Strong thunderstorms will impact portions of northern Kent, southeastern Newaygo and Montcalm Counties through 445 AM EDT"],
    "eventMotionDescription": ["2026-09-02T07:47:00-00:00...storm...275DEG...27KT...43.43,-85.39 43.28,-85.62"],
    "maxWindGust": ["40 MPH"],
    "maxHailSize": ["0.75"],
    "BLOCKCHANNEL": ["EAS", "NWEM", "CMAS"],
    "EAS-ORG": ["WXR"]
  },
  "scope": "Public",
  "code": "IPAWSv1.0",
  "language": "en-US",
  "web": "http://www.weather.gov",
  "eventCode": {
    "SAME": ["SPS"],
    "NationalWeatherService": ["SPS"]
  },
  "replacedBy": "https://api.weather.gov/alerts/urn:oid:2.49.0.1.840.0.f4da6cfeb37f8f03543bc53e7427630116978049.001.1",
  "replacedAt": "2026-09-02T13:10:00-04:00"
}"##;

    fn alert() -> Alert {
        serde_json::from_str(ALERT).unwrap()
    }

    fn without(keys: &[&str]) -> Value {
        let mut value: Value = serde_json::from_str(ALERT).unwrap();
        for key in keys {
            value.as_object_mut().unwrap().remove(*key);
        }
        value
    }

    #[test]
    fn full_alert_round_trips_to_the_same_json() {
        let original: Value = serde_json::from_str(ALERT).unwrap();
        let parsed = alert();
        assert_eq!(serde_json::to_value(&parsed).unwrap(), original);
    }

    #[test]
    fn typed_fields_carry_noaa_values() {
        let alert = alert();
        assert_eq!(
            alert.id.as_str(),
            "urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1"
        );
        assert_eq!(alert.sent.to_string(), "2026-09-02T03:48:00-04:00");
        assert_eq!(alert.sent.offset().seconds(), -4 * 3600);
        assert_eq!(alert.onset, Some(alert.sent));
        assert_eq!(alert.ends, None);
        assert!(alert.sent < alert.expires);
        assert_eq!(alert.status, AlertStatus::Actual);
        assert_eq!(alert.message_type, AlertMessageType::Alert);
        assert_eq!(alert.category, AlertCategory::Met);
        assert_eq!(alert.severity, AlertSeverity::Moderate);
        assert_eq!(alert.certainty, AlertCertainty::Observed);
        assert_eq!(alert.urgency, AlertUrgency::Expected);
        assert_eq!(alert.response, Some(AlertResponse::Execute));
        assert_eq!(alert.scope, AlertScope::Public);
        assert_eq!(alert.geocode.same, ["026123", "026117", "026081"]);
        assert_eq!(alert.geocode.ugc[0].as_str(), "MIZ044");
        assert_eq!(alert.geocode.ugc[0].state(), "MI");
        assert!(alert.references.is_empty());
        assert_eq!(alert.note, None);
        assert_eq!(alert.event_code["SAME"], ["SPS"]);
        assert_eq!(
            alert.replaced_by.as_deref(),
            Some(
                "https://api.weather.gov/alerts/urn:oid:2.49.0.1.840.0.f4da6cfeb37f8f03543bc53e7427630116978049.001.1"
            )
        );
        assert_eq!(
            alert.replaced_at.unwrap().to_string(),
            "2026-09-02T13:10:00-04:00"
        );
    }

    #[test]
    fn affected_zone_ids_parse_the_last_path_segment_and_skip_failures() {
        let mut alert = alert();
        let zones: Vec<String> = alert
            .affected_zone_ids()
            .map(|zone| zone.to_string())
            .collect();
        assert_eq!(zones, ["MIZ044", "MIZ051", "MIZ057"]);

        alert.affected_zones = vec![
            "https://api.weather.gov/zones/county/MIC123/".to_owned(),
            "https://api.weather.gov/zones/forecast/".to_owned(),
            "not a zone".to_owned(),
            "".to_owned(),
        ];
        let zones: Vec<String> = alert
            .affected_zone_ids()
            .map(|zone| zone.to_string())
            .collect();
        assert_eq!(zones, ["MIC123"]);
    }

    #[test]
    fn parameter_returns_values_or_an_empty_slice() {
        let alert = alert();
        assert_eq!(alert.parameter("maxWindGust"), ["40 MPH"]);
        assert_eq!(alert.parameter("BLOCKCHANNEL"), ["EAS", "NWEM", "CMAS"]);
        assert!(alert.parameter("VTEC").is_empty());
        assert!(alert.parameter("").is_empty());
    }

    #[test]
    fn null_and_absent_optional_fields_both_read_as_none() {
        let null: Alert = serde_json::from_str(ALERT).unwrap();
        assert_eq!(null.note, None);
        let absent: Alert = serde_json::from_value(without(&[
            "note",
            "onset",
            "ends",
            "headline",
            "description",
            "instruction",
            "response",
            "code",
            "language",
            "web",
            "replacedBy",
            "replacedAt",
            "@id",
            "@type",
        ]))
        .unwrap();
        assert_eq!(absent.note, None);
        assert_eq!(absent.onset, None);
        assert_eq!(absent.headline, None);
        assert_eq!(absent.response, None);
        assert_eq!(absent.replaced_by, None);
        assert_eq!(absent.replaced_at, None);
        assert_eq!(absent.at_id, None);
        let string: Alert = serde_json::from_value({
            let mut value: Value = serde_json::from_str(ALERT).unwrap();
            value["note"] = json!("Shelter in place.");
            value
        })
        .unwrap();
        assert_eq!(string.note.as_deref(), Some("Shelter in place."));
    }

    #[test]
    fn always_sent_options_serialize_as_null_and_omitted_ones_are_skipped() {
        let alert: Alert = serde_json::from_value(without(&[
            "note",
            "onset",
            "ends",
            "headline",
            "description",
            "instruction",
            "response",
            "code",
            "language",
            "web",
            "replacedBy",
            "replacedAt",
            "@id",
            "@type",
        ]))
        .unwrap();
        let value = serde_json::to_value(&alert).unwrap();
        let object = value.as_object().unwrap();
        for key in [
            "onset",
            "ends",
            "headline",
            "description",
            "instruction",
            "response",
            "code",
            "language",
            "web",
            "note",
        ] {
            assert_eq!(object[key], Value::Null, "{key}");
        }
        for key in ["@id", "@type", "replacedBy", "replacedAt"] {
            assert!(!object.contains_key(key), "{key} should be skipped");
        }
    }

    #[test]
    fn lists_and_maps_default_to_empty() {
        let alert: Alert = serde_json::from_value(without(&[
            "geocode",
            "affectedZones",
            "references",
            "parameters",
            "eventCode",
        ]))
        .unwrap();
        assert_eq!(alert.geocode, AlertGeocode::default());
        assert!(alert.affected_zones.is_empty());
        assert!(alert.references.is_empty());
        assert!(alert.parameters.is_empty());
        assert!(alert.event_code.is_empty());
        assert_eq!(alert.affected_zone_ids().count(), 0);
        let geocode: AlertGeocode = serde_json::from_value(json!({"SAME": ["026123"]})).unwrap();
        assert!(geocode.ugc.is_empty());
    }

    #[test]
    fn missing_required_scalars_are_decode_errors() {
        for key in [
            "id",
            "areaDesc",
            "sent",
            "effective",
            "expires",
            "status",
            "messageType",
            "category",
            "severity",
            "certainty",
            "urgency",
            "event",
            "sender",
            "senderName",
            "scope",
        ] {
            let error = serde_json::from_value::<Alert>(without(&[key])).unwrap_err();
            assert!(error.to_string().contains(key), "{key}: {error}");
        }
    }

    #[test]
    fn invalid_typed_values_are_decode_errors() {
        let mut value: Value = serde_json::from_str(ALERT).unwrap();
        value["geocode"]["UGC"] = json!(["XXZ040"]);
        let error = serde_json::from_value::<Alert>(value).unwrap_err();
        assert!(error.to_string().contains("invalid zone id"), "{error}");

        let mut value: Value = serde_json::from_str(ALERT).unwrap();
        value["sent"] = json!("2026-09-02T03:48:00");
        let error = serde_json::from_value::<Alert>(value).unwrap_err();
        assert!(error.to_string().contains("invalid timestamp"), "{error}");

        let mut value: Value = serde_json::from_str(ALERT).unwrap();
        value["severity"] = json!("Catastrophic");
        assert!(serde_json::from_value::<Alert>(value).is_err());
    }

    #[test]
    fn references_carry_typed_identifier_and_sent() {
        let reference: AlertReference = serde_json::from_value(json!({
            "@id": "https://api.weather.gov/alerts/urn:oid:2.49.0.1.840.0.abc.001.1",
            "identifier": "urn:oid:2.49.0.1.840.0.abc.001.1",
            "sender": "w-nws.webmaster@noaa.gov",
            "sent": "2026-09-02T02:42:00-05:00"
        }))
        .unwrap();
        assert_eq!(
            reference.identifier.as_str(),
            "urn:oid:2.49.0.1.840.0.abc.001.1"
        );
        assert_eq!(reference.sent.to_string(), "2026-09-02T02:42:00-05:00");
        let value = serde_json::to_value(&reference).unwrap();
        assert_eq!(value["sent"], "2026-09-02T02:42:00-05:00");
        assert!(value.get("@id").is_some());
    }

    #[test]
    fn status_display_is_lowercase_and_from_str_is_case_insensitive() {
        assert_eq!(AlertStatus::Actual.to_string(), "actual");
        assert_eq!(AlertStatus::Test.to_string(), "test");
        assert_eq!(
            "ACTUAL".parse::<AlertStatus>().unwrap(),
            AlertStatus::Actual
        );
        assert_eq!("draft".parse::<AlertStatus>().unwrap(), AlertStatus::Draft);
        assert!("real".parse::<AlertStatus>().is_err());
        assert_eq!(
            serde_json::to_string(&AlertStatus::Actual).unwrap(),
            "\"Actual\""
        );
        assert_eq!(
            serde_json::from_str::<AlertStatus>("\"System\"").unwrap(),
            AlertStatus::System
        );
        assert!(serde_json::from_str::<AlertStatus>("\"actual\"").is_err());
    }

    #[test]
    fn message_type_and_urgency_parse_case_insensitively() {
        assert_eq!(
            "UPDATE".parse::<AlertMessageType>().unwrap(),
            AlertMessageType::Update
        );
        assert_eq!(AlertMessageType::Cancel.to_string(), "Cancel");
        assert!("notice".parse::<AlertMessageType>().is_err());
        assert_eq!(
            "immediate".parse::<AlertUrgency>().unwrap(),
            AlertUrgency::Immediate
        );
        assert_eq!(AlertUrgency::Past.to_string(), "Past");
        assert!("soon".parse::<AlertUrgency>().is_err());
        assert_eq!(
            serde_json::to_string(&AlertUrgency::Expected).unwrap(),
            "\"Expected\""
        );
    }

    #[test]
    fn severity_and_certainty_parse_case_sensitively() {
        assert_eq!(
            "Extreme".parse::<AlertSeverity>().unwrap(),
            AlertSeverity::Extreme
        );
        assert!("extreme".parse::<AlertSeverity>().is_err());
        assert_eq!(AlertSeverity::Minor.to_string(), "Minor");
        assert!(AlertSeverity::Extreme < AlertSeverity::Unknown);
        assert_eq!(
            "Likely".parse::<AlertCertainty>().unwrap(),
            AlertCertainty::Likely
        );
        assert!("likely".parse::<AlertCertainty>().is_err());
        assert_eq!(AlertCertainty::default(), AlertCertainty::Observed);
        assert_eq!(
            serde_json::from_str::<AlertCertainty>("\"Unlikely\"").unwrap(),
            AlertCertainty::Unlikely
        );
    }

    #[test]
    fn category_response_and_scope_keep_cap_spellings() {
        assert_eq!(AlertCategory::Cbrne.to_string(), "CBRNE");
        assert_eq!(
            serde_json::to_string(&AlertCategory::Cbrne).unwrap(),
            "\"CBRNE\""
        );
        assert_eq!(
            serde_json::from_str::<AlertCategory>("\"Env\"").unwrap(),
            AlertCategory::Env
        );
        assert_eq!(AlertResponse::AllClear.to_string(), "AllClear");
        assert_eq!(
            serde_json::from_str::<AlertResponse>("\"None\"").unwrap(),
            AlertResponse::None
        );
        assert_eq!(AlertScope::Restricted.to_string(), "Restricted");
        assert_eq!(
            serde_json::to_string(&AlertScope::Public).unwrap(),
            "\"Public\""
        );
    }

    #[test]
    fn active_alert_counts_require_totals_and_default_maps() {
        let counts: ActiveAlertCounts = serde_json::from_value(json!({
            "@context": {"@version": "1.1"},
            "total": 226,
            "land": 218,
            "marine": 8,
            "regions": {"AT": 3, "GM": 5},
            "areas": {"AZ": 12, "CA": 4},
            "zones": {"AZZ540": 2}
        }))
        .unwrap();
        assert_eq!(counts.total, 226);
        assert_eq!(counts.regions["GM"], 5);
        assert_eq!(counts.areas.len(), 2);
        assert_eq!(counts.zones["AZZ540"], 2);
        let value = serde_json::to_value(&counts).unwrap();
        assert_eq!(value["total"], 226);
        assert_eq!(value["areas"]["AZ"], 12);

        let bare: ActiveAlertCounts =
            serde_json::from_value(json!({"total": 0, "land": 0, "marine": 0})).unwrap();
        assert!(bare.regions.is_empty() && bare.areas.is_empty() && bare.zones.is_empty());
        assert!(serde_json::from_value::<ActiveAlertCounts>(json!({"total": 1})).is_err());
        assert!(
            serde_json::from_value::<ActiveAlertCounts>(
                json!({"total": -1, "land": 0, "marine": 0})
            )
            .is_err()
        );
    }

    #[test]
    fn alert_event_types_round_trip() {
        let types: AlertEventTypes = serde_json::from_value(json!({
            "@context": {"@version": "1.1"},
            "eventTypes": ["911 Telephone Outage Emergency", "Tornado Warning"]
        }))
        .unwrap();
        assert_eq!(types.event_types.len(), 2);
        assert_eq!(
            serde_json::to_value(&types).unwrap(),
            json!({"eventTypes": ["911 Telephone Outage Emergency", "Tornado Warning"]})
        );
        let empty: AlertEventTypes = serde_json::from_value(json!({})).unwrap();
        assert!(empty.event_types.is_empty());
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schemas_inline_enums_and_typed_values() {
        let schema = schemars::schema_for!(Alert);
        let value = schema.as_value();
        assert_eq!(value["type"], "object", "{value}");
        let properties = &value["properties"];
        let severities: Vec<&str> = properties["severity"]["oneOf"]
            .as_array()
            .unwrap_or_else(|| panic!("severity should be a oneOf: {value}"))
            .iter()
            .map(|variant| variant["const"].as_str().unwrap())
            .collect();
        assert_eq!(
            severities,
            ["Extreme", "Severe", "Moderate", "Minor", "Unknown"]
        );
        assert_eq!(properties["sent"]["type"], "string", "{value}");
        assert!(properties["sent"]["pattern"].is_string(), "{value}");
        // Nested structs are referenced from `$defs`; typed ids inline.
        let geocode = &value["$defs"]["AlertGeocode"]["properties"];
        assert_eq!(geocode["UGC"]["items"]["type"], "string", "{value}");
        assert!(geocode["UGC"]["items"]["pattern"].is_string(), "{value}");
        assert!(properties.get("replacedBy").is_some(), "{value}");
        let required = value["required"].as_array().unwrap();
        assert!(required.contains(&json!("event")), "{value}");
        assert!(!required.contains(&json!("headline")), "{value}");

        let counts = schemars::schema_for!(ActiveAlertCounts);
        let counts = counts.as_value();
        assert_eq!(counts["properties"]["zones"]["type"], "object", "{counts}");
        let types = schemars::schema_for!(AlertEventTypes);
        assert_eq!(
            types.as_value()["properties"]["eventTypes"]["type"],
            "array"
        );
    }
}
