//! API endpoint modules for the NOAA Weather API.
//!
//! Each submodule corresponds to a family of endpoints on
//! [`api.weather.gov`](https://api.weather.gov). All async functions accept a
//! [`Configuration`](configuration::Configuration) as their first argument and
//! return a model or the shared [`Error`] type.

use std::{borrow::Cow, error, fmt};

use bytes::Bytes;
use mime::Mime;
use reqwest::StatusCode;
use url::Url;

/// The body and response metadata returned for a non-success HTTP status.
#[derive(Debug, Clone)]
pub struct ResponseContent {
    bytes: Bytes,
    status: StatusCode,
    url: Url,
    problem_detail: Option<crate::models::ProblemDetail>,
    content_type: Option<Mime>,
}

impl ResponseContent {
    /// Returns the response body without copying it.
    #[must_use]
    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Returns the response body as a byte slice.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    /// Returns the response body decoded as UTF-8, replacing invalid sequences.
    #[must_use]
    pub fn text(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.bytes)
    }

    /// Returns the HTTP status code.
    #[must_use]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns the final response URL after redirects.
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Returns the parsed response content type, if the header was valid.
    #[must_use]
    pub const fn content_type(&self) -> Option<&Mime> {
        self.content_type.as_ref()
    }

    /// Returns the parsed NWS problem detail, when the body contained one.
    #[must_use]
    pub const fn problem_detail(&self) -> Option<&crate::models::ProblemDetail> {
        self.problem_detail.as_ref()
    }
}

/// An undecoded binary response and its metadata.
#[derive(Debug, Clone)]
pub struct BinaryPayload {
    bytes: Bytes,
    content_type: Mime,
    final_url: Url,
}

impl BinaryPayload {
    /// Returns the payload as a byte slice.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    /// Returns the reference-counted payload bytes.
    #[must_use]
    pub const fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Consumes the payload and returns its reference-counted bytes.
    #[must_use]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Returns the response content type.
    #[must_use]
    pub const fn content_type(&self) -> &Mime {
        &self.content_type
    }

    /// Returns the final response URL after redirects.
    #[must_use]
    pub const fn final_url(&self) -> &Url {
        &self.final_url
    }

    /// Returns the payload length in bytes.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the payload is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl AsRef<[u8]> for BinaryPayload {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// A successful response violated the endpoint's media-type contract.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ProtocolError {
    /// The response omitted its `Content-Type` header.
    MissingContentType {
        /// Media type expected by the endpoint.
        expected: &'static str,
        /// Final response URL after redirects.
        url: Url,
    },
    /// The response contained a syntactically invalid `Content-Type` header.
    MalformedContentType {
        /// Media type expected by the endpoint.
        expected: &'static str,
        /// Unparsed header value.
        actual: String,
        /// Final response URL after redirects.
        url: Url,
    },
    /// The response media type was valid but incompatible with the endpoint.
    IncompatibleContentType {
        /// Media type expected by the endpoint.
        expected: &'static str,
        /// Parsed response media type.
        actual: Mime,
        /// Final response URL after redirects.
        url: Url,
    },
}

impl ProtocolError {
    /// Returns the endpoint's expected media-type description.
    #[must_use]
    pub const fn expected(&self) -> &'static str {
        match self {
            Self::MissingContentType { expected, .. }
            | Self::MalformedContentType { expected, .. }
            | Self::IncompatibleContentType { expected, .. } => expected,
        }
    }

    /// Returns the received content type, if one was present.
    #[must_use]
    pub fn actual(&self) -> Option<&str> {
        match self {
            Self::MissingContentType { .. } => None,
            Self::MalformedContentType { actual, .. } => Some(actual),
            Self::IncompatibleContentType { actual, .. } => Some(actual.as_ref()),
        }
    }

    /// Returns the final response URL after redirects.
    #[must_use]
    pub const fn url(&self) -> &Url {
        match self {
            Self::MissingContentType { url, .. }
            | Self::MalformedContentType { url, .. }
            | Self::IncompatibleContentType { url, .. } => url,
        }
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingContentType { expected, url } => write!(
                formatter,
                "expected {expected} response from {url}, but Content-Type was missing"
            ),
            Self::MalformedContentType {
                expected,
                actual,
                url,
            } => write!(
                formatter,
                "expected {expected} response from {url}, received malformed Content-Type {actual:?}"
            ),
            Self::IncompatibleContentType {
                expected,
                actual,
                url,
            } => write!(
                formatter,
                "expected {expected} response from {url}, received incompatible Content-Type {actual}"
            ),
        }
    }
}

impl error::Error for ProtocolError {}

/// Errors returned by NOAA API functions.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The HTTP request failed before a response body was available.
    Transport(reqwest::Error),
    /// A JSON success body could not be decoded.
    Json(serde_json::Error),
    /// An XML success body could not be decoded.
    #[cfg(feature = "xml")]
    Xml(quick_xml::DeError),
    /// An IWXXM TAF body could not be normalized into forecast meaning.
    #[cfg(feature = "xml")]
    TerminalAerodromeForecast(Box<crate::models::terminal_aerodrome_forecast::TafDecodeError>),
    /// The server returned a non-success HTTP status.
    Response(Box<ResponseContent>),
    /// A successful response violated the endpoint's media-type contract.
    Protocol(Box<ProtocolError>),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(source) => write!(formatter, "HTTP transport error: {source}"),
            Self::Json(source) => write!(formatter, "JSON decode error: {source}"),
            #[cfg(feature = "xml")]
            Self::Xml(source) => write!(formatter, "XML decode error: {source}"),
            #[cfg(feature = "xml")]
            Self::TerminalAerodromeForecast(source) => {
                write!(formatter, "TAF decode error: {source}")
            }
            Self::Response(response) => write!(
                formatter,
                "HTTP {} response from {}: {}",
                response.status(),
                response.url(),
                response.text()
            ),
            Self::Protocol(source) => source.fmt(formatter),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Transport(source) => Some(source),
            Self::Json(source) => Some(source),
            #[cfg(feature = "xml")]
            Self::Xml(source) => Some(source),
            #[cfg(feature = "xml")]
            Self::TerminalAerodromeForecast(source) => Some(source.as_ref()),
            Self::Response(_) => None,
            Self::Protocol(source) => Some(source.as_ref()),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Self::Transport(source)
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Self::Json(source)
    }
}

impl From<ProtocolError> for Error {
    fn from(source: ProtocolError) -> Self {
        Self::Protocol(Box::new(source))
    }
}

impl From<ResponseContent> for Error {
    fn from(response: ResponseContent) -> Self {
        Self::Response(Box::new(response))
    }
}

#[cfg(feature = "xml")]
impl From<quick_xml::DeError> for Error {
    fn from(source: quick_xml::DeError) -> Self {
        Self::Xml(source)
    }
}

#[cfg(feature = "xml")]
impl From<crate::models::terminal_aerodrome_forecast::TafDecodeError> for Error {
    fn from(source: crate::models::terminal_aerodrome_forecast::TafDecodeError) -> Self {
        Self::TerminalAerodromeForecast(Box::new(source))
    }
}

/// Encodes one `application/x-www-form-urlencoded` name or value.
///
/// This preserves the helper's historical form/query behavior, including
/// encoding spaces as `+`. It is not a path-segment encoder; path spaces must
/// be encoded as `%20` instead.
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod alerts;
pub mod aviation;
pub mod configuration;
pub mod glossary;
pub mod gridpoints;
mod http;
#[cfg(all(test, feature = "xml"))]
pub(crate) use http::measure_allocations;
pub mod offices;
pub mod points;
pub mod products;
pub mod radar;
#[cfg(feature = "radio")]
pub mod radio;
pub mod stations;
pub mod zones;

#[cfg(test)]
mod tests {
    use super::{Error, urlencode};

    #[test]
    fn error_remains_compact() {
        let size = std::mem::size_of::<Error>();
        assert!(size <= 48, "Error occupied {size} bytes");
    }

    #[test]
    fn urlencode_preserves_public_form_encoding_behavior() {
        assert_eq!(urlencode("space slash/%"), "space+slash%2F%25");
    }
}
