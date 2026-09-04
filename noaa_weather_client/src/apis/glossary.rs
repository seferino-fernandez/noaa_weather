//! NWS glossary terms: the `/glossary` endpoint.
//!
//! Obtain the handle with [`Client::glossary`].

use super::Error;
use crate::client::{Client, http};
use crate::models::GlossaryResponse;

/// The `/glossary` endpoint, obtained from [`Client::glossary`].
#[derive(Clone, Copy, Debug)]
pub struct Glossary<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/glossary` endpoint.
    #[must_use]
    pub fn glossary(&self) -> Glossary<'_> {
        Glossary { client: self }
    }
}

impl Glossary<'_> {
    /// Returns every glossary term and its definition.
    ///
    /// `GET /glossary`
    ///
    /// ```no_run
    /// use noaa_weather_client::Client;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let glossary = client.glossary().terms().await?;
    /// println!("{} terms", glossary.glossary.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn terms(&self) -> Result<GlossaryResponse, Error> {
        http::request(self.client, "/glossary")
            .json(http::JsonMedia::JsonLd)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use crate::{Error, client::test_support::client_for};

    #[tokio::test]
    async fn requests_json_ld_and_returns_typed_terms() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/glossary"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"glossary":[{"term":"Virga","definition":"Precipitation that evaporates before reaching the ground."}]}"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let response = client_for(&server).glossary().terms().await.unwrap();
        assert_eq!(response.glossary[0].term, "Virga");
        assert_eq!(
            response.glossary[0].definition,
            "Precipitation that evaporates before reaching the ground."
        );
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
    }

    #[tokio::test]
    async fn rejects_generic_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/glossary"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(r#"{"glossary":[]}"#, "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let Error::Protocol(error) = client_for(&server).glossary().terms().await.unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert_eq!(error.expected(), Some("application/ld+json"));
        assert_eq!(error.actual(), Some("application/json"));
    }

    #[tokio::test]
    async fn retains_problem_detail_for_non_success_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/glossary"))
            .respond_with(ResponseTemplate::new(503).set_body_raw(
                r#"{"type":"https://api.weather.gov/problems/unavailable","title":"Unavailable","status":503,"detail":"Try later","instance":"urn:test","correlationId":"test-correlation"}"#,
                "application/problem+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let error = client_for(&server).glossary().terms().await.unwrap_err();
        let Error::Response(response) = error else {
            panic!("expected response error");
        };
        let problem = response.problem_detail().expect("typed problem detail");
        assert_eq!(problem.title, "Unavailable");
        assert_eq!(problem.status, 503.0);
    }
}
