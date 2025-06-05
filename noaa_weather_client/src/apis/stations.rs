use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models::{self};
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_observation_station`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObsStationError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_observation_stations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObsStationsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_latest_observations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StationObservationLatestError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_observations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StationObservationListError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_observation_by_time`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StationObservationTimeError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_terminal_aerodrome_forecast`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TafError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_terminal_aerodrome_forecasts`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TafsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Returns metadata about a given observation station
///
/// Corresponds to the `/stations/{stationId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the observation station (e.g., "KPHX", "KDEN").
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<ObsStationError>`] if the request fails (e.g., station not found)
/// or the response cannot be parsed.
pub async fn get_observation_station(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::ObservationStationGeoJson, Error<ObsStationError>> {
    let uri_str = format!(
        "{}/stations/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(id)
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
                "Received `text/plain` content type response that cannot be converted to `ObservationStationGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ObservationStationGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ObservationStationGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ObsStationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of observation stations.
///
/// Corresponds to the `/stations` endpoint.
/// Supports filtering by station ID and state/territory.
/// Supports pagination via `limit` and `cursor`.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: Optional list of station IDs to filter by.
/// * `state`: Optional list of state/territory abbreviations ([`models::AreaCode`]) to filter by.
/// * `limit`: Optional limit on the number of stations returned.
/// * `cursor`: Optional pagination cursor for fetching subsequent results.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<ObsStationsError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_observation_stations(
    configuration: &configuration::Configuration,
    id: Option<Vec<String>>,
    state: Option<Vec<models::AreaCode>>,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationStationCollectionGeoJson, Error<ObsStationsError>> {
    let uri_str = format!("{}/stations", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = id {
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
    if let Some(ref param_value) = state {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("state".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "state",
                &param_value
                    .iter()
                    .map(|param| param.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
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
        let entity: Option<ObsStationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns the latest observation for a station
///
/// Corresponds to the `/stations/{stationId}/observations/latest` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the observation station.
/// * `require_qc`: Optional flag to require quality controlled data. Set to `false` by default.
///   Note that non-QC'd data is preliminary.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<StationObservationLatestError>`] if the request fails, no observation is available,
/// or the response cannot be parsed.
pub async fn get_latest_observations(
    configuration: &configuration::Configuration,
    station_id: &str,
    require_qc: Option<bool>,
) -> Result<models::ObservationGeoJson, Error<StationObservationLatestError>> {
    let uri_str = format!(
        "{}/stations/{stationId}/observations/latest",
        configuration.base_path,
        stationId = crate::apis::urlencode(station_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = require_qc {
        req_builder = req_builder.query(&[("require_qc", &param_value.to_string())]);
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
                "Received `text/plain` content type response that cannot be converted to `ObservationGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ObservationGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ObservationGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StationObservationLatestError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of observations for a given station
///
/// Corresponds to the `/stations/{stationId}/observations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the observation station.
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
/// Returns an [`Error<StationObservationListError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_observations(
    configuration: &configuration::Configuration,
    station_id: &str,
    start: Option<String>,
    end: Option<String>,
    limit: Option<i32>,
) -> Result<models::ObservationCollectionGeoJson, Error<StationObservationListError>> {
    let uri_str = format!(
        "{}/stations/{stationId}/observations",
        configuration.base_path,
        stationId = crate::apis::urlencode(station_id)
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
        let entity: Option<StationObservationListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a single observation.
///
/// Corresponds to the `/stations/{stationId}/observations/{time}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the observation station.
/// * `time`: The specific ISO 8601 timestamp of the desired observation.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error<StationObservationTimeError>`] if the request fails (e.g., no observation
/// found for the exact time) or the response cannot be parsed.
pub async fn get_observation_by_time(
    configuration: &configuration::Configuration,
    station_id: &str,
    time: String,
) -> Result<models::ObservationGeoJson, Error<StationObservationTimeError>> {
    let uri_str = format!(
        "{}/stations/{stationId}/observations/{time}",
        configuration.base_path,
        stationId = crate::apis::urlencode(station_id),
        time = time
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
                "Received `text/plain` content type response that cannot be converted to `ObservationGeoJson`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `ObservationGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `ObservationGeoJson`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<StationObservationTimeError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a single Terminal Aerodrome Forecast (TAF).
///
/// Corresponds to the `/stations/{stationId}/tafs/{date}/{time}` endpoint.
/// Note: This endpoint seems less common; typically, users fetch all current TAFs.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the airport station (typically ICAO identifier like "KPHX").
/// * `date`: The date of the TAF in `YYYY-MM-DD` format.
/// * `time`: The time of the TAF in `HHMM` format (UTC) Regex: `^([01][0-9]|2[0-3])[0-5][0-9]$`.
///
/// # Returns
///
/// A `Result` containing a [`models::TerminalAerodromeForecast`] on success, representing the TAF data.
///
/// # Errors
///
/// Returns an [`Error<TafError>`] if the request fails or the response cannot be parsed.
pub async fn get_terminal_aerodrome_forecast(
    configuration: &configuration::Configuration,
    station_id: &str,
    date: String,
    time: &str,
) -> Result<models::TerminalAerodromeForecast, Error<TafError>> {
    let uri_str = format!(
        "{}/stations/{stationId}/tafs/{date}/{time}",
        configuration.base_path,
        stationId = crate::apis::urlencode(station_id),
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
                "Received `text/plain` content type response that cannot be converted to `TerminalAerodromeForecast`",
            ))),
            ContentType::Xml => {
                let mut deserializer = quick_xml::de::Deserializer::from_str(&content);
                let taf = models::TerminalAerodromeForecast::deserialize(&mut deserializer)
                    .map_err(Error::Xml)?;
                Ok(taf)
            }
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `TerminalAerodromeForecast`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TafError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns metadata for Terminal Aerodrome Forecasts for the specified airport station.
///
/// Corresponds to the `/stations/{stationId}/tafs` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the airport station (typically ICAO identifier like "KPHX").
///
/// # Returns
///
/// A `Result` containing a [`models::TerminalAerodromeForecastsResponse`] on success, representing the TAF metadata collection.
///
/// # Errors
///
/// Returns an [`Error<TafsError>`] if the request fails or the response cannot be parsed.
pub async fn get_terminal_aerodrome_forecasts(
    configuration: &configuration::Configuration,
    station_id: &str,
) -> Result<models::TerminalAerodromeForecastsResponse, Error<TafsError>> {
    let uri_str = format!(
        "{}/stations/{stationId}/tafs",
        configuration.base_path,
        stationId = crate::apis::urlencode(station_id)
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
                "Received `text/plain` content type response that cannot be converted to `TerminalAerodromeForecastsResponse`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `TerminalAerodromeForecastsResponse`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `TerminalAerodromeForecastsResponse`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TafsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
