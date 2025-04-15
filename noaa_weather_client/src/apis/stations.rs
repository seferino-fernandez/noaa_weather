use super::{ContentType, Error, configuration, points};
use crate::apis::ResponseContent;
use crate::apis::points::PointError;
use crate::models::city_coordinates::USCity;
use crate::models::{self};
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`obs_station`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObsStationError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`obs_stations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObsStationsError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`station_observation_latest`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StationObservationLatestError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`station_observation_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StationObservationListError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`station_observation_time`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StationObservationTimeError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`taf`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TafError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tafs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TafsError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_city_weather`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCityWeatherError {
    PointError(models::ProblemDetail),
    StationError(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// Returns metadata about a given observation station
pub async fn obs_station(
    configuration: &configuration::Configuration,
    station_id: &str,
) -> Result<models::ObservationStationGeoJson, Error<ObsStationError>> {
    let uri_str = format!(
        "{}/stations/{stationId}",
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
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::ObservationStationGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ObservationStationGeoJson`"
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
pub async fn obs_stations(
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
                    .map(|p| ("id".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "id",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
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
                    .map(|p| ("state".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "state",
                &param_value
                    .iter()
                    .map(|p| p.to_string())
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
        .and_then(|v| v.to_str().ok())
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
        let entity: Option<ObsStationsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns the latest observation for a station
pub async fn station_observation_latest(
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
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::ObservationGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ObservationGeoJson`"
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
pub async fn station_observation_list(
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
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::ObservationCollectionGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ObservationCollectionGeoJson`"
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
pub async fn station_observation_time(
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
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `models::ObservationGeoJson`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `models::ObservationGeoJson`"
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

/// Returns a single Terminal Aerodrome Forecast.
pub async fn taf(
    configuration: &configuration::Configuration,
    station_id: &str,
    date: String,
    time: &str,
) -> Result<serde_json::Value, Error<TafError>> {
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
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
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

/// Returns Terminal Aerodrome Forecasts for the specified airport station.
pub async fn tafs(
    configuration: &configuration::Configuration,
    station_id: &str,
) -> Result<serde_json::Value, Error<TafsError>> {
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
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => Err(Error::from(serde_json::Error::custom(
                "Received `text/plain` content type response that cannot be converted to `serde_json::Value`",
            ))),
            ContentType::Unsupported(unknown_type) => {
                Err(Error::from(serde_json::Error::custom(format!(
                    "Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`"
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

/// Returns the current weather observation for a given US city
///
/// This function combines the point and station APIs to get current weather data
/// for a predefined US city.
pub async fn get_city_weather(
    configuration: &configuration::Configuration,
    city: USCity,
) -> Result<models::ObservationGeoJson, Error<GetCityWeatherError>> {
    // First get the point data to find the nearest station
    let point_data = points::point(configuration, &city.coordinate_string())
        .await
        .map_err(|e| {
            // Convert point error to GetCityWeatherError
            match e {
                Error::ResponseError(resp) => Error::ResponseError(ResponseContent {
                    status: resp.status,
                    content: resp.content,
                    entity: resp.entity.and_then(|e| match e {
                        PointError::DefaultResponse(problem) => {
                            Some(GetCityWeatherError::PointError(problem))
                        }
                        _ => None,
                    }),
                }),
                _ => Error::from(serde_json::Error::custom("Error getting point data")),
            }
        })?;

    // Get the station ID directly from the point data
    let station_id = point_data.properties.radar_station.ok_or_else(|| {
        Error::from(serde_json::Error::custom(
            "No radar station found for location",
        ))
    })?;

    // Get the latest observation for the station
    let observation = station_observation_latest(configuration, &station_id, None)
        .await
        .map_err(|e| match e {
            Error::ResponseError(resp) => Error::ResponseError(ResponseContent {
                status: resp.status,
                content: resp.content,
                entity: resp.entity.and_then(|e| match e {
                    StationObservationLatestError::DefaultResponse(problem) => {
                        Some(GetCityWeatherError::StationError(problem))
                    }
                    _ => None,
                }),
            }),
            _ => Error::from(serde_json::Error::custom(
                "Error getting station observation",
            )),
        })?;
    Ok(observation)
}
