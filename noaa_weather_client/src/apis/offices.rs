//! NWS forecast office metadata and headlines.
//!
//! Covers the `/offices/{officeId}` endpoints for retrieving office
//! information and published headline summaries.

use super::{BinaryPayload, Error};
use crate::client::{Client, http};
use crate::models;

/// Returns metadata about a specific NWS office.
///
/// Corresponds to the `/offices/{id}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS office ID (e.g., "TOP", "WRH", "NWS").
///
/// # Returns
///
/// A `Result` containing [`models::Office`] metadata on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., invalid office ID)
/// or the response cannot be parsed.
pub async fn get_forecast_office(
    client: &Client,
    id: &models::NwsOfficeId,
) -> Result<models::Office, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a specific news headline for a given NWS office.
///
/// Corresponds to the `/offices/{id}/headlines/{headlineId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS office ID.
/// * `headline_id`: The unique identifier of the headline.
///
/// # Returns
///
/// A `Result` containing a [`models::OfficeHeadline`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., headline not found)
/// or the response cannot be parsed.
pub async fn get_forecast_office_headline(
    client: &Client,
    id: &models::NwsOfficeId,
    headline_id: &str,
) -> Result<models::OfficeHeadline, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .literal_path("headlines")
        .path_segment(headline_id)
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns a collection of recent news headlines for a given NWS office.
///
/// Corresponds to the `/offices/{id}/headlines` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS office ID.
///
/// # Returns
///
/// A `Result` containing an [`models::OfficeHeadlineCollection`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_forecast_office_headlines(
    client: &Client,
    id: &models::NwsOfficeId,
) -> Result<models::OfficeHeadlineCollection, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .literal_path("headlines")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Returns the active briefing metadata for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/briefing` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS office ID.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_forecast_office_briefing(
    client: &Client,
    id: &models::NwsOfficeId,
) -> Result<models::OfficeBriefingResponse, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .literal_path("briefing")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Downloads the latest briefing document for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/briefing/download/latest` endpoint.
/// The configured HTTP client's redirect policy determines whether its redirect
/// response is followed.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS office ID.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails, a redirect is not followed, or the
/// final response is not a PDF.
pub async fn get_latest_forecast_office_briefing_document(
    client: &Client,
    id: &models::NwsOfficeId,
) -> Result<BinaryPayload, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .literal_path("briefing")
        .literal_path("download")
        .literal_path("latest")
        .binary(http::BinaryMedia::Pdf)
        .await
}

/// Downloads a briefing document by its identifier for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/briefing/download/{briefingId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS forecast office ID.
/// * `briefing_id`: The identifier of the briefing document.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response is not a PDF.
pub async fn get_forecast_office_briefing_document(
    client: &Client,
    id: &models::NwsOfficeId,
    briefing_id: &str,
) -> Result<BinaryPayload, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .literal_path("briefing")
        .literal_path("download")
        .path_segment(briefing_id)
        .binary(http::BinaryMedia::Pdf)
        .await
}

/// Returns the active weather stories for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/weatherstories` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS office ID.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_forecast_office_weather_stories(
    client: &Client,
    id: &models::NwsOfficeId,
) -> Result<models::OfficeWeatherStoryCollection, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .literal_path("weatherstories")
        .json(http::JsonMedia::JsonLd)
        .await
}

/// Downloads a weather-story image by its identifier for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/weatherstories/download/{imageId}` endpoint.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `id`: The NWS forecast office ID.
/// * `image_id`: The identifier of the weather-story image.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response is not an image.
pub async fn get_forecast_office_weather_story_image(
    client: &Client,
    id: &models::NwsOfficeId,
    image_id: &str,
) -> Result<BinaryPayload, Error> {
    http::request(client, "/offices")
        .path_segment(id)
        .literal_path("weatherstories")
        .literal_path("download")
        .path_segment(image_id)
        .binary(http::BinaryMedia::Image)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{
        get_forecast_office, get_forecast_office_briefing, get_forecast_office_briefing_document,
        get_forecast_office_headline, get_forecast_office_headlines,
        get_forecast_office_weather_stories, get_forecast_office_weather_story_image,
        get_latest_forecast_office_briefing_document,
    };
    use crate::{
        Error,
        client::test_support::client_for,
        models::{NwsForecastOfficeId, NwsOfficeId, NwsRegionalHqid},
    };

    fn psr() -> NwsOfficeId {
        NwsForecastOfficeId::Psr.into()
    }

    #[tokio::test]
    async fn office_metadata_accepts_forecast_and_regional_hq_ids() {
        for (office, expected_path) in [
            (NwsOfficeId::from(NwsForecastOfficeId::Psr), "/offices/PSR"),
            (NwsOfficeId::from(NwsRegionalHqid::Wrh), "/offices/WRH"),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path(expected_path))
                .and(header("Accept", "application/ld+json"))
                .respond_with(ResponseTemplate::new(200).set_body_raw(
                    format!(r#"{{"id":"https://api.weather.gov{expected_path}"}}"#),
                    "application/ld+json",
                ))
                .expect(1)
                .mount(&server)
                .await;

            let response = get_forecast_office(&client_for(&server), &office)
                .await
                .unwrap();
            assert_eq!(
                response.id.as_deref(),
                Some(format!("https://api.weather.gov{expected_path}").as_str())
            );
        }
    }

    #[tokio::test]
    async fn office_metadata_rejects_generic_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/json"))
            .expect(1)
            .mount(&server)
            .await;

        let Error::Protocol(error) = get_forecast_office(&client_for(&server), &psr())
            .await
            .unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ld+json"));
        assert_eq!(error.actual(), Some("application/json"));
    }

    #[tokio::test]
    async fn office_headline_encodes_its_dynamic_segment_and_requests_json_ld() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/headlines/headline%20%2F%25%3F"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(1)
            .mount(&server)
            .await;

        get_forecast_office_headline(&client_for(&server), &psr(), "headline /%?")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn office_headlines_request_json_ld() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/headlines"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(r#"{"@context":[],"@graph":[]}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        get_forecast_office_headlines(&client_for(&server), &psr())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn briefing_accepts_direct_wrapped_and_null_responses() {
        let direct_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"id":"brief-1","officeId":"PSR","title":"Monsoon outlook"}"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&direct_server)
            .await;

        let direct = get_forecast_office_briefing(&client_for(&direct_server), &psr())
            .await
            .unwrap();
        assert_eq!(direct.context, None);
        assert_eq!(
            direct.briefing.unwrap().title.as_deref(),
            Some("Monsoon outlook")
        );

        let wrapped_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"@context":{"@version":"1.1"},"briefing":{"download":"https://example.test/brief.pdf"}}"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&wrapped_server)
            .await;

        let wrapped = get_forecast_office_briefing(&client_for(&wrapped_server), &psr())
            .await
            .unwrap();
        assert!(wrapped.context.is_some());
        assert_eq!(
            wrapped.briefing.unwrap().download.as_deref(),
            Some("https://example.test/brief.pdf")
        );

        let null_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(r#"{"briefing":null}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&null_server)
            .await;

        let null = get_forecast_office_briefing(&client_for(&null_server), &psr())
            .await
            .unwrap();
        assert_eq!(null.briefing, None);
    }

    #[tokio::test]
    async fn briefing_rejects_generic_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(r#"{"briefing":null}"#, "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let Error::Protocol(error) = get_forecast_office_briefing(&client_for(&server), &psr())
            .await
            .unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ld+json"));
        assert_eq!(error.actual(), Some("application/json"));
    }

    #[tokio::test]
    async fn weather_stories_accept_bare_and_wrapped_responses() {
        let bare_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/weatherstories"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"[{"title":"Heat","order":0},{"title":null,"description":null}]"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&bare_server)
            .await;

        let bare = get_forecast_office_weather_stories(&client_for(&bare_server), &psr())
            .await
            .unwrap();
        assert_eq!(bare.context, None);
        assert_eq!(bare.stories.len(), 2);
        assert_eq!(bare.stories[0].order, Some(0));
        assert_eq!(bare.stories[1].title, None);
        assert_eq!(bare.stories[1].description, None);
        assert_eq!(bare.stories[1].alt_text, None);

        let wrapped_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/weatherstories"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"@context":{"@version":"1.1"},"stories":[{"officeId":"PSR"}]}"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&wrapped_server)
            .await;

        let wrapped = get_forecast_office_weather_stories(&client_for(&wrapped_server), &psr())
            .await
            .unwrap();
        assert!(wrapped.context.is_some());
        assert_eq!(wrapped.stories[0].office_id.as_deref(), Some("PSR"));
        assert_eq!(wrapped.stories[0].alt_text, None);
        assert_eq!(wrapped.stories[0].order, None);
    }

    #[tokio::test]
    async fn weather_stories_reject_generic_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/weatherstories"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("[]", "application/json"))
            .expect(1)
            .mount(&server)
            .await;

        let Error::Protocol(error) =
            get_forecast_office_weather_stories(&client_for(&server), &psr())
                .await
                .unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ld+json"));
        assert_eq!(error.actual(), Some("application/json"));
    }

    #[tokio::test]
    async fn briefing_documents_encode_ids_and_preserve_pdf_payload_metadata() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing/download/briefing%202026%2F%25"))
            .and(header("Accept", "application/pdf"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"%PDF-briefing")
                    .insert_header("Content-Type", "application/pdf; version=1.7"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let payload =
            get_forecast_office_briefing_document(&client_for(&server), &psr(), "briefing 2026/%")
                .await
                .unwrap();
        assert_eq!(payload.as_bytes(), b"%PDF-briefing");
        assert_eq!(payload.content_type().essence_str(), "application/pdf");
        assert_eq!(
            payload.final_url().path(),
            "/offices/PSR/briefing/download/briefing%202026%2F%25"
        );
    }

    #[tokio::test]
    async fn latest_briefing_follows_relative_redirect_to_pdf() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing/download/latest"))
            .and(header("Accept", "application/pdf"))
            .respond_with(ResponseTemplate::new(302).insert_header("Location", "/files/latest.pdf"))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/files/latest.pdf"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"%PDF-latest")
                    .insert_header("Content-Type", "application/pdf"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let payload = get_latest_forecast_office_briefing_document(&client_for(&server), &psr())
            .await
            .unwrap();
        assert_eq!(payload.as_bytes(), b"%PDF-latest");
        assert_eq!(payload.content_type().essence_str(), "application/pdf");
        assert_eq!(payload.final_url().path(), "/files/latest.pdf");
    }

    #[tokio::test]
    async fn weather_story_images_encode_ids_and_preserve_image_payload_metadata() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/offices/PSR/weatherstories/download/image%202026%2F%25",
            ))
            .and(header("Accept", "image/*"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"image-bytes")
                    .insert_header("Content-Type", "image/avif"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let payload =
            get_forecast_office_weather_story_image(&client_for(&server), &psr(), "image 2026/%")
                .await
                .unwrap();
        assert_eq!(payload.as_bytes(), b"image-bytes");
        assert_eq!(payload.content_type().essence_str(), "image/avif");
        assert_eq!(
            payload.final_url().path(),
            "/offices/PSR/weatherstories/download/image%202026%2F%25"
        );
    }
}
