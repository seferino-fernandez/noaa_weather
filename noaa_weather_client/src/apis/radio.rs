use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_point_radio`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPointRadioError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_area_radio`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAreaRadioError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Returns the NOAA Weather Radio broadcast for a geographic point.
///
/// Corresponds to the `/points/{latitude},{longitude}/radio` endpoint.
/// The response is an SSML (Speech Synthesis Markup Language) document
/// containing the radio broadcast script for the area covering the given point.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `latitude`: The latitude of the point (e.g., 33.4484).
/// * `longitude`: The longitude of the point (e.g., -112.0740).
///
/// # Returns
///
/// A `Result` containing a [`models::RadioBroadcast`] on success.
///
/// # Errors
///
/// Returns an [`Error<GetPointRadioError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_point_radio(
    configuration: &configuration::Configuration,
    latitude: f64,
    longitude: f64,
) -> Result<models::RadioBroadcast, Error<GetPointRadioError>> {
    let uri_str = format!(
        "{}/points/{latitude},{longitude}/radio",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(user_agent) = &configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(api_key) = &configuration.api_key {
        req_builder = req_builder.header("X-Api-Key", api_key.clone());
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
            ContentType::Xml => {
                let mut deserializer = quick_xml::de::Deserializer::from_str(&content);
                let broadcast =
                    models::RadioBroadcast::deserialize(&mut deserializer).map_err(Error::Xml)?;
                Ok(broadcast)
            }
            ContentType::Json => Err(Error::from(serde_json::Error::custom(
                "Received `application/json` content type response that cannot be converted to `RadioBroadcast`",
            ))),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `RadioBroadcast`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadioBroadcast`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetPointRadioError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
}

/// Returns the NOAA Weather Radio broadcast for a given transmitter call sign.
///
/// Corresponds to the `/radio/{callSign}/broadcast` endpoint.
/// The response is an SSML (Speech Synthesis Markup Language) document
/// containing the current broadcast script for the specified radio transmitter.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `call_sign`: The transmitter call sign (e.g., "KEC94").
///
/// # Returns
///
/// A `Result` containing a [`models::RadioBroadcast`] on success.
///
/// # Errors
///
/// Returns an [`Error<GetAreaRadioError>`] if the request fails (e.g., call sign not found)
/// or the response cannot be parsed.
pub async fn get_area_radio(
    configuration: &configuration::Configuration,
    call_sign: &str,
) -> Result<models::RadioBroadcast, Error<GetAreaRadioError>> {
    let uri_str = format!(
        "{}/radio/{call_sign}/broadcast",
        configuration.base_path,
        call_sign = crate::apis::urlencode(call_sign)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(user_agent) = &configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(api_key) = &configuration.api_key {
        req_builder = req_builder.header("X-Api-Key", api_key.clone());
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
            ContentType::Xml => {
                let mut deserializer = quick_xml::de::Deserializer::from_str(&content);
                let broadcast =
                    models::RadioBroadcast::deserialize(&mut deserializer).map_err(Error::Xml)?;
                Ok(broadcast)
            }
            ContentType::Json => Err(Error::from(serde_json::Error::custom(
                "Received `application/json` content type response that cannot be converted to `RadioBroadcast`",
            ))),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `RadioBroadcast`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadioBroadcast`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetAreaRadioError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            content,
            entity,
            status,
        }))
    }
}
