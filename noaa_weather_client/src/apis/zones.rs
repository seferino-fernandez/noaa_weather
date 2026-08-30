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
    let uri_str = format!("/zones/{type}/{id}",
        type=r#type,
        id=crate::apis::urlencode(id)
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = effective {
        req_builder = req_builder.query(&[("effective", &param_value)]);
    }

    req_builder.json().await
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
    let uri_str = format!("/zones/{type}/{id}/forecast",
        type=crate::apis::urlencode(r#type),
        id=crate::apis::urlencode(id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
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
    let uri_str = "/zones".to_owned();
    let mut req_builder = http::get(configuration, &uri_str);

    // Apply parameters from the struct
    if let Some(param_value) = params.id {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("id".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[("id", &param_value.join(","))]),
        };
    }
    if let Some(param_value) = params.area {
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
    if let Some(param_value) = params.region {
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
    if let Some(param_value) = params.r#type {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("type".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "type",
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
    if let Some(param_value) = params.include_geometry {
        req_builder = req_builder.query(&[("include_geometry", &param_value.to_string())]);
    }
    if let Some(param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(param_value) = params.effective {
        req_builder = req_builder.query(&[("effective", &param_value)]);
    }

    req_builder.json().await
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
    let uri_str = format!("/zones/{type}", type = r#type);
    let mut req_builder = http::get(configuration, &uri_str);

    // Apply parameters from the struct
    if let Some(param_value) = params.id {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("id".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "id",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = params.area {
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
    if let Some(param_value) = params.region {
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
    if let Some(param_value) = params.type_filter {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("type".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "type",
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
    if let Some(param_value) = params.include_geometry {
        req_builder = req_builder.query(&[("include_geometry", &param_value.to_string())]);
    }
    if let Some(param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(param_value) = params.effective {
        req_builder = req_builder.query(&[("effective", &param_value)]);
    }

    req_builder.json().await
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
    let uri_str = format!(
        "/zones/forecast/{id}/observations",
        id = crate::apis::urlencode(id)
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = start {
        req_builder = req_builder.query(&[("start", &param_value)]);
    }
    if let Some(param_value) = end {
        req_builder = req_builder.query(&[("end", &param_value)]);
    }
    if let Some(param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }

    req_builder.json().await
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
    let uri_str = format!(
        "/zones/forecast/{id}/stations",
        id = crate::apis::urlencode(id)
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(param_value) = cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_owned())]);
    }
    req_builder.json().await
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::get_stations_by_zone;
    use crate::apis::configuration::Configuration;

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
        let configuration = Configuration::new(None, Some(server.uri()), None, None);

        get_stations_by_zone(&configuration, "AZZ540", Some(15), Some("next"))
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), Some("limit=15&cursor=next"));
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }
}
