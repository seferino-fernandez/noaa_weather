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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let e = match self {
            Error::Reqwest(e) => e.to_string(),
            Error::Serde(e) => e.to_string(),
            Error::Io(e) => e.to_string(),
            Error::Xml(e) => e.to_string(),
            Error::ResponseError(e) => e.content.to_string(),
        };
        write!(f, "{}", e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::Xml(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
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
            return Self::Text;
        } else if content_type.starts_with("application") && content_type.contains("xml") {
            return Self::Xml;
        } else {
            return Self::Unsupported(content_type.to_string());
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
