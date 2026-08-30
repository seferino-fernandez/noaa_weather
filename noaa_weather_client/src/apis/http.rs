use bytes::Bytes;
use mime::Mime;
use reqwest::header::{CONTENT_TYPE, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;

use super::{BinaryPayload, Error, ProtocolError, ResponseContent, configuration::Configuration};

const API_KEY_HEADER: &str = "X-Api-Key";
const JSON_EXPECTED: &str = "application/json or application/*+json";
#[cfg(feature = "xml")]
const XML_EXPECTED: &str = "application/xml, text/xml, or application/*+xml";

pub(crate) fn get(configuration: &Configuration, path: &str) -> NoaaRequest {
    let url = format!(
        "{}/{}",
        configuration.base_path.trim_end_matches('/'),
        path.trim_start_matches('/')
    );
    let mut request = configuration.client.get(url);
    if let Some(user_agent) = &configuration.user_agent {
        request = request.header(reqwest::header::USER_AGENT, user_agent);
    }
    if let Some(api_key) = &configuration.api_key {
        request = request.header(API_KEY_HEADER, api_key);
    }
    NoaaRequest { request }
}

pub(crate) struct NoaaRequest {
    request: reqwest::RequestBuilder,
}

impl NoaaRequest {
    pub(crate) fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.request = self.request.query(query);
        self
    }

    pub(crate) fn header(mut self, name: &'static str, value: impl AsRef<str>) -> Self {
        self.request = self.request.header(name, value.as_ref());
        self
    }

    pub(crate) async fn json<T: DeserializeOwned>(self) -> Result<T, Error> {
        let response = self.send().await?;
        ensure_content_type(&response, JSON_EXPECTED, is_json)?;
        serde_json::from_slice(&response.bytes).map_err(Error::from)
    }

    #[cfg(feature = "xml")]
    pub(crate) async fn xml<T: DeserializeOwned>(self) -> Result<T, Error> {
        let response = self.send().await?;
        ensure_content_type(&response, XML_EXPECTED, is_xml)?;
        quick_xml::de::from_reader(response.bytes.as_ref()).map_err(Error::from)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) async fn binary(self, media: BinaryMedia) -> Result<BinaryPayload, Error> {
        let response = self.send().await?;
        let expected = media.expected();
        let content_type = ensure_content_type(&response, expected, |mime| media.matches(mime))?;
        Ok(BinaryPayload {
            bytes: response.bytes,
            content_type,
            final_url: response.url,
        })
    }

    async fn send(self) -> Result<ReceivedResponse, Error> {
        let response = self.request.send().await?;
        let status = response.status();
        let url = response.url().clone();
        let content_type = response.headers().get(CONTENT_TYPE).cloned();
        let bytes = response.bytes().await?;

        if status.is_success() {
            return Ok(ReceivedResponse {
                bytes,
                content_type,
                url,
            });
        }

        let parsed_content_type = content_type
            .as_ref()
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse().ok());
        let problem_detail = serde_json::from_slice(&bytes).ok();
        Err(Error::Response(Box::new(ResponseContent {
            bytes,
            status,
            url,
            problem_detail,
            content_type: parsed_content_type,
        })))
    }
}

struct ReceivedResponse {
    bytes: Bytes,
    content_type: Option<HeaderValue>,
    url: Url,
}

#[derive(Clone, Copy)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum BinaryMedia {
    Pdf,
    Image,
}

impl BinaryMedia {
    const fn expected(self) -> &'static str {
        match self {
            Self::Pdf => "application/pdf",
            Self::Image => "image/*",
        }
    }

    fn matches(self, content_type: &Mime) -> bool {
        match self {
            Self::Pdf => {
                content_type.type_() == mime::APPLICATION && content_type.subtype() == mime::PDF
            }
            Self::Image => content_type.type_() == mime::IMAGE,
        }
    }
}

fn ensure_content_type(
    response: &ReceivedResponse,
    expected: &'static str,
    accepts: impl FnOnce(&Mime) -> bool,
) -> Result<Mime, Error> {
    let Some(header) = &response.content_type else {
        return Err(protocol(ProtocolError::MissingContentType {
            expected,
            url: response.url.clone(),
        }));
    };
    let header = header.to_str().map_err(|_| {
        protocol(ProtocolError::MalformedContentType {
            expected,
            actual: String::from_utf8_lossy(header.as_bytes()).into_owned(),
            url: response.url.clone(),
        })
    })?;
    let content_type = header.parse::<Mime>().map_err(|_| {
        protocol(ProtocolError::MalformedContentType {
            expected,
            actual: header.to_owned(),
            url: response.url.clone(),
        })
    })?;
    if !accepts(&content_type) {
        return Err(protocol(ProtocolError::IncompatibleContentType {
            expected,
            actual: content_type,
            url: response.url.clone(),
        }));
    }
    Ok(content_type)
}

fn protocol(error: ProtocolError) -> Error {
    Error::Protocol(Box::new(error))
}

fn is_json(content_type: &Mime) -> bool {
    content_type.type_() == mime::APPLICATION
        && (content_type.subtype() == mime::JSON || content_type.suffix() == Some(mime::JSON))
}

#[cfg(feature = "xml")]
fn is_xml(content_type: &Mime) -> bool {
    (content_type.type_() == mime::APPLICATION || content_type.type_() == mime::TEXT)
        && (content_type.subtype() == mime::XML
            || (content_type.type_() == mime::APPLICATION
                && content_type.suffix() == Some(mime::XML)))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use reqwest::{Client, redirect::Policy};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path, query_param},
    };

    use super::{BinaryMedia, get};
    #[cfg(feature = "radio")]
    use crate::apis::radio;
    use crate::{
        Error, ProtocolError,
        apis::{alerts, configuration::Configuration},
    };

    const ALERT_TYPES: &str = r#"{"eventTypes":["Test Warning"]}"#;
    const ALERT_COLLECTION: &str = r#"{"type":"FeatureCollection","features":[]}"#;
    #[cfg(feature = "radio")]
    const RADIO_BROADCAST: &str = r#"<speak version="1.1" xml:lang="en-US"></speak>"#;

    fn configuration(server: &MockServer, suffix: &str) -> Configuration {
        Configuration::new(None, Some(format!("{}{suffix}", server.uri())), None, None)
    }

    #[tokio::test]
    async fn public_endpoint_joins_prefixed_base_path_and_manages_headers() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/wiremock/prefix/alerts/types"))
            .and(header("User-Agent", "foundation-tests/1.0"))
            .and(header("X-Api-Key", "secret"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(ALERT_TYPES, "application/json"))
            .expect(1)
            .mount(&server)
            .await;

        let config = Configuration::new(
            Some("foundation-tests/1.0".to_owned()),
            Some(format!("{}/wiremock/prefix", server.uri())),
            None,
            Some("secret".to_owned()),
        );
        let response = alerts::get_alert_types(&config).await.unwrap();
        assert_eq!(response.event_types.unwrap(), ["Test Warning"]);
    }

    #[tokio::test]
    async fn public_endpoint_joins_a_trailing_slash_base_path() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/prefix/alerts/types"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(ALERT_TYPES, "application/json"))
            .expect(1)
            .mount(&server)
            .await;

        let config = configuration(&server, "/prefix/");
        alerts::get_alert_types(&config).await.unwrap();
    }

    #[tokio::test]
    async fn endpoint_query_values_are_encoded_once() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts/active"))
            .and(query_param("event", "Flood Watch,High Wind Warning"))
            .and(query_param("point", "39.7,-104.9"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(ALERT_COLLECTION, "application/json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let params = alerts::ActiveAlertsParams {
            event: Some(vec![
                "Flood Watch".to_owned(),
                "High Wind Warning".to_owned(),
            ]),
            point: Some("39.7,-104.9"),
            ..Default::default()
        };
        alerts::get_active_alerts(&configuration(&server, ""), params)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn json_terminal_accepts_vendor_json() {
        let server = MockServer::start().await;
        Mock::given(path("/alerts/types"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(ALERT_TYPES, "application/problem+json; charset=utf-8"),
            )
            .mount(&server)
            .await;

        alerts::get_alert_types(&configuration(&server, ""))
            .await
            .unwrap();
    }

    #[tokio::test]
    #[cfg(feature = "radio")]
    async fn xml_terminal_accepts_xml_and_vendor_xml() {
        for content_type in ["application/xml", "application/vnd.noaa+xml"] {
            let server = MockServer::start().await;
            Mock::given(path("/radio/KEC94/broadcast"))
                .respond_with(
                    ResponseTemplate::new(200).set_body_raw(RADIO_BROADCAST, content_type),
                )
                .mount(&server)
                .await;

            let response = radio::get_area_radio(&configuration(&server, ""), "KEC94")
                .await
                .unwrap();
            assert_eq!(response.version, "1.1");
            assert_eq!(response.lang, "en-US");
        }
    }

    #[tokio::test]
    async fn response_error_keeps_typed_problem_and_raw_body() {
        let server = MockServer::start().await;
        let problem = r#"{"type":"urn:test","title":"Bad point","status":400,"detail":"outside forecast area","instance":"urn:instance","correlationId":"abc-123"}"#;
        Mock::given(path("/alerts/types"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_string(problem)
                    .insert_header("Content-Type", "text/plain"),
            )
            .mount(&server)
            .await;

        let Error::Response(response) = alerts::get_alert_types(&configuration(&server, ""))
            .await
            .unwrap_err()
        else {
            panic!("expected response error");
        };
        assert_eq!(response.status(), 400);
        assert_eq!(response.as_bytes(), problem.as_bytes());
        assert_eq!(response.text(), problem);
        assert_eq!(response.content_type().unwrap(), &mime::TEXT_PLAIN);
        assert_eq!(response.problem_detail().unwrap().title, "Bad point");
        assert_eq!(response.url().path(), "/alerts/types");
    }

    #[tokio::test]
    async fn response_error_keeps_an_unrecognized_binary_body() {
        let server = MockServer::start().await;
        let body = b"not-json\xff";
        Mock::given(path("/alerts/types"))
            .respond_with(ResponseTemplate::new(503).set_body_bytes(body))
            .mount(&server)
            .await;

        let Error::Response(response) = alerts::get_alert_types(&configuration(&server, ""))
            .await
            .unwrap_err()
        else {
            panic!("expected response error");
        };
        assert_eq!(response.as_bytes(), body);
        assert!(response.problem_detail().is_none());
        assert!(response.content_type().is_none());
        assert!(response.text().contains('\u{fffd}'));
    }

    #[tokio::test]
    #[cfg(feature = "radio")]
    async fn malformed_success_documents_keep_decode_sources() {
        let server = MockServer::start().await;
        Mock::given(path("/alerts/types"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{", "application/json"))
            .mount(&server)
            .await;
        assert!(matches!(
            alerts::get_alert_types(&configuration(&server, "")).await,
            Err(Error::Json(_))
        ));

        let server = MockServer::start().await;
        Mock::given(path("/radio/KEC94/broadcast"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("<speak>", "application/xml"))
            .mount(&server)
            .await;
        assert!(matches!(
            radio::get_area_radio(&configuration(&server, ""), "KEC94").await,
            Err(Error::Xml(_))
        ));
    }

    #[tokio::test]
    async fn successful_response_requires_a_valid_compatible_content_type() {
        for (content_type, expected_variant) in [
            (None, "missing"),
            (Some("not a mime"), "malformed"),
            (Some("text/plain"), "incompatible"),
        ] {
            let server = MockServer::start().await;
            let template = match content_type {
                None => ResponseTemplate::new(200).set_body_bytes(ALERT_TYPES),
                Some(content_type) => ResponseTemplate::new(200)
                    .set_body_bytes(ALERT_TYPES)
                    .insert_header("Content-Type", content_type),
            };
            Mock::given(path("/alerts/types"))
                .respond_with(template)
                .mount(&server)
                .await;

            let Error::Protocol(protocol) = alerts::get_alert_types(&configuration(&server, ""))
                .await
                .unwrap_err()
            else {
                panic!("expected protocol error");
            };
            match (expected_variant, protocol.as_ref()) {
                ("missing", ProtocolError::MissingContentType { .. })
                | ("malformed", ProtocolError::MalformedContentType { .. })
                | ("incompatible", ProtocolError::IncompatibleContentType { .. }) => {}
                _ => panic!("unexpected protocol error: {protocol:?}"),
            }
            assert!(protocol.expected().contains("application/json"));
            assert_eq!(protocol.url().path(), "/alerts/types");
        }
    }

    #[tokio::test]
    async fn binary_terminal_preserves_pdf_bytes_media_and_redirect_url() {
        let server = MockServer::start().await;
        Mock::given(path("/document"))
            .respond_with(ResponseTemplate::new(302).insert_header("Location", "/files/report.pdf"))
            .mount(&server)
            .await;
        Mock::given(path("/files/report.pdf"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"%PDF-foundation")
                    .insert_header("Content-Type", "application/pdf; version=1.7"),
            )
            .mount(&server)
            .await;

        let payload = get(&configuration(&server, ""), "/document")
            .binary(BinaryMedia::Pdf)
            .await
            .unwrap();
        assert_eq!(payload.as_bytes(), b"%PDF-foundation");
        assert_eq!(payload.bytes().as_ref(), b"%PDF-foundation");
        assert_eq!(payload.len(), 15);
        assert!(!payload.is_empty());
        assert_eq!(payload.content_type().essence_str(), "application/pdf");
        assert_eq!(payload.final_url().path(), "/files/report.pdf");
        assert_eq!(payload.clone().into_bytes().as_ref(), b"%PDF-foundation");
    }

    #[tokio::test]
    async fn binary_terminal_accepts_any_image_subtype() {
        let server = MockServer::start().await;
        Mock::given(path("/map"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(b"image-data")
                    .insert_header("Content-Type", "image/avif"),
            )
            .mount(&server)
            .await;

        let payload = get(&configuration(&server, ""), "/map")
            .binary(BinaryMedia::Image)
            .await
            .unwrap();
        assert_eq!(payload.as_ref(), b"image-data");
        assert_eq!(
            payload.content_type(),
            &mime::Mime::from_str("image/avif").unwrap()
        );
    }

    #[tokio::test]
    async fn no_follow_redirect_is_a_response_error() {
        let server = MockServer::start().await;
        Mock::given(path("/alerts/types"))
            .respond_with(
                ResponseTemplate::new(302)
                    .insert_header("Location", "/elsewhere")
                    .set_body_string("redirecting"),
            )
            .mount(&server)
            .await;

        let client = Client::builder().redirect(Policy::none()).build().unwrap();
        let config = Configuration::new(None, Some(server.uri()), Some(client), None);
        let Error::Response(response) = alerts::get_alert_types(&config).await.unwrap_err() else {
            panic!("expected response error");
        };
        assert_eq!(response.status(), 302);
        assert_eq!(response.url().path(), "/alerts/types");
        assert_eq!(response.as_bytes(), b"redirecting");
    }
}
