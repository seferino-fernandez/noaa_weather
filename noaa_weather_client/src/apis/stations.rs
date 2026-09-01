//! Weather observation stations, surface observations, and TAFs.
//!
//! Covers the `/stations` endpoints for station metadata, latest and
//! historical surface observations, and Terminal Aerodrome Forecasts.

use super::Error;
use crate::client::{Client, http};
use crate::models;

/// Returns metadata about a given observation station
///
/// Corresponds to the `/stations/{stationId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    id: &str,
) -> Result<models::ObservationStationGeoJson, Error> {
    http::request(client, "/stations")
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
/// * `client`: The API client.
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
    client: &Client,
    id: Option<Vec<String>>,
    state: Option<Vec<models::AreaCode>>,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationStationCollectionGeoJson, Error> {
    http::request(client, "/stations")
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
/// * `client`: The API client.
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
    client: &Client,
    station_id: &str,
    require_quality_controlled: Option<bool>,
) -> Result<models::ObservationGeoJson, Error> {
    http::request(client, "/stations")
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
/// * `client`: The API client.
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
    client: &Client,
    station_id: &str,
    start: Option<String>,
    end: Option<String>,
    limit: Option<i32>,
    cursor: Option<&str>,
) -> Result<models::ObservationCollectionGeoJson, Error> {
    http::request(client, "/stations")
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
/// * `client`: The API client.
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
    client: &Client,
    station_id: &str,
    time: String,
) -> Result<models::ObservationGeoJson, Error> {
    http::request(client, "/stations")
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
/// * `client`: The API client.
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
    client: &Client,
    station_id: &str,
    date: &str,
    time: &str,
) -> Result<models::TerminalAerodromeForecast, Error> {
    let bytes = http::request(client, "/stations")
        .path_segment(station_id)
        .literal_path("tafs")
        .path_segment(date)
        .path_segment(time)
        .xml_bytes(http::XmlMedia::Iwxxm)
        .await?;

    models::terminal_aerodrome_forecast::decode_iwxxm(&bytes).map_err(Error::from)
}

/// Returns metadata for Terminal Aerodrome Forecasts for the specified airport station.
///
/// Corresponds to the `/stations/{stationId}/tafs` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    station_id: &str,
) -> Result<models::TerminalAerodromeForecastsResponse, Error> {
    http::request(client, "/stations")
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
    use crate::client::test_support::client_for;

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
            &client_for(&server),
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

        get_observation_station(&client_for(&server), "K/PHX%")
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
            &client_for(&server),
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

        super::get_latest_observations(&client_for(&server), "KPHX", Some(false))
            .await
            .unwrap();
        super::get_observations(
            &client_for(&server),
            "KPHX",
            Some("2026-08-30T00:00:00Z".to_owned()),
            None,
            Some(0),
            Some("next page"),
        )
        .await
        .unwrap();
        super::get_terminal_aerodrome_forecasts(&client_for(&server), "KPHX")
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
    async fn taf_document_serializes_semantic_json_without_wire_artifacts() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KCXL/tafs/2026-08-30/1500"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/cancellation.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KCXL",
            "2026-08-30",
            "1500",
        )
        .await
        .unwrap();
        let json = serde_json::to_value(forecast).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "bulletinIdentifier": "A_LTUS99KCXL301500_C_KCXL_20260830150000.xml",
                "reportMetadata": {
                    "status": { "kind": "amendment" },
                    "permissibleUsage": { "kind": "operational" }
                },
                "issuedAt": "2026-08-30T15:00:00Z",
                "aerodrome": {
                    "designator": "KCXL",
                    "icaoIdentifier": "KCXL"
                },
                "report": {
                    "kind": "cancellation",
                    "cancelledPeriod": {
                        "start": "2026-08-30T12:00:00Z",
                        "end": "2026-08-31T12:00:00Z"
                    }
                }
            }),
        );
        let encoded = json.to_string();
        for wire_artifact in ["ns0", "ns1", "xlink", "xmlns", "meteorologicalInformation"] {
            assert!(
                !encoded.contains(wire_artifact),
                "found {wire_artifact} in {encoded}"
            );
        }
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_serializes_forecast_states_as_semantic_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KXYZ/tafs/2026-08-30/1200"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/semantic_edges.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KXYZ",
            "2026-08-30",
            "1200",
        )
        .await
        .unwrap();
        let json = serde_json::to_value(forecast).unwrap();

        assert_eq!(
            json.pointer("/report/groups/0/conditions/visibility/state"),
            Some(&serde_json::json!("value")),
        );
        assert_eq!(
            json.pointer("/report/groups/0/conditions/visibility/value/meters"),
            Some(&serde_json::json!(800.0)),
        );
        assert_eq!(
            json.pointer("/report/groups/2/conditions/visibility/state"),
            Some(&serde_json::json!("unavailable")),
        );
        assert_eq!(
            json.pointer("/report/groups/2/conditions/visibility/value/reason/kind"),
            Some(&serde_json::json!("notObservable")),
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_errors_preserve_semantic_context_and_sources() {
        use std::error::Error as _;

        use crate::{apis::Error, models::terminal_aerodrome_forecast::TafDecodeErrorKind};

        let fixture = include_str!("../../tests/fixtures/taf/kflg_normal.xml");
        let cases = [
            (
                "KUNT",
                fixture.replacen("uom=\"m\"", "uom=\"sm\"", 1),
                TafDecodeErrorKind::UnsupportedUnit,
                "TAF.forecastGroup.visibility",
                false,
            ),
            (
                "KNUM",
                fixture.replacen(
                    ">10000</prevailingVisibility>",
                    ">not-a-number</prevailingVisibility>",
                    1,
                ),
                TafDecodeErrorKind::InvalidNumber,
                "TAF.forecastGroup.visibility",
                true,
            ),
            (
                "KCAV",
                fixture.replacen(
                    "cloudAndVisibilityOK=\"false\"",
                    "cloudAndVisibilityOK=\"true\"",
                    1,
                ),
                TafDecodeErrorKind::InvalidCombination,
                "TAF.forecastGroup.cloudAndVisibilityOK",
                false,
            ),
            (
                "KXML",
                fixture.replacen("</TAF>", "", 1),
                TafDecodeErrorKind::MalformedXml,
                "TAF",
                true,
            ),
            (
                "KNSP",
                fixture.replacen(
                    "xmlns=\"http://icao.int/iwxxm/2021-2\"",
                    "xmlns=\"urn:unsupported:iwxxm\"",
                    1,
                ),
                TafDecodeErrorKind::InvalidValue,
                "MeteorologicalBulletin.meteorologicalInformation.TAF",
                false,
            ),
            (
                "KPER",
                fixture.replacen(
                    "2026-08-31T18:00:00Z</ns1:endPosition>",
                    "2026-08-29T18:00:00Z</ns1:endPosition>",
                    1,
                ),
                TafDecodeErrorKind::InvalidPeriod,
                "TAF.validPeriod",
                false,
            ),
            (
                "KPOS",
                fixture.replacen("35.14 -111.67", "95 -111.67", 1),
                TafDecodeErrorKind::InvalidCoordinate,
                "TAF.aerodrome.position",
                false,
            ),
            (
                "KUSE",
                fixture.replacen(
                    "permissibleUsage=\"OPERATIONAL\"",
                    "permissibleUsage=\"OPERATIONAL\" permissibleUsageReason=\"TEST\"",
                    1,
                ),
                TafDecodeErrorKind::InvalidCombination,
                "TAF.permissibleUsageReason",
                false,
            ),
            (
                "KCVK",
                fixture.replacen(" cloudAndVisibilityOK=\"false\"", "", 1),
                TafDecodeErrorKind::MissingRequiredField,
                "TAF.forecastGroup.cloudAndVisibilityOK",
                false,
            ),
            (
                "KVOP",
                fixture.replacen(
                    "<prevailingVisibility uom=\"m\">10000</prevailingVisibility>",
                    "",
                    1,
                ),
                TafDecodeErrorKind::InvalidCombination,
                "TAF.forecastGroup.visibilityOperator",
                false,
            ),
        ];
        let server = MockServer::start().await;
        for (station, body, _, _, _) in &cases {
            Mock::given(method("GET"))
                .and(path(format!("/stations/{station}/tafs/2026-08-30/2257")))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(body.clone(), "application/vnd.wmo.iwxxm+xml"),
                )
                .mount(&server)
                .await;
        }

        for (station, _, expected_kind, expected_path, has_decode_source) in cases {
            let error = super::get_terminal_aerodrome_forecast(
                &client_for(&server),
                station,
                "2026-08-30",
                "2257",
            )
            .await
            .unwrap_err();
            let Error::TerminalAerodromeForecast(decode) = &error else {
                panic!("expected semantic TAF decode error, got {error}");
            };

            assert_eq!(decode.kind(), expected_kind);
            assert_eq!(decode.path(), expected_path);
            assert_eq!(decode.source().is_some(), has_decode_source);
            assert!(error.source().is_some());
            assert!(error.to_string().contains(expected_path));
        }
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_distinguishes_cancellation_and_translation_failure() {
        use jiff::Timestamp;

        use crate::models::terminal_aerodrome_forecast::{
            ForecastReport, MissingForecastReason, ReportStatus,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KCXL/tafs/2026-08-30/1500"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/cancellation.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KERR/tafs/2026-08-30/1600"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/translation_failed.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let cancelled = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KCXL",
            "2026-08-30",
            "1500",
        )
        .await
        .unwrap();
        let ForecastReport::Cancellation { cancelled_period } = cancelled.report() else {
            panic!("expected cancellation");
        };
        assert_eq!(
            cancelled.report_metadata().status(),
            &ReportStatus::Amendment
        );
        assert_eq!(
            (cancelled_period.start(), cancelled_period.end()),
            (
                "2026-08-30T12:00:00Z".parse::<Timestamp>().unwrap(),
                "2026-08-31T12:00:00Z".parse::<Timestamp>().unwrap(),
            ),
        );

        let missing = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KERR",
            "2026-08-30",
            "1600",
        )
        .await
        .unwrap();
        assert_eq!(
            missing.report(),
            &ForecastReport::Missing {
                reason: MissingForecastReason::TranslationFailed {
                    tac: "TAF KERR malformed source".into(),
                },
            },
        );
        let translation = missing.report_metadata().translation().unwrap();
        assert_eq!(
            translation.source_bulletin_identifier(),
            Some("FTUS99KERR301600")
        );
        assert_eq!(translation.centre_designator(), Some("KERR"));
        assert_eq!(translation.centre_name(), Some("Fixture Translator"));
        assert_eq!(
            translation.source_bulletin_received_at(),
            Some("2026-08-30T16:01:00Z".parse::<Timestamp>().unwrap()),
        );
        assert_eq!(
            translation.translated_at(),
            Some("2026-08-30T16:02:00Z".parse::<Timestamp>().unwrap()),
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_normalizes_temperature_vertical_visibility_and_nil_meaning() {
        use jiff::Timestamp;

        use crate::models::terminal_aerodrome_forecast::{
            ForecastClouds, ForecastGroupKind, ForecastValue, ForecastVisibility, ForecastWeather,
            ForecastWind, MissingReason,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KXYZ/tafs/2026-08-30/1200"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/semantic_edges.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KXYZ",
            "2026-08-30",
            "1200",
        )
        .await
        .unwrap();
        let base = forecast.base_forecast().unwrap();
        let temperatures = base.conditions().temperatures();

        assert_eq!(base.conditions().weather(), &ForecastWeather::NoSignificant);
        assert_eq!(
            base.conditions().clouds(),
            &ForecastClouds::VerticalVisibility {
                feet: ForecastValue::Value(300.0),
            },
        );
        assert_eq!(temperatures.len(), 1);
        assert_eq!(temperatures[0].maximum().celsius(), 7.0);
        assert_eq!(
            temperatures[0].maximum().occurs_at(),
            "2026-08-30T21:00:00Z".parse::<Timestamp>().unwrap(),
        );
        assert_eq!(temperatures[0].minimum().celsius(), -5.0);
        assert_eq!(
            temperatures[0].minimum().occurs_at(),
            "2026-08-31T10:00:00Z".parse::<Timestamp>().unwrap(),
        );

        let changes = forecast.change_forecasts();
        assert_eq!(
            changes[0].conditions().weather(),
            &ForecastWeather::NoSignificant
        );
        assert_eq!(
            changes[0].conditions().clouds(),
            &ForecastClouds::NoSignificant
        );
        assert_eq!(
            changes[1].conditions().clouds(),
            &ForecastClouds::VerticalVisibility {
                feet: ForecastValue::Unavailable {
                    reason: MissingReason::NotObservable,
                },
            },
        );
        assert_eq!(
            changes[1].conditions().wind(),
            &ForecastWind::Unavailable {
                reason: MissingReason::NotObservable,
            },
        );
        assert_eq!(
            changes[1].conditions().visibility(),
            &ForecastVisibility::Unavailable {
                reason: MissingReason::NotObservable,
            },
        );
        assert_eq!(
            changes.iter().map(|group| group.kind()).collect::<Vec<_>>(),
            [
                &ForecastGroupKind::Becoming,
                &ForecastGroupKind::From,
                &ForecastGroupKind::Probability {
                    percent: 40,
                    temporary: false,
                },
                &ForecastGroupKind::Probability {
                    percent: 30,
                    temporary: true,
                },
                &ForecastGroupKind::Probability {
                    percent: 40,
                    temporary: true,
                },
            ],
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_preserves_weather_qualifiers_and_cloud_types() {
        use crate::models::terminal_aerodrome_forecast::{
            CloudAmount, CloudType, ForecastClouds, ForecastValue, ForecastWeather,
            WeatherDescriptor, WeatherIntensity, WeatherPhenomenon,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KFLG",
            "2026-08-30",
            "2257",
        )
        .await
        .unwrap();
        let conditions = forecast.change_forecasts()[0].conditions();
        let ForecastWeather::Phenomena { items } = conditions.weather() else {
            panic!("expected significant weather");
        };
        let ForecastClouds::Layers { layers } = conditions.clouds() else {
            panic!("expected cloud layers");
        };

        assert_eq!(
            (
                items[0].code(),
                items[0].intensity(),
                items[0].is_in_vicinity(),
                items[0].descriptor(),
                items[0].phenomena(),
                layers[0].amount(),
                layers[0].base_feet(),
                layers[0].cloud_type(),
            ),
            (
                "-TSRA",
                WeatherIntensity::Light,
                false,
                Some(&WeatherDescriptor::Thunderstorm),
                &[WeatherPhenomenon::Rain][..],
                &ForecastValue::Value(CloudAmount::Broken),
                &ForecastValue::Value(5_000.0),
                Some(&ForecastValue::Value(CloudType::Cumulonimbus)),
            ),
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_normalizes_period_visibility_and_wind() {
        use crate::models::terminal_aerodrome_forecast::{Comparison, WindDirection};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KFLG",
            "2026-08-30",
            "2257",
        )
        .await
        .unwrap();
        let base = forecast.base_forecast().unwrap();
        let visibility = base.conditions().visibility().value().unwrap();
        let wind = base.conditions().wind().value().unwrap();

        assert_eq!(
            (
                base.valid_period().start().to_string(),
                base.valid_period().end().to_string(),
                base.conditions().is_cavok(),
                visibility.meters(),
                visibility.comparison(),
                wind.direction(),
                wind.speed().knots(),
                wind.speed().comparison(),
                wind.gust().map(|gust| gust.knots()),
            ),
            (
                "2026-08-30T17:39:00Z".to_owned(),
                "2026-08-31T02:00:00Z".to_owned(),
                false,
                10_000.0,
                &Comparison::Above,
                WindDirection::Degrees(220.0),
                15.0,
                &Comparison::Exact,
                Some(25.0),
            ),
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_normalizes_report_metadata_times_and_position() {
        use crate::models::terminal_aerodrome_forecast::{
            ForecastReport, PermissibleUsage, ReportStatus,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KFLG",
            "2026-08-30",
            "2257",
        )
        .await
        .unwrap();
        let position = forecast.aerodrome().position().unwrap();
        let ForecastReport::Forecast { valid_period, .. } = forecast.report() else {
            panic!("expected an ordinary forecast report");
        };

        assert_eq!(
            (
                forecast.bulletin_identifier(),
                forecast.report_metadata().status(),
                forecast.report_metadata().permissible_usage(),
                forecast.issued_at().to_string(),
                position.latitude(),
                position.longitude(),
                valid_period.start().to_string(),
                valid_period.end().to_string(),
            ),
            (
                "A_LTUS45KFGZ301700_C_KFGZ_20260830173930.xml",
                &ReportStatus::Normal,
                &PermissibleUsage::Operational,
                "2026-08-30T17:39:00Z".to_owned(),
                35.14,
                -111.67,
                "2026-08-30T18:00:00Z".to_owned(),
                "2026-08-31T18:00:00Z".to_owned(),
            ),
        );
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn taf_document_normalizes_identity_and_forecast_groups() {
        use crate::models::terminal_aerodrome_forecast::ForecastGroupKind;

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let forecast = super::get_terminal_aerodrome_forecast(
            &client_for(&server),
            "KFLG",
            "2026-08-30",
            "2257",
        )
        .await
        .unwrap();

        assert_eq!(
            (
                forecast.aerodrome().icao_identifier(),
                forecast.groups().len(),
                forecast.base_forecast().map(|group| group.kind()),
            ),
            ("KFLG", 6, Some(&ForecastGroupKind::Base)),
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
            &client_for(&server),
            "K/PHX%",
            "2026/08%30",
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
