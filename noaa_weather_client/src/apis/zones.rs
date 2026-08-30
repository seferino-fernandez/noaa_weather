//! NWS forecast zones and zone-level forecasts and observations.
//!
//! Covers the `/zones` endpoints for listing zones by type, retrieving
//! zone metadata, current zone forecasts, and zone observation data.

use super::{Error, configuration, http};
use crate::models;

/// Parameters for the [`get_zones`] function.
#[derive(Clone, Debug, Default)]
pub struct GetZonesParams<'a> {
    /// Optional list of zone IDs to filter by.
    pub id: Option<Vec<String>>,
    /// Optional list of area codes ([`models::AreaCode`]) to filter by.
    pub area: Option<Vec<models::AreaCode>>,
    /// Optional list of region codes ([`models::RegionCode`]) to filter by.
    pub region: Option<Vec<models::RegionCode>>,
    /// Optional list of zone types ([`models::NwsZoneType`]) to filter by.
    pub r#type: Option<Vec<models::NwsZoneType>>,
    /// Optional point (latitude,longitude string) to find zones containing this point.
    pub point: Option<&'a str>,
    /// Optional flag to include geometry in the response (defaults to false).
    pub include_geometry: Option<bool>,
    /// Optional limit on the number of results returned.
    pub limit: Option<i32>,
    /// Optional effective date/time (ISO 8601 string) to filter zones active at this time.
    pub effective: Option<String>,
}

impl GetZonesParams<'_> {
    /// Creates a new [`GetZonesParams`] with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Parameters for the [`get_zones_by_type`] function.
#[derive(Clone, Debug, Default)]
pub struct GetZonesByTypeParams<'a> {
    /// Optional list of zone IDs to filter by.
    pub id: Option<Vec<String>>,
    /// Optional list of area codes ([`models::AreaCode`]) to filter by.
    pub area: Option<Vec<models::AreaCode>>,
    /// Optional list of region codes ([`models::RegionCode`]) to filter by.
    pub region: Option<Vec<models::RegionCode>>,
    // Note: The primary 'type' is a path parameter in the function signature.
    // This 'type_filter' corresponds to the optional 'type' query parameter.
    /// Optional *additional* list of zone types ([`models::NwsZoneType`]) to filter by.
    /// The primary type filter is passed as a path parameter to [`get_zones_by_type`].
    pub type_filter: Option<Vec<models::NwsZoneType>>,
    /// Optional point (latitude,longitude string) to find zones containing this point.
    pub point: Option<&'a str>,
    /// Optional flag to include geometry in the response (defaults to false).
    pub include_geometry: Option<bool>,
    /// Optional limit on the number of results returned.
    pub limit: Option<i32>,
    /// Optional effective date/time (ISO 8601 string) to filter zones active at this time.
    pub effective: Option<String>,
}

impl GetZonesByTypeParams<'_> {
    /// Creates a new [`GetZonesByTypeParams`] with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Returns metadata about a given zone
///
/// Corresponds to the `/zones/{type}/{zoneId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `r#type`: The type of NWS zone (e.g., Forecast, Public, Fire).
/// * `id`: The ID of the zone (e.g., "AZZ540", "WVC001").
/// * `effective`: Optional effective date/time (ISO 8601 string) for historical zone boundaries.
///
/// # Returns
///
/// A `Result` containing a [`models::ZoneGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., zone not found) or the response cannot be parsed.
pub async fn get_zone(
    configuration: &configuration::Configuration,
    r#type: models::NwsZoneType,
    id: &str,
    effective: Option<String>,
) -> Result<models::ZoneGeoJson, Error> {
    http::request(configuration, "/zones")
        .path_segment(r#type)
        .path_segment(id)
        .query_scalar("effective", effective)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns the current zone forecast for a given zone
///
/// Corresponds to the `/zones/{type}/{zoneId}/forecast` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `r#type`: The type of NWS zone as a string slice (e.g., "forecast", "public").
/// * `id`: The ID of the zone (e.g., "AZZ540", "WVC001").
///
/// # Returns
///
/// A `Result` containing a [`models::ZoneForecastGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_current_zone_forecast(
    configuration: &configuration::Configuration,
    r#type: &str,
    id: &str,
) -> Result<models::ZoneForecastGeoJson, Error> {
    http::request(configuration, "/zones")
        .path_segment(r#type)
        .path_segment(id)
        .literal_path("forecast")
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a list of zones
///
/// Corresponds to the `/zones` endpoint.
/// Supports filtering by various criteria specified in [`GetZonesParams`].
/// Supports pagination via `limit` (implicitly handled by API if cursor used).
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `params`: A [`GetZonesParams`] struct containing query parameters.
///
/// # Returns
///
/// A `Result` containing a [`models::ZoneCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_zones(
    configuration: &configuration::Configuration,
    params: GetZonesParams<'_>,
) -> Result<models::ZoneCollectionGeoJson, Error> {
    http::request(configuration, "/zones")
        .query_csv("id", params.id)
        .query_csv("area", params.area)
        .query_csv("region", params.region)
        .query_csv("type", params.r#type)
        .query_scalar("point", params.point)
        .query_scalar("include_geometry", params.include_geometry)
        .query_scalar("limit", params.limit)
        .query_scalar("effective", params.effective)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a list of zones of a given type
///
/// Corresponds to the `/zones/{type}` endpoint.
/// Supports filtering by various criteria specified in [`GetZonesByTypeParams`].
/// Supports pagination via `limit` (implicitly handled by API if cursor used).
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `r#type`: The primary type of NWS zone to retrieve (e.g., Forecast, Public).
/// * `params`: A [`GetZonesByTypeParams`] struct containing additional query parameters.
///
/// # Returns
///
/// A `Result` containing a [`models::ZoneCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_zones_by_type(
    configuration: &configuration::Configuration,
    r#type: models::NwsZoneType,
    params: GetZonesByTypeParams<'_>,
) -> Result<models::ZoneCollectionGeoJson, Error> {
    http::request(configuration, "/zones")
        .path_segment(r#type)
        .query_csv("id", params.id)
        .query_csv("area", params.area)
        .query_csv("region", params.region)
        .query_csv("type", params.type_filter)
        .query_scalar("point", params.point)
        .query_scalar("include_geometry", params.include_geometry)
        .query_scalar("limit", params.limit)
        .query_scalar("effective", params.effective)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a list of observations for a given zone
///
/// Corresponds to the `/zones/forecast/{zoneId}/observations` endpoint.
/// Note: This endpoint appears limited to *forecast* zones only based on the path.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the forecast zone (e.g., "AZZ540").
/// * `start`: Optional start time (ISO 8601 format or relative duration).
/// * `end`: Optional end time (ISO 8601 format or relative duration).
/// * `limit`: Optional limit on the number of observations returned.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_zone_observations(
    configuration: &configuration::Configuration,
    id: &str,
    start: Option<String>,
    end: Option<String>,
    limit: Option<i32>,
) -> Result<models::ObservationCollectionGeoJson, Error> {
    http::request(configuration, "/zones/forecast")
        .path_segment(id)
        .literal_path("observations")
        .query_scalar("start", start)
        .query_scalar("end", end)
        .query_scalar("limit", limit)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a list of observation stations for a given zone
///
/// Corresponds to the `/zones/forecast/{zoneId}/stations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the forecast zone (e.g., "AZZ540").
/// * `limit`: Optional limit on the number of stations returned.
/// * `cursor`: Optional pagination cursor for paginated results.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_stations_by_zone(
    configuration: &configuration::Configuration,
    id: &str,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationStationCollectionGeoJson, Error> {
    http::request(configuration, "/zones/forecast")
        .path_segment(id)
        .literal_path("stations")
        .query_scalar("limit", limit)
        .query_scalar("cursor", cursor)
        .json(http::JsonMedia::GeoJson)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{
        GetZonesByTypeParams, GetZonesParams, get_current_zone_forecast, get_stations_by_zone,
        get_zone, get_zone_observations, get_zones, get_zones_by_type,
    };
    use crate::{
        apis::configuration::Configuration,
        models::{AreaCode, NwsZoneType},
    };

    const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;

    fn configuration(server: &MockServer) -> Configuration {
        Configuration::new(None, Some(server.uri()), None, None)
    }

    async fn mount_geo_json(server: &MockServer, body: &'static str) {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/geo+json"))
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn current_forecast_encodes_dynamic_segments_and_requests_geo_json() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        get_current_zone_forecast(&configuration(&server), "public zone/type", "AZ Z/540")
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/zones/public%20zone%2Ftype/AZ%20Z%2F540/forecast"
        );
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
    }

    #[tokio::test]
    async fn zone_metadata_encodes_id_and_preserves_empty_effective() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        get_zone(
            &configuration(&server),
            NwsZoneType::Forecast,
            "AZ Z/540",
            Some(String::new()),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/zones/forecast/AZ%20Z%2F540");
        assert_eq!(requests[0].url.query(), Some("effective="));
    }

    #[tokio::test]
    async fn zones_preserve_csv_and_scalar_empty_and_omission_contracts() {
        let server = MockServer::start().await;
        mount_geo_json(&server, r#"{"type":"FeatureCollection","features":[]}"#).await;

        get_zones(
            &configuration(&server),
            GetZonesParams {
                id: Some(vec!["AZ Z/1".to_owned(), "AZZ2".to_owned()]),
                area: Some(Vec::<AreaCode>::new()),
                region: None,
                r#type: Some(vec![NwsZoneType::Forecast, NwsZoneType::Public]),
                point: Some(""),
                include_geometry: Some(false),
                limit: None,
                effective: Some("2026 / now".to_owned()),
            },
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some(
                "id=AZ+Z%2F1%2CAZZ2&area=&type=forecast%2Cpublic&point=&include_geometry=false&effective=2026+%2F+now"
            )
        );
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
    }

    #[tokio::test]
    async fn zones_by_type_preserve_literal_path_and_type_filter_name() {
        let server = MockServer::start().await;
        mount_geo_json(&server, r#"{"type":"FeatureCollection","features":[]}"#).await;

        get_zones_by_type(
            &configuration(&server),
            NwsZoneType::Public,
            GetZonesByTypeParams {
                id: Some(Vec::new()),
                type_filter: Some(vec![NwsZoneType::Fire, NwsZoneType::County]),
                limit: Some(0),
                ..GetZonesByTypeParams::default()
            },
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/zones/public");
        assert_eq!(
            requests[0].url.query(),
            Some("id=&type=fire%2Ccounty&limit=0")
        );
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
    }

    #[tokio::test]
    async fn zone_observations_keep_forecast_literal_and_query_contract() {
        let server = MockServer::start().await;
        mount_geo_json(&server, r#"{"type":"FeatureCollection","features":[]}"#).await;

        get_zone_observations(
            &configuration(&server),
            "AZ Z/540",
            Some(String::new()),
            None,
            Some(12),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/zones/forecast/AZ%20Z%2F540/observations"
        );
        assert_eq!(requests[0].url.query(), Some("start=&limit=12"));
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
    }

    #[tokio::test]
    async fn zone_stations_preserves_query_and_omits_feature_flags() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .mount(&server)
            .await;
        get_stations_by_zone(&configuration(&server), "AZ Z/540", Some(15), Some(""))
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/zones/forecast/AZ%20Z%2F540/stations"
        );
        assert_eq!(requests[0].url.query(), Some("limit=15&cursor="));
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }
}
