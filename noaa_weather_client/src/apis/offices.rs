//! Forecast office metadata, headlines, briefings, and weather stories: the
//! `/offices` family.
//!
//! Obtain the handle with [`Client::offices`]. Every operation takes an
//! [`OfficeId`], which accepts forecast offices as well as regional (`WRH`)
//! and national (`NWS`) headquarters. Briefing documents and weather-story
//! images are binary and return a [`BinaryPayload`].
//!
//! ```no_run
//! use noaa_weather_client::{Client, OfficeId};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let office: OfficeId = "PSR".parse()?;
//! let headlines = client.offices().headlines(&office).await?;
//! # let _ = headlines;
//! # Ok(())
//! # }
//! ```

use super::{BinaryPayload, Error};
use crate::client::{Client, http};
use crate::ids::OfficeId;
use crate::models;

/// The `/offices` endpoints, obtained from [`Client::offices`].
#[derive(Clone, Copy, Debug)]
pub struct Offices<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/offices` endpoints.
    #[must_use]
    pub fn offices(&self) -> Offices<'_> {
        Offices { client: self }
    }
}

impl Offices<'_> {
    fn office(&self, office: &OfficeId) -> http::ContractRequest<'_> {
        http::request(self.client, "/offices").path_segment(office)
    }

    /// Returns metadata for one office.
    ///
    /// `GET /offices/{officeId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let office: OfficeId = "TOP".parse()?;
    /// let metadata = client.offices().get(&office).await?;
    /// println!("{:?}", metadata.name);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the office is unknown, or
    /// the response cannot be decoded.
    pub async fn get(&self, office: &OfficeId) -> Result<models::Office, Error> {
        self.office(office).json(http::JsonMedia::JsonLd).await
    }

    /// Returns the recent news headlines published by one office.
    ///
    /// `GET /offices/{officeId}/headlines`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let office: OfficeId = "PSR".parse()?;
    /// let headlines = client.offices().headlines(&office).await?;
    /// # let _ = headlines;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn headlines(
        &self,
        office: &OfficeId,
    ) -> Result<models::OfficeHeadlineCollection, Error> {
        self.office(office)
            .literal_path("headlines")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns one headline by its server-issued id.
    ///
    /// `GET /offices/{officeId}/headlines/{headlineId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let office: OfficeId = "PSR".parse()?;
    /// let headline = client
    ///     .offices()
    ///     .headline(&office, "593627f70073a49e2483c3e0bf4f8221")
    ///     .await?;
    /// # let _ = headline;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the headline is unknown, or
    /// the response cannot be decoded.
    pub async fn headline(
        &self,
        office: &OfficeId,
        headline_id: &str,
    ) -> Result<models::OfficeHeadline, Error> {
        self.office(office)
            .literal_path("headlines")
            .path_segment(headline_id)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns the active briefing metadata for one office, if any.
    ///
    /// `GET /offices/{officeId}/briefing`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let office: OfficeId = "PSR".parse()?;
    /// let briefing = client.offices().briefing(&office).await?;
    /// # let _ = briefing;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn briefing(
        &self,
        office: &OfficeId,
    ) -> Result<models::OfficeBriefingResponse, Error> {
        self.office(office)
            .literal_path("briefing")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Downloads the latest briefing PDF for one office, following NOAA's
    /// redirect to the document.
    ///
    /// `GET /offices/{officeId}/briefing/download/latest`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build()?;
    /// let office: OfficeId = "PSR".parse()?;
    /// let pdf = client.offices().latest_briefing_document(&office).await?;
    /// std::fs::write("briefing.pdf", pdf.as_bytes())?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, a redirect cannot be
    /// followed, or the final response is not a PDF.
    pub async fn latest_briefing_document(
        &self,
        office: &OfficeId,
    ) -> Result<BinaryPayload, Error> {
        self.office(office)
            .literal_path("briefing/download/latest")
            .binary(http::BinaryMedia::Pdf)
            .await
    }

    /// Downloads one briefing PDF by its server-issued id.
    ///
    /// `GET /offices/{officeId}/briefing/download/{briefingId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build()?;
    /// let office: OfficeId = "PSR".parse()?;
    /// let pdf = client.offices().briefing_document(&office, "brief-1").await?;
    /// # let _ = pdf;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response is not a
    /// PDF.
    pub async fn briefing_document(
        &self,
        office: &OfficeId,
        briefing_id: &str,
    ) -> Result<BinaryPayload, Error> {
        self.office(office)
            .literal_path("briefing/download")
            .path_segment(briefing_id)
            .binary(http::BinaryMedia::Pdf)
            .await
    }

    /// Returns the active weather stories for one office.
    ///
    /// `GET /offices/{officeId}/weatherstories`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let office: OfficeId = "PSR".parse()?;
    /// let stories = client.offices().weather_stories(&office).await?;
    /// # let _ = stories;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn weather_stories(
        &self,
        office: &OfficeId,
    ) -> Result<models::OfficeWeatherStoryCollection, Error> {
        self.office(office)
            .literal_path("weatherstories")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Downloads one weather-story image by its server-issued id.
    ///
    /// `GET /offices/{officeId}/weatherstories/download/{imageId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, OfficeId};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build()?;
    /// let office: OfficeId = "PSR".parse()?;
    /// let image = client.offices().weather_story_image(&office, "story-1").await?;
    /// println!("{}", image.content_type());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response is not an
    /// image.
    pub async fn weather_story_image(
        &self,
        office: &OfficeId,
        image_id: &str,
    ) -> Result<BinaryPayload, Error> {
        self.office(office)
            .literal_path("weatherstories/download")
            .path_segment(image_id)
            .binary(http::BinaryMedia::Image)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use crate::{Error, OfficeId, client::test_support::client_for};

    fn psr() -> OfficeId {
        "psr".parse().unwrap()
    }

    #[tokio::test]
    async fn office_metadata_accepts_forecast_and_regional_hq_ids() {
        for (office, expected_path) in [("PSR", "/offices/PSR"), ("wrh", "/offices/WRH")] {
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

            let response = client_for(&server)
                .offices()
                .get(&office.parse().unwrap())
                .await
                .unwrap();
            assert_eq!(
                response.id.as_deref(),
                Some(format!("https://api.weather.gov{expected_path}").as_str())
            );
            let requests = server.received_requests().await.unwrap();
            assert_eq!(requests[0].url.query(), None);
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

        let Error::Protocol(error) = client_for(&server).offices().get(&psr()).await.unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ld+json"));
        assert_eq!(error.actual(), Some("application/json"));
    }

    #[tokio::test]
    async fn headline_routes_encode_opaque_ids_and_request_json_ld() {
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
        Mock::given(method("GET"))
            .and(path("/offices/PSR/headlines/headline%20%2F%25%3F"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/ld+json"))
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        client.offices().headlines(&psr()).await.unwrap();
        client
            .offices()
            .headline(&psr(), "headline /%?")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn briefing_accepts_direct_wrapped_and_null_responses() {
        for (body, expect_context, expect_title) in [
            (
                r#"{"id":"brief-1","officeId":"PSR","title":"Monsoon outlook"}"#,
                false,
                Some("Monsoon outlook"),
            ),
            (
                r#"{"@context":{"@version":"1.1"},"briefing":{"title":"Wrapped"}}"#,
                true,
                Some("Wrapped"),
            ),
            (r#"{"briefing":null}"#, false, None),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/offices/PSR/briefing"))
                .and(header("Accept", "application/ld+json"))
                .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/ld+json"))
                .expect(1)
                .mount(&server)
                .await;

            let response = client_for(&server)
                .offices()
                .briefing(&psr())
                .await
                .unwrap();
            assert_eq!(response.context.is_some(), expect_context, "{body}");
            assert_eq!(
                response
                    .briefing
                    .as_ref()
                    .and_then(|briefing| briefing.title.as_deref()),
                expect_title,
                "{body}"
            );
        }
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

        let bare = client_for(&bare_server)
            .offices()
            .weather_stories(&psr())
            .await
            .unwrap();
        assert_eq!(bare.context, None);
        assert_eq!(bare.stories.len(), 2);
        assert_eq!(bare.stories[0].order, Some(0));
        assert_eq!(bare.stories[1].title, None);

        let wrapped_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/offices/PSR/weatherstories"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"@context":{"@version":"1.1"},"stories":[{"officeId":"PSR"}]}"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&wrapped_server)
            .await;

        let wrapped = client_for(&wrapped_server)
            .offices()
            .weather_stories(&psr())
            .await
            .unwrap();
        assert!(wrapped.context.is_some());
        assert_eq!(wrapped.stories[0].office_id.as_deref(), Some("PSR"));
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

        let payload = client_for(&server)
            .offices()
            .briefing_document(&psr(), "briefing 2026/%")
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

        let payload = client_for(&server)
            .offices()
            .latest_briefing_document(&psr())
            .await
            .unwrap();
        assert_eq!(payload.as_bytes(), b"%PDF-latest");
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

        let payload = client_for(&server)
            .offices()
            .weather_story_image(&psr(), "image 2026/%")
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
