//! API endpoint modules for the NOAA Weather API.
//!
//! Each submodule corresponds to a family of endpoints on
//! [`api.weather.gov`](https://api.weather.gov). All async functions accept a
//! [`Configuration`](configuration::Configuration) as their first argument and return
//! `Result<T, Error<E>>` where `E` is an endpoint-specific error enum.
//!
//! # Modules
//!
//! | Module | Endpoints |
//! |--------|-----------|
//! | [`alerts`] | Active and historical weather alerts |
//! | [`aviation`] | SIGMETs and Center Weather Advisories |
//! | [`gridpoints`] | Gridpoint forecasts and station lists |
//! | [`offices`] | NWS forecast office info and headlines |
//! | [`points`] | Point metadata and nearest stations |
//! | [`products`] | NWS text products (forecasts, discussions) |
//! | [`radar`] | Radar servers, stations, and data queues |
//! | [`stations`] | Observation stations, observations, and TAFs |
//! | [`zones`] | Forecast zones and zone-level forecasts |
//!
//! The [`radio`] module is available with the **`radio`** feature and provides
//! NOAA Weather Radio broadcast content in SSML format.

use std::{error, fmt};

pub(crate) const API_KEY_HEADER: &str = "X-Api-Key";

/// The raw body and status code of a non-success API response.
///
/// Returned inside [`Error::ResponseError`] when the server replies with a
/// 4xx or 5xx status. The `entity` field attempts to deserialize the body
/// into the endpoint-specific error type `T`.
#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    /// The raw response body as a string.
    pub content: String,
    /// The deserialized error payload, if parsing succeeded.
    pub entity: Option<T>,
    /// The HTTP status code of the response.
    pub status: reqwest::StatusCode,
}

/// Errors returned by API functions.
///
/// The type parameter `T` is an endpoint-specific enum (e.g.,
/// [`alerts::ActiveAlertsError`]) that captures structured error payloads from
/// the NWS API.
#[derive(Debug)]
pub enum Error<T> {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// The server returned a non-success HTTP status.
    ResponseError(ResponseContent<T>),
    /// The HTTP request itself failed (network, TLS, timeout, etc.).
    Reqwest(reqwest::Error),
    /// The JSON response body could not be deserialized.
    Serde(serde_json::Error),
    /// The XML response body could not be deserialized.
    Xml(quick_xml::DeError),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match self {
            Self::Reqwest(reqwest_error) => reqwest_error.to_string(),
            Self::Serde(serde_error) => serde_error.to_string(),
            Self::Io(io_error) => io_error.to_string(),
            Self::Xml(xml_error) => xml_error.to_string(),
            Self::ResponseError(response_error) => response_error.content.clone(),
        };
        write!(formatter, "{error_message}")
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Self::Reqwest(reqwest_error) => reqwest_error,
            Self::Serde(serde_error) => serde_error,
            Self::Io(io_error) => io_error,
            Self::Xml(xml_error) => xml_error,
            Self::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Self::Reqwest(reqwest_error)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(serde_error: serde_json::Error) -> Self {
        Self::Serde(serde_error)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(io_error: std::io::Error) -> Self {
        Self::Io(io_error)
    }
}

/// Percent-encodes a string for use in URL path segments or query parameters.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

/// A content type supported by this client.
#[derive(Debug)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
    Xml,
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text") && content_type.contains("plain") {
            Self::Text
        } else if content_type.starts_with("application") && content_type.contains("xml") {
            Self::Xml
        } else {
            Self::Unsupported(content_type.to_owned())
        }
    }
}

pub mod alerts;
pub mod aviation;
pub mod configuration;
pub mod gridpoints;
pub mod offices;
pub mod points;
pub mod products;
pub mod radar;
#[cfg(feature = "radio")]
pub mod radio;
pub mod stations;
pub mod zones;
