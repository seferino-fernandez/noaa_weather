use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_zone`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_current_zone_forecast`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneForecastError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_zones`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneListError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_zones_by_type`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneListTypeError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_zone_observations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneObsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_stations_by_zone`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneStationsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

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
    pub fn new() -> Self {
        Default::default()
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
    pub fn new() -> Self {
        Default::default()
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
/// Returns an [`Error<ZoneError>`] if the request fails (e.g., zone not found) or the response cannot be parsed.
pub async fn get_zone(
    configuration: &configuration::Configuration,
    r#type: models::NwsZoneType,
    id: &str,
    effective: Option<String>,
) -> Result<models::ZoneGeoJson, Error<ZoneError>> {
    let uri_str = format!("{}/zones/{type}/{id}",
        configuration.base_path,
        type=r#type,
        id=crate::apis::urlencode(id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = effective {
        req_builder = req_builder.query(&[("effective", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `ZoneGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ZoneGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ZoneGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ZoneError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
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
/// Returns an [`Error<ZoneForecastError>`] if the request fails or the response cannot be parsed.
pub async fn get_current_zone_forecast(
    configuration: &configuration::Configuration,
    r#type: &str,
    id: &str,
) -> Result<models::ZoneForecastGeoJson, Error<ZoneForecastError>> {
    let uri_str = format!("{}/zones/{type}/{id}/forecast",
        configuration.base_path,
        type=crate::apis::urlencode(r#type),
        id=crate::apis::urlencode(id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `ZoneForecastGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ZoneForecastGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ZoneForecastGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ZoneForecastError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
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
/// Returns an [`Error<ZoneListError>`] if the request fails or the response cannot be parsed.
pub async fn get_zones(
    configuration: &configuration::Configuration,
    params: GetZonesParams<'_>,
) -> Result<models::ZoneCollectionGeoJson, Error<ZoneListError>> {
    let uri_str = format!("{}/zones", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    // Apply parameters from the struct
    if let Some(ref param_value) = params.id {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("id".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "id",
                &param_value
                    .iter()
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.area {
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
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.region {
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
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.r#type {
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
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.point {
        req_builder = req_builder.query(&[("point", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.include_geometry {
        req_builder = req_builder.query(&[("include_geometry", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.effective {
        req_builder = req_builder.query(&[("effective", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `ZoneCollectionGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ZoneCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ZoneCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ZoneListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
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
/// Returns an [`Error<ZoneListTypeError>`] if the request fails or the response cannot be parsed.
pub async fn get_zones_by_type(
    configuration: &configuration::Configuration,
    r#type: models::NwsZoneType,
    params: GetZonesByTypeParams<'_>,
) -> Result<models::ZoneCollectionGeoJson, Error<ZoneListTypeError>> {
    let uri_str = format!("{}/zones/{type}", configuration.base_path, type = r#type);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    // Apply parameters from the struct
    if let Some(ref param_value) = params.id {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("id".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "id",
                &param_value
                    .iter()
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.area {
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
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.region {
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
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.type_filter {
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
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = params.point {
        req_builder = req_builder.query(&[("point", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.include_geometry {
        req_builder = req_builder.query(&[("include_geometry", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.effective {
        req_builder = req_builder.query(&[("effective", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `ZoneCollectionGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ZoneCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ZoneCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ZoneListTypeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
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
/// Returns an [`Error<ZoneObsError>`] if the request fails or the response cannot be parsed.
pub async fn get_zone_observations(
    configuration: &configuration::Configuration,
    id: &str,
    start: Option<String>,
    end: Option<String>,
    limit: Option<i32>,
) -> Result<models::ObservationCollectionGeoJson, Error<ZoneObsError>> {
    let uri_str = format!(
        "{}/zones/forecast/{id}/observations",
        configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `ObservationCollectionGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ObservationCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ObservationCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ZoneObsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
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
/// Returns an [`Error<ZoneStationsError>`] if the request fails or the response cannot be parsed.
pub async fn get_stations_by_zone(
    configuration: &configuration::Configuration,
    id: &str,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationStationCollectionGeoJson, Error<ZoneStationsError>> {
    let uri_str = format!(
        "{}/zones/forecast/{id}/stations",
        configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|header| header.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `ObservationStationCollectionGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ObservationStationCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ObservationStationCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ZoneStationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
}
