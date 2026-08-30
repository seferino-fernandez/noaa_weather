//! NWS forecast office metadata and headlines.
//!
//! Covers the `/offices/{officeId}` endpoints for retrieving office
//! information and published headline summaries.

use super::{BinaryPayload, Error, configuration, http};
use crate::models;

/// Returns metadata about a specific NWS office.
///
/// Corresponds to the `/offices/{id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
) -> Result<models::Office, Error> {
    let uri_str = format!("/offices/{id}", id = id);
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a specific news headline for a given NWS office.
///
/// Corresponds to the `/offices/{id}/headlines/{headlineId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
    headline_id: &str,
) -> Result<models::OfficeHeadline, Error> {
    let uri_str = format!(
        "/offices/{id}/headlines/{headlineId}",
        id = id,
        headlineId = crate::apis::urlencode(headline_id)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a collection of recent news headlines for a given NWS office.
///
/// Corresponds to the `/offices/{id}/headlines` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
) -> Result<models::OfficeHeadlineCollection, Error> {
    let uri_str = format!("/offices/{id}/headlines", id = id);
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns the active briefing metadata for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/briefing` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The NWS office ID.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_forecast_office_briefing(
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
) -> Result<models::OfficeBriefingResponse, Error> {
    let uri_str = format!("/offices/{id}/briefing", id = id);

    http::get(configuration, &uri_str)
        .header("Accept", "application/ld+json")
        .json()
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
/// * `configuration`: The API client configuration.
/// * `id`: The NWS office ID.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails, a redirect is not followed, or the
/// final response is not a PDF.
pub async fn get_latest_forecast_office_briefing_document(
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
) -> Result<BinaryPayload, Error> {
    let uri_str = format!("/offices/{id}/briefing/download/latest", id = id);

    http::get(configuration, &uri_str)
        .header("Accept", "application/pdf")
        .binary(http::BinaryMedia::Pdf)
        .await
}

/// Downloads a briefing document by its identifier for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/briefing/download/{briefingId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The NWS forecast office ID.
/// * `briefing_id`: The identifier of the briefing document.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response is not a PDF.
pub async fn get_forecast_office_briefing_document(
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
    briefing_id: &str,
) -> Result<BinaryPayload, Error> {
    let uri_str = format!(
        "/offices/{id}/briefing/download/{briefing_id}",
        id = id,
        briefing_id = crate::apis::urlencode(briefing_id)
    );

    http::get(configuration, &uri_str)
        .header("Accept", "application/pdf")
        .binary(http::BinaryMedia::Pdf)
        .await
}

/// Returns the active weather stories for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/weatherstories` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The NWS office ID.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_forecast_office_weather_stories(
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
) -> Result<models::OfficeWeatherStoryCollection, Error> {
    let uri_str = format!("/offices/{id}/weatherstories", id = id);

    http::get(configuration, &uri_str)
        .header("Accept", "application/ld+json")
        .json()
        .await
}

/// Downloads a weather-story image by its identifier for a specific NWS office.
///
/// Corresponds to the `/offices/{officeId}/weatherstories/download/{imageId}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `id`: The NWS forecast office ID.
/// * `image_id`: The identifier of the weather-story image.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response is not an image.
pub async fn get_forecast_office_weather_story_image(
    configuration: &configuration::Configuration,
    id: &models::NwsOfficeId,
    image_id: &str,
) -> Result<BinaryPayload, Error> {
    let uri_str = format!(
        "/offices/{id}/weatherstories/download/{image_id}",
        id = id,
        image_id = crate::apis::urlencode(image_id)
    );

    http::get(configuration, &uri_str)
        .header("Accept", "image/*")
        .binary(http::BinaryMedia::Image)
        .await
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, StatusCode, redirect::Policy};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{
        get_forecast_office, get_forecast_office_briefing, get_forecast_office_briefing_document,
        get_forecast_office_weather_stories, get_forecast_office_weather_story_image,
        get_latest_forecast_office_briefing_document,
    };
    use crate::{
        Error,
        apis::configuration::Configuration,
        models::{NwsForecastOfficeId, NwsOfficeId, NwsRegionalHqid},
    };

    fn configuration(server: &MockServer) -> Configuration {
        Configuration::new(None, Some(server.uri()), None, None)
    }

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
                .respond_with(ResponseTemplate::new(200).set_body_raw(
                    format!(r#"{{"id":"https://api.weather.gov{expected_path}"}}"#),
                    "application/ld+json",
                ))
                .expect(1)
                .mount(&server)
                .await;

            let response = get_forecast_office(&configuration(&server), &office)
                .await
                .unwrap();
            assert_eq!(
                response.id.as_deref(),
                Some(format!("https://api.weather.gov{expected_path}").as_str())
            );
        }
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

        let direct = get_forecast_office_briefing(&configuration(&direct_server), &psr())
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

        let wrapped = get_forecast_office_briefing(&configuration(&wrapped_server), &psr())
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

        let null = get_forecast_office_briefing(&configuration(&null_server), &psr())
            .await
            .unwrap();
        assert_eq!(null.briefing, None);
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

        let bare = get_forecast_office_weather_stories(&configuration(&bare_server), &psr())
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

        let wrapped = get_forecast_office_weather_stories(&configuration(&wrapped_server), &psr())
            .await
            .unwrap();
        assert!(wrapped.context.is_some());
        assert_eq!(wrapped.stories[0].office_id.as_deref(), Some("PSR"));
        assert_eq!(wrapped.stories[0].alt_text, None);
        assert_eq!(wrapped.stories[0].order, None);
    }

    #[tokio::test]
    async fn briefing_documents_encode_ids_and_preserve_pdf_payload_metadata() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing/download/briefing%2F2026"))
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
            get_forecast_office_briefing_document(&configuration(&server), &psr(), "briefing/2026")
                .await
                .unwrap();
        assert_eq!(payload.as_bytes(), b"%PDF-briefing");
        assert_eq!(payload.content_type().essence_str(), "application/pdf");
        assert_eq!(
            payload.final_url().path(),
            "/offices/PSR/briefing/download/briefing%2F2026"
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

        let payload = get_latest_forecast_office_briefing_document(&configuration(&server), &psr())
            .await
            .unwrap();
        assert_eq!(payload.as_bytes(), b"%PDF-latest");
        assert_eq!(payload.content_type().essence_str(), "application/pdf");
        assert_eq!(payload.final_url().path(), "/files/latest.pdf");
    }

    #[tokio::test]
    async fn latest_briefing_respects_a_no_follow_redirect_policy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/briefing/download/latest"))
            .and(header("Accept", "application/pdf"))
            .respond_with(
                ResponseTemplate::new(302)
                    .insert_header("Location", "/files/latest.pdf")
                    .set_body_string("redirecting"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder().redirect(Policy::none()).build().unwrap();
        let config = Configuration::new(None, Some(server.uri()), Some(client), None);
        let Error::Response(response) =
            get_latest_forecast_office_briefing_document(&config, &psr())
                .await
                .unwrap_err()
        else {
            panic!("expected redirect response error");
        };
        assert_eq!(response.status(), StatusCode::FOUND);
        assert_eq!(
            response.url().path(),
            "/offices/PSR/briefing/download/latest"
        );
        assert_eq!(response.as_bytes(), b"redirecting");
    }

    #[tokio::test]
    async fn weather_story_images_encode_ids_and_preserve_image_payload_metadata() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/weatherstories/download/image%2F2026"))
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
            get_forecast_office_weather_story_image(&configuration(&server), &psr(), "image/2026")
                .await
                .unwrap();
        assert_eq!(payload.as_bytes(), b"image-bytes");
        assert_eq!(payload.content_type().essence_str(), "image/avif");
        assert_eq!(
            payload.final_url().path(),
            "/offices/PSR/weatherstories/download/image%2F2026"
        );
    }
}
