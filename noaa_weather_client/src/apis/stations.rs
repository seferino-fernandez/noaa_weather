//! Weather observation stations, surface observations, and TAFs.
//!
//! Covers the `/stations` endpoints for station metadata, latest and
//! historical surface observations, and Terminal Aerodrome Forecasts.

use super::{Error, configuration, http};
use crate::models;

/// Returns metadata about a given observation station
///
/// Corresponds to the `/stations/{stationId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the observation station (e.g., "KPHX", "KDEN").
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., station not found)
/// or the response cannot be parsed.
pub async fn get_observation_station(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<models::ObservationStationGeoJson, Error> {
    let uri_str = format!("/stations/{id}", id = crate::apis::urlencode(id));
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a list of observation stations.
///
/// Corresponds to the `/stations` endpoint.
/// Supports filtering by station ID and state/territory.
/// Supports pagination via `limit` and `cursor`.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: Optional list of station IDs to filter by.
/// * `state`: Optional list of state/territory abbreviations ([`models::AreaCode`]) to filter by.
/// * `limit`: Optional limit on the number of stations returned.
/// * `cursor`: Optional pagination cursor for fetching subsequent results.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_observation_stations(
    configuration: &configuration::Configuration,
    id: Option<Vec<String>>,
    state: Option<Vec<models::AreaCode>>,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationStationCollectionGeoJson, Error> {
    let uri_str = "/stations".to_owned();
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = id {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("id".to_owned(), param.clone()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "id",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = state {
        req_builder = match "csv" {
            "multi" => req_builder.query(
                &param_value
                    .iter()
                    .map(|param| ("state".to_owned(), param.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "state",
                &param_value
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(","),
            )]),
        };
    }
    if let Some(param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(param_value) = cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_owned())]);
    }
    req_builder.json().await
}

/// Returns the latest observation for a station
///
/// Corresponds to the `/stations/{stationId}/observations/latest` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the observation station.
/// * `require_quality_controlled`: Optional flag to require quality controlled data. Set to `false` by default.
///   Note that non-QC'd data is preliminary.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails, no observation is available,
/// or the response cannot be parsed.
pub async fn get_latest_observations(
    configuration: &configuration::Configuration,
    station_id: &str,
    require_quality_controlled: Option<bool>,
) -> Result<models::ObservationGeoJson, Error> {
    let uri_str = format!(
        "/stations/{stationId}/observations/latest",
        stationId = crate::apis::urlencode(station_id)
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = require_quality_controlled {
        req_builder = req_builder.query(&[("require_qc", &param_value.to_string())]);
    }

    req_builder.json().await
}

/// Returns a list of observations for a given station
///
/// Corresponds to the `/stations/{stationId}/observations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the observation station.
/// * `start`: Optional start time (ISO 8601 format or relative duration).
/// * `end`: Optional end time (ISO 8601 format or relative duration).
/// * `limit`: Optional limit on the number of observations returned.
/// * `cursor`: Optional pagination cursor for fetching subsequent results.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_observations(
    configuration: &configuration::Configuration,
    station_id: &str,
    start: Option<String>,
    end: Option<String>,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationCollectionGeoJson, Error> {
    let uri_str = format!(
        "/stations/{stationId}/observations",
        stationId = crate::apis::urlencode(station_id)
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = start {
        req_builder = req_builder.query(&[("start", &param_value.to_string())]);
    }
    if let Some(param_value) = end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(param_value) = cursor {
        req_builder = req_builder.query(&[("cursor", &param_value.to_owned())]);
    }

    req_builder.json().await
}

/// Returns a single observation.
///
/// Corresponds to the `/stations/{stationId}/observations/{time}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the observation station.
/// * `time`: The specific ISO 8601 timestamp of the desired observation.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., no observation
/// found for the exact time) or the response cannot be parsed.
pub async fn get_observation_by_time(
    configuration: &configuration::Configuration,
    station_id: &str,
    time: String,
) -> Result<models::ObservationGeoJson, Error> {
    let uri_str = format!(
        "/stations/{stationId}/observations/{time}",
        stationId = crate::apis::urlencode(station_id),
        time = time
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a single Terminal Aerodrome Forecast (TAF).
///
/// Corresponds to the `/stations/{stationId}/tafs/{date}/{time}` endpoint.
/// Note: This endpoint seems less common; typically, users fetch all current TAFs.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the airport station (typically ICAO identifier like "KPHX").
/// * `date`: The date of the TAF in `YYYY-MM-DD` format.
/// * `time`: The time of the TAF in `HHMM` format (UTC) Regex: `^([01][0-9]|2[0-3])[0-5][0-9]$`.
///
/// # Returns
///
/// A `Result` containing a [`models::TerminalAerodromeForecast`] on success, representing the TAF data.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
#[cfg(feature = "xml")]
pub async fn get_terminal_aerodrome_forecast(
    configuration: &configuration::Configuration,
    station_id: &str,
    date: String,
    time: &str,
) -> Result<models::TerminalAerodromeForecast, Error> {
    let uri_str = format!(
        "/stations/{stationId}/tafs/{date}/{time}",
        stationId = crate::apis::urlencode(station_id),
        date = date,
        time = crate::apis::urlencode(time)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.xml().await
}

/// Returns metadata for Terminal Aerodrome Forecasts for the specified airport station.
///
/// Corresponds to the `/stations/{stationId}/tafs` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the airport station (typically ICAO identifier like "KPHX").
///
/// # Returns
///
/// A `Result` containing a [`models::TerminalAerodromeForecastsResponse`] on success, representing the TAF metadata collection.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_terminal_aerodrome_forecasts(
    configuration: &configuration::Configuration,
    station_id: &str,
) -> Result<models::TerminalAerodromeForecastsResponse, Error> {
    let uri_str = format!(
        "/stations/{stationId}/tafs",
        stationId = crate::apis::urlencode(station_id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{get_observation_station, get_observation_stations};
    use crate::apis::configuration::Configuration;

    fn configuration(server: &MockServer) -> Configuration {
        Configuration::new(None, Some(server.uri()), None, None)
    }

    #[tokio::test]
    async fn station_requests_omit_feature_flags_and_preserve_queries() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .mount(&server)
            .await;

        get_observation_stations(
            &configuration(&server),
            Some(vec!["KPHX".to_owned(), "KIWA".to_owned()]),
            None,
            Some(20),
            Some("next-page"),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("id=KPHX%2CKIWA&limit=20&cursor=next-page")
        );
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }

    #[tokio::test]
    async fn single_station_request_omits_feature_flags() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"Feature","geometry":null,"properties":{}}"#,
                "application/geo+json",
            ))
            .mount(&server)
            .await;

        get_observation_station(&configuration(&server), "KPHX")
            .await
            .unwrap();
        let requests = server.received_requests().await.unwrap();
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }
}
