use crate::models;
use serde::{Deserialize, Serialize};

/// Alert : An object representing a public alert message. Unless otherwise noted, the fields in this object correspond to the National Weather Service CAP v1.2 specification, which extends the OASIS Common Alerting Protocol (CAP) v1.2 specification and USA Integrated Public Alert and Warning System (IPAWS) Profile v1.0. Refer to this documentation for more complete information. http://docs.oasis-open.org/emergency/cap/v1.2/CAP-v1.2-os.html http://docs.oasis-open.org/emergency/cap/v1.2/ipaws-profile/v1.0/cs01/cap-v1.2-ipaws-profile-cs01.html https://alerts.weather.gov/#technical-notes-v12
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    /// The identifier of the alert message.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A textual description of the area affected by the alert.
    #[serde(rename = "areaDesc", skip_serializing_if = "Option::is_none")]
    pub area_desc: Option<String>,
    #[serde(rename = "geocode", skip_serializing_if = "Option::is_none")]
    pub geocode: Option<Box<models::AlertGeocode>>,
    /// An array of API links for zones affected by the alert. This is an API-specific extension field and is not part of the CAP specification.
    #[serde(rename = "affectedZones", skip_serializing_if = "Option::is_none")]
    pub affected_zones: Option<Vec<String>>,
    /// A list of prior alerts that this alert updates or replaces.
    #[serde(rename = "references", skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<models::AlertReferencesInner>>,
    /// The time of the origination of the alert message.
    #[serde(rename = "sent", skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
    /// The effective time of the information of the alert message.
    #[serde(rename = "effective", skip_serializing_if = "Option::is_none")]
    pub effective: Option<String>,
    /// The expected time of the beginning of the subject event of the alert message.
    #[serde(
        rename = "onset",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub onset: Option<Option<String>>,
    /// The expiry time of the information of the alert message.
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// The expected end time of the subject event of the alert message.
    #[serde(
        rename = "ends",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ends: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::AlertStatus>,
    #[serde(rename = "messageType", skip_serializing_if = "Option::is_none")]
    pub message_type: Option<models::AlertMessageType>,
    /// The code denoting the category of the subject event of the alert message.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<models::AlertSeverity>,
    #[serde(rename = "certainty", skip_serializing_if = "Option::is_none")]
    pub certainty: Option<models::AlertCertainty>,
    #[serde(rename = "urgency", skip_serializing_if = "Option::is_none")]
    pub urgency: Option<models::AlertUrgency>,
    /// The text denoting the type of the subject event of the alert message.
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Email address of the NWS webmaster.
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// The text naming the originator of the alert message.
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    /// The text headline of the alert message.
    #[serde(
        rename = "headline",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub headline: Option<Option<String>>,
    /// The text describing the subject event of the alert message.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The text describing the recommended action to be taken by recipients of the alert message.
    #[serde(
        rename = "instruction",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub instruction: Option<Option<String>>,
    /// The code denoting the type of action recommended for the target audience. This corresponds to responseType in the CAP specification.
    #[serde(rename = "response", skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,
    /// System-specific additional parameters associated with the alert message. The keys in this object correspond to parameter definitions in the NWS CAP specification.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, Vec<serde_json::Value>>>,
}

impl Alert {
    /// An object representing a public alert message. Unless otherwise noted, the fields in this object correspond to the National Weather Service CAP v1.2 specification, which extends the OASIS Common Alerting Protocol (CAP) v1.2 specification and USA Integrated Public Alert and Warning System (IPAWS) Profile v1.0. Refer to this documentation for more complete information. http://docs.oasis-open.org/emergency/cap/v1.2/CAP-v1.2-os.html http://docs.oasis-open.org/emergency/cap/v1.2/ipaws-profile/v1.0/cs01/cap-v1.2-ipaws-profile-cs01.html https://alerts.weather.gov/#technical-notes-v12
    pub fn new() -> Alert {
        Alert {
            id: None,
            area_desc: None,
            geocode: None,
            affected_zones: None,
            references: None,
            sent: None,
            effective: None,
            onset: None,
            expires: None,
            ends: None,
            status: None,
            message_type: None,
            category: None,
            severity: None,
            certainty: None,
            urgency: None,
            event: None,
            sender: None,
            sender_name: None,
            headline: None,
            description: None,
            instruction: None,
            response: None,
            parameters: None,
        }
    }
}

/// The code denoting the category of the subject event of the alert message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "Met")]
    Met,
    #[serde(rename = "Geo")]
    Geo,
    #[serde(rename = "Safety")]
    Safety,
    #[serde(rename = "Security")]
    Security,
    #[serde(rename = "Rescue")]
    Rescue,
    #[serde(rename = "Fire")]
    Fire,
    #[serde(rename = "Health")]
    Health,
    #[serde(rename = "Env")]
    Env,
    #[serde(rename = "Transport")]
    Transport,
    #[serde(rename = "Infra")]
    Infra,
    #[serde(rename = "CBRNE")]
    Cbrne,
    #[serde(rename = "Other")]
    Other,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

/// The code denoting the type of action recommended for the target audience. This corresponds to responseType in the CAP specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Response {
    #[serde(rename = "Shelter")]
    Shelter,
    #[serde(rename = "Evacuate")]
    Evacuate,
    #[serde(rename = "Prepare")]
    Prepare,
    #[serde(rename = "Execute")]
    Execute,
    #[serde(rename = "Avoid")]
    Avoid,
    #[serde(rename = "Monitor")]
    Monitor,
    #[serde(rename = "Assess")]
    Assess,
    #[serde(rename = "AllClear")]
    AllClear,
    #[serde(rename = "None")]
    None,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
