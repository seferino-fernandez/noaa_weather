//! Client configuration for connecting to the NOAA Weather API.

/// Configuration for all API requests.
///
/// Holds the base URL, HTTP client, and optional authentication credentials.
/// Use [`Default::default()`] for a ready-to-use configuration targeting
/// `https://api.weather.gov`.
///
/// # Examples
///
/// ```
/// use noaa_weather_client::Configuration;
///
/// // Default configuration — ready to use immediately.
/// let config = Configuration::default();
///
/// // Custom user agent.
/// let config = Configuration::new(
///     Some("my-app/1.0 (contact@example.com)".to_owned()),
///     None,
///     None,
///     None,
/// );
/// ```
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Base URL for the API. Defaults to `https://api.weather.gov`.
    pub base_path: String,
    /// HTTP client used for all requests.
    pub client: reqwest::Client,
    /// `User-Agent` header value. NOAA recommends a unique identifier.
    pub user_agent: Option<String>,
    /// Optional API key sent via the `X-Api-Key` header.
    pub api_key: Option<String>,
}

impl Configuration {
    /// Creates a new configuration with explicit values.
    ///
    /// Any `None` parameter falls back to its default: the production NOAA
    /// base URL, a default `reqwest::Client`, and no user agent or API key.
    pub fn new(
        user_agent: Option<String>,
        base_path: Option<String>,
        client: Option<reqwest::Client>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            base_path: base_path.unwrap_or("https://api.weather.gov".to_owned()),
            client: client.unwrap_or_default(),
            user_agent,
            api_key,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            base_path: "https://api.weather.gov".to_owned(),
            client: reqwest::Client::new(),
            user_agent: Some(
                "(noaa_weather_client_rs, com.github.noaa_weather_client_rs)".to_owned(),
            ),
            api_key: None,
        }
    }
}
