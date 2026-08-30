//! Radar infrastructure: servers, stations, data queues, and wind profilers.
//!
//! Covers the `/radar` endpoints for metadata about NEXRAD radar stations,
//! distribution servers, and data queue status.

use super::{Error, configuration, http};
use crate::models::{self, RadarQueueHost};

/// Parameters for the [`get_radar_data_queue`] function.
///
/// This struct encapsulates the optional query parameters for filtering radar data queue entries.
#[derive(Debug, Clone, Default)]
pub struct RadarDataQueueQueryParams<'a> {
    /// Limit the number of results returned; the API accepts values from 1 through 50,000.
    pub limit: Option<i32>,
    /// Filter by arrival time range (ISO 8601 format, e.g., "start/end", "start/", "/end").
    pub arrived: Option<&'a str>,
    /// Filter by creation time range (ISO 8601 format).
    pub created: Option<&'a str>,
    /// Filter by publication time range (ISO 8601 format).
    pub published: Option<&'a str>,
    /// Filter by radar station ID.
    pub station: Option<&'a str>,
    /// Filter by data type.
    pub r#type: Option<&'a str>,
    /// Filter by feed type.
    pub feed: Option<&'a str>,
    /// Filter by resolution.
    pub resolution: Option<i32>,
}

/// Returns metadata about a given radar wind profiler station.
///
/// Corresponds to the `/radar/profilers/{id}` endpoint.
/// Optionally filters data by time and interval.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the radar wind profiler station.
/// * `time`: Optional specific time for the data (ISO 8601 format or relative time).
/// * `interval`: Optional time interval for the data (ISO 8601 duration format).
///
/// # Returns
///
/// A `Result` containing a [`serde_json::Value`] on success, representing the profiler metadata.
///
/// *Note: The exact structure of the returned JSON is unknown.*
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_wind_profiler(
    configuration: &configuration::Configuration,
    id: &str,
    time: Option<&str>,
    interval: Option<&str>,
) -> Result<serde_json::Value, Error> {
    http::request(configuration, "/radar/profilers")
        .path_segment(id)
        .query_scalar("time", time)
        .query_scalar("interval", interval)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns metadata about a given radar queue on a specific host.
///
/// Corresponds to the `/radar/queues/{host}` endpoint.
/// Allows filtering queue entries by various criteria like time, station, type, etc.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `host`: The hostname of the radar queue server.
/// * `params`: A [`RadarDataQueueQueryParams`] struct containing the query parameters.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarQueuesResponse`] on success, representing the queue metadata.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_data_queue(
    configuration: &configuration::Configuration,
    host: &RadarQueueHost,
    params: RadarDataQueueQueryParams<'_>,
) -> Result<models::RadarQueuesResponse, Error> {
    http::request(configuration, "/radar/queues")
        .path_segment(host)
        .query_scalar("limit", params.limit)
        .query_scalar("arrived", params.arrived)
        .query_scalar("created", params.created)
        .query_scalar("published", params.published)
        .query_scalar("station", params.station)
        .query_scalar("type", params.r#type)
        .query_scalar("feed", params.feed)
        .query_scalar("resolution", params.resolution)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns metadata about a given radar server.
///
/// Corresponds to the `/radar/servers/{id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the radar server.
/// * `reporting_host`: Optional filter by reporting host.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarServer`] on success, representing the server metadata.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_server(
    configuration: &configuration::Configuration,
    id: &str,
    reporting_host: Option<&str>,
) -> Result<models::RadarServer, Error> {
    http::request(configuration, "/radar/servers")
        .path_segment(id)
        .query_scalar("reportingHost", reporting_host)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of radar servers.
///
/// Corresponds to the `/radar/servers` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `reporting_host`: Optional filter by reporting host.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarServersResponse`] on success, representing the list of servers.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_servers(
    configuration: &configuration::Configuration,
    reporting_host: Option<&str>,
) -> Result<models::RadarServersResponse, Error> {
    http::request(configuration, "/radar/servers")
        .query_scalar("reportingHost", reporting_host)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns metadata about a given radar station.
///
/// Corresponds to the `/radar/stations/{id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The ID of the radar station (e.g., "KABQ", "KMUX").
/// * `reporting_host`: Optional filter by reporting host.
/// * `host`: Optional filter by host server.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarStationFeature`] on success, representing the station metadata.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_station(
    configuration: &configuration::Configuration,
    id: &str,
    reporting_host: Option<&str>,
    host: Option<&RadarQueueHost>,
) -> Result<models::RadarStationFeature, Error> {
    http::request(configuration, "/radar/stations")
        .path_segment(id)
        .query_scalar("reportingHost", reporting_host)
        .query_scalar("host", host)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns alarm metadata for a given radar station.
///
/// Corresponds to the `/radar/stations/{stationId}/alarms` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_id`: The ID of the radar station.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarStationAlarmsResponse`] on success, representing the station alarms.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_station_alarms(
    configuration: &configuration::Configuration,
    station_id: &str,
) -> Result<models::RadarStationAlarmsResponse, Error> {
    http::request(configuration, "/radar/stations")
        .path_segment(station_id)
        .literal_path("alarms")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a list of radar stations, optionally filtered.
///
/// Corresponds to the `/radar/stations` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `station_type`: Optional filter by station type(s) (e.g., "WSR-88D", "TDWR").
/// * `reporting_host`: Optional filter by reporting host.
/// * `host`: Optional filter by host server.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarStationsResponse`] on success, representing the list of stations.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_radar_stations(
    configuration: &configuration::Configuration,
    station_type: Option<Vec<String>>,
    reporting_host: Option<&str>,
    host: Option<&RadarQueueHost>,
) -> Result<models::RadarStationsResponse, Error> {
    http::request(configuration, "/radar/stations")
        .query_csv("stationType", station_type)
        .query_scalar("reportingHost", reporting_host)
        .query_scalar("host", host)
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns radar SPGDS telemetry.
///
/// Corresponds to the `/radar/spgds` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `published`: Optional publication time interval in ISO 8601 format.
///
/// # Returns
///
/// A `Result` containing a [`models::RadarSpgdsResponse`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_radar_spgds(
    configuration: &configuration::Configuration,
    published: Option<&str>,
) -> Result<models::RadarSpgdsResponse, Error> {
    http::request(configuration, "/radar/spgds")
        .query_scalar("published", published)
        .json(http::JsonMedia::JsonLd)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{get_radar_server, get_radar_spgds, get_radar_stations};
    use crate::{Error, apis::configuration::Configuration, models::RadarQueueHost};

    fn configuration(server: &MockServer) -> Configuration {
        Configuration::new(None, Some(server.uri()), None, None)
    }

    #[tokio::test]
    async fn station_types_are_one_csv_query_and_station_collection_is_geo_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radar/stations"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"FeatureCollection","features":[]}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        get_radar_stations(
            &configuration(&server),
            Some(vec!["WSR-88D".to_owned(), "TD/WR".to_owned()]),
            Some("report/host"),
            Some(&RadarQueueHost::Rds),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("stationType=WSR-88D%2CTD%2FWR&reportingHost=report%2Fhost&host=rds")
        );
        assert_eq!(
            requests[0]
                .url
                .query_pairs()
                .filter(|(name, _)| name == "stationType")
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn server_id_is_an_encoded_json_ld_path_segment_with_scalar_reporting_host() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radar/servers/ldm%2Fone%20host"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(r#"{"id":"ldm/one host"}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let response =
            get_radar_server(&configuration(&server), "ldm/one host", Some("report/host"))
                .await
                .unwrap();

        assert_eq!(response.id.as_deref(), Some("ldm/one host"));
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), Some("reportingHost=report%2Fhost"));
    }

    #[tokio::test]
    async fn requests_json_ld_without_query_and_decodes_tolerant_spgds_telemetry() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radar/spgds"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "@context": {},
                    "@graph": [{
                        "@type": "SPGDS",
                        "id": 7,
                        "timestamp": true,
                        "dataflow": {"state": 1, "unknown": []},
                        "ldm": {"conns": 47.5},
                        "throughput": {"in": false, "out": "42"},
                        "spg": {"TXYZ": {"swimDataState": 0, "ldmPingState": true}},
                        "unknown": {"nested": "ignored"}
                    }]
                }"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let response = get_radar_spgds(&configuration(&server), None)
            .await
            .unwrap();

        assert_eq!(response.spgds.len(), 1);
        let entry = &response.spgds[0];
        assert_eq!(entry.r#type.as_deref(), Some("SPGDS"));
        assert_eq!(entry.id.as_deref(), Some("7"));
        assert_eq!(entry.timestamp.as_deref(), Some("true"));
        assert_eq!(entry.dataflow.as_ref().unwrap().state.as_deref(), Some("1"));
        assert_eq!(entry.ldm.as_ref().unwrap().conns.as_deref(), Some("47.5"));
        assert_eq!(
            entry.throughput.as_ref().unwrap().inbound.as_deref(),
            Some("false")
        );
        assert_eq!(entry.spg["TXYZ"].swim_data_state.as_deref(), Some("0"));
        assert_eq!(entry.spg["TXYZ"].ldm_ping_state.as_deref(), Some("true"));

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
    }

    #[tokio::test]
    async fn sends_published_interval_once_with_exact_percent_encoding() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radar/spgds"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(r#"{"@graph":[]}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;
        let published = "2026-01-01T00:00:00+00:00/2026-01-01T01:30:00+00:00";

        let response = get_radar_spgds(&configuration(&server), Some(published))
            .await
            .unwrap();

        assert!(response.spgds.is_empty());
        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("published=2026-01-01T00%3A00%3A00%2B00%3A00%2F2026-01-01T01%3A30%3A00%2B00%3A00")
        );
    }

    #[tokio::test]
    async fn retains_typed_problem_detail_for_non_success_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radar/spgds"))
            .respond_with(ResponseTemplate::new(503).set_body_raw(
                r#"{"type":"https://api.weather.gov/problems/unavailable","title":"Unavailable","status":503,"detail":"Try later","instance":"urn:test","correlationId":"test-correlation"}"#,
                "application/problem+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let error = get_radar_spgds(&configuration(&server), None)
            .await
            .unwrap_err();
        let Error::Response(response) = error else {
            panic!("expected response error");
        };
        let problem = response.problem_detail().expect("typed problem detail");
        assert_eq!(problem.title, "Unavailable");
        assert_eq!(problem.status, 503.0);
    }
}
