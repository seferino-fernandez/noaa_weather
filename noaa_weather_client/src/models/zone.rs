use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    /// A geometry represented in Well-Known Text (WKT) format.
    #[serde(
        rename = "geometry",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub geometry: Option<Option<String>>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    /// UGC identifier for a NWS forecast zone or county. The first two letters will correspond to either a state code or marine area code (see #/components/schemas/StateTerritoryCode and #/components/schemas/MarineAreaCode for lists of valid letter combinations). The third letter will be Z for public/fire zone or C for county.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::NwsZoneType>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "effectiveDate", skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<models::ZoneState>>,
    #[serde(rename = "forecastOffice", skip_serializing_if = "Option::is_none")]
    pub forecast_office: Option<String>,
    #[serde(rename = "gridIdentifier", skip_serializing_if = "Option::is_none")]
    pub grid_identifier: Option<String>,
    #[serde(
        rename = "awipsLocationIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub awips_location_identifier: Option<String>,
    #[serde(rename = "cwa", skip_serializing_if = "Option::is_none")]
    pub cwa: Option<Vec<models::NwsForecastOfficeId>>,
    #[serde(rename = "forecastOffices", skip_serializing_if = "Option::is_none")]
    pub forecast_offices: Option<Vec<String>>,
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Vec<String>>,
    #[serde(
        rename = "observationStations",
        skip_serializing_if = "Option::is_none"
    )]
    pub observation_stations: Option<Vec<String>>,
    #[serde(
        rename = "radarStation",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub radar_station: Option<Option<String>>,
}

impl Zone {
    pub fn new() -> Zone {
        Zone {
            at_context: None,
            geometry: None,
            at_id: None,
            at_type: None,
            id: None,
            r#type: None,
            name: None,
            effective_date: None,
            expiration_date: None,
            state: None,
            forecast_office: None,
            grid_identifier: None,
            awips_location_identifier: None,
            cwa: None,
            forecast_offices: None,
            time_zone: None,
            observation_stations: None,
            radar_station: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "wx:Zone")]
    WxColonZone,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::WxColonZone
    }
}
