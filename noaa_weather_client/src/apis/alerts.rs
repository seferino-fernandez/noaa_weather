//! Weather alerts, warnings, and watches from the NWS alert system.
//!
//! Covers the `/alerts` family of endpoints. Use [`ActiveAlertsParams`] and
//! [`GetAlertsParams`] to filter by severity, urgency, area, and more.

use super::{Error, configuration, http};
use crate::models::{self, AreaCode};

/// Parameters for the [`get_active_alerts`] function.
///
/// This struct encapsulates the optional query parameters for filtering active alerts.
#[derive(Debug, Clone, Default)]
pub struct ActiveAlertsParams<'a> {
    /// Filter by alert status (actual, exercise, system, test, draft).
    pub status: Option<Vec<models::AlertStatus>>,
    /// Filter by message type (alert, update, cancel).
    pub message_type: Option<Vec<models::AlertMessageType>>,
    /// Filter by event name (e.g., "Tornado Warning", "Flood Watch").
    pub event: Option<Vec<String>>,
    /// Filter by NWS public zone/county code or SAME code.
    pub code: Option<Vec<String>>,
    /// Filter by state/territory or marine area code.
    pub area: Option<Vec<models::AreaCode>>,
    /// Filter by point (latitude,longitude).
    pub point: Option<&'a str>,
    /// Filter by marine region code.
    pub region: Option<Vec<models::MarineRegionCode>>,
    /// Filter by region type (land or marine).
    pub region_type: Option<models::RegionType>,
    /// Filter by NWS public zone or county identifier.
    pub zone: Option<Vec<String>>,
    /// Filter by alert urgency.
    pub urgency: Option<Vec<models::AlertUrgency>>,
    /// Filter by alert severity.
    pub severity: Option<Vec<models::AlertSeverity>>,
    /// Filter by alert certainty.
    pub certainty: Option<Vec<models::AlertCertainty>>,
}

/// Parameters for the [`get_alerts`] function.
///
/// This struct encapsulates the query parameters for retrieving alerts, including filtering options and pagination.
#[derive(Debug, Clone, Default)]
pub struct GetAlertsParams<'a> {
    /// Start time for the query period (ISO 8601 format).
    pub start: Option<String>,
    /// End time for the query period (ISO 8601 format).
    pub end: Option<String>,
    /// Filter by alert status (actual, exercise, system, test, draft).
    pub status: Option<Vec<models::AlertStatus>>,
    /// Filter by alert message type (alert, update, cancel).
    pub message_type: Option<Vec<models::AlertMessageType>>,
    /// Filter by event name.
    pub event: Option<Vec<String>>,
    /// Filter by NWS public zone/county code or SAME code.
    pub code: Option<Vec<String>>,
    /// Filter by state/territory or marine area code.
    pub area: Option<Vec<models::AreaCode>>,
    /// Filter by point (latitude,longitude).
    pub point: Option<&'a str>,
    /// Filter by marine region code.
    pub region: Option<Vec<models::MarineRegionCode>>,
    /// Filter by region type (land or marine).
    pub region_type: Option<models::RegionType>,
    /// Filter by NWS public zone or county identifier.
    pub zone: Option<Vec<String>>,
    /// Filter by alert urgency.
    pub urgency: Option<Vec<models::AlertUrgency>>,
    /// Filter by alert severity.
    pub severity: Option<Vec<models::AlertSeverity>>,
    /// Filter by alert certainty.
    pub certainty: Option<Vec<models::AlertCertainty>>,
    /// Limit the number of results returned.
    pub limit: Option<i32>,
    /// Cursor for pagination to retrieve the next set of results.
    pub cursor: Option<&'a str>,
}

/// Returns all currently active alerts based on specified filter parameters.
///
/// Corresponds to the `/alerts/active` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `params`: A [`ActiveAlertsParams`] struct containing the query parameters.
///
/// # Returns
///
/// A `Result` containing a [`models::AlertCollectionGeoJson`] on success,
/// detailing the collection of active alerts matching the criteria.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_active_alerts(
    configuration: &configuration::Configuration,
    params: ActiveAlertsParams<'_>,
) -> Result<models::AlertCollectionGeoJson, Error> {
    let uri_str = "/alerts/active".to_owned();
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = &params.status {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("status".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "status",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.message_type {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("message_type".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "message_type",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.event {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("event".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "event",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.code {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("code".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "code",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.area {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("area".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "area",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = params.point {
        req_builder = req_builder.query(&[("point", &param_value.to_owned())]);
    }
    if let Some(param_value) = &params.region {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("region".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "region",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.region_type {
        req_builder = req_builder.query(&[("region_type", &param_value.to_string())]);
    }
    if let Some(param_value) = &params.zone {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("zone".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "zone",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.urgency {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("urgency".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "urgency",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.severity {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("severity".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "severity",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.certainty {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("certainty".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "certainty",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }

    req_builder.json().await
}

/// Returns active alerts for the given area (state or marine area).
///
/// Corresponds to the `/alerts/active/area/{area}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `area`: The state/territory abbreviation or marine area code (e.g., "AL", "GM", "CA").
///
/// # Returns
///
/// A `Result` containing a [`models::AlertCollectionGeoJson`] on success,
/// detailing the collection of active alerts for the specified area.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_active_alerts_for_area(
    configuration: &configuration::Configuration,
    area: &AreaCode,
) -> Result<models::AlertCollectionGeoJson, Error> {
    let uri_str = format!("/alerts/active/area/{area}", area = area);
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns info on the number of active alerts, optionally summarized by area, region, and zone.
///
/// Corresponds to the `/alerts/active/count` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
///
/// # Returns
///
/// A `Result` containing a [`models::ActiveAlertsCountResponse`] on success,
/// providing counts of active alerts.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_active_alerts_count(
    configuration: &configuration::Configuration,
) -> Result<models::ActiveAlertsCountResponse, Error> {
    let uri_str = "/alerts/active/count".to_owned();
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns active alerts for the given marine region.
///
/// Corresponds to the `/alerts/active/region/{region}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `region`: The [`models::MarineRegionCode`] for the desired marine region.
///
/// # Returns
///
/// A `Result` containing a [`models::AlertCollectionGeoJson`] on success,
/// detailing the collection of active alerts for the specified region.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_active_alerts_for_marine_region(
    configuration: &configuration::Configuration,
    region: models::MarineRegionCode,
) -> Result<models::AlertCollectionGeoJson, Error> {
    let uri_str = format!("/alerts/active/region/{region}", region = region);
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns active alerts for the given NWS public zone or county.
///
/// Corresponds to the `/alerts/active/zone/{zoneId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `zone_id`: The NWS public zone or county identifier (e.g., "CAZ043", "CAC073").
///
/// # Returns
///
/// A `Result` containing a [`models::AlertCollectionGeoJson`] on success,
/// detailing the collection of active alerts for the specified zone.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_active_alerts_for_zone(
    configuration: &configuration::Configuration,
    zone_id: &str,
) -> Result<models::AlertCollectionGeoJson, Error> {
    let uri_str = format!(
        "/alerts/active/zone/{zoneId}",
        zoneId = crate::apis::urlencode(zone_id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns all alerts matching the given parameters, including past alerts.
///
/// Corresponds to the `/alerts` endpoint.
/// Supports pagination via the `cursor` field in [`GetAlertsParams`].
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `params`: A [`GetAlertsParams`] struct containing the query parameters.
///
/// # Returns
///
/// A `Result` containing a [`models::AlertCollectionGeoJson`] on success,
/// detailing the collection of alerts matching the criteria.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_alerts(
    configuration: &configuration::Configuration,
    params: GetAlertsParams<'_>,
) -> Result<models::AlertCollectionGeoJson, Error> {
    let uri_str = "/alerts".to_owned();
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = &params.start {
        req_builder = req_builder.query(&[("start", &param_value.clone())]);
    }
    if let Some(param_value) = &params.end {
        req_builder = req_builder.query(&[("end", &param_value.clone())]);
    }
    if let Some(param_value) = &params.status {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("status".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "status",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.message_type {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("message_type".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "message_type",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.event {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("event".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "event",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.code {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("code".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "code",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.area {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("area".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "area",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.point {
        req_builder = req_builder.query(&[("point", &(*param_value).to_owned())]);
    }
    if let Some(param_value) = &params.region {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("region".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "region",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.region_type {
        req_builder = req_builder.query(&[("region_type", &param_value.to_string())]);
    }
    if let Some(param_value) = &params.zone {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("zone".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "zone",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.urgency {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("urgency".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "urgency",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.severity {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("severity".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "severity",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.certainty {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("certainty".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "certainty",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = &params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(param_value) = &params.cursor {
        req_builder = req_builder.query(&[("cursor", &(*param_value).to_owned())]);
    }

    req_builder.json().await
}

/// Returns an alert by the alert ID.
///
/// Corresponds to the `/alerts/{id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The unique identifier of the alert.
///
/// # Returns
///
/// A `Result` containing a [`models::AlertGeoJson`] on success,
/// detailing the specific alert.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails, the alert ID is not found,
/// or the response cannot be parsed.
pub async fn get_alert(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::AlertGeoJson, Error> {
    let uri_str = format!("/alerts/{id}", id = crate::apis::urlencode(id));
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a list of alert types recognized by the NWS API.
///
/// Corresponds to the `/alerts/types` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
///
/// # Returns
///
/// A `Result` containing a [`models::AlertTypesResponse`] on success,
/// listing the valid event types.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_alert_types(
    configuration: &configuration::Configuration,
) -> Result<models::AlertTypesResponse, Error> {
    let uri_str = "/alerts/types".to_owned();
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{GetAlertsParams, get_alerts};
    use crate::apis::configuration::Configuration;

    #[tokio::test]
    async fn ordinary_alert_query_never_sends_removed_active_parameter() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .mount(&server)
            .await;
        let configuration = Configuration::new(None, Some(server.uri()), None, None);

        get_alerts(
            &configuration,
            GetAlertsParams {
                start: Some("2026-08-30T00:00:00Z".to_owned()),
                ..GetAlertsParams::default()
            },
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        let query = requests[0].url.query().unwrap();
        assert!(query.starts_with("start="));
        assert!(!query.contains("active"));
    }
}
