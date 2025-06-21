#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
    pub user_agent: Option<String>,
}

impl Configuration {
    pub fn new(
        user_agent: Option<String>,
        base_path: Option<String>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Self {
            base_path: base_path.unwrap_or("https://api.weather.gov".to_owned()),
            client: client.unwrap_or_default(),
            user_agent,
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
        }
    }
}
