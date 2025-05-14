use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Office {
    #[serde(rename = "@context", skip_serializing_if = "Option::is_none")]
    pub at_context: Option<Box<models::JsonLdContext>>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub at_type: Option<AtType>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub at_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<models::OfficeAddress>,
    #[serde(
        alias = "telephone",
        alias = "phone",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number: Option<String>,
    #[serde(
        alias = "fax",
        alias = "faxNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub fax_number: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(
        alias = "webSiteUrl",
        alias = "sameAs",
        skip_serializing_if = "Option::is_none"
    )]
    pub website_url: Option<String>,
    #[serde(rename = "nwsRegion", skip_serializing_if = "Option::is_none")]
    pub nws_region: Option<String>,
    #[serde(rename = "parentOrganization", skip_serializing_if = "Option::is_none")]
    pub parent_organization: Option<String>,
    #[serde(
        rename = "responsibleCounties",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_counties: Option<Vec<String>>,
    #[serde(
        rename = "responsibleForecastZones",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_forecast_zones: Option<Vec<String>>,
    #[serde(
        rename = "responsibleFireZones",
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_fire_zones: Option<Vec<String>>,
    #[serde(
        rename = "approvedObservationStations",
        skip_serializing_if = "Option::is_none"
    )]
    pub approved_observation_stations: Option<Vec<String>>,
}

impl Office {
    pub fn new() -> Office {
        Office {
            at_context: None,
            at_type: None,
            at_id: None,
            id: None,
            name: None,
            address: None,
            phone_number: None,
            fax_number: None,
            email: None,
            website_url: None,
            nws_region: None,
            parent_organization: None,
            responsible_counties: None,
            responsible_forecast_zones: None,
            responsible_fire_zones: None,
            approved_observation_stations: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AtType {
    #[serde(rename = "GovernmentOrganization")]
    GovernmentOrganization,
}

impl Default for AtType {
    fn default() -> AtType {
        Self::GovernmentOrganization
    }
}
