//! API endpoint modules for the NOAA Weather API.
//!
//! Each submodule corresponds to a family of endpoints on
//! [`api.weather.gov`](https://api.weather.gov). All async functions accept a
//! [`Client`](crate::Client) as their first argument and return a model or
//! the shared [`Error`] type.

use std::{borrow::Cow, error, fmt, time::Duration};

use bytes::Bytes;
use mime::Mime;
use reqwest::StatusCode;
use url::Url;

/// The body and response metadata returned for a non-success HTTP status.
#[derive(Debug, Clone)]
pub struct ResponseContent {
    pub(crate) bytes: Bytes,
    pub(crate) status: StatusCode,
    pub(crate) url: Url,
    pub(crate) problem_detail: Option<crate::models::ProblemDetail>,
    pub(crate) content_type: Option<Mime>,
    pub(crate) retry_after: Option<Duration>,
    pub(crate) correlation_id: Option<Box<str>>,
    pub(crate) request_id: Option<Box<str>>,
    pub(crate) attempts: u8,
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

    /// Returns the server's `Retry-After` header as a delay, when present
    /// and parseable as seconds or an HTTP-date.
    #[must_use]
    pub const fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }

    /// Returns the `X-Correlation-Id` response header, when present.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Returns the `X-Request-Id` response header, when present.
    #[must_use]
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    /// Returns how many HTTP attempts were made before this response was
    /// returned; `1` when no retry happened.
    #[must_use]
    pub const fn attempts(&self) -> u8 {
        self.attempts
    }
}

/// An undecoded binary response and its metadata.
#[derive(Debug, Clone)]
pub struct BinaryPayload {
    pub(crate) bytes: Bytes,
    pub(crate) content_type: Mime,
    pub(crate) final_url: Url,
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

/// Why a redirect could not be followed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RedirectReason {
    /// The server kept redirecting past the client's hop limit.
    TooManyRedirects {
        /// Maximum number of redirects one request will follow.
        limit: u8,
    },
    /// A redirect status arrived without a `Location` header.
    MissingLocation,
    /// The `Location` header was not a usable HTTP(S) URL.
    InvalidLocation {
        /// The raw header value, lossily decoded.
        location: String,
    },
    /// The redirect would have moved an `https` request to plain `http`.
    InsecureDowngrade {
        /// The refused target URL.
        target: Url,
    },
}

impl fmt::Display for RedirectReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyRedirects { limit } => {
                write!(formatter, "more than {limit} redirects")
            }
            Self::MissingLocation => formatter.write_str("redirect without a Location header"),
            Self::InvalidLocation { location } => {
                write!(formatter, "redirect to unusable Location {location:?}")
            }
            Self::InsecureDowngrade { target } => {
                write!(formatter, "refused redirect from https to {target}")
            }
        }
    }
}

/// The response violated the client's protocol expectations.
///
/// Protocol errors are never retried: the server answered, but not in a way
/// the endpoint contract or the client's safety limits allow.
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
    /// A redirect could not be followed safely.
    Redirect {
        /// URL of the response that carried the unusable redirect.
        url: Url,
        /// Why the redirect was refused.
        reason: RedirectReason,
    },
    /// The response body exceeded the client's configured size cap.
    ResponseTooLarge {
        /// Maximum accepted body size in bytes.
        limit: usize,
        /// Final response URL after redirects.
        url: Url,
    },
}

impl ProtocolError {
    /// Returns the endpoint's expected media-type description, for
    /// content-type violations.
    #[must_use]
    pub const fn expected(&self) -> Option<&'static str> {
        match self {
            Self::MissingContentType { expected, .. }
            | Self::MalformedContentType { expected, .. }
            | Self::IncompatibleContentType { expected, .. } => Some(expected),
            Self::Redirect { .. } | Self::ResponseTooLarge { .. } => None,
        }
    }

    /// Returns the received content type, if one was present.
    #[must_use]
    pub fn actual(&self) -> Option<&str> {
        match self {
            Self::MalformedContentType { actual, .. } => Some(actual),
            Self::IncompatibleContentType { actual, .. } => Some(actual.as_ref()),
            Self::MissingContentType { .. }
            | Self::Redirect { .. }
            | Self::ResponseTooLarge { .. } => None,
        }
    }

    /// Returns the URL of the response that violated the protocol.
    #[must_use]
    pub const fn url(&self) -> &Url {
        match self {
            Self::MissingContentType { url, .. }
            | Self::MalformedContentType { url, .. }
            | Self::IncompatibleContentType { url, .. }
            | Self::Redirect { url, .. }
            | Self::ResponseTooLarge { url, .. } => url,
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
            Self::Redirect { url, reason } => {
                write!(formatter, "could not follow redirect from {url}: {reason}")
            }
            Self::ResponseTooLarge { limit, url } => write!(
                formatter,
                "response from {url} exceeded the {limit}-byte body limit"
            ),
        }
    }
}

impl error::Error for ProtocolError {}

/// Errors returned by NOAA API functions.
///
/// The type is compact and non-generic; HTTP response and protocol details
/// are boxed. Helper methods such as [`Error::status`],
/// [`Error::is_retryable`], and [`Error::retry_after`] answer common
/// questions without matching on variants.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The HTTP request failed before a complete response was available.
    Transport {
        /// The underlying reqwest failure from the last attempt.
        source: reqwest::Error,
        /// Number of attempts made, including the failed one.
        attempts: u8,
    },
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
    /// The response violated the client's protocol expectations.
    Protocol(Box<ProtocolError>),
}

impl Error {
    /// Returns the HTTP status for a non-success response error.
    #[must_use]
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Response(response) => Some(response.status()),
            _ => None,
        }
    }

    /// Returns whether the server answered `404 Not Found`.
    #[must_use]
    pub fn is_not_found(&self) -> bool {
        self.status() == Some(StatusCode::NOT_FOUND)
    }

    /// Returns whether the server answered `429 Too Many Requests`.
    #[must_use]
    pub fn is_rate_limited(&self) -> bool {
        self.status() == Some(StatusCode::TOO_MANY_REQUESTS)
    }

    /// Returns the server's `Retry-After` delay from a response error.
    #[must_use]
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Response(response) => response.retry_after(),
            _ => None,
        }
    }

    /// Returns the parsed NWS problem detail from a response error.
    #[must_use]
    pub fn problem(&self) -> Option<&crate::models::ProblemDetail> {
        match self {
            Self::Response(response) => response.problem_detail(),
            _ => None,
        }
    }

    /// Returns whether the default [`RetryPolicy`](crate::RetryPolicy) treats
    /// this failure as transient.
    ///
    /// This reports the classification only. The client may already have
    /// exhausted its attempts, or declined to wait for a long `Retry-After`;
    /// see [`Error::attempts`].
    #[must_use]
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Transport { source, .. } => crate::client::retry::retryable_transport(source),
            Self::Response(response) => crate::client::retry::retryable_status(response.status()),
            _ => false,
        }
    }

    /// Returns how many HTTP attempts were made before this error, or `1`
    /// for errors that carry no attempt count.
    #[must_use]
    pub fn attempts(&self) -> u8 {
        match self {
            Self::Transport { attempts, .. } => *attempts,
            Self::Response(response) => response.attempts(),
            _ => 1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport {
                source,
                attempts: 1,
            } => {
                write!(formatter, "HTTP transport error: {source}")
            }
            Self::Transport { source, attempts } => {
                write!(
                    formatter,
                    "HTTP transport error after {attempts} attempts: {source}"
                )
            }
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
            Self::Transport { source, .. } => Some(source),
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

pub mod alerts;
pub mod aviation;
pub mod glossary;
pub mod gridpoints;
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
    use super::Error;

    #[test]
    fn error_remains_compact() {
        let size = std::mem::size_of::<Error>();
        assert!(size <= 48, "Error occupied {size} bytes");
    }
}
