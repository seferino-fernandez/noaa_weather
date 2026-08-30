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

use super::{Error, configuration, http};
use crate::models;

/// Returns the NOAA Weather Radio broadcast for a geographic point.
///
/// Corresponds to the `/points/{latitude},{longitude}/radio` endpoint.
/// The response is an SSML (Speech Synthesis Markup Language) document
/// containing the radio broadcast script for the area covering the given point.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    latitude: f64,
    longitude: f64,
) -> Result<models::RadioBroadcast, Error> {
    let uri_str = format!("/points/{latitude},{longitude}/radio");
    let req_builder = http::get(configuration, &uri_str);

    req_builder.xml().await
}

/// Returns the NOAA Weather Radio broadcast for a given transmitter call sign.
///
/// Corresponds to the `/radio/{callSign}/broadcast` endpoint.
/// The response is an SSML (Speech Synthesis Markup Language) document
/// containing the current broadcast script for the specified radio transmitter.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    call_sign: &str,
) -> Result<models::RadioBroadcast, Error> {
    let uri_str = format!(
        "/radio/{call_sign}/broadcast",
        call_sign = crate::apis::urlencode(call_sign)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.xml().await
}

/// Returns a page of NOAA Weather Radio transmitters.
///
/// Corresponds to the `/radio` endpoint. Pass the opaque `cursor` from a
/// collection's pagination link to request a subsequent page.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    cursor: Option<&str>,
) -> Result<models::RadioTransmitterCollection, Error> {
    let mut req_builder =
        http::get(configuration, "/radio").header("Accept", "application/ld+json");

    if let Some(cursor) = cursor {
        req_builder = req_builder.query(&[("cursor", cursor)]);
    }

    req_builder.json().await
}

/// Returns metadata for a NOAA Weather Radio transmitter.
///
/// Corresponds to the `/radio/{callSign}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    call_sign: &str,
) -> Result<models::RadioTransmitter, Error> {
    let uri_str = format!(
        "/radio/{call_sign}",
        call_sign = crate::apis::urlencode(call_sign)
    );

    http::get(configuration, &uri_str)
        .header("Accept", "application/ld+json")
        .json()
        .await
}

/// Returns NOAA Weather Radio transmitters serving a county zone.
///
/// Corresponds to the `/zones/county/{zoneId}/radio` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    zone_id: &str,
) -> Result<models::RadioTransmitterCollection, Error> {
    let uri_str = format!(
        "/zones/county/{zone_id}/radio",
        zone_id = crate::apis::urlencode(zone_id)
    );

    http::get(configuration, &uri_str)
        .header("Accept", "application/ld+json")
        .json()
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{
        get_radio_transmitter, get_radio_transmitters, get_radio_transmitters_for_county_zone,
    };
    use crate::apis::configuration::Configuration;

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

    fn configuration(server: &MockServer) -> Configuration {
        Configuration::new(None, Some(server.uri()), None, None)
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

        let first_page = get_radio_transmitters(&configuration(&server), None)
            .await
            .unwrap();
        let second_page = get_radio_transmitters(&configuration(&server), Some("opaque+/=? value"))
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
    async fn transmitter_detail_percent_encodes_call_sign_and_returns_an_object() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radio/K%2FA%3F"))
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

        let transmitter = get_radio_transmitter(&configuration(&server), "K/A?")
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
            .and(path("/zones/county/AZC%2F013%3F/radio"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(r#"{"@graph":[{"callSign":"KPHX"}]}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let transmitters =
            get_radio_transmitters_for_county_zone(&configuration(&server), "AZC/013?")
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
