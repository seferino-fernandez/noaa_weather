//! Aviation weather products: SIGMETs and Center Weather Advisories (CWAs).
//!
//! Covers the `/aviation` endpoints for in-flight weather hazard reports
//! issued by Air Traffic Service Units and Center Weather Service Units.

use super::{Error, configuration, http};
use crate::models;

/// Returns a specific Center Weather Advisory (CWA) identified by CWSU, date, and sequence number.
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}/cwas/{date}/{sequence}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the issuing Center Weather Service Unit (CWSU).
/// * `date`: The date of the advisory in `YYYY-MM-DD` format.
/// * `sequence`: The sequence number of the advisory (must be >= 100).
///
/// # Returns
///
/// A `Result` containing a [`models::CenterWeatherAdvisoryGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_advisories_by_date_and_sequence(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
    date: String,
    sequence: i32,
) -> Result<models::CenterWeatherAdvisoryGeoJson, Error> {
    http::request(configuration, "/aviation/cwsus")
        .path_segment(center_weather_service_unit_id)
        .literal_path("cwas")
        .path_segment(date)
        .path_segment(sequence)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a collection of current Center Weather Advisories (CWAs) for a specific Center Weather Service Unit (CWSU).
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}/cwas` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the Center Weather Service Unit (CWSU).
///
/// # Returns
///
/// A `Result` containing a [`models::CenterWeatherAdvisoryCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_advisories(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<models::CenterWeatherAdvisoryCollectionGeoJson, Error> {
    http::request(configuration, "/aviation/cwsus")
        .path_segment(center_weather_service_unit_id)
        .literal_path("cwas")
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns metadata about a specific Center Weather Service Unit (CWSU).
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the Center Weather Service Unit (CWSU).
///
/// # Returns
///
/// A `Result` containing [`models::Office`] metadata on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_service_unit(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<models::CwsuOffice, Error> {
    http::request(configuration, "/aviation/cwsus")
        .path_segment(center_weather_service_unit_id)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a specific SIGMET or AIRMET product.
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}/{date}/{time}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the issuing Air Traffic Service Unit (ATSU).
/// * `date`: The date of issuance in `YYYY-MM-DD` format.
/// * `time`: The time of issuance in `HHMM` format (UTC).
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmet(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
    date: String,
    time: &str,
) -> Result<models::SigmetGeoJson, Error> {
    http::request(configuration, "/aviation/sigmets")
        .path_segment(air_traffic_service_unit)
        .path_segment(date)
        .path_segment(time)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a collection of SIGMET/AIRMET products based on query parameters.
///
/// Corresponds to the `/aviation/sigmets` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `start`: Optional start time for the query period (ISO 8601 format).
/// * `end`: Optional end time for the query period (ISO 8601 format).
/// * `date`: Optional date filter (`YYYY-MM-DD` format).
/// * `air_traffic_service_unit`: Optional Air Traffic Service Unit (ATSU) identifier filter.
/// * `sequence`: Optional sequence number filter.
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets(
    configuration: &configuration::Configuration,
    start: Option<String>,
    end: Option<String>,
    date: Option<String>,
    air_traffic_service_unit: Option<&str>,
    sequence: Option<&str>,
) -> Result<models::SigmetCollectionGeoJson, Error> {
    http::request(configuration, "/aviation/sigmets")
        .query_scalar("start", start)
        .query_scalar("end", end)
        .query_scalar("date", date)
        .query_scalar("atsu", air_traffic_service_unit)
        .query_scalar("sequence", sequence)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a collection of SIGMET/AIRMET products for a specific Air Traffic Service Unit (ATSU).
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the Air Traffic Service Unit (ATSU).
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets_by_air_traffic_service_unit(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
) -> Result<models::SigmetCollectionGeoJson, Error> {
    http::request(configuration, "/aviation/sigmets")
        .path_segment(air_traffic_service_unit)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a collection of SIGMET/AIRMET products for a specific Air Traffic Service Unit (ATSU) on a specific date.
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}/{date}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the Air Traffic Service Unit (ATSU).
/// * `date`: The date filter in `YYYY-MM-DD` format.
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets_by_air_traffic_service_unit_and_date(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
    date: String,
) -> Result<models::SigmetCollectionGeoJson, Error> {
    http::request(configuration, "/aviation/sigmets")
        .path_segment(air_traffic_service_unit)
        .path_segment(date)
        .json(http::JsonMedia::GeoJson)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{
        get_center_weather_advisories_by_date_and_sequence, get_center_weather_service_unit,
        get_sigmet, get_sigmets,
    };
    use crate::{apis::configuration::Configuration, models::NwsCenterWeatherServiceUnitId};

    fn configuration(server: &MockServer) -> Configuration {
        Configuration::new(None, Some(server.uri()), None, None)
    }

    #[tokio::test]
    async fn sigmet_identifiers_date_and_time_are_distinct_encoded_geo_json_segments() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/aviation/sigmets/WA%2FFC%201/2026%2F08%2030/12:30%2FZ",
            ))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"Feature","geometry":null,"properties":{}}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        get_sigmet(
            &configuration(&server),
            "WA/FC 1",
            "2026/08 30".to_owned(),
            "12:30/Z",
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn cwa_typed_identifier_date_and_sequence_are_distinct_geo_json_segments() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/aviation/cwsus/ZAB/cwas/2026%2F08%2030/101"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"Feature","geometry":null,"properties":{}}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        get_center_weather_advisories_by_date_and_sequence(
            &configuration(&server),
            NwsCenterWeatherServiceUnitId::Zab,
            "2026/08 30".to_owned(),
            101,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn cwsu_metadata_is_requested_as_json_ld() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/aviation/cwsus/ZAB"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(1)
            .mount(&server)
            .await;

        get_center_weather_service_unit(
            &configuration(&server),
            NwsCenterWeatherServiceUnitId::Zab,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn sigmet_filters_preserve_order_encoding_empty_and_omitted_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/aviation/sigmets"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        get_sigmets(
            &configuration(&server),
            Some("2026-08-30T00:00:00+00:00".to_owned()),
            None,
            Some(String::new()),
            Some("WA/FC 1"),
            Some("2/3"),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("start=2026-08-30T00%3A00%3A00%2B00%3A00&date=&atsu=WA%2FFC+1&sequence=2%2F3")
        );
        assert!(!requests[0].url.query_pairs().any(|(name, _)| name == "end"));
    }
}
