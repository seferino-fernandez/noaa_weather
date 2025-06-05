use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    Xml(quick_xml::DeError),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_message = match self {
            Error::Reqwest(reqwest_error) => reqwest_error.to_string(),
            Error::Serde(serde_error) => serde_error.to_string(),
            Error::Io(io_error) => io_error.to_string(),
            Error::Xml(xml_error) => xml_error.to_string(),
            Error::ResponseError(response_error) => response_error.content.to_string(),
        };
        write!(formatter, "{error_message}")
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(reqwest_error) => reqwest_error,
            Error::Serde(serde_error) => serde_error,
            Error::Io(io_error) => io_error,
            Error::Xml(xml_error) => xml_error,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::Reqwest(reqwest_error)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(serde_error: serde_json::Error) -> Self {
        Error::Serde(serde_error)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(io_error: std::io::Error) -> Self {
        Error::Io(io_error)
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
    Xml,
    Unsupported(String),
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
            Self::Unsupported(content_type.to_string())
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
