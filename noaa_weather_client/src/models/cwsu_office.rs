use serde::{Deserialize, Serialize};

/// Represents a Center Weather Service Unit (CWSU) office with flat address fields as returned by the get_center_weather_service_unit endpoint.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CwsuOffice {
    /// The unique identifier for the office (e.g., "ZLA").
    pub id: Option<String>,
    /// The name of the office (e.g., "Los Angeles, CA").
    pub name: Option<String>,
    /// The city where the office is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The state where the office is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The street address of the office.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// The postal/zip code for the office.
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// The email address for the office.
    pub email: Option<String>,
    /// The fax number for the office.
    #[serde(rename = "fax", skip_serializing_if = "Option::is_none")]
    pub fax_number: Option<String>,
    /// The phone number for the office.
    #[serde(alias = "phone", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The website URL for the office.
    #[serde(rename = "webSiteUrl", skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    /// The NWS region code for the office.
    #[serde(rename = "nwsRegion", skip_serializing_if = "Option::is_none")]
    pub nws_region: Option<String>,
    /// The parent organization, if any.
    pub parent: Option<String>,
}
