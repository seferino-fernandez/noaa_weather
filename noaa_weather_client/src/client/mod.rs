//! The HTTP client and its request policy.
//!
//! [`Client`] owns everything that applies to every NOAA request: the base
//! URL, identity headers, timeouts, redirect handling, retry policy, and the
//! response size cap. Endpoint functions under [`crate::apis`] borrow a
//! `Client` and describe only their path, query, and media type.

use std::{fmt, sync::Arc, time::Duration};

use reqwest::header::HeaderValue;
use url::Url;

pub(crate) mod http;
mod redirect;
pub(crate) mod retry;
mod secret;

pub use retry::RetryPolicy;
use secret::Secret;

#[cfg(all(test, feature = "xml"))]
pub(crate) use http::measure_allocations;

const DEFAULT_BASE_URL: &str = "https://api.weather.gov";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_MAX_RESPONSE_BYTES: usize = 32 * 1024 * 1024;

/// A configured, cheaply cloneable handle for calling the NOAA Weather API.
///
/// Create one with [`Client::builder`] and share it across tasks; clones
/// reuse the same connection pool. Every endpoint function in
/// [`crate::apis`] takes `&Client` as its first argument.
///
/// # Examples
///
/// ```
/// use noaa_weather_client::Client;
///
/// let client = Client::builder("my-weather-app/2.0 (weather@example.com)")
///     .build()?;
/// assert_eq!(client.base_url().as_str(), "https://api.weather.gov/");
/// # Ok::<(), noaa_weather_client::BuildError>(())
/// ```
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    http: reqwest::Client,
    base_url: Url,
    user_agent: Box<str>,
    api_key: Option<Secret>,
    timeout: Duration,
    connect_timeout: Duration,
    retry: RetryPolicy,
    max_response_bytes: usize,
}

impl Client {
    /// Starts building a client that identifies itself with `user_agent`.
    ///
    /// NOAA asks every caller to send a distinctive `User-Agent`, ideally
    /// including contact information. The value must be a non-empty, valid
    /// HTTP header value; its format is not otherwise policed.
    pub fn builder(user_agent: impl Into<String>) -> ClientBuilder {
        ClientBuilder {
            user_agent: user_agent.into(),
            api_key: None,
            base_url: DEFAULT_BASE_URL.to_owned(),
            timeout: DEFAULT_TIMEOUT,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            retry: RetryPolicy::default(),
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
            reqwest: None,
        }
    }

    /// Returns the validated base URL every request path is joined onto.
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.inner.base_url
    }

    /// Returns the `User-Agent` sent with every request.
    #[must_use]
    pub fn user_agent(&self) -> &str {
        &self.inner.user_agent
    }

    fn inner(&self) -> &Inner {
        &self.inner
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner();
        formatter
            .debug_struct("Client")
            .field("base_url", &inner.base_url.as_str())
            .field("user_agent", &inner.user_agent)
            .field("api_key", &ApiKeyField(inner.api_key.as_ref()))
            .field("timeout", &inner.timeout)
            .field("connect_timeout", &inner.connect_timeout)
            .field("retry", &inner.retry)
            .field("max_response_bytes", &inner.max_response_bytes)
            .finish()
    }
}

/// Prints `[redacted]` when a key is configured and `None` otherwise.
struct ApiKeyField<'a>(Option<&'a Secret>);

impl fmt::Debug for ApiKeyField<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(secret) => secret.fmt(formatter),
            None => formatter.write_str("None"),
        }
    }
}

/// Configures and validates a [`Client`].
///
/// Obtain one from [`Client::builder`]. Every setter has a documented
/// default, so `Client::builder(user_agent).build()` is a complete
/// production configuration.
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use noaa_weather_client::{Client, RetryPolicy};
///
/// let client = Client::builder("my-weather-app/2.0 (weather@example.com)")
///     .timeout(Duration::from_secs(10))
///     .retry(RetryPolicy::default().max_attempts(5))
///     .max_response_bytes(8 * 1024 * 1024)
///     .build()?;
/// # let _ = client;
/// # Ok::<(), noaa_weather_client::BuildError>(())
/// ```
pub struct ClientBuilder {
    user_agent: String,
    api_key: Option<Secret>,
    base_url: String,
    timeout: Duration,
    connect_timeout: Duration,
    retry: RetryPolicy,
    max_response_bytes: usize,
    reqwest: Option<reqwest::ClientBuilder>,
}

impl ClientBuilder {
    /// Sends `key` as the `X-Api-Key` header on requests to the base URL's
    /// origin. Default: no key.
    ///
    /// The key is dropped when a redirect leaves that origin, and it never
    /// appears in `Debug` output.
    #[must_use]
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(Secret::new(key));
        self
    }

    /// Sets the absolute `http` or `https` URL that endpoint paths are
    /// appended to. Default: `https://api.weather.gov`.
    ///
    /// The value is parsed by [`ClientBuilder::build`].
    #[must_use]
    pub fn base_url(mut self, url: impl AsRef<str>) -> Self {
        self.base_url = url.as_ref().to_owned();
        self
    }

    /// Sets the total time allowed for one attempt, including redirects and
    /// reading the body. Default: 30 seconds.
    #[must_use]
    pub const fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Sets the time allowed to establish a connection. Default: 10 seconds.
    #[must_use]
    pub const fn connect_timeout(mut self, duration: Duration) -> Self {
        self.connect_timeout = duration;
        self
    }

    /// Sets the retry policy. Default: [`RetryPolicy::default`].
    #[must_use]
    pub fn retry(mut self, policy: RetryPolicy) -> Self {
        self.retry = policy;
        self
    }

    /// Sets the largest response body, success or error, the client will
    /// buffer. Default: 32 MiB.
    ///
    /// A declared `Content-Length` above the cap is refused before any of
    /// the body is read. A streamed body without a usable length is read
    /// only until it passes the cap. Bodies are counted after transparent
    /// gzip decompression, and either case fails with
    /// [`ProtocolError::ResponseTooLarge`](crate::ProtocolError::ResponseTooLarge).
    #[must_use]
    pub const fn max_response_bytes(mut self, bytes: usize) -> Self {
        self.max_response_bytes = bytes;
        self
    }

    /// Supplies a pre-configured `reqwest::ClientBuilder` for settings this
    /// builder does not expose, such as proxies, TLS roots, or pool sizing.
    ///
    /// [`ClientBuilder::build`] always applies the client's own timeout,
    /// connect timeout, `User-Agent`, gzip decompression, and a no-follow
    /// redirect policy on top, overriding any equivalent setting made on the
    /// supplied builder.
    #[must_use]
    pub fn reqwest_builder(mut self, builder: reqwest::ClientBuilder) -> Self {
        self.reqwest = Some(builder);
        self
    }

    /// Validates the configuration and builds the client.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::InvalidUserAgent`] for an empty or non-header
    /// user agent, [`BuildError::InvalidApiKey`] for a key that is not a
    /// valid header value, [`BuildError::InvalidBaseUrl`] when the base URL
    /// is not an absolute `http` or `https` URL, and [`BuildError::Http`]
    /// when the underlying HTTP client cannot be initialized.
    pub fn build(self) -> Result<Client, BuildError> {
        let user_agent = HeaderValue::from_str(&self.user_agent)
            .ok()
            .filter(|_| !self.user_agent.trim().is_empty())
            .ok_or(BuildError::InvalidUserAgent)?;
        if let Some(api_key) = &self.api_key
            && HeaderValue::from_str(api_key.expose()).is_err()
        {
            return Err(BuildError::InvalidApiKey);
        }
        let base_url = parse_base_url(&self.base_url)?;
        let http = self
            .reqwest
            .unwrap_or_else(reqwest::Client::builder)
            .timeout(self.timeout)
            .connect_timeout(self.connect_timeout)
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(user_agent)
            .gzip(true)
            .build()
            .map_err(BuildError::Http)?;
        Ok(Client {
            inner: Arc::new(Inner {
                http,
                base_url,
                user_agent: self.user_agent.into_boxed_str(),
                api_key: self.api_key,
                timeout: self.timeout,
                connect_timeout: self.connect_timeout,
                retry: self.retry,
                max_response_bytes: self.max_response_bytes,
            }),
        })
    }
}

fn parse_base_url(text: &str) -> Result<Url, BuildError> {
    let invalid = |source| BuildError::InvalidBaseUrl {
        url: text.to_owned(),
        source,
    };
    let url = Url::parse(text).map_err(|source| invalid(Some(source)))?;
    if !matches!(url.scheme(), "http" | "https") || url.cannot_be_a_base() {
        return Err(invalid(None));
    }
    Ok(url)
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClientBuilder")
            .field("base_url", &self.base_url)
            .field("user_agent", &self.user_agent)
            .field("api_key", &ApiKeyField(self.api_key.as_ref()))
            .field("timeout", &self.timeout)
            .field("connect_timeout", &self.connect_timeout)
            .field("retry", &self.retry)
            .field("max_response_bytes", &self.max_response_bytes)
            .field("reqwest_builder", &self.reqwest.is_some())
            .finish()
    }
}

/// Why [`ClientBuilder::build`] rejected a configuration.
#[derive(Debug)]
#[non_exhaustive]
pub enum BuildError {
    /// The user agent was empty or not a valid HTTP header value.
    InvalidUserAgent,
    /// The API key was not a valid HTTP header value.
    InvalidApiKey,
    /// The base URL was not an absolute `http` or `https` URL.
    InvalidBaseUrl {
        /// The rejected text.
        url: String,
        /// The parser's complaint, when the text was not a URL at all.
        source: Option<url::ParseError>,
    },
    /// The underlying HTTP client could not be initialized.
    Http(reqwest::Error),
}

impl fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUserAgent => {
                formatter.write_str("user agent must be a non-empty HTTP header value")
            }
            Self::InvalidApiKey => formatter.write_str("API key must be a valid HTTP header value"),
            Self::InvalidBaseUrl {
                url,
                source: Some(source),
            } => write!(formatter, "base URL {url:?} is not a valid URL: {source}"),
            Self::InvalidBaseUrl { url, source: None } => {
                write!(formatter, "base URL {url:?} is not an absolute http(s) URL")
            }
            Self::Http(source) => write!(formatter, "could not initialize HTTP client: {source}"),
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidBaseUrl {
                source: Some(source),
                ..
            } => Some(source),
            Self::Http(source) => Some(source),
            Self::InvalidUserAgent | Self::InvalidApiKey | Self::InvalidBaseUrl { .. } => None,
        }
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use wiremock::MockServer;

    use super::{Client, ClientBuilder, RetryPolicy};

    pub(crate) const USER_AGENT: &str = "noaa-weather-tests/1.0";

    /// A builder pointed at `server` that never retries.
    pub(crate) fn builder_for(server: &MockServer) -> ClientBuilder {
        builder_with_base(server.uri())
    }

    /// A builder pointed at an arbitrary base URL that never retries.
    pub(crate) fn builder_with_base(base_url: impl AsRef<str>) -> ClientBuilder {
        Client::builder(USER_AGENT)
            .base_url(base_url)
            .retry(RetryPolicy::none())
    }

    /// A ready client pointed at `server` that never retries.
    pub(crate) fn client_for(server: &MockServer) -> Client {
        builder_for(server).build().unwrap()
    }

    /// A ready client for an arbitrary base URL that never retries.
    pub(crate) fn client_with_base(base_url: impl AsRef<str>) -> Client {
        builder_with_base(base_url).build().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{BuildError, Client, RetryPolicy};

    #[test]
    fn default_build_targets_production_with_documented_policy() {
        let client = Client::builder("test-agent/1.0").build().unwrap();
        assert_eq!(client.base_url().as_str(), "https://api.weather.gov/");
        assert_eq!(client.user_agent(), "test-agent/1.0");
        let inner = client.inner();
        assert_eq!(inner.timeout, Duration::from_secs(30));
        assert_eq!(inner.connect_timeout, Duration::from_secs(10));
        assert_eq!(inner.retry, RetryPolicy::default());
        assert_eq!(inner.max_response_bytes, 32 * 1024 * 1024);
        assert!(inner.api_key.is_none());
    }

    #[test]
    fn user_agent_must_be_a_non_empty_header_value() {
        assert!(matches!(
            Client::builder("").build(),
            Err(BuildError::InvalidUserAgent)
        ));
        assert!(matches!(
            Client::builder("   ").build(),
            Err(BuildError::InvalidUserAgent)
        ));
        assert!(matches!(
            Client::builder("line\nbreak").build(),
            Err(BuildError::InvalidUserAgent)
        ));
        assert!(matches!(
            Client::builder("ok/1.0").api_key("bad\nkey").build(),
            Err(BuildError::InvalidApiKey)
        ));
    }

    #[test]
    fn base_url_must_be_an_absolute_http_url() {
        for text in [
            "not a url",
            "mailto:test",
            "ftp://example.test/",
            "data:text/plain,x",
        ] {
            let error = Client::builder("ok/1.0")
                .base_url(text)
                .build()
                .unwrap_err();
            assert!(
                matches!(&error, BuildError::InvalidBaseUrl { url, .. } if url == text),
                "{text}: {error}"
            );
        }
        let ok = Client::builder("ok/1.0")
            .base_url("http://localhost:8080/prefix/")
            .build()
            .unwrap();
        assert_eq!(ok.base_url().as_str(), "http://localhost:8080/prefix/");
    }

    #[test]
    fn debug_output_redacts_the_api_key_and_describes_policy() {
        let builder = Client::builder("debug-agent/1.0")
            .api_key("super-secret")
            .timeout(Duration::from_secs(5));
        let builder_debug = format!("{builder:?}");
        assert!(
            builder_debug.contains("api_key: [redacted]"),
            "{builder_debug}"
        );
        assert!(!builder_debug.contains("super-secret"));
        assert!(builder_debug.contains("debug-agent/1.0"));

        let client = builder.build().unwrap();
        let client_debug = format!("{client:?}");
        assert!(
            client_debug.contains("api_key: [redacted]"),
            "{client_debug}"
        );
        assert!(!client_debug.contains("super-secret"));
        assert!(client_debug.contains("https://api.weather.gov/"));
        assert!(client_debug.contains("timeout: 5s"));
        assert!(client_debug.contains("RetryPolicy"));

        let anonymous = Client::builder("debug-agent/1.0").build().unwrap();
        assert!(format!("{anonymous:?}").contains("api_key: None"));
    }

    #[test]
    fn build_error_messages_are_specific() {
        assert_eq!(
            Client::builder("").build().unwrap_err().to_string(),
            "user agent must be a non-empty HTTP header value"
        );
        let invalid = Client::builder("ok/1.0")
            .base_url("not a url")
            .build()
            .unwrap_err();
        assert!(
            invalid
                .to_string()
                .starts_with("base URL \"not a url\" is not a valid URL")
        );
        let scheme = Client::builder("ok/1.0")
            .base_url("ftp://example.test/")
            .build()
            .unwrap_err();
        assert_eq!(
            scheme.to_string(),
            "base URL \"ftp://example.test/\" is not an absolute http(s) URL"
        );
    }

    #[test]
    fn client_is_send_sync_and_clone_shares_state() {
        fn assert_send_sync<T: Send + Sync + Clone + 'static>() {}
        assert_send_sync::<Client>();
        let client = Client::builder("ok/1.0").build().unwrap();
        let copy = client.clone();
        assert!(std::sync::Arc::ptr_eq(&client.inner, &copy.inner));
    }
}
