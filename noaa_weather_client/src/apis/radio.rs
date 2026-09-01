//! NOAA Weather Radio transmitter metadata and broadcast transcripts.
//!
//! The JSON-LD transmitter metadata APIs are `/radio`, `/radio/{callSign}`,
//! and `/zones/county/{zoneId}/radio`. They return
//! [`RadioTransmitter`](crate::models::RadioTransmitter) records, either
//! directly or in a
//! [`RadioTransmitterCollection`](crate::models::RadioTransmitterCollection).
//!
//! The `/points/{point}/radio` and `/radio/{callSign}/broadcast` APIs return
//! SSML (Speech Synthesis Markup Language) documents containing the current
//! broadcast script for a location or transmitter.
//!
//! This module is only available when the **`radio`** feature is enabled:
//!
//! ```toml
//! [dependencies]
//! noaa_weather_client = { version = "1", features = ["radio"] }
//! ```
//!
//! Broadcast responses use [`RadioBroadcast`](crate::models::RadioBroadcast),
//! whose structured paragraphs and sentences can be rendered as plain text via
//! [`Sentence::full_text`](crate::models::Sentence::full_text).

use super::Error;
use crate::client::{Client, http};
use crate::models;

/// Returns the NOAA Weather Radio broadcast for a geographic point.
///
/// Corresponds to the `/points/{latitude},{longitude}/radio` endpoint.
/// The response is an SSML (Speech Synthesis Markup Language) document
/// containing the radio broadcast script for the area covering the given point.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `latitude`: The latitude of the point (e.g., 33.4484).
/// * `longitude`: The longitude of the point (e.g., -112.0740).
///
/// # Returns
///
/// A `Result` containing a [`models::RadioBroadcast`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_point_radio(
    client: &Client,
    latitude: f64,
    longitude: f64,
) -> Result<models::RadioBroadcast, Error> {
    http::request(client, "/points")
        .path_segment(format_args!("{latitude},{longitude}"))
        .literal_path("radio")
        .xml(http::XmlMedia::Ssml)
        .await
}

/// Returns the NOAA Weather Radio broadcast for a given transmitter call sign.
///
/// Corresponds to the `/radio/{callSign}/broadcast` endpoint.
/// The response is an SSML (Speech Synthesis Markup Language) document
/// containing the current broadcast script for the specified radio transmitter.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `call_sign`: The transmitter call sign (e.g., "KEC94").
///
/// # Returns
///
/// A `Result` containing a [`models::RadioBroadcast`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., call sign not found)
/// or the response cannot be parsed.
pub async fn get_area_radio(
    client: &Client,
    call_sign: &str,
) -> Result<models::RadioBroadcast, Error> {
    http::request(client, "/radio")
        .path_segment(call_sign)
        .literal_path("broadcast")
        .xml(http::XmlMedia::Ssml)
        .await
}

/// Returns a page of NOAA Weather Radio transmitters.
///
/// Corresponds to the `/radio` endpoint. Pass the opaque `cursor` from a
/// collection's pagination link to request a subsequent page.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `cursor`: An optional opaque pagination cursor.
///
/// # Returns
///
/// A `Result` containing a [`models::RadioTransmitterCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_radio_transmitters(
    client: &Client,
    cursor: Option<&str>,
) -> Result<models::RadioTransmitterCollection, Error> {
    http::request(client, "/radio")
        .query_scalar("cursor", cursor)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns metadata for a NOAA Weather Radio transmitter.
///
/// Corresponds to the `/radio/{callSign}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `call_sign`: The transmitter call sign.
///
/// # Returns
///
/// A `Result` containing a [`models::RadioTransmitter`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_radio_transmitter(
    client: &Client,
    call_sign: &str,
) -> Result<models::RadioTransmitter, Error> {
    http::request(client, "/radio")
        .path_segment(call_sign)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns NOAA Weather Radio transmitters serving a county zone.
///
/// Corresponds to the `/zones/county/{zoneId}/radio` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `zone_id`: The county zone ID.
///
/// # Returns
///
/// A `Result` containing a [`models::RadioTransmitterCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_radio_transmitters_for_county_zone(
    client: &Client,
    zone_id: &str,
) -> Result<models::RadioTransmitterCollection, Error> {
    http::request(client, "/zones/county")
        .path_segment(zone_id)
        .literal_path("radio")
        .json(http::JsonMedia::JsonLd)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{
        get_area_radio, get_point_radio, get_radio_transmitter, get_radio_transmitters,
        get_radio_transmitters_for_county_zone,
    };
    use crate::{Error, client::test_support::client_for};

    const TRANSMITTERS: &str = r#"{
        "@graph": [{
            "callSign": "KAAA",
            "transmitterFrequency": "162.550",
            "sameCodes": ["004013", "004013"],
            "counties": ["AZC013", "AZC013"]
        }, {
            "callSign": "KAAA"
        }],
        "pagination": {"next": "https://api.weather.gov/radio?cursor=next-page"}
    }"#;
    const RADIO_BROADCAST: &str = r#"<speak version="1.1" xml:lang="en-US"></speak>"#;

    #[tokio::test]
    async fn transmitter_broadcast_encodes_call_sign_and_requests_ssml() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio/K%20E%2F%25%3F/broadcast"))
            .and(header("Accept", "application/ssml+xml"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(RADIO_BROADCAST, "application/ssml+xml"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let broadcast = get_area_radio(&client_for(&server), "K E/%?")
            .await
            .unwrap();
        assert_eq!(broadcast.version, "1.1");
        assert_eq!(broadcast.lang, "en-US");
    }

    #[tokio::test]
    async fn point_broadcast_encodes_one_coordinate_segment_and_requests_ssml() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/points/33.4484,-112.074/radio"))
            .and(header("Accept", "application/ssml+xml"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(RADIO_BROADCAST, "application/ssml+xml"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let broadcast = get_point_radio(&client_for(&server), 33.4484, -112.074)
            .await
            .unwrap();
        assert_eq!(broadcast.version, "1.1");
        assert_eq!(broadcast.lang, "en-US");
    }

    #[tokio::test]
    async fn transmitter_broadcast_rejects_generic_xml() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio/KEC94/broadcast"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(RADIO_BROADCAST, "application/xml"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let Error::Protocol(error) = get_area_radio(&client_for(&server), "KEC94")
            .await
            .unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ssml+xml"));
        assert_eq!(error.actual(), Some("application/xml"));
    }

    #[tokio::test]
    async fn transmitter_list_omits_or_encodes_opaque_cursor_and_preserves_pagination() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(TRANSMITTERS, "application/ld+json"),
            )
            .expect(2)
            .mount(&server)
            .await;

        let first_page = get_radio_transmitters(&client_for(&server), None)
            .await
            .unwrap();
        let second_page = get_radio_transmitters(&client_for(&server), Some("opaque+/=? value"))
            .await
            .unwrap();

        assert_eq!(
            first_page.pagination.unwrap().next,
            "https://api.weather.gov/radio?cursor=next-page"
        );
        assert_eq!(
            second_page.transmitters[0].frequency.as_deref(),
            Some("162.550")
        );
        assert_eq!(
            second_page
                .transmitters
                .iter()
                .map(|transmitter| transmitter.call_sign.as_deref())
                .collect::<Vec<_>>(),
            [Some("KAAA"), Some("KAAA")]
        );
        assert_eq!(second_page.transmitters[0].same_codes, ["004013", "004013"]);
        assert_eq!(second_page.transmitters[0].counties, ["AZC013", "AZC013"]);

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/radio");
        assert_eq!(requests[0].url.query(), None);
        assert_eq!(requests[1].url.path(), "/radio");
        assert_eq!(
            requests[1].url.query(),
            Some("cursor=opaque%2B%2F%3D%3F+value")
        );
    }

    #[tokio::test]
    async fn transmitter_list_rejects_generic_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(r#"{"@graph":[]}"#, "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let Error::Protocol(error) = get_radio_transmitters(&client_for(&server), None)
            .await
            .unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ld+json"));
        assert_eq!(error.actual(), Some("application/json"));
    }

    #[tokio::test]
    async fn transmitter_detail_percent_encodes_call_sign_and_returns_an_object() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio/K%20A%2F%25%3F"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "@context": "https://geojson.org/geojson-ld/geojson-context.jsonld",
                    "@id": "https://api.weather.gov/radio/KAAA",
                    "callSign": "KAAA",
                    "transmitterFrequency": "162.550",
                    "sameCodes": [],
                    "counties": []
                }"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let transmitter = get_radio_transmitter(&client_for(&server), "K A/%?")
            .await
            .unwrap();

        assert_eq!(transmitter.call_sign.as_deref(), Some("KAAA"));
        assert_eq!(transmitter.frequency.as_deref(), Some("162.550"));
        assert!(transmitter.same_codes.is_empty());
        assert!(transmitter.counties.is_empty());
    }

    #[tokio::test]
    async fn county_zone_transmitters_use_the_county_path_without_pagination() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/zones/county/AZC%20013%2F%25%3F/radio"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(r#"{"@graph":[{"callSign":"KPHX"}]}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let transmitters =
            get_radio_transmitters_for_county_zone(&client_for(&server), "AZC 013/%?")
                .await
                .unwrap();

        assert_eq!(transmitters.pagination, None);
        assert_eq!(
            transmitters.transmitters[0].call_sign.as_deref(),
            Some("KPHX")
        );
        assert!(transmitters.transmitters[0].same_codes.is_empty());
        assert!(transmitters.transmitters[0].counties.is_empty());
    }
}
