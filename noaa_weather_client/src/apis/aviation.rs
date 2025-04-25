use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_center_weather_advisories_by_date_and_sequence`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CenterWeatherAdvisoryError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_center_weather_advisories`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CenterWeatherAdvisoryCollectionError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_center_weather_service_unit`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CenterWeatherServiceUnitError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_sigmet`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_sigmets`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetQueryError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_sigmets_by_air_traffic_service_unit`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetsByAtsuError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_sigmets_by_air_traffic_service_unit_and_date`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetsByAtsuAndDateError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Returns a specific Center Weather Advisory (CWA) identified by CWSU, date, and sequence number.
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}/cwas/{date}/{sequence}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the issuing Center Weather Service Unit (CWSU).
/// * `date`: The date of the advisory in `YYYY-MM-DD` format.
/// * `sequence`: The sequence number of the advisory (must be >= 100).
///
/// # Returns
///
/// A `Result` containing a [`models::CenterWeatherAdvisoryGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<CenterWeatherAdvisoryError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_advisories_by_date_and_sequence(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
    date: String,
    sequence: i32,
) -> Result<models::CenterWeatherAdvisoryGeoJson, Error<CenterWeatherAdvisoryError>> {
    let uri_str = format!(
        "{}/aviation/cwsus/{center_weather_service_unit_id}/cwas/{date}/{sequence}",
        configuration.base_path,
        center_weather_service_unit_id = center_weather_service_unit_id,
        date = date,
        sequence = sequence
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

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
                "Received `text/plain` content type response that cannot be converted to `models::CenterWeatherAdvisoryGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CenterWeatherAdvisoryGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CenterWeatherAdvisoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a collection of current Center Weather Advisories (CWAs) for a specific Center Weather Service Unit (CWSU).
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}/cwas` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the Center Weather Service Unit (CWSU).
///
/// # Returns
///
/// A `Result` containing a [`models::CenterWeatherAdvisoryCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<CenterWeatherAdvisoryCollectionError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_advisories(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<
    models::CenterWeatherAdvisoryCollectionGeoJson,
    Error<CenterWeatherAdvisoryCollectionError>,
> {
    let uri_str = format!(
        "{}/aviation/cwsus/{center_weather_service_unit_id}/cwas",
        configuration.base_path,
        center_weather_service_unit_id = center_weather_service_unit_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

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
                "Received `text/plain` content type response that cannot be converted to `models::CenterWeatherAdvisoryCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::CenterWeatherAdvisoryCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CenterWeatherAdvisoryCollectionError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns metadata about a specific Center Weather Service Unit (CWSU).
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the Center Weather Service Unit (CWSU).
///
/// # Returns
///
/// A `Result` containing [`models::Office`] metadata on success.
///
/// # Errors
///
/// Returns an [`Error<CenterWeatherServiceUnitError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_service_unit(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<models::Office, Error<CenterWeatherServiceUnitError>> {
    let uri_str = format!(
        "{}/aviation/cwsus/{center_weather_service_unit_id}",
        configuration.base_path,
        center_weather_service_unit_id = center_weather_service_unit_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

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
                "Received `text/plain` content type response that cannot be converted to `models::Office`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::Office`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CenterWeatherServiceUnitError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a specific SIGMET or AIRMET product.
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}/{date}/{time}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the issuing Air Traffic Service Unit (ATSU).
/// * `date`: The date of issuance in `YYYY-MM-DD` format.
/// * `time`: The time of issuance in `HHMM` format (UTC).
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<SigmetError>`] if the request fails or the response cannot be parsed.
pub async fn get_sigmet(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
    date: String,
    time: &str,
) -> Result<models::SigmetGeoJson, Error<SigmetError>> {
    let uri_str = format!(
        "{}/aviation/sigmets/{air_traffic_service_unit}/{date}/{time}",
        configuration.base_path,
        air_traffic_service_unit = crate::apis::urlencode(air_traffic_service_unit),
        date = date,
        time = crate::apis::urlencode(time)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

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
                "Received `text/plain` content type response that cannot be converted to `models::SigmetGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SigmetGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SigmetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a collection of SIGMET/AIRMET products based on query parameters.
///
/// Corresponds to the `/aviation/sigmets` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `start`: Optional start time for the query period (ISO 8601 format).
/// * `end`: Optional end time for the query period (ISO 8601 format).
/// * `date`: Optional date filter (`YYYY-MM-DD` format).
/// * `atsu`: Optional Air Traffic Service Unit (ATSU) identifier filter.
/// * `sequence`: Optional sequence number filter.
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<SigmetQueryError>`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets(
    configuration: &configuration::Configuration,
    start: Option<String>,
    end: Option<String>,
    date: Option<String>,
    atsu: Option<&str>,
    sequence: Option<&str>,
) -> Result<models::SigmetCollectionGeoJson, Error<SigmetQueryError>> {
    let uri_str = format!("{}/aviation/sigmets", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = date {
        req_builder = req_builder.query(&[("date", &param_value.to_string())]);
    }
    if let Some(ref param_value) = atsu {
        req_builder = req_builder.query(&[("atsu", &param_value.to_string())]);
    }
    if let Some(ref param_value) = sequence {
        req_builder = req_builder.query(&[("sequence", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

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
                "Received `text/plain` content type response that cannot be converted to `models::SigmetCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SigmetCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SigmetQueryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a collection of SIGMET/AIRMET products for a specific Air Traffic Service Unit (ATSU).
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the Air Traffic Service Unit (ATSU).
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<SigmetsByAtsuError>`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets_by_air_traffic_service_unit(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
) -> Result<models::SigmetCollectionGeoJson, Error<SigmetsByAtsuError>> {
    let uri_str = format!(
        "{}/aviation/sigmets/{air_traffic_service_unit}",
        configuration.base_path,
        air_traffic_service_unit = crate::apis::urlencode(air_traffic_service_unit)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

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
                "Received `text/plain` content type response that cannot be converted to `models::SigmetCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SigmetCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SigmetsByAtsuError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a collection of SIGMET/AIRMET products for a specific Air Traffic Service Unit (ATSU) on a specific date.
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}/{date}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the Air Traffic Service Unit (ATSU).
/// * `date`: The date filter in `YYYY-MM-DD` format.
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<SigmetsByAtsuAndDateError>`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets_by_air_traffic_service_unit_and_date(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
    date: String,
) -> Result<models::SigmetCollectionGeoJson, Error<SigmetsByAtsuAndDateError>> {
    let uri_str = format!(
        "{}/aviation/sigmets/{air_traffic_service_unit}/{date}",
        configuration.base_path,
        air_traffic_service_unit = crate::apis::urlencode(air_traffic_service_unit),
        date = date
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header(reqwest::header::USER_AGENT, value);
    };

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
                "Received `text/plain` content type response that cannot be converted to `models::SigmetCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::SigmetCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SigmetsByAtsuAndDateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
