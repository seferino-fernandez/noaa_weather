use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
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
    /// Forecast office identifiers responsible for the zone.
    #[serde(rename = "cwa", skip_serializing_if = "Option::is_none")]
    pub cwa: Option<Vec<String>>,
    /// API URLs for forecast offices responsible for the zone.
    #[serde(rename = "forecastOffices", skip_serializing_if = "Option::is_none")]
    pub forecast_offices: Option<Vec<String>>,
    #[serde(rename = "gridIdentifier", skip_serializing_if = "Option::is_none")]
    pub grid_identifier: Option<String>,
    #[serde(
        rename = "awipsLocationIdentifier",
        skip_serializing_if = "Option::is_none"
    )]
    pub awips_location_identifier: Option<String>,
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
            cwa: None,
            forecast_offices: None,
            grid_identifier: None,
            awips_location_identifier: None,
            time_zone: None,
            observation_stations: None,
            radar_station: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Zone;

    #[test]
    fn preserves_all_office_keys() {
        let zone: Zone = serde_json::from_str(
            r#"{"forecastOffice":"https://api.weather.gov/offices/PSR","cwa":["PSR"],"forecastOffices":["https://api.weather.gov/offices/PSR"]}"#,
        )
        .unwrap();
        assert_eq!(
            zone.forecast_office.as_deref(),
            Some("https://api.weather.gov/offices/PSR")
        );
        let serialized = serde_json::to_value(zone).unwrap();
        assert_eq!(serialized["cwa"], serde_json::json!(["PSR"]));
        assert_eq!(
            serialized["forecastOffices"],
            serde_json::json!(["https://api.weather.gov/offices/PSR"])
        );
    }
}
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum AtType {
    #[serde(rename = "wx:Zone")]
    #[default]
    WxColonZone,
}
