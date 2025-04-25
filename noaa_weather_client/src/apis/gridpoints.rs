use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_gridpoint`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridpointError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_gridpoint_forecast`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridpointForecastError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_gridpoint_forecast_hourly`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridpointForecastHourlyError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_gridpoint_stations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GridpointStationsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Returns raw numerical forecast data for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}` endpoint.
/// This endpoint provides detailed forecast data layers like temperature, humidity, wind, etc.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office (e.g., TOP, LWX).
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
///
/// # Returns
///
/// A `Result` containing a [`models::GridpointGeoJson`] on success, which includes the detailed
/// forecast layers in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error<GridpointError>`] if the request fails (e.g., invalid grid coordinates)
/// or the response cannot be parsed.
pub async fn get_gridpoint(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
) -> Result<models::GridpointGeoJson, Error<GridpointError>> {
    let uri_str = format!(
        "{}/gridpoints/{forecast_office_id}/{x},{y}",
        configuration.base_path,
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
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
                "Received `text/plain` content type response that cannot be converted to `models::GridpointGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GridpointGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GridpointError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a textual forecast for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/forecast` endpoint.
/// This provides a human-readable, multi-day forecast summary.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office.
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
/// * `feature_flags`: Optional list of feature flags to enable experimental API features.
/// * `units`: Optional units for the forecast (us or si).
///
/// # Returns
///
/// A `Result` containing a [`models::GridpointForecastGeoJson`] on success, which includes
/// forecast periods with textual descriptions in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error<GridpointForecastError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_gridpoint_forecast(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    feature_flags: Option<Vec<String>>,
    units: Option<models::GridpointForecastUnits>,
) -> Result<models::GridpointForecastGeoJson, Error<GridpointForecastError>> {
    let uri_str = format!(
        "{}/gridpoints/{forecast_office_id}/{x},{y}/forecast",
        configuration.base_path,
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = units {
        req_builder = req_builder.query(&[("units", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = feature_flags {
        req_builder = req_builder.header("Feature-Flags", param_value.join(",").to_string());
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
                "Received `text/plain` content type response that cannot be converted to `models::GridpointForecastGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GridpointForecastGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GridpointForecastError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a textual hourly forecast for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/forecast/hourly` endpoint.
/// This provides a human-readable, hour-by-hour forecast summary.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office.
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
/// * `feature_flags`: Optional list of feature flags to enable experimental API features.
/// * `units`: Optional units for the forecast (us or si).
///
/// # Returns
///
/// A `Result` containing a [`models::GridpointForecastGeoJson`] on success, which includes
/// hourly forecast periods with textual descriptions in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error<GridpointForecastHourlyError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_gridpoint_forecast_hourly(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    feature_flags: Option<Vec<String>>,
    units: Option<models::GridpointForecastUnits>,
) -> Result<models::GridpointForecastGeoJson, Error<GridpointForecastHourlyError>> {
    let uri_str = format!(
        "{}/gridpoints/{forecast_office_id}/{x},{y}/forecast/hourly",
        configuration.base_path,
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = units {
        req_builder = req_builder.query(&[("units", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = feature_flags {
        req_builder = req_builder.header("Feature-Flags", param_value.join(",").to_string());
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
                "Received `text/plain` content type response that cannot be converted to `models::GridpointForecastGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::GridpointForecastGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GridpointForecastHourlyError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of observation stations usable for a given 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/stations` endpoint.
/// This helps identify nearby stations for obtaining current observations.
/// Supports pagination via `limit` and `cursor`.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office.
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
/// * `limit`: Optional limit on the number of stations returned.
/// * `cursor`: Optional pagination cursor for fetching subsequent results.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<GridpointStationsError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_gridpoint_stations(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationStationCollectionGeoJson, Error<GridpointStationsError>> {
    let uri_str = format!(
        "{}/gridpoints/{forecast_office_id}/{x},{y}/stations",
        configuration.base_path,
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
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
                "Received `text/plain` content type response that cannot be converted to `models::ObservationStationCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ObservationStationCollectionGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GridpointStationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
