use std::{fmt, fmt::Write as _};

use bytes::Bytes;
use mime::Mime;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderValue};
use serde::de::DeserializeOwned;
use url::Url;

use super::{BinaryPayload, Error, ProtocolError, ResponseContent, configuration::Configuration};

const API_KEY_HEADER: &str = "X-Api-Key";
const FEATURE_FLAGS_HEADER: &str = "Feature-Flags";

// WHATWG path-segment escaping for special URLs. Backslash must be escaped in
// addition to the normal path-segment set because HTTPS treats it as a path
// separator.
const FRAGMENT_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const PATH_ENCODE_SET: &AsciiSet = &FRAGMENT_ENCODE_SET.add(b'#').add(b'?').add(b'{').add(b'}');
const SPECIAL_PATH_SEGMENT_ENCODE_SET: &AsciiSet = &PATH_ENCODE_SET.add(b'/').add(b'%').add(b'\\');

/// Begins a GET request whose request-contract mechanics are owned here.
pub(crate) fn request<'configuration>(
    configuration: &'configuration Configuration,
    literal_path: &'static str,
) -> ContractRequest<'configuration> {
    let mut url = configuration.base_path.trim_end_matches('/').to_owned();
    append_literal_path(&mut url, literal_path);
    ContractRequest {
        configuration,
        target: match Url::parse(&url) {
            Ok(parsed)
                if matches!(parsed.scheme(), "http" | "https") && !parsed.cannot_be_a_base() =>
            {
                RequestTarget::Parsed(parsed)
            }
            _ => RequestTarget::Invalid(url),
        },
        value_buffer: None,
        feature_flags: None,
    }
}

/// A private request contract assembled before a reqwest request is created.
pub(crate) struct ContractRequest<'configuration> {
    configuration: &'configuration Configuration,
    target: RequestTarget,
    value_buffer: Option<String>,
    feature_flags: Option<String>,
}

impl ContractRequest<'_> {
    /// Appends a trusted, compile-time path component without escaping it.
    pub(crate) fn literal_path(mut self, literal: &'static str) -> Self {
        self.target.append_literal_path(literal);
        self
    }

    /// Appends one untrusted dynamic path segment with special-URL escaping.
    pub(crate) fn path_segment(mut self, segment: impl fmt::Display) -> Self {
        let ContractRequest {
            target,
            value_buffer,
            ..
        } = &mut self;
        let segment = render_display(value_buffer, segment);
        target.append_path_segment(segment);
        self
    }

    /// Appends an optional scalar query value using HTML form encoding.
    pub(crate) fn query_scalar<T: fmt::Display>(
        mut self,
        name: &'static str,
        value: Option<T>,
    ) -> Self {
        if let Some(value) = value {
            let ContractRequest {
                target,
                value_buffer,
                ..
            } = &mut self;
            let value = render_display(value_buffer, value);
            target.append_query_pair(name, value);
        }
        self
    }

    /// Appends an optional CSV query as a single HTML-form-encoded value.
    pub(crate) fn query_csv<I, T>(mut self, name: &'static str, values: Option<I>) -> Self
    where
        I: IntoIterator<Item = T>,
        T: fmt::Display,
    {
        if let Some(values) = values {
            let ContractRequest {
                target,
                value_buffer,
                ..
            } = &mut self;
            let value_buffer = value_buffer.get_or_insert_with(|| String::with_capacity(256));
            value_buffer.clear();
            for (index, value) in values.into_iter().enumerate() {
                if index != 0 {
                    value_buffer.push(',');
                }
                write!(value_buffer, "{value}").expect("writing to String cannot fail");
            }
            target.append_query_pair(name, value_buffer);
        }
        self
    }

    /// Selects the closed set of forecast feature flags NOAA supports.
    pub(crate) fn feature_flags<I>(mut self, flags: I) -> Self
    where
        I: IntoIterator<Item = FeatureFlag>,
    {
        self.feature_flags = Some(csv(flags));
        self
    }

    /// Requests, validates, and decodes one JSON media family.
    pub(crate) async fn json<T: DeserializeOwned>(self, media: JsonMedia) -> Result<T, Error> {
        let response = self.send(media.accept()).await?;
        ensure_content_type(&response, media.expected(), |mime| media.matches(mime))?;
        serde_json::from_slice(&response.bytes).map_err(Error::from)
    }

    /// Requests, validates, and decodes one XML media family.
    #[cfg(feature = "xml")]
    pub(crate) async fn xml<T: DeserializeOwned>(self, media: XmlMedia) -> Result<T, Error> {
        let response = self.send(media.accept()).await?;
        ensure_content_type(&response, media.expected(), |mime| media.matches(mime))?;
        quick_xml::de::from_reader(response.bytes.as_ref()).map_err(Error::from)
    }

    /// Requests and validates one binary media family without decoding it.
    pub(crate) async fn binary(self, media: BinaryMedia) -> Result<BinaryPayload, Error> {
        let response = self.send(media.accept()).await?;
        let content_type =
            ensure_content_type(&response, media.expected(), |mime| media.matches(mime))?;
        Ok(BinaryPayload {
            bytes: response.bytes,
            content_type,
            final_url: response.url,
        })
    }

    fn into_builder(self, accept: &'static str) -> reqwest::RequestBuilder {
        let mut request = self
            .target
            .into_builder(self.configuration)
            .header(ACCEPT, accept);
        if let Some(feature_flags) = self.feature_flags {
            request = request.header(FEATURE_FLAGS_HEADER, feature_flags);
        }
        request
    }

    async fn send(self, accept: &'static str) -> Result<ReceivedResponse, Error> {
        receive(self.into_builder(accept)).await
    }
}

enum RequestTarget {
    Parsed(Url),
    Invalid(String),
}

impl RequestTarget {
    fn append_literal_path(&mut self, literal: &'static str) {
        match self {
            Self::Parsed(url) => {
                let literal = literal.trim_matches('/');
                if !literal.is_empty() {
                    url.path_segments_mut()
                        .expect("HTTP URLs always support path segments")
                        .extend(literal.split('/'));
                }
            }
            Self::Invalid(url) => append_literal_path(url, literal),
        }
    }

    fn append_path_segment(&mut self, segment: &str) {
        match self {
            Self::Parsed(url) => {
                // URL setters ignore dot segments. Supplying a percent-encoded
                // dot makes the setter encode `%` and retain one opaque segment.
                let segment = match segment {
                    "." => "%2E",
                    ".." => "%2E%2E",
                    segment => segment,
                };
                url.path_segments_mut()
                    .expect("HTTP URLs always support path segments")
                    .push(segment);
            }
            Self::Invalid(url) => {
                url.push('/');
                let segment_start = url.len();
                write!(EncodedPathSegment(url), "{segment}")
                    .expect("writing to String cannot fail");
                let dot_count = match &url[segment_start..] {
                    "." => 1,
                    ".." => 2,
                    _ => 0,
                };
                if dot_count != 0 {
                    url.truncate(segment_start);
                    for _ in 0..dot_count {
                        url.push_str("%252E");
                    }
                }
            }
        }
    }

    fn append_query_pair(&mut self, name: &str, value: &str) {
        match self {
            Self::Parsed(url) => {
                url.query_pairs_mut().append_pair(name, value);
            }
            Self::Invalid(url) => {
                url.push(if url.contains('?') { '&' } else { '?' });
                url.extend(url::form_urlencoded::byte_serialize(name.as_bytes()));
                url.push('=');
                url.extend(url::form_urlencoded::byte_serialize(value.as_bytes()));
            }
        }
    }

    fn into_builder(self, configuration: &Configuration) -> reqwest::RequestBuilder {
        match self {
            Self::Parsed(url) => configured_get(configuration, url),
            Self::Invalid(url) => configured_get(configuration, url),
        }
    }
}

fn render_display<T: fmt::Display>(buffer: &mut Option<String>, value: T) -> &str {
    let buffer = buffer.get_or_insert_with(|| String::with_capacity(256));
    buffer.clear();
    write!(buffer, "{value}").expect("writing to String cannot fail");
    buffer
}

fn append_literal_path(url: &mut String, literal: &'static str) {
    let literal = literal.trim_matches('/');
    if !literal.is_empty() {
        url.push('/');
        url.push_str(literal);
    }
}

struct EncodedPathSegment<'url>(&'url mut String);

impl fmt::Write for EncodedPathSegment<'_> {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        self.0
            .extend(utf8_percent_encode(value, SPECIAL_PATH_SEGMENT_ENCODE_SET));
        Ok(())
    }
}

fn csv<I, T>(values: I) -> String
where
    I: IntoIterator<Item = T>,
    T: fmt::Display,
{
    let mut result = String::new();
    for (index, value) in values.into_iter().enumerate() {
        if index != 0 {
            result.push(',');
        }
        write!(&mut result, "{value}").expect("writing to String cannot fail");
    }
    result
}

#[derive(Clone, Copy)]
pub(crate) enum FeatureFlag {
    ForecastTemperatureQuantitativeValue,
    ForecastWindSpeedQuantitativeValue,
}

impl fmt::Display for FeatureFlag {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ForecastTemperatureQuantitativeValue => "forecast_temperature_qv",
            Self::ForecastWindSpeedQuantitativeValue => "forecast_wind_speed_qv",
        })
    }
}

#[derive(Clone, Copy)]
pub(crate) enum JsonMedia {
    GeoJson,
    JsonLd,
}

impl JsonMedia {
    const fn accept(self) -> &'static str {
        match self {
            Self::GeoJson => "application/geo+json",
            Self::JsonLd => "application/ld+json",
        }
    }

    const fn expected(self) -> &'static str {
        self.accept()
    }

    fn matches(self, content_type: &Mime) -> bool {
        content_type.essence_str() == self.accept()
    }
}

#[cfg(feature = "xml")]
#[derive(Clone, Copy)]
pub(crate) enum XmlMedia {
    Iwxxm,
    #[cfg(feature = "radio")]
    Ssml,
}

#[cfg(feature = "xml")]
impl XmlMedia {
    const fn accept(self) -> &'static str {
        match self {
            Self::Iwxxm => "application/vnd.wmo.iwxxm+xml",
            #[cfg(feature = "radio")]
            Self::Ssml => "application/ssml+xml",
        }
    }

    const fn expected(self) -> &'static str {
        self.accept()
    }

    fn matches(self, content_type: &Mime) -> bool {
        content_type.essence_str() == self.accept()
    }
}

fn configured_get(
    configuration: &Configuration,
    url: impl reqwest::IntoUrl,
) -> reqwest::RequestBuilder {
    let mut request = configuration.client.get(url);
    if let Some(user_agent) = &configuration.user_agent {
        request = request.header(reqwest::header::USER_AGENT, user_agent);
    }
    if let Some(api_key) = &configuration.api_key {
        request = request.header(API_KEY_HEADER, api_key);
    }
    request
}

async fn receive(request: reqwest::RequestBuilder) -> Result<ReceivedResponse, Error> {
    let response = request.send().await?;
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

struct ReceivedResponse {
    bytes: Bytes,
    content_type: Option<HeaderValue>,
    url: Url,
}

#[derive(Clone, Copy)]
pub(crate) enum BinaryMedia {
    Pdf,
    Image,
}

impl BinaryMedia {
    const fn accept(self) -> &'static str {
        self.expected()
    }

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

#[cfg(test)]
mod tests {
    use std::{
        alloc::{GlobalAlloc, Layout, System},
        hint::black_box,
        str::FromStr as _,
        sync::atomic::{AtomicBool, Ordering},
        time::Instant,
    };

    use reqwest::{Client, redirect::Policy};
    use stats_alloc::{INSTRUMENTED_SYSTEM, Region};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    #[cfg(feature = "xml")]
    use super::XmlMedia;
    use super::{BinaryMedia, FeatureFlag, JsonMedia, configured_get, request};
    #[cfg(feature = "radio")]
    use crate::apis::radio;
    use crate::{
        Error, ProtocolError,
        apis::{alerts, configuration::Configuration},
    };

    static TRACK_ALLOCATIONS: AtomicBool = AtomicBool::new(false);

    struct BenchmarkAllocator;

    #[global_allocator]
    static GLOBAL: BenchmarkAllocator = BenchmarkAllocator;

    // Allocation accounting is enabled only around the deterministic
    // allocation phase. Timing uses the system allocator directly.
    unsafe impl GlobalAlloc for BenchmarkAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
                // SAFETY: the layout is forwarded unchanged to a global allocator.
                unsafe { INSTRUMENTED_SYSTEM.alloc(layout) }
            } else {
                // SAFETY: the layout is forwarded unchanged to the system allocator.
                unsafe { System.alloc(layout) }
            }
        }

        unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
            if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
                // SAFETY: StatsAlloc only instruments the same System allocator
                // used by the other branch, so toggling cannot change ownership.
                unsafe { INSTRUMENTED_SYSTEM.dealloc(pointer, layout) }
            } else {
                // SAFETY: both branches ultimately allocate through System.
                unsafe { System.dealloc(pointer, layout) }
            }
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
                // SAFETY: the layout is forwarded unchanged to a global allocator.
                unsafe { INSTRUMENTED_SYSTEM.alloc_zeroed(layout) }
            } else {
                // SAFETY: the layout is forwarded unchanged to the system allocator.
                unsafe { System.alloc_zeroed(layout) }
            }
        }

        unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            if TRACK_ALLOCATIONS.load(Ordering::Relaxed) {
                // SAFETY: StatsAlloc wraps System, and all arguments are
                // forwarded unchanged regardless of the tracking state.
                unsafe { INSTRUMENTED_SYSTEM.realloc(pointer, layout, new_size) }
            } else {
                // SAFETY: both branches ultimately reallocate through System.
                unsafe { System.realloc(pointer, layout, new_size) }
            }
        }
    }

    const ALERT_TYPES: &str = r#"{"eventTypes":["Test Warning"]}"#;
    fn configuration(server: &MockServer, suffix: &str) -> Configuration {
        Configuration::new(None, Some(format!("{}{suffix}", server.uri())), None, None)
    }

    #[tokio::test]
    async fn public_endpoint_joins_prefixed_base_path_and_manages_headers() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/wiremock/prefix/alerts/types"))
            .and(header("Accept", "application/ld+json"))
            .and(header("User-Agent", "foundation-tests/1.0"))
            .and(header("X-Api-Key", "secret"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(ALERT_TYPES, "application/ld+json"),
            )
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
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(ALERT_TYPES, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let config = configuration(&server, "/prefix/");
        alerts::get_alert_types(&config).await.unwrap();
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
            .respond_with(ResponseTemplate::new(200).set_body_raw("{", "application/ld+json"))
            .mount(&server)
            .await;
        assert!(matches!(
            alerts::get_alert_types(&configuration(&server, "")).await,
            Err(Error::Json(_))
        ));

        let server = MockServer::start().await;
        Mock::given(path("/radio/KEC94/broadcast"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw("<speak>", "application/ssml+xml"),
            )
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
            assert_eq!(protocol.expected(), "application/ld+json");
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

        let payload = request(&configuration(&server, ""), "/document")
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

        let payload = request(&configuration(&server, ""), "/map")
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

    #[tokio::test]
    async fn contract_request_distinguishes_literal_paths_from_encoded_segments() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        request(&configuration(&server, "/prefix"), "/stations")
            .literal_path("observations")
            .path_segment(r"space slash/percent%question?#braces{}backslash\")
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/prefix/stations/observations/space%20slash%2Fpercent%25question%3F%23braces%7B%7Dbackslash%5C"
        );
    }

    #[tokio::test]
    async fn contract_request_keeps_empty_and_dot_segments_from_changing_route_structure() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(3)
            .mount(&server)
            .await;
        let config = configuration(&server, "");

        for segment in ["", ".", ".."] {
            request(&config, "/stations")
                .path_segment(segment)
                .literal_path("observations")
                .json::<serde_json::Value>(JsonMedia::GeoJson)
                .await
                .unwrap();
        }

        let paths = server
            .received_requests()
            .await
            .unwrap()
            .into_iter()
            .map(|request| request.url.path().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            [
                "/stations//observations",
                "/stations/%252E/observations",
                "/stations/%252E%252E/observations",
            ]
        );
    }

    #[tokio::test]
    async fn contract_request_preserves_optional_scalar_form_semantics() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        request(&configuration(&server, ""), "/test")
            .query_scalar::<&str>("omitted", None)
            .query_scalar("empty", Some(""))
            .query_scalar("value", Some("space,slash/value"))
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("empty=&value=space%2Cslash%2Fvalue")
        );
    }

    #[tokio::test]
    async fn contract_request_serializes_optional_csv_as_one_form_value() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        request(&configuration(&server, ""), "/test")
            .query_csv::<[&str; 0], &str>("omitted", None)
            .query_csv("empty", Some([] as [&str; 0]))
            .query_csv("event", Some(["Flood Watch", "Wind/Warning"]))
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("empty=&event=Flood+Watch%2CWind%2FWarning")
        );
        assert_eq!(
            requests[0]
                .url
                .query_pairs()
                .filter(|(key, _)| key == "event")
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn contract_json_media_atomically_sets_accept_and_validates_response() {
        for (media, content_type) in [
            (JsonMedia::GeoJson, "application/geo+json"),
            (JsonMedia::JsonLd, "application/ld+json"),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(header("Accept", content_type))
                .respond_with(ResponseTemplate::new(200).set_body_raw("{}", content_type))
                .expect(1)
                .mount(&server)
                .await;

            request(&configuration(&server, ""), "/media")
                .json::<serde_json::Value>(media)
                .await
                .unwrap();
        }
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    async fn contract_xml_media_atomically_sets_accept_and_validates_response() {
        #[derive(Debug, serde::Deserialize)]
        struct Document;

        let content_type = "application/vnd.wmo.iwxxm+xml";
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(header("Accept", content_type))
            .respond_with(ResponseTemplate::new(200).set_body_raw("<Document/>", content_type))
            .expect(1)
            .mount(&server)
            .await;

        request(&configuration(&server, ""), "/media")
            .xml::<Document>(XmlMedia::Iwxxm)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn contract_binary_media_atomically_sets_accept_and_validates_response() {
        for (media, accept, content_type) in [
            (BinaryMedia::Pdf, "application/pdf", "application/pdf"),
            (BinaryMedia::Image, "image/*", "image/png"),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(header("Accept", accept))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_bytes(b"media")
                        .insert_header("Content-Type", content_type),
                )
                .expect(1)
                .mount(&server)
                .await;

            request(&configuration(&server, ""), "/media")
                .binary(media)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn contract_rejects_success_media_incompatible_with_its_accept_choice() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/json"))
            .mount(&server)
            .await;

        let Error::Protocol(protocol) = request(&configuration(&server, ""), "/media")
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap_err()
        else {
            panic!("expected protocol error");
        };
        assert!(matches!(
            protocol.as_ref(),
            ProtocolError::IncompatibleContentType { .. }
        ));
        assert_eq!(protocol.expected(), "application/geo+json");
    }

    #[tokio::test]
    async fn contract_invalid_url_remains_a_transport_error() {
        let config = Configuration::new(None, Some("not a url".to_owned()), None, None);
        assert!(matches!(
            request(&config, "/media")
                .json::<serde_json::Value>(JsonMedia::GeoJson)
                .await,
            Err(Error::Transport(_))
        ));
    }

    #[tokio::test]
    async fn non_http_base_with_a_dynamic_path_remains_a_transport_error() {
        let config = Configuration::new(None, Some("mailto:test".to_owned()), None, None);
        assert!(matches!(
            request(&config, "/stations")
                .path_segment("KPHX")
                .json::<serde_json::Value>(JsonMedia::GeoJson)
                .await,
            Err(Error::Transport(_))
        ));
    }

    #[tokio::test]
    async fn contract_request_preserves_configured_identity_headers() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(header("User-Agent", "contract-tests/1.0"))
            .and(header("X-Api-Key", "secret"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;
        let config = Configuration::new(
            Some("contract-tests/1.0".to_owned()),
            Some(server.uri()),
            None,
            Some("secret".to_owned()),
        );

        request(&config, "/media")
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn contract_feature_flags_are_closed_and_comma_separated() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        request(&configuration(&server, ""), "/forecast")
            .feature_flags([
                FeatureFlag::ForecastTemperatureQuantitativeValue,
                FeatureFlag::ForecastWindSpeedQuantitativeValue,
            ])
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].headers["Feature-Flags"].to_str().unwrap(),
            "forecast_temperature_qv,forecast_wind_speed_qv"
        );
    }

    // Run with:
    // cargo test -p noaa_weather_client --all-features --release \
    //   apis::http::tests::request_construction_benchmark -- \
    //   --ignored --exact --nocapture --test-threads=1
    #[test]
    #[ignore = "manual allocation and timing acceptance benchmark"]
    fn request_construction_benchmark() {
        const ITERATIONS: usize = 5_000;
        const SAMPLES: usize = 21;
        const EVENTS: [&str; 6] = [
            "Flood Watch",
            "High Wind Warning",
            "Winter Storm Warning",
            "Dense Fog Advisory",
            "Red Flag Warning",
            "Tornado Warning",
        ];

        let config = Configuration::new(
            Some("request-benchmark/1.0".to_owned()),
            Some("https://api.weather.gov".to_owned()),
            None,
            Some("benchmark-key".to_owned()),
        );

        let legacy_scalar = || legacy_scalar_request(&config);
        let contract_scalar = || contract_scalar_request(&config);
        let legacy_csv = || legacy_csv_request(&config, &EVENTS);
        let contract_csv = || contract_csv_request(&config, &EVENTS);

        black_box(legacy_scalar());
        black_box(contract_scalar());
        black_box(legacy_csv());
        black_box(contract_csv());

        let legacy_csv_allocations = allocation_operations(ITERATIONS, legacy_csv);
        let contract_csv_allocations = allocation_operations(ITERATIONS, contract_csv);
        assert!(
            contract_csv_allocations < legacy_csv_allocations,
            "CSV-heavy construction must allocate less: legacy={legacy_csv_allocations}, contract={contract_csv_allocations}"
        );

        let legacy_scalar_allocations = allocation_operations(ITERATIONS, legacy_scalar);
        let contract_scalar_allocations = allocation_operations(ITERATIONS, contract_scalar);

        let mut scalar_medians = Vec::with_capacity(3);
        let mut csv_medians = Vec::with_capacity(3);
        for round in 0..3 {
            scalar_medians.push(paired_medians(
                ITERATIONS,
                SAMPLES,
                round % 2 != 0,
                legacy_scalar,
                contract_scalar,
            ));
            csv_medians.push(paired_medians(
                ITERATIONS,
                SAMPLES,
                round % 2 != 0,
                legacy_csv,
                contract_csv,
            ));
        }

        let scalar_regressions = repeated_regressions(&scalar_medians);
        let csv_regressions = repeated_regressions(&csv_medians);
        eprintln!(
            "request construction allocations/{ITERATIONS}: scalar legacy={legacy_scalar_allocations} contract={contract_scalar_allocations}; CSV legacy={legacy_csv_allocations} contract={contract_csv_allocations}"
        );
        eprintln!(
            "paired medians (legacy ns, contract ns, contract basis points): scalar={scalar_medians:?}; CSV={csv_medians:?}"
        );
        assert!(
            scalar_regressions < 2,
            "scalar-heavy contract construction repeatedly exceeded the 5% median limit: {scalar_medians:?}"
        );
        assert!(
            csv_regressions < 2,
            "CSV-heavy contract construction repeatedly exceeded the 5% median limit: {csv_medians:?}"
        );
    }

    fn allocation_operations(iterations: usize, build: impl Fn() -> reqwest::Request) -> usize {
        let region = Region::new(&INSTRUMENTED_SYSTEM);
        TRACK_ALLOCATIONS.store(true, Ordering::SeqCst);
        for _ in 0..iterations {
            black_box(build());
        }
        TRACK_ALLOCATIONS.store(false, Ordering::SeqCst);
        let stats = region.change();
        stats.allocations + stats.reallocations
    }

    fn paired_medians(
        iterations: usize,
        samples: usize,
        contract_first: bool,
        legacy: impl Fn() -> reqwest::Request,
        contract: impl Fn() -> reqwest::Request,
    ) -> (u128, u128, u128) {
        let mut legacy_nanos = Vec::with_capacity(samples);
        let mut contract_nanos = Vec::with_capacity(samples);
        let mut basis_points = Vec::with_capacity(samples);
        for sample in 0..samples {
            let contract_runs_first = contract_first ^ (sample % 2 != 0);
            let (legacy_elapsed, contract_elapsed) = if contract_runs_first {
                let contract_elapsed = elapsed_nanos(iterations, &contract);
                let legacy_elapsed = elapsed_nanos(iterations, &legacy);
                (legacy_elapsed, contract_elapsed)
            } else {
                let legacy_elapsed = elapsed_nanos(iterations, &legacy);
                let contract_elapsed = elapsed_nanos(iterations, &contract);
                (legacy_elapsed, contract_elapsed)
            };
            legacy_nanos.push(legacy_elapsed);
            contract_nanos.push(contract_elapsed);
            basis_points.push(contract_elapsed.saturating_mul(10_000) / legacy_elapsed);
        }
        legacy_nanos.sort_unstable();
        contract_nanos.sort_unstable();
        basis_points.sort_unstable();
        (
            legacy_nanos[samples / 2],
            contract_nanos[samples / 2],
            basis_points[samples / 2],
        )
    }

    fn elapsed_nanos(iterations: usize, build: impl Fn() -> reqwest::Request) -> u128 {
        let started = Instant::now();
        for _ in 0..iterations {
            black_box(build());
        }
        started.elapsed().as_nanos() / iterations as u128
    }

    fn repeated_regressions(samples: &[(u128, u128, u128)]) -> usize {
        samples
            .iter()
            .filter(|(_, _, contract_basis_points)| *contract_basis_points > 10_500)
            .count()
    }

    fn legacy_scalar_request(config: &Configuration) -> reqwest::Request {
        let path = format!("/radar/queues/{}", "rds");
        let url = format!(
            "{}/{}",
            config.base_path.trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        let mut builder = configured_get(config, url);
        for (name, value) in [
            ("limit", "50000"),
            ("arrived", "2026-08-30T12:34:56+00:00"),
            ("created", "2026-08-30T12:30:00+00:00"),
            ("published", "2026-08-30T12:35:00+00:00"),
            ("station", "KPHX"),
            ("type", "NEXRAD"),
            ("feed", "level2"),
        ] {
            let value = value.to_owned();
            builder = builder.query(&[(name, &value)]);
        }
        let resolution = 1_i32.to_owned();
        builder = builder.query(&[("resolution", &resolution)]);
        builder.build().expect("benchmark URL must be valid")
    }

    fn contract_scalar_request(config: &Configuration) -> reqwest::Request {
        request(config, "/radar/queues")
            .path_segment("rds")
            .query_scalar("limit", Some(50_000))
            .query_scalar("arrived", Some("2026-08-30T12:34:56+00:00"))
            .query_scalar("created", Some("2026-08-30T12:30:00+00:00"))
            .query_scalar("published", Some("2026-08-30T12:35:00+00:00"))
            .query_scalar("station", Some("KPHX"))
            .query_scalar("type", Some("NEXRAD"))
            .query_scalar("feed", Some("level2"))
            .query_scalar("resolution", Some(1_i32))
            .into_builder(JsonMedia::JsonLd.accept())
            .build()
            .expect("benchmark URL must be valid")
    }

    fn legacy_csv_request(config: &Configuration, values: &[&str]) -> reqwest::Request {
        let path = "/alerts/active".to_owned();
        let url = format!(
            "{}/{}",
            config.base_path.trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        let mut builder = configured_get(config, url);
        for name in ["area", "event", "message_type", "severity", "urgency"] {
            let value = values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",");
            builder = builder.query(&[(name, &value)]);
        }
        builder.build().expect("benchmark URL must be valid")
    }

    fn contract_csv_request(config: &Configuration, values: &[&str]) -> reqwest::Request {
        request(config, "/alerts/active")
            .query_csv("area", Some(values.iter().copied()))
            .query_csv("event", Some(values.iter().copied()))
            .query_csv("message_type", Some(values.iter().copied()))
            .query_csv("severity", Some(values.iter().copied()))
            .query_csv("urgency", Some(values.iter().copied()))
            .into_builder(JsonMedia::GeoJson.accept())
            .build()
            .expect("benchmark URL must be valid")
    }
}
