use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub content: String,
    pub entity: Option<T>,
    pub status: reqwest::StatusCode,
}

#[derive(Debug)]
pub enum Error<T> {
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
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
pub mod stations;
pub mod zones;
