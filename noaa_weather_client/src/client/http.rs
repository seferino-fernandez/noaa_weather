//! The private request contract and response pipeline.
//!
//! Endpoint modules describe a request with [`request`] and finish it with a
//! media-typed terminal (`json`, `xml`, `binary`). Everything between — the
//! retry loop, manual redirects, the body size cap, and content-type
//! validation — is owned here so that each endpoint stays a thin mapping.

use std::{fmt, fmt::Write as _};

use bytes::{Bytes, BytesMut};
use jiff::Timestamp;
use mime::Mime;
use reqwest::{
    StatusCode,
    header::{CONTENT_TYPE, HeaderMap, HeaderValue, RETRY_AFTER},
};
use serde::de::DeserializeOwned;
use tracing::Instrument as _;
use url::Url;

use super::{
    Client, Inner,
    redirect::{self, HopError, HopHeaders},
    retry,
    secret::Secret,
};
use crate::apis::{BinaryPayload, Error, ProtocolError, ResponseContent};

pub(crate) use crate::time::RFC3339_SECONDS;

const CORRELATION_ID_HEADER: &str = "X-Correlation-Id";
const REQUEST_ID_HEADER: &str = "X-Request-Id";

/// Begins a GET request whose request-contract mechanics are owned here.
pub(crate) fn request<'client>(
    client: &'client Client,
    literal_path: &'static str,
) -> ContractRequest<'client> {
    let mut url = client.base_url().clone();
    url.path_segments_mut()
        .expect("validated base URLs support path segments")
        .pop_if_empty();
    let mut request = ContractRequest {
        client,
        url,
        value_buffer: None,
        feature_flags: None,
    };
    request.append_literal_path(literal_path);
    request
}

/// A private request contract assembled before a reqwest request is created.
pub(crate) struct ContractRequest<'client> {
    client: &'client Client,
    url: Url,
    value_buffer: Option<String>,
    feature_flags: Option<String>,
}

impl ContractRequest<'_> {
    /// Appends a trusted, compile-time path component without escaping it.
    pub(crate) fn literal_path(mut self, literal: &'static str) -> Self {
        self.append_literal_path(literal);
        self
    }

    /// Appends one untrusted dynamic path segment with special-URL escaping.
    pub(crate) fn path_segment(mut self, segment: impl fmt::Display) -> Self {
        let ContractRequest {
            url, value_buffer, ..
        } = &mut self;
        let segment = render_display(value_buffer, segment);
        // URL setters ignore dot segments. Supplying a percent-encoded dot
        // makes the setter encode `%` and retain one opaque segment.
        let segment = match segment {
            "." => "%2E",
            ".." => "%2E%2E",
            segment => segment,
        };
        url.path_segments_mut()
            .expect("HTTP URLs always support path segments")
            .push(segment);
        self
    }

    /// Appends every parameter described by one operation's query struct.
    pub(crate) fn query(mut self, params: &impl QueryParams) -> Self {
        params.append_to(&mut self);
        self
    }

    /// Appends one optional scalar in place; `None` appends nothing.
    pub(crate) fn scalar<T: fmt::Display>(&mut self, name: &'static str, value: Option<&T>) {
        if let Some(value) = value {
            self.push_scalar(name, value);
        }
    }

    /// Appends one optional instant as an RFC 3339 UTC timestamp with whole
    /// seconds, which is the only form NOAA accepts; `None` appends nothing.
    pub(crate) fn instant(&mut self, name: &'static str, value: Option<&Timestamp>) {
        if let Some(value) = value {
            self.push_scalar(name, &value.strftime(RFC3339_SECONDS));
        }
    }

    /// Appends a list as one CSV value in place; an empty list appends
    /// nothing.
    pub(crate) fn list<T: fmt::Display>(&mut self, name: &'static str, values: &[T]) {
        if !values.is_empty() {
            self.push_csv(name, values);
        }
    }

    fn push_scalar<T: fmt::Display>(&mut self, name: &'static str, value: &T) {
        let ContractRequest {
            url, value_buffer, ..
        } = self;
        let value = render_display(value_buffer, value);
        url.query_pairs_mut().append_pair(name, value);
    }

    fn push_csv<I, T>(&mut self, name: &'static str, values: I)
    where
        I: IntoIterator<Item = T>,
        T: fmt::Display,
    {
        let ContractRequest {
            url, value_buffer, ..
        } = self;
        let value_buffer = value_buffer.get_or_insert_with(|| String::with_capacity(256));
        value_buffer.clear();
        for (index, value) in values.into_iter().enumerate() {
            if index != 0 {
                value_buffer.push(',');
            }
            write!(value_buffer, "{value}").expect("writing to String cannot fail");
        }
        url.query_pairs_mut().append_pair(name, value_buffer);
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
    pub(crate) async fn xml<T: DeserializeOwned>(self, media: XmlMedia) -> Result<T, Error> {
        let response = self.send(media.accept()).await?;
        ensure_content_type(&response, media.expected(), |mime| media.matches(mime))?;
        quick_xml::de::from_reader(response.bytes.as_ref()).map_err(Error::from)
    }

    /// Requests and validates one XML media family without decoding it.
    pub(crate) async fn xml_bytes(self, media: XmlMedia) -> Result<Bytes, Error> {
        let response = self.send(media.accept()).await?;
        ensure_content_type(&response, media.expected(), |mime| media.matches(mime))?;
        Ok(response.bytes)
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

    fn append_literal_path(&mut self, literal: &'static str) {
        let literal = literal.trim_matches('/');
        if !literal.is_empty() {
            self.url
                .path_segments_mut()
                .expect("HTTP URLs always support path segments")
                .extend(literal.split('/'));
        }
    }

    async fn send(self, accept: &'static str) -> Result<ReceivedResponse, Error> {
        let feature_flags = self.feature_flags.as_deref().map(|flags| {
            HeaderValue::from_str(flags).expect("feature flags are a closed ASCII set")
        });
        execute(self.client.inner(), self.url, accept, feature_flags).await
    }
}

/// The optional parameters of one NOAA operation, encoded onto a request.
///
/// Each handle module implements this once per `*Query` struct so that the
/// wire names (`message_type`, `reportingHost`, ...) and the CSV and RFC 3339
/// encodings live next to the struct, while serde derives on the same struct
/// stay free for JSON and MCP schemas.
pub(crate) trait QueryParams {
    fn append_to(&self, request: &mut ContractRequest<'_>);
}

fn render_display<T: fmt::Display>(buffer: &mut Option<String>, value: T) -> &str {
    let buffer = buffer.get_or_insert_with(|| String::with_capacity(256));
    buffer.clear();
    write!(buffer, "{value}").expect("writing to String cannot fail");
    buffer
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

#[derive(Clone, Copy)]
pub(crate) enum XmlMedia {
    Iwxxm,
    Ssml,
}

impl XmlMedia {
    const fn accept(self) -> &'static str {
        match self {
            Self::Iwxxm => "application/vnd.wmo.iwxxm+xml",
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

/// A complete response with a success status.
struct ReceivedResponse {
    bytes: Bytes,
    content_type: Option<HeaderValue>,
    url: Url,
}

/// One attempt's fully read response, before status classification.
struct RawResponse {
    status: StatusCode,
    url: Url,
    bytes: Bytes,
    content_type: Option<HeaderValue>,
    retry_after: Option<std::time::Duration>,
    correlation_id: Option<Box<str>>,
    request_id: Option<Box<str>>,
}

/// Why one attempt ended without a complete response.
enum AttemptError {
    /// A transport failure; retried when transient.
    Transport(reqwest::Error),
    /// A redirect or size-cap violation; never retried.
    Protocol(Box<ProtocolError>),
}

impl From<HopError> for AttemptError {
    fn from(error: HopError) -> Self {
        match error {
            HopError::Transport(source) => Self::Transport(source),
            HopError::Redirect(error) => Self::Protocol(error),
        }
    }
}

/// Runs the retry loop around single attempts until success or a final error.
async fn execute(
    inner: &Inner,
    url: Url,
    accept: &'static str,
    feature_flags: Option<HeaderValue>,
) -> Result<ReceivedResponse, Error> {
    let headers = HopHeaders {
        accept,
        feature_flags: feature_flags.as_ref(),
        api_key: inner.api_key.as_ref().map(Secret::expose),
    };
    let mut attempt: u8 = 1;
    loop {
        let span = tracing::debug_span!(
            "noaa_weather.request",
            method = "GET",
            url = %url,
            attempt
        );
        let outcome = attempt_once(inner, &url, headers).instrument(span).await;
        let retry = match &outcome {
            Ok(raw) if raw.status.is_success() => None,
            Ok(raw) if retry::retryable_status(raw.status) => {
                Some((RetryReason::Status(raw.status), raw.retry_after))
            }
            Err(AttemptError::Transport(source)) if retry::retryable_transport(source) => {
                Some((RetryReason::Transport, None))
            }
            Ok(_) | Err(_) => None,
        };

        if let Some((reason, retry_after)) = retry
            && let Some(delay) = inner.retry.delay_before_retry(attempt, retry_after)
        {
            tracing::warn!(
                attempt,
                delay_ms = u64::try_from(delay.as_millis()).unwrap_or(u64::MAX),
                reason = %reason,
                "retrying NOAA request"
            );
            tokio::time::sleep(delay).await;
            attempt += 1;
            continue;
        }

        return match outcome {
            Ok(raw) if raw.status.is_success() => Ok(ReceivedResponse {
                bytes: raw.bytes,
                content_type: raw.content_type,
                url: raw.url,
            }),
            Ok(raw) => Err(response_error(raw, attempt)),
            Err(AttemptError::Transport(source)) => Err(Error::Transport {
                source,
                attempts: attempt,
            }),
            Err(AttemptError::Protocol(error)) => Err(Error::Protocol(error)),
        };
    }
}

enum RetryReason {
    Status(StatusCode),
    Transport,
}

impl fmt::Display for RetryReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(status) => write!(formatter, "HTTP {status}"),
            Self::Transport => formatter.write_str("transport failure"),
        }
    }
}

/// Follows redirects, records response metadata, and reads the body under
/// the size cap.
async fn attempt_once(
    inner: &Inner,
    url: &Url,
    headers: HopHeaders<'_>,
) -> Result<RawResponse, AttemptError> {
    let response = redirect::follow(&inner.http, url.clone(), headers).await?;
    let status = response.status();
    let final_url = response.url().clone();
    tracing::debug!(status = status.as_u16(), url = %final_url, "received response");
    let response_headers = response.headers();
    let content_type = response_headers.get(CONTENT_TYPE).cloned();
    let retry_after = response_headers
        .get(RETRY_AFTER)
        .and_then(|value| retry::parse_retry_after(value, Timestamp::now()));
    let correlation_id = header_text(response_headers, CORRELATION_ID_HEADER);
    let request_id = header_text(response_headers, REQUEST_ID_HEADER);
    let bytes = read_body(response, inner.max_response_bytes, &final_url).await?;
    Ok(RawResponse {
        status,
        url: final_url,
        bytes,
        content_type,
        retry_after,
        correlation_id,
        request_id,
    })
}

fn header_text(headers: &HeaderMap, name: &'static str) -> Option<Box<str>> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(Box::from)
}

/// Buffers the body, refusing to exceed `limit` bytes after decompression.
async fn read_body(
    mut response: reqwest::Response,
    limit: usize,
    url: &Url,
) -> Result<Bytes, AttemptError> {
    let too_large = || {
        AttemptError::Protocol(Box::new(ProtocolError::ResponseTooLarge {
            limit,
            url: url.clone(),
        }))
    };
    let declared = response.content_length();
    if declared.is_some_and(|length| length > u64::try_from(limit).unwrap_or(u64::MAX)) {
        return Err(too_large());
    }

    let mut first: Option<Bytes> = None;
    let mut buffer: Option<BytesMut> = None;
    while let Some(chunk) = response.chunk().await.map_err(AttemptError::Transport)? {
        let received = first.as_ref().map_or(0, Bytes::len)
            + buffer.as_ref().map_or(0, BytesMut::len)
            + chunk.len();
        if received > limit {
            return Err(too_large());
        }
        match (&mut first, &mut buffer) {
            (None, None) => first = Some(chunk),
            (Some(_), None) => {
                let capacity = declared
                    .and_then(|length| usize::try_from(length).ok())
                    .unwrap_or(received)
                    .min(limit);
                let mut combined = BytesMut::with_capacity(capacity);
                combined.extend_from_slice(first.take().as_deref().unwrap_or_default());
                combined.extend_from_slice(&chunk);
                buffer = Some(combined);
            }
            (_, Some(combined)) => combined.extend_from_slice(&chunk),
        }
    }
    Ok(match (first, buffer) {
        (_, Some(combined)) => combined.freeze(),
        (Some(single), None) => single,
        (None, None) => Bytes::new(),
    })
}

fn response_error(raw: RawResponse, attempts: u8) -> Error {
    let parsed_content_type = raw
        .content_type
        .as_ref()
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok());
    let problem_detail = serde_json::from_slice(&raw.bytes).ok();
    Error::Response(Box::new(ResponseContent {
        bytes: raw.bytes,
        status: raw.status,
        url: raw.url,
        problem_detail,
        content_type: parsed_content_type,
        retry_after: raw.retry_after,
        correlation_id: raw.correlation_id,
        request_id: raw.request_id,
        attempts,
    }))
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
pub(crate) use tests::measure_allocations;

#[cfg(test)]
mod tests {
    use std::{
        alloc::{GlobalAlloc, Layout, System},
        cell::Cell,
        hint::black_box,
        str::FromStr as _,
        time::Instant,
    };

    use reqwest::header::HeaderValue;
    use stats_alloc::{INSTRUMENTED_SYSTEM, Region, Stats};
    use url::Url;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::XmlMedia;
    use super::{BinaryMedia, FeatureFlag, JsonMedia, request};
    use crate::{
        Client, Error, ProtocolError,
        client::{
            redirect::{HopHeaders, hop_request},
            test_support::{USER_AGENT, builder_for, client_for, client_with_base},
        },
    };

    thread_local! {
        static TRACK_ALLOCATIONS: Cell<bool> = const { Cell::new(false) };
    }

    struct BenchmarkAllocator;

    #[global_allocator]
    static GLOBAL: BenchmarkAllocator = BenchmarkAllocator;

    // Allocation accounting is enabled only around the deterministic
    // allocation phase. Timing uses the system allocator directly.
    unsafe impl GlobalAlloc for BenchmarkAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            if tracking_allocations() {
                // SAFETY: the layout is forwarded unchanged to a global allocator.
                unsafe { INSTRUMENTED_SYSTEM.alloc(layout) }
            } else {
                // SAFETY: the layout is forwarded unchanged to the system allocator.
                unsafe { System.alloc(layout) }
            }
        }

        unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
            if tracking_allocations() {
                // SAFETY: StatsAlloc only instruments the same System allocator
                // used by the other branch, so toggling cannot change ownership.
                unsafe { INSTRUMENTED_SYSTEM.dealloc(pointer, layout) }
            } else {
                // SAFETY: both branches ultimately allocate through System.
                unsafe { System.dealloc(pointer, layout) }
            }
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            if tracking_allocations() {
                // SAFETY: the layout is forwarded unchanged to a global allocator.
                unsafe { INSTRUMENTED_SYSTEM.alloc_zeroed(layout) }
            } else {
                // SAFETY: the layout is forwarded unchanged to the system allocator.
                unsafe { System.alloc_zeroed(layout) }
            }
        }

        unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            if tracking_allocations() {
                // SAFETY: StatsAlloc wraps System, and all arguments are
                // forwarded unchanged regardless of the tracking state.
                unsafe { INSTRUMENTED_SYSTEM.realloc(pointer, layout, new_size) }
            } else {
                // SAFETY: both branches ultimately reallocate through System.
                unsafe { System.realloc(pointer, layout, new_size) }
            }
        }
    }

    fn tracking_allocations() -> bool {
        TRACK_ALLOCATIONS.try_with(Cell::get).unwrap_or(false)
    }

    struct AllocationTracking;

    impl Drop for AllocationTracking {
        fn drop(&mut self) {
            TRACK_ALLOCATIONS.with(|tracking| tracking.set(false));
        }
    }

    pub(crate) fn measure_allocations<T>(operation: impl FnOnce() -> T) -> (T, Stats) {
        let region = Region::new(&INSTRUMENTED_SYSTEM);
        TRACK_ALLOCATIONS.with(|tracking| {
            assert!(!tracking.replace(true), "nested allocation measurement");
        });
        let guard = AllocationTracking;
        let value = operation();
        drop(guard);
        (value, region.change())
    }

    const ALERT_TYPES: &str = r#"{"eventTypes":["Test Warning"]}"#;

    fn client(server: &MockServer, suffix: &str) -> Client {
        client_with_base(format!("{}{suffix}", server.uri()))
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

        let client = Client::builder("foundation-tests/1.0")
            .base_url(format!("{}/wiremock/prefix", server.uri()))
            .api_key("secret")
            .build()
            .unwrap();
        let response = client.alerts().types().await.unwrap();
        assert_eq!(response.event_types, ["Test Warning"]);
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

        client(&server, "/prefix/").alerts().types().await.unwrap();
    }

    #[tokio::test]
    async fn response_error_keeps_typed_problem_raw_body_and_tracing_ids() {
        let server = MockServer::start().await;
        let problem = r#"{"type":"urn:test","title":"Bad point","status":400,"detail":"outside forecast area","instance":"urn:instance","correlationId":"abc-123"}"#;
        Mock::given(path("/alerts/types"))
            .respond_with(
                ResponseTemplate::new(400)
                    .set_body_string(problem)
                    .insert_header("Content-Type", "text/plain")
                    .insert_header("X-Correlation-Id", "abc-123")
                    .insert_header("X-Request-Id", "req-9"),
            )
            .mount(&server)
            .await;

        let error = client_for(&server).alerts().types().await.unwrap_err();
        assert_eq!(error.status().map(|status| status.as_u16()), Some(400));
        assert_eq!(error.attempts(), 1);
        assert!(!error.is_retryable());
        assert_eq!(error.problem().unwrap().title, "Bad point");
        let Error::Response(response) = error else {
            panic!("expected response error");
        };
        assert_eq!(response.status(), 400);
        assert_eq!(response.as_bytes(), problem.as_bytes());
        assert_eq!(response.text(), problem);
        assert_eq!(response.content_type().unwrap(), &mime::TEXT_PLAIN);
        assert_eq!(response.problem_detail().unwrap().title, "Bad point");
        assert_eq!(response.url().path(), "/alerts/types");
        assert_eq!(response.correlation_id(), Some("abc-123"));
        assert_eq!(response.request_id(), Some("req-9"));
        assert_eq!(response.retry_after(), None);
    }

    #[tokio::test]
    async fn response_error_keeps_an_unrecognized_binary_body() {
        let server = MockServer::start().await;
        let body = b"not-json\xff";
        Mock::given(path("/alerts/types"))
            .respond_with(ResponseTemplate::new(503).set_body_bytes(body))
            .mount(&server)
            .await;

        let error = client_for(&server).alerts().types().await.unwrap_err();
        assert!(error.is_retryable());
        assert_eq!(error.attempts(), 1);
        let Error::Response(response) = error else {
            panic!("expected response error");
        };
        assert_eq!(response.as_bytes(), body);
        assert!(response.problem_detail().is_none());
        assert!(response.content_type().is_none());
        assert!(response.correlation_id().is_none());
        assert!(response.text().contains('\u{fffd}'));
    }

    #[tokio::test]
    async fn malformed_success_documents_keep_decode_sources() {
        let server = MockServer::start().await;
        Mock::given(path("/alerts/types"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{", "application/ld+json"))
            .mount(&server)
            .await;
        assert!(matches!(
            client_for(&server).alerts().types().await,
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
            client_for(&server)
                .radio()
                .broadcast(&"KEC94".parse().unwrap())
                .await,
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

            let Error::Protocol(protocol) = client_for(&server).alerts().types().await.unwrap_err()
            else {
                panic!("expected protocol error");
            };
            match (expected_variant, protocol.as_ref()) {
                ("missing", ProtocolError::MissingContentType { .. })
                | ("malformed", ProtocolError::MalformedContentType { .. })
                | ("incompatible", ProtocolError::IncompatibleContentType { .. }) => {}
                _ => panic!("unexpected protocol error: {protocol:?}"),
            }
            assert_eq!(protocol.expected(), Some("application/ld+json"));
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

        let payload = request(&client_for(&server), "/document")
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

        let payload = request(&client_for(&server), "/map")
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
    async fn redirect_error_url_is_reported_after_all_hops_and_never_retried() {
        let server = MockServer::start().await;
        Mock::given(path("/alerts/types"))
            .respond_with(ResponseTemplate::new(302).insert_header("Location", "/moved"))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(path("/moved"))
            .respond_with(ResponseTemplate::new(307).set_body_string("no location"))
            .expect(1)
            .mount(&server)
            .await;

        let client = builder_for(&server)
            .retry(crate::RetryPolicy::default().base_delay(std::time::Duration::from_millis(1)))
            .build()
            .unwrap();
        let error = client.alerts().types().await.unwrap_err();
        assert!(!error.is_retryable());
        assert_eq!(error.attempts(), 1);
        let Error::Protocol(protocol) = error else {
            panic!("expected protocol error");
        };
        assert!(matches!(
            protocol.as_ref(),
            ProtocolError::Redirect {
                reason: crate::apis::RedirectReason::MissingLocation,
                ..
            }
        ));
        assert_eq!(protocol.url().path(), "/moved");
        assert_eq!(protocol.expected(), None);
        assert_eq!(protocol.actual(), None);
    }

    #[tokio::test]
    async fn error_bodies_are_subject_to_the_size_cap() {
        let server = MockServer::start().await;
        Mock::given(path("/alerts/types"))
            .respond_with(ResponseTemplate::new(500).set_body_bytes(vec![b'x'; 64]))
            .expect(1)
            .mount(&server)
            .await;

        let client = builder_for(&server).max_response_bytes(16).build().unwrap();
        let error = client.alerts().types().await.unwrap_err();
        assert!(!error.is_retryable());
        assert!(matches!(
            error,
            Error::Protocol(ref protocol)
                if matches!(protocol.as_ref(), ProtocolError::ResponseTooLarge { limit: 16, .. })
        ));
    }

    #[tokio::test]
    async fn contract_request_distinguishes_literal_paths_from_encoded_segments() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        request(&client(&server, "/prefix"), "/stations")
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
        let client = client_for(&server);

        for segment in ["", ".", ".."] {
            request(&client, "/stations")
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

        let client = client_for(&server);
        let mut contract = request(&client, "/test");
        contract.scalar::<&str>("omitted", None);
        contract.scalar("empty", Some(&""));
        contract.scalar("value", Some(&"space,slash/value"));
        contract
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
    async fn contract_request_serializes_a_list_as_one_form_value_or_nothing() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw("{}", "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        let mut contract = request(&client, "/test");
        contract.list::<&str>("empty", &[]);
        contract.list("event", &["Flood Watch", "Wind/Warning"]);
        contract
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("event=Flood+Watch%2CWind%2FWarning")
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

            request(&client_for(&server), "/media")
                .json::<serde_json::Value>(media)
                .await
                .unwrap();
        }
    }

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

        request(&client_for(&server), "/media")
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

            request(&client_for(&server), "/media")
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

        let Error::Protocol(protocol) = request(&client_for(&server), "/media")
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
        assert_eq!(protocol.expected(), Some("application/geo+json"));
    }

    #[tokio::test]
    async fn connection_refused_is_a_transport_error_with_attempt_count() {
        // Bind and immediately drop a listener so the port is closed.
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);

        let client = client_with_base(format!("http://{address}"));
        let error = request(&client, "/media")
            .json::<serde_json::Value>(JsonMedia::GeoJson)
            .await
            .unwrap_err();
        assert!(error.is_retryable(), "{error}");
        assert_eq!(error.attempts(), 1);
        assert!(matches!(error, Error::Transport { attempts: 1, .. }));
        assert!(error.status().is_none());
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
        let client = Client::builder("contract-tests/1.0")
            .base_url(server.uri())
            .api_key("secret")
            .build()
            .unwrap();

        request(&client, "/media")
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

        request(&client_for(&server), "/forecast")
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
        assert_eq!(
            requests[0].headers["User-Agent"].to_str().unwrap(),
            USER_AGENT
        );
    }

    // Run with:
    // cargo test -p noaa_weather_client --all-features --release \
    //   client::http::tests::request_construction_benchmark -- \
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

        let client = Client::builder("request-benchmark/1.0")
            .api_key("benchmark-key")
            .build()
            .unwrap();

        let legacy_scalar = || legacy_scalar_request(&client);
        let contract_scalar = || contract_scalar_request(&client);
        let legacy_csv = || legacy_csv_request(&client, &EVENTS);
        let contract_csv = || contract_csv_request(&client, &EVENTS);

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
        let (_, stats) = measure_allocations(|| {
            for _ in 0..iterations {
                black_box(build());
            }
        });
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

    /// Builds the reqwest request for the first hop of a contract, which is
    /// what the pipeline sends.
    fn first_hop(
        client: &Client,
        contract: super::ContractRequest<'_>,
        accept: &'static str,
    ) -> reqwest::Request {
        let feature_flags = contract
            .feature_flags
            .as_deref()
            .map(|flags| HeaderValue::from_str(flags).unwrap());
        let inner = client.inner();
        let headers = HopHeaders {
            accept,
            feature_flags: feature_flags.as_ref(),
            api_key: inner.api_key.as_ref().map(super::Secret::expose),
        };
        hop_request(&inner.http, contract.url.clone(), &headers, &contract.url)
            .build()
            .expect("benchmark URL must be valid")
    }

    fn legacy_get(client: &Client, url: String) -> reqwest::RequestBuilder {
        let inner = client.inner();
        let mut builder = inner
            .http
            .get(url)
            .header(reqwest::header::ACCEPT, "application/ld+json");
        if let Some(api_key) = &inner.api_key {
            builder = builder.header("X-Api-Key", api_key.expose());
        }
        builder
    }

    fn legacy_scalar_request(client: &Client) -> reqwest::Request {
        let path = format!("/radar/queues/{}", "rds");
        let url = format!(
            "{}/{}",
            client.base_url().as_str().trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        let mut builder = legacy_get(client, url);
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

    fn contract_scalar_request(client: &Client) -> reqwest::Request {
        let mut contract = request(client, "/radar/queues").path_segment("rds");
        contract.scalar("limit", Some(&50_000));
        contract.scalar("arrived", Some(&"2026-08-30T12:34:56+00:00"));
        contract.scalar("created", Some(&"2026-08-30T12:30:00+00:00"));
        contract.scalar("published", Some(&"2026-08-30T12:35:00+00:00"));
        contract.scalar("station", Some(&"KPHX"));
        contract.scalar("type", Some(&"NEXRAD"));
        contract.scalar("feed", Some(&"level2"));
        contract.scalar("resolution", Some(&1_i32));
        first_hop(client, contract, JsonMedia::JsonLd.accept())
    }

    fn legacy_csv_request(client: &Client, values: &[&str]) -> reqwest::Request {
        let path = "/alerts/active".to_owned();
        let url = format!(
            "{}/{}",
            client.base_url().as_str().trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        let mut builder = legacy_get(client, url);
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

    fn contract_csv_request(client: &Client, values: &[&str]) -> reqwest::Request {
        let mut contract = request(client, "/alerts/active");
        for name in ["area", "event", "message_type", "severity", "urgency"] {
            contract.list(name, values);
        }
        first_hop(client, contract, JsonMedia::GeoJson.accept())
    }

    #[test]
    fn request_builder_joins_base_paths_without_touching_dynamic_segments() {
        let client = client_with_base("http://localhost:1/prefix/");
        let contract = request(&client, "/points/").path_segment("39.7,-97.1");
        assert_eq!(
            contract.url,
            Url::parse("http://localhost:1/prefix/points/39.7,-97.1").unwrap()
        );
    }
}
