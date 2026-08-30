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
    http::request(configuration, "/stations")
        .path_segment(id)
        .json(http::JsonMedia::GeoJson)
        .await
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
    http::request(configuration, "/stations")
        .query_csv("id", id)
        .query_csv("state", state)
        .query_scalar("limit", limit)
        .query_scalar("cursor", cursor)
        .json(http::JsonMedia::GeoJson)
        .await
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
    http::request(configuration, "/stations")
        .path_segment(station_id)
        .literal_path("observations/latest")
        .query_scalar("require_qc", require_quality_controlled)
        .json(http::JsonMedia::GeoJson)
        .await
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
    http::request(configuration, "/stations")
        .path_segment(station_id)
        .literal_path("observations")
        .query_scalar("start", start)
        .query_scalar("end", end)
        .query_scalar("limit", limit)
        .query_scalar("cursor", cursor)
        .json(http::JsonMedia::GeoJson)
        .await
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
    http::request(configuration, "/stations")
        .path_segment(station_id)
        .literal_path("observations")
        .path_segment(time)
        .json(http::JsonMedia::GeoJson)
        .await
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
    http::request(configuration, "/stations")
        .path_segment(station_id)
        .literal_path("tafs")
        .path_segment(date)
        .path_segment(time)
        .xml(http::XmlMedia::Iwxxm)
        .await
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
    http::request(configuration, "/stations")
        .path_segment(station_id)
        .literal_path("tafs")
        .json(http::JsonMedia::JsonLd)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use super::{get_observation_by_time, get_observation_station, get_observation_stations};
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
            Some(vec!["AZ".parse().unwrap(), "CA".parse().unwrap()]),
            Some(20),
            Some("next-page"),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("id=KPHX%2CKIWA&state=AZ%2CCA&limit=20&cursor=next-page")
        );
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/geo+json"
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

        get_observation_station(&configuration(&server), "K/PHX%")
            .await
            .unwrap();
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/stations/K%2FPHX%25");
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/geo+json"
        );
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }

    #[tokio::test]
    async fn observation_path_encodes_station_and_time_as_distinct_segments() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;

        let result = get_observation_by_time(
            &configuration(&server),
            "K/PHX%",
            "2026-08-30T12:34:56Z/path%".to_owned(),
        )
        .await;

        assert!(result.is_err());
        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/stations/K%2FPHX%25/observations/2026-08-30T12:34:56Z%2Fpath%25"
        );
    }

    #[tokio::test]
    async fn remaining_station_routes_preserve_queries_and_media_contracts() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/observations/latest"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"Feature","geometry":null,"properties":{}}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/observations"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/tafs"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(r#"{"@graph":[]}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        super::get_latest_observations(&configuration(&server), "KPHX", Some(false))
            .await
            .unwrap();
        super::get_observations(
            &configuration(&server),
            "KPHX",
            Some("2026-08-30T00:00:00Z".to_owned()),
            None,
            Some(0),
            Some("next page"),
        )
        .await
        .unwrap();
        super::get_terminal_aerodrome_forecasts(&configuration(&server), "KPHX")
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        let contracts = requests
            .iter()
            .map(|request| {
                (
                    request.url.path().to_owned(),
                    request.url.query().map(str::to_owned),
                    request.headers["accept"].to_str().unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            contracts,
            [
                (
                    "/stations/KPHX/observations/latest".to_owned(),
                    Some("require_qc=false".to_owned()),
                    "application/geo+json".to_owned(),
                ),
                (
                    "/stations/KPHX/observations".to_owned(),
                    Some("start=2026-08-30T00%3A00%3A00Z&limit=0&cursor=next+page".to_owned(),),
                    "application/geo+json".to_owned(),
                ),
                (
                    "/stations/KPHX/tafs".to_owned(),
                    None,
                    "application/ld+json".to_owned(),
                ),
            ]
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_encodes_dynamic_segments_and_requests_iwxxm() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;

        let result = super::get_terminal_aerodrome_forecast(
            &configuration(&server),
            "K/PHX%",
            "2026/08%30".to_owned(),
            "12/34%",
        )
        .await;

        assert!(result.is_err());
        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/stations/K%2FPHX%25/tafs/2026%2F08%2530/12%2F34%25"
        );
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/vnd.wmo.iwxxm+xml"
        );
    }
}
