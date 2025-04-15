use super::{ContentType, Error, configuration};
use crate::apis::ResponseContent;
use crate::models;
use reqwest;
use serde::de::Error as _;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`cwa`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CwaError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cwas`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CwasError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`cwsu`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CwsuError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sigmet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sigmet_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetQueryError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sigmets_by_atsu`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetsByAtsuError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sigmets_by_atsuby_date`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigmetsByAtsubyDateError {
    DefaultResponse(models::ProblemDetail),
    UnknownValue(serde_json::Value),
}

/// Returns a list of Center Weather Advisories from a CWSU
pub async fn cwa(
    configuration: &configuration::Configuration,
    cwsu_id: models::NwsCenterWeatherServiceUnitId,
    date: String,
    sequence: i32,
) -> Result<models::CenterWeatherAdvisoryGeoJson, Error<CwaError>> {
    let uri_str = format!(
        "{}/aviation/cwsus/{cwsuId}/cwas/{date}/{sequence}",
        configuration.base_path,
        cwsuId = cwsu_id,
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
        .and_then(|v| v.to_str().ok())
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
        let entity: Option<CwaError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a list of Center Weather Advisories from a CWSU
pub async fn cwas(
    configuration: &configuration::Configuration,
    cwsu_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<models::CenterWeatherAdvisoryCollectionGeoJson, Error<CwasError>> {
    let uri_str = format!(
        "{}/aviation/cwsus/{cwsuId}/cwas",
        configuration.base_path,
        cwsuId = cwsu_id
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
        let entity: Option<CwasError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns metadata about a Center Weather Service Unit
pub async fn cwsu(
    configuration: &configuration::Configuration,
    cwsu_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<models::Office, Error<CwsuError>> {
    let uri_str = format!(
        "{}/aviation/cwsus/{cwsuId}",
        configuration.base_path,
        cwsuId = cwsu_id
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
        let entity: Option<CwsuError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Returns a specific SIGMET/AIRMET
pub async fn sigmet(
    configuration: &configuration::Configuration,
    atsu: &str,
    date: String,
    time: &str,
) -> Result<models::SigmetGeoJson, Error<SigmetError>> {
    let uri_str = format!(
        "{}/aviation/sigmets/{atsu}/{date}/{time}",
        configuration.base_path,
        atsu = crate::apis::urlencode(atsu),
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

/// Returns a list of SIGMET/AIRMETs
pub async fn sigmet_query(
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
        .and_then(|v| v.to_str().ok())
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

/// Returns a list of SIGMET/AIRMETs for the specified ATSU
pub async fn sigmets_by_atsu(
    configuration: &configuration::Configuration,
    atsu: &str,
) -> Result<models::SigmetCollectionGeoJson, Error<SigmetsByAtsuError>> {
    let uri_str = format!(
        "{}/aviation/sigmets/{atsu}",
        configuration.base_path,
        atsu = crate::apis::urlencode(atsu)
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

/// Returns a list of SIGMET/AIRMETs for the specified ATSU for the specified date
pub async fn sigmets_by_atsuby_date(
    configuration: &configuration::Configuration,
    atsu: &str,
    date: String,
) -> Result<models::SigmetCollectionGeoJson, Error<SigmetsByAtsubyDateError>> {
    let uri_str = format!(
        "{}/aviation/sigmets/{atsu}/{date}",
        configuration.base_path,
        atsu = crate::apis::urlencode(atsu),
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
        .and_then(|v| v.to_str().ok())
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
        let entity: Option<SigmetsByAtsubyDateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
