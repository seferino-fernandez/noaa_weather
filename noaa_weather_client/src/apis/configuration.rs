#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
}

impl Configuration {
    pub fn new(
        user_agent: Option<String>,
        base_path: Option<String>,
        client: Option<reqwest::Client>,
    ) -> Self {
        Configuration {
            base_path: base_path.unwrap_or("https://api.weather.gov".to_owned()),
            user_agent,
            client: client.unwrap_or_default(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://api.weather.gov".to_owned(),
            user_agent: Some(
                "(noaa_weather_client_rs, com.github.noaa_weather_client_rs)".to_owned(),
            ),
            client: reqwest::Client::new(),
        }
    }
}
