use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_point`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_point_stations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointStationsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Returns metadata about a specific latitude/longitude point.
///
/// Corresponds to the `/points/{point}` endpoint.
/// This metadata includes the responsible forecast office, grid coordinates, and links to
/// relevant forecast endpoints for the location.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `point`: The latitude and longitude of the point, formatted as "latitude,longitude" (e.g., "39.7456,-97.0892").
///
/// # Returns
///
/// A `Result` containing a [`models::PointGeoJson`] on success, which includes the point metadata
/// in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error<PointError>`] if the request fails (e.g., invalid point format,
/// point outside CONUS) or the response cannot be parsed.
pub async fn get_point(
    configuration: &configuration::Configuration,
    point: &str,
) -> Result<models::PointGeoJson, Error<PointError>> {
    let uri_str = format!(
        "{}/points/{point}",
        configuration.base_path,
        point = crate::apis::urlencode(point)
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
    let content_type = ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `PointGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `PointGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `PointGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PointError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of observation stations potentially relevant to a given latitude/longitude point.
///
/// Corresponds to the `/points/{point}/stations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `point`: The latitude and longitude of the point, formatted as "latitude,longitude".
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<PointStationsError>`] if the request fails or the response cannot be parsed.
pub async fn get_point_stations(
    configuration: &configuration::Configuration,
    point: &str,
) -> Result<models::ObservationStationCollectionGeoJson, Error<PointStationsError>> {
    let uri_str = format!(
        "{}/points/{point}/stations",
        configuration.base_path,
        point = crate::apis::urlencode(point)
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
    let content_type = ContentType::from(content_type);

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
        let entity: Option<PointStationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
