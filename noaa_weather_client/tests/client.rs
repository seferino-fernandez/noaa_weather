//! Public-interface tests for `Client` request policy against wiremock.

use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use noaa_weather_client::{
    BuildError, Client, ClientBuilder, Error, ProtocolError, RedirectReason, RetryPolicy,
    apis::{alerts, points},
};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

const USER_AGENT: &str = "noaa-weather-integration/1.0 (tests@example.com)";
const POINT: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
const POINT_PATH: &str = "/points/39.7456,-97.0892";

fn builder_for(server: &MockServer) -> ClientBuilder {
    Client::builder(USER_AGENT)
        .base_url(server.uri())
        .retry(RetryPolicy::none())
}

fn fast_retries() -> RetryPolicy {
    RetryPolicy::default()
        .base_delay(Duration::from_millis(10))
        .max_delay(Duration::from_millis(50))
}

fn point_response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_raw(POINT, "application/geo+json")
}

async fn get_point(client: &Client) -> Result<(), Error> {
    points::get_point(client, 39.7456, -97.0892).await.map(drop)
}

#[tokio::test]
async fn identity_and_media_headers_are_sent_on_every_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(POINT_PATH))
        .and(header("User-Agent", USER_AGENT))
        .and(header("Accept", "application/geo+json"))
        .and(header("X-Api-Key", "integration-key"))
        .respond_with(point_response())
        .expect(1)
        .mount(&server)
        .await;

    let client = builder_for(&server)
        .api_key("integration-key")
        .build()
        .unwrap();
    get_point(&client).await.unwrap();
    assert_eq!(client.user_agent(), USER_AGENT);
    assert_eq!(client.base_url().as_str(), format!("{}/", server.uri()));
}

#[test]
fn debug_output_never_contains_the_api_key() {
    let builder = Client::builder(USER_AGENT).api_key("hunter2-secret");
    let builder_debug = format!("{builder:?}");
    assert!(!builder_debug.contains("hunter2-secret"), "{builder_debug}");
    assert!(builder_debug.contains("[redacted]"), "{builder_debug}");

    let client = builder.build().unwrap();
    let client_debug = format!("{client:?}");
    assert!(!client_debug.contains("hunter2-secret"), "{client_debug}");
    assert!(client_debug.contains("[redacted]"), "{client_debug}");
    assert!(client_debug.contains(USER_AGENT), "{client_debug}");
}

#[tokio::test]
async fn rate_limit_with_retry_after_is_retried_after_the_requested_delay() {
    let server = MockServer::start().await;
    Mock::given(path(POINT_PATH))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "1"))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(path(POINT_PATH))
        .respond_with(point_response())
        .mount(&server)
        .await;

    let client = builder_for(&server)
        .retry(fast_retries().max_delay(Duration::from_secs(2)))
        .build()
        .unwrap();
    let started = Instant::now();
    get_point(&client).await.unwrap();
    assert!(
        started.elapsed() >= Duration::from_millis(900),
        "Retry-After was not honored: {:?}",
        started.elapsed()
    );
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[tokio::test]
async fn persistent_server_errors_exhaust_the_policy_and_report_attempts() {
    let server = MockServer::start().await;
    Mock::given(path(POINT_PATH))
        .respond_with(ResponseTemplate::new(503).set_body_string("unavailable"))
        .expect(3)
        .mount(&server)
        .await;

    let client = builder_for(&server).retry(fast_retries()).build().unwrap();
    let error = get_point(&client).await.unwrap_err();
    assert_eq!(error.attempts(), 3);
    assert!(error.is_retryable());
    assert_eq!(error.status().map(|status| status.as_u16()), Some(503));
    assert!(!error.is_rate_limited());
    assert!(!error.is_not_found());
    let Error::Response(response) = error else {
        panic!("expected response error");
    };
    assert_eq!(response.attempts(), 3);
    assert_eq!(response.text(), "unavailable");
}

#[tokio::test]
async fn a_long_retry_after_stops_retrying_immediately() {
    let server = MockServer::start().await;
    Mock::given(path(POINT_PATH))
        .respond_with(ResponseTemplate::new(503).insert_header("Retry-After", "3600"))
        .expect(1)
        .mount(&server)
        .await;

    let client = builder_for(&server).retry(fast_retries()).build().unwrap();
    let started = Instant::now();
    let error = get_point(&client).await.unwrap_err();
    assert!(started.elapsed() < Duration::from_secs(2));
    assert_eq!(error.attempts(), 1);
    assert_eq!(error.retry_after(), Some(Duration::from_secs(3600)));
    assert!(error.is_retryable());
}

/// Serves one raw HTTP/1.1 response to the first connection and reports how
/// many requests arrived.
async fn raw_server(response: Vec<u8>) -> (String, Arc<AtomicUsize>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let requests = Arc::new(AtomicUsize::new(0));
    let seen = Arc::clone(&requests);
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut head = Vec::new();
        let mut buffer = [0_u8; 1024];
        while !head.windows(4).any(|window| window == b"\r\n\r\n") {
            let read = socket.read(&mut buffer).await.unwrap();
            if read == 0 {
                return;
            }
            head.extend_from_slice(&buffer[..read]);
        }
        seen.fetch_add(1, Ordering::SeqCst);
        let _ = socket.write_all(&response).await;
        let _ = socket.shutdown().await;
    });
    (format!("http://{address}"), requests)
}

/// Serves a chunked response without a `Content-Length`.
async fn chunked_server(chunks: usize, chunk_size: usize) -> String {
    let mut response = b"HTTP/1.1 200 OK\r\nContent-Type: application/geo+json\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n".to_vec();
    for _ in 0..chunks {
        response.extend_from_slice(format!("{chunk_size:x}\r\n").as_bytes());
        response.extend(std::iter::repeat_n(b'{', chunk_size));
        response.extend_from_slice(b"\r\n");
    }
    response.extend_from_slice(b"0\r\n\r\n");
    raw_server(response).await.0
}

#[tokio::test]
async fn declared_content_length_over_the_cap_is_refused_before_reading() {
    // The declared length exceeds the cap but the bytes actually sent do
    // not, so only the Content-Length precheck can produce ResponseTooLarge.
    // Reading the body instead would hit a truncated-body transport error.
    let (base_url, requests) = raw_server(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/geo+json\r\nContent-Length: 1000\r\nConnection: close\r\n\r\n{\"a\":1}\n\n\n".to_vec(),
    )
    .await;
    let client = Client::builder(USER_AGENT)
        .base_url(&base_url)
        .retry(fast_retries())
        .max_response_bytes(100)
        .build()
        .unwrap();

    let error = get_point(&client).await.unwrap_err();
    assert_eq!(requests.load(Ordering::SeqCst), 1);
    assert!(!error.is_retryable(), "{error}");
    assert_eq!(error.attempts(), 1);
    let Error::Protocol(protocol) = error else {
        panic!("expected protocol error, got: {error}");
    };
    assert!(
        matches!(
            protocol.as_ref(),
            ProtocolError::ResponseTooLarge { limit: 100, .. }
        ),
        "{protocol:?}"
    );
    assert_eq!(protocol.url().path(), POINT_PATH);
}

#[tokio::test]
async fn streamed_body_over_the_cap_is_refused_while_reading() {
    let base_url = chunked_server(20, 50).await;
    let client = Client::builder(USER_AGENT)
        .base_url(&base_url)
        .retry(RetryPolicy::none())
        .max_response_bytes(120)
        .build()
        .unwrap();

    let error = get_point(&client).await.unwrap_err();
    assert!(!error.is_retryable());
    let Error::Protocol(protocol) = error else {
        panic!("expected protocol error, got another kind");
    };
    assert!(
        matches!(
            protocol.as_ref(),
            ProtocolError::ResponseTooLarge { limit: 120, .. }
        ),
        "{protocol:?}"
    );
}

#[tokio::test]
async fn streamed_body_under_the_cap_is_accepted() {
    let base_url = chunked_server(2, 1).await;
    let client = Client::builder(USER_AGENT)
        .base_url(&base_url)
        .retry(RetryPolicy::none())
        .max_response_bytes(120)
        .build()
        .unwrap();

    // Two `{` bytes are not a valid GeoJSON document; the body was read.
    assert!(matches!(get_point(&client).await, Err(Error::Json(_))));
}

#[tokio::test]
async fn cross_origin_redirect_drops_the_api_key_but_keeps_media_headers() {
    let origin = MockServer::start().await;
    let elsewhere = MockServer::start().await;
    Mock::given(path(POINT_PATH))
        .respond_with(
            ResponseTemplate::new(302)
                .insert_header("Location", format!("{}{POINT_PATH}", elsewhere.uri())),
        )
        .expect(1)
        .mount(&origin)
        .await;
    Mock::given(path(POINT_PATH))
        .and(header("Accept", "application/geo+json"))
        .and(header("User-Agent", USER_AGENT))
        .respond_with(point_response())
        .expect(1)
        .mount(&elsewhere)
        .await;

    let client = builder_for(&origin).api_key("origin-only").build().unwrap();
    get_point(&client).await.unwrap();

    let first = origin.received_requests().await.unwrap();
    assert_eq!(first[0].headers["X-Api-Key"], "origin-only");
    let second = elsewhere.received_requests().await.unwrap();
    assert_eq!(second.len(), 1);
    assert!(
        !second[0].headers.contains_key("X-Api-Key"),
        "API key leaked across origins: {:?}",
        second[0].headers
    );
}

#[tokio::test]
async fn same_origin_redirects_keep_the_api_key_and_report_the_final_url() {
    let server = MockServer::start().await;
    Mock::given(path(POINT_PATH))
        .respond_with(ResponseTemplate::new(301).insert_header("Location", "/moved"))
        .mount(&server)
        .await;
    Mock::given(path("/moved"))
        .and(header("X-Api-Key", "origin-only"))
        .respond_with(ResponseTemplate::new(404).set_body_raw(
            r#"{"type":"urn:noaa","title":"Not Found","status":404,"detail":"gone","instance":"urn:i","correlationId":"c"}"#,
            "application/problem+json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = builder_for(&server).api_key("origin-only").build().unwrap();
    let error = get_point(&client).await.unwrap_err();
    assert!(error.is_not_found());
    assert_eq!(error.problem().unwrap().title, "Not Found");
    let Error::Response(response) = error else {
        panic!("expected response error");
    };
    assert_eq!(response.url().path(), "/moved");
}

#[tokio::test]
async fn redirect_chains_stop_after_five_hops_without_retrying() {
    let server = MockServer::start().await;
    Mock::given(path(POINT_PATH))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", "/hop1"))
        .mount(&server)
        .await;
    for hop in 1..=6 {
        Mock::given(path(format!("/hop{hop}")))
            .respond_with(
                ResponseTemplate::new(302).insert_header("Location", format!("/hop{}", hop + 1)),
            )
            .mount(&server)
            .await;
    }

    let client = builder_for(&server).retry(fast_retries()).build().unwrap();
    let error = get_point(&client).await.unwrap_err();
    assert!(!error.is_retryable());
    assert_eq!(error.attempts(), 1);
    let Error::Protocol(protocol) = error else {
        panic!("expected protocol error");
    };
    assert!(
        matches!(
            protocol.as_ref(),
            ProtocolError::Redirect {
                reason: RedirectReason::TooManyRedirects { limit: 5 },
                ..
            }
        ),
        "{protocol:?}"
    );
    assert_eq!(protocol.url().path(), "/hop5");
    assert_eq!(server.received_requests().await.unwrap().len(), 6);
}

#[tokio::test]
async fn a_no_retry_policy_fails_on_the_first_server_error() {
    let server = MockServer::start().await;
    Mock::given(path("/alerts/types"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;

    let client = builder_for(&server).build().unwrap();
    let error = alerts::get_alert_types(&client).await.unwrap_err();
    assert_eq!(error.attempts(), 1);
    assert_eq!(error.status().map(|status| status.as_u16()), Some(503));
}

#[test]
fn build_rejects_an_empty_user_agent_and_an_invalid_base_url() {
    assert!(matches!(
        Client::builder("").build(),
        Err(BuildError::InvalidUserAgent)
    ));
    let error = Client::builder(USER_AGENT)
        .base_url("weather.gov/api")
        .build()
        .unwrap_err();
    assert!(
        matches!(&error, BuildError::InvalidBaseUrl { url, .. } if url == "weather.gov/api"),
        "{error}"
    );
    assert!(matches!(
        Client::builder(USER_AGENT)
            .base_url("ftp://weather.gov/")
            .build(),
        Err(BuildError::InvalidBaseUrl { .. })
    ));
    let _: &dyn std::error::Error = &error;
}
