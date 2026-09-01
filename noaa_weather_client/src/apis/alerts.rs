//! Weather alerts, warnings, and watches from the NWS alert system.
//!
//! Covers the `/alerts` family of endpoints. Use [`ActiveAlertsParams`] and
//! [`GetAlertsParams`] to filter by severity, urgency, area, and more.

use super::Error;
use crate::client::{Client, http};
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
/// * `client`: The API client.
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
    client: &Client,
    params: ActiveAlertsParams<'_>,
) -> Result<models::AlertCollectionGeoJson, Error> {
    http::request(client, "/alerts/active")
        .query_csv("status", params.status)
        .query_csv("message_type", params.message_type)
        .query_csv("event", params.event)
        .query_csv("code", params.code)
        .query_csv("area", params.area)
        .query_scalar("point", params.point)
        .query_csv("region", params.region)
        .query_scalar("region_type", params.region_type)
        .query_csv("zone", params.zone)
        .query_csv("urgency", params.urgency)
        .query_csv("severity", params.severity)
        .query_csv("certainty", params.certainty)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns active alerts for the given area (state or marine area).
///
/// Corresponds to the `/alerts/active/area/{area}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    area: &AreaCode,
) -> Result<models::AlertCollectionGeoJson, Error> {
    http::request(client, "/alerts/active/area")
        .path_segment(area)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns info on the number of active alerts, optionally summarized by area, region, and zone.
///
/// Corresponds to the `/alerts/active/count` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
) -> Result<models::ActiveAlertsCountResponse, Error> {
    http::request(client, "/alerts/active/count")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns active alerts for the given marine region.
///
/// Corresponds to the `/alerts/active/region/{region}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    region: models::MarineRegionCode,
) -> Result<models::AlertCollectionGeoJson, Error> {
    http::request(client, "/alerts/active/region")
        .path_segment(region)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns active alerts for the given NWS public zone or county.
///
/// Corresponds to the `/alerts/active/zone/{zoneId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    zone_id: &str,
) -> Result<models::AlertCollectionGeoJson, Error> {
    http::request(client, "/alerts/active/zone")
        .path_segment(zone_id)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns all alerts matching the given parameters, including past alerts.
///
/// Corresponds to the `/alerts` endpoint.
/// Supports pagination via the `cursor` field in [`GetAlertsParams`].
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    params: GetAlertsParams<'_>,
) -> Result<models::AlertCollectionGeoJson, Error> {
    http::request(client, "/alerts")
        .query_scalar("start", params.start)
        .query_scalar("end", params.end)
        .query_csv("status", params.status)
        .query_csv("message_type", params.message_type)
        .query_csv("event", params.event)
        .query_csv("code", params.code)
        .query_csv("area", params.area)
        .query_scalar("point", params.point)
        .query_csv("region", params.region)
        .query_scalar("region_type", params.region_type)
        .query_csv("zone", params.zone)
        .query_csv("urgency", params.urgency)
        .query_csv("severity", params.severity)
        .query_csv("certainty", params.certainty)
        .query_scalar("limit", params.limit)
        .query_scalar("cursor", params.cursor)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns an alert by the alert ID.
///
/// Corresponds to the `/alerts/{id}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
pub async fn get_alert(client: &Client, id: &str) -> Result<models::AlertGeoJson, Error> {
    http::request(client, "/alerts")
        .path_segment(id)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a list of alert types recognized by the NWS API.
///
/// Corresponds to the `/alerts/types` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
pub async fn get_alert_types(client: &Client) -> Result<models::AlertTypesResponse, Error> {
    http::request(client, "/alerts/types")
        .json(http::JsonMedia::JsonLd)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{
        ActiveAlertsParams, GetAlertsParams, get_active_alerts, get_active_alerts_count,
        get_active_alerts_for_area, get_active_alerts_for_marine_region,
        get_active_alerts_for_zone, get_alert, get_alert_types, get_alerts,
    };
    use crate::client::test_support::client_for;
    use crate::models::{AreaCode, MarineRegionCode, StateTerritoryCode};

    #[tokio::test]
    async fn ordinary_alert_query_never_sends_removed_active_parameter() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_alerts(
            &client,
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

    #[tokio::test]
    async fn alert_id_is_one_encoded_path_segment_requested_as_geo_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/space%20slash%2Fid"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"Feature","geometry":null,"properties":{}}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_alert(&client, "space slash/id").await.unwrap();
    }

    #[tokio::test]
    async fn active_alert_filters_keep_csv_empty_and_omitted_meanings() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_active_alerts(
            &client,
            ActiveAlertsParams {
                event: Some(vec!["Flood Watch".to_owned(), "Wind/Warning".to_owned()]),
                code: None,
                point: Some(""),
                zone: Some(Vec::new()),
                ..ActiveAlertsParams::default()
            },
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("event=Flood+Watch%2CWind%2FWarning&point=&zone=")
        );
        assert_eq!(
            requests[0]
                .url
                .query_pairs()
                .filter(|(name, _)| name == "event")
                .count(),
            1
        );
        assert!(
            !requests[0]
                .url
                .query_pairs()
                .any(|(name, _)| name == "code")
        );
    }

    #[tokio::test]
    async fn active_alert_count_is_requested_as_json_ld() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active/count"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_active_alerts_count(&client).await.unwrap();
    }

    #[tokio::test]
    async fn recognized_alert_types_are_requested_as_json_ld() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/types"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_alert_types(&client).await.unwrap();
    }

    #[tokio::test]
    async fn active_alerts_for_an_area_are_requested_as_geo_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active/area/CA"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);
        let area = AreaCode::StateTerritoryCode(StateTerritoryCode::Ca);

        get_active_alerts_for_area(&client, &area).await.unwrap();
    }

    #[tokio::test]
    async fn active_alerts_for_a_marine_region_are_requested_as_geo_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active/region/GM"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_active_alerts_for_marine_region(&client, MarineRegionCode::Gm)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn active_alerts_for_a_zone_keep_the_zone_id_in_one_path_segment() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active/zone/CAZ%20043%2Falternate"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_active_alerts_for_zone(&client, "CAZ 043/alternate")
            .await
            .unwrap();
    }
}
