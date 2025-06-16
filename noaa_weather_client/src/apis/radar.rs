use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// Errors that can occur when calling the [`get_radar_wind_profiler`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RadarWindProfilerError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_radar_data_queue`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RadarDataQueueError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_radar_server`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RadarServerError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_radar_servers`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RadarServersError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_radar_station`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RadarStationError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_radar_station_alarms`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RadarStationAlarmsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Errors that can occur when calling the [`get_radar_stations`] function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RadarStationsError {
    /// Standard NWS API problem detail response.
    DefaultResponse(models::ProblemDetail),
    /// An unexpected error occurred (e.g., invalid JSON returned by the API).
    UnknownValue(serde_json::Value),
}

/// Parameters for the [`get_radar_data_queue`] function.
///
/// This struct encapsulates the optional query parameters for filtering radar data queue entries.
#[derive(Debug, Clone, Default)]
pub struct RadarDataQueueQueryParams<'a> {
    /// Limit the number of results returned.
    pub limit: Option<i32>,
    /// Filter by arrival time range (ISO 8601 format, e.g., "start/end", "start/", "/end").
    pub arrived: Option<&'a str>,
    /// Filter by creation time range (ISO 8601 format).
    pub created: Option<&'a str>,
    /// Filter by publication time range (ISO 8601 format).
    pub published: Option<&'a str>,
    /// Filter by radar station ID.
    pub station: Option<&'a str>,
    /// Filter by data type.
    pub r#type: Option<&'a str>,
    /// Filter by feed type.
    pub feed: Option<&'a str>,
    /// Filter by resolution.
    pub resolution: Option<i32>,
}

/// Returns metadata about a given radar wind profiler station.
///
/// Corresponds to the `/radar/profilers/{id}` endpoint.
/// Optionally filters data by time and interval.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the radar wind profiler station.
/// * `time`: Optional specific time for the data (ISO 8601 format or relative time).
/// * `interval`: Optional time interval for the data (ISO 8601 duration format).
///
/// # Returns
///
/// A `Result` containing a [`serde_json::Value`] on success, representing the profiler metadata.
///
/// *Note: The exact structure of the returned JSON is unknown.*
///
/// # Errors
///
/// Returns an [`Error<RadarProfilerError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_wind_profiler(
    configuration: &configuration::Configuration,
    id: &str,
    time: Option<&str>,
    interval: Option<&str>,
) -> Result<serde_json::Value, Error<RadarWindProfilerError>> {
    let uri_str = format!(
        "{}/radar/profilers/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = time {
        req_builder = req_builder.query(&[("time", &param_value.to_string())]);
    }
    if let Some(ref param_value) = interval {
        req_builder = req_builder.query(&[("interval", &param_value.to_string())]);
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
                "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `serde_json::Value`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RadarWindProfilerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns metadata about a given radar queue on a specific host.
///
/// Corresponds to the `/radar/queues/{host}` endpoint.
/// Allows filtering queue entries by various criteria like time, station, type, etc.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `host`: The hostname of the radar queue server.
/// * `params`: A [`RadarDataQueueQueryParams`] struct containing the query parameters.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarQueuesResponse`] on success, representing the queue metadata.
///
/// # Errors
///
/// Returns an [`Error<RadarQueueError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_data_queue(
    configuration: &configuration::Configuration,
    host: &str,
    params: RadarDataQueueQueryParams<'_>,
) -> Result<models::RadarQueuesResponse, Error<RadarDataQueueError>> {
    let uri_str = format!(
        "{}/radar/queues/{host}",
        configuration.base_path,
        host = crate::apis::urlencode(host)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.arrived {
        req_builder = req_builder.query(&[("arrived", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.created {
        req_builder = req_builder.query(&[("created", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.published {
        req_builder = req_builder.query(&[("published", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.station {
        req_builder = req_builder.query(&[("station", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.r#type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.feed {
        req_builder = req_builder.query(&[("feed", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.resolution {
        req_builder = req_builder.query(&[("resolution", &param_value.to_string())]);
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
                "Received `text/plain` content type response that cannot be converted to `RadarQueuesResponse`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `RadarQueuesResponse`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadarQueuesResponse`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RadarDataQueueError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns metadata about a given radar server.
///
/// Corresponds to the `/radar/servers/{id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the radar server.
/// * `reporting_host`: Optional filter by reporting host.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarServer`] on success, representing the server metadata.
///
/// # Errors
///
/// Returns an [`Error<RadarServerError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_server(
    configuration: &configuration::Configuration,
    id: &str,
    reporting_host: Option<&str>,
) -> Result<models::RadarServer, Error<RadarServerError>> {
    let uri_str = format!(
        "{}/radar/servers/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = reporting_host {
        req_builder = req_builder.query(&[("reportingHost", &param_value.to_string())]);
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
                "Received `text/plain` content type response that cannot be converted to `RadarServer`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `RadarServer`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadarServer`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RadarServerError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of radar servers.
///
/// Corresponds to the `/radar/servers` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `reporting_host`: Optional filter by reporting host.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarServersResponse`] on success, representing the list of servers.
///
/// # Errors
///
/// Returns an [`Error<RadarServersError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_servers(
    configuration: &configuration::Configuration,
    reporting_host: Option<&str>,
) -> Result<models::RadarServersResponse, Error<RadarServersError>> {
    let uri_str = format!("{}/radar/servers", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = reporting_host {
        req_builder = req_builder.query(&[("reportingHost", &param_value.to_string())]);
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
                "Received `text/plain` content type response that cannot be converted to `RadarServersResponse`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `RadarServersResponse`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadarServersResponse"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RadarServersError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns metadata about a given radar station.
///
/// Corresponds to the `/radar/stations/{id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the radar station (e.g., "KABQ", "KMUX").
/// * `reporting_host`: Optional filter by reporting host.
/// * `host`: Optional filter by host server.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarStationFeature`] on success, representing the station metadata.
///
/// # Errors
///
/// Returns an [`Error<RadarStationError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_station(
    configuration: &configuration::Configuration,
    id: &str,
    reporting_host: Option<&str>,
    host: Option<&str>,
) -> Result<models::RadarStationFeature, Error<RadarStationError>> {
    let uri_str = format!(
        "{}/radar/stations/{id}",
        configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = reporting_host {
        req_builder = req_builder.query(&[("reportingHost", &param_value.to_string())]);
    }
    if let Some(ref param_value) = host {
        req_builder = req_builder.query(&[("host", &param_value.to_string())]);
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
                "Received `text/plain` content type response that cannot be converted to `RadarStationFeature`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `RadarStationFeature`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadarStationFeature`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RadarStationError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns alarm metadata for a given radar station.
///
/// Corresponds to the `/radar/stations/{stationId}/alarms` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the radar station.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarStationAlarmsResponse`] on success, representing the station alarms.
///
/// # Errors
///
/// Returns an [`Error<RadarStationAlarmsError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_station_alarms(
    configuration: &configuration::Configuration,
    station_id: &str,
) -> Result<models::RadarStationAlarmsResponse, Error<RadarStationAlarmsError>> {
    let uri_str = format!(
        "{}/radar/stations/{stationId}/alarms",
        configuration.base_path,
        stationId = crate::apis::urlencode(station_id)
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
                "Received `text/plain` content type response that cannot be converted to `RadarStationAlarmsResponse`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `RadarStationAlarmsResponse`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadarStationAlarmsResponse`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RadarStationAlarmsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of radar stations, optionally filtered.
///
/// Corresponds to the `/radar/stations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_type`: Optional filter by station type(s) (e.g., "WSR-88D", "TDWR").
/// * `reporting_host`: Optional filter by reporting host.
/// * `host`: Optional filter by host server.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarStationsResponse`] on success, representing the list of stations.
///
/// # Errors
///
/// Returns an [`Error<RadarStationsError>`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_stations(
    configuration: &configuration::Configuration,
    station_type: Option<Vec<String>>,
    reporting_host: Option<&str>,
    host: Option<&str>,
) -> Result<models::RadarStationsResponse, Error<RadarStationsError>> {
    let uri_str = format!("{}/radar/stations", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = station_type {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|p| ("stationType".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "stationType",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
    }
    if let Some(ref param_value) = reporting_host {
        req_builder = req_builder.query(&[("reportingHost", &param_value.to_string())]);
    }
    if let Some(ref param_value) = host {
        req_builder = req_builder.query(&[("host", &param_value.to_string())]);
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
                "Received `text/plain` content type response that cannot be converted to `RadarStationsResponse`",
            ))),
            ContentType::Xml => Err(Error::from(serde_json::Error::custom(
                "Received `application/xml` content type response that cannot be converted to `RadarStationsResponse`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `RadarStationsResponse`"
                ))))
            }
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<RadarStationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
