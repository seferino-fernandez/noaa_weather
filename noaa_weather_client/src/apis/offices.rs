use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_forecast_office`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OfficeError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_forecast_office_headline`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OfficeHeadlineError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_forecast_office_headlines`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OfficeHeadlinesError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Returns metadata about a specific NWS forecast office.
///
/// Corresponds to the `/offices/{id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The NWS forecast office ID (e.g., "TOP", "LWX").
///
/// # Returns
///
/// A `Result` containing [`models::Office`] metadata on success.
///
/// # Errors
///
/// Returns an [`Error<OfficeError>`] if the request fails (e.g., invalid office ID)
/// or the response cannot be parsed.
pub async fn get_forecast_office(
    configuration: &configuration::Configuration,
    id: &models::NwsForecastOfficeId,
) -> Result<models::Office, Error<OfficeError>> {
    let uri_str = format!("{}/offices/{id}", configuration.base_path, id = id);
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
                "Received `text/plain` content type response that cannot be converted to `Office`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `Office`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `Office`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<OfficeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a specific news headline for a given NWS forecast office.
///
/// Corresponds to the `/offices/{id}/headlines/{headlineId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The NWS forecast office ID.
/// * `headline_id`: The unique identifier of the headline.
///
/// # Returns
///
/// A `Result` containing a [`models::OfficeHeadline`] on success.
///
/// # Errors
///
/// Returns an [`Error<OfficeHeadlineError>`] if the request fails (e.g., headline not found)
/// or the response cannot be parsed.
pub async fn get_forecast_office_headline(
    configuration: &configuration::Configuration,
    id: &models::NwsForecastOfficeId,
    headline_id: &str,
) -> Result<models::OfficeHeadline, Error<OfficeHeadlineError>> {
    let uri_str = format!(
        "{}/offices/{id}/headlines/{headlineId}",
        configuration.base_path,
        id = id,
        headlineId = crate::apis::urlencode(headline_id)
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
                "Received `text/plain` content type response that cannot be converted to `OfficeHeadline`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `OfficeHeadline`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `OfficeHeadline`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<OfficeHeadlineError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a collection of recent news headlines for a given NWS forecast office.
///
/// Corresponds to the `/offices/{id}/headlines` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The NWS forecast office ID.
///
/// # Returns
///
/// A `Result` containing an [`models::OfficeHeadlineCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error<OfficeHeadlinesError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_forecast_office_headlines(
    configuration: &configuration::Configuration,
    id: &models::NwsForecastOfficeId,
) -> Result<models::OfficeHeadlineCollection, Error<OfficeHeadlinesError>> {
    let uri_str = format!(
        "{}/offices/{id}/headlines",
        configuration.base_path,
        id = id
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
                "Received `text/plain` content type response that cannot be converted to `OfficeHeadlineCollection`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `OfficeHeadlineCollection`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `OfficeHeadlineCollection`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<OfficeHeadlinesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
