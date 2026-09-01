//! NWS glossary terms.

use super::Error;
use crate::client::{Client, http};
use crate::models::GlossaryResponse;

/// Returns the NWS glossary.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_glossary(client: &Client) -> Result<GlossaryResponse, Error> {
    http::request(client, "/glossary")
        .json(http::JsonMedia::JsonLd)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::get_glossary;
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

        let response = get_glossary(&client_for(&server)).await.unwrap();
        assert_eq!(response.glossary[0].term.as_deref(), Some("Virga"));
        assert_eq!(
            response.glossary[0].definition.as_deref(),
            Some("Precipitation that evaporates before reaching the ground.")
        );
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

        let Error::Protocol(error) = get_glossary(&client_for(&server)).await.unwrap_err() else {
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

        let error = get_glossary(&client_for(&server)).await.unwrap_err();
        let Error::Response(response) = error else {
            panic!("expected response error");
        };
        let problem = response.problem_detail().expect("typed problem detail");
        assert_eq!(problem.title, "Unavailable");
        assert_eq!(problem.status, 503.0);
    }
}
