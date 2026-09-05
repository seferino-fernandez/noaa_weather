//! Stable, bounded caller-visible projection of client failures.

use noaa_weather_client::stations::taf::TafDecodeErrorKind;
use noaa_weather_client::{Error as ClientError, ProtocolError, RedirectReason};
use rmcp::model::{ContentBlock, IntoContents};
use serde::Serialize;
use serde_json::{Value, json};

const MAX_MESSAGE_BYTES: usize = 1_024;
const MAX_TITLE_BYTES: usize = 256;
const MAX_URI_BYTES: usize = 512;
const MAX_ID_OR_PATH_BYTES: usize = 256;
const MAX_INPUT_BYTES: usize = 512;
const MAX_FAILURE_BYTES: usize = 4 * 1_024;

/// A caller-visible MCP tool failure encoded as bounded JSON text.
#[derive(Debug)]
#[allow(
    dead_code,
    reason = "family tools consume this shared type after foundation"
)]
pub(super) struct ToolFailure(String);

impl From<ClientError> for ToolFailure {
    fn from(error: ClientError) -> Self {
        Self(project(error))
    }
}

impl IntoContents for ToolFailure {
    fn into_contents(self) -> Vec<ContentBlock> {
        vec![ContentBlock::text(self.0)]
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FailureBody {
    code: &'static str,
    message: String,
    retryable: bool,
    attempts: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_after: Option<RetryDelay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Value>,
}

#[derive(Clone, Copy, Serialize)]
struct RetryDelay {
    seconds: u64,
    nanoseconds: u32,
}

fn project(error: ClientError) -> String {
    let retryable = error.is_retryable();
    let attempts = error.attempts();
    let status = error.status().map(|status| status.as_u16());
    let retry_after = error.retry_after().map(|delay| RetryDelay {
        seconds: delay.as_secs(),
        nanoseconds: delay.subsec_nanos(),
    });

    let mut body = FailureBody {
        code: "client_error",
        message: "The NOAA client could not complete the request.".to_owned(),
        retryable,
        attempts,
        status,
        retry_after,
        correlation_id: None,
        request_id: None,
        details: None,
    };

    match &error {
        ClientError::Transport { source, .. } => {
            body.code = "transport_error";
            body.message = "The NOAA request could not be completed.".to_owned();
            let subtype = if source.is_timeout() {
                "timeout"
            } else if source.is_connect() {
                "connect"
            } else if source.is_body() {
                "body"
            } else if source.is_decode() {
                "decode"
            } else if source.is_redirect() {
                "redirect"
            } else if source.is_status() {
                "status"
            } else if source.is_builder() {
                "builder"
            } else if source.is_request() {
                "request"
            } else {
                "other"
            };
            body.details = Some(json!({ "transportSubtype": subtype }));
        }
        ClientError::Json(source) => {
            body.code = "invalid_json_response";
            body.message =
                "NOAA returned a response that could not be decoded as the endpoint's expected JSON shape."
                    .to_owned();
            body.details = Some(json!({
                "category": json_category(source),
                "line": source.line(),
                "column": source.column(),
            }));
        }
        ClientError::Xml(_) => {
            body.code = "invalid_xml_response";
            body.message = "NOAA returned a response that was not valid XML.".to_owned();
        }
        ClientError::TerminalAerodromeForecast(source) => {
            body.code = "invalid_taf_response";
            body.message = "NOAA returned a TAF response that could not be decoded.".to_owned();
            body.details = Some(json!({
                "kind": taf_kind(source.kind()),
                "path": bounded(source.path(), MAX_ID_OR_PATH_BYTES),
            }));
        }
        ClientError::Response(response) => {
            body.code = if error.is_not_found() {
                "not_found"
            } else if error.is_rate_limited() {
                "rate_limited"
            } else {
                "upstream_http_error"
            };
            body.correlation_id = response
                .correlation_id()
                .map(|value| bounded(value, MAX_ID_OR_PATH_BYTES));
            body.request_id = response
                .request_id()
                .map(|value| bounded(value, MAX_ID_OR_PATH_BYTES));
            if let Some(problem) = response.problem_detail() {
                body.message = if problem.detail.is_empty() {
                    bounded(&problem.title, MAX_MESSAGE_BYTES)
                } else {
                    bounded(&problem.detail, MAX_MESSAGE_BYTES)
                };
                body.details = Some(json!({
                    "problem": {
                        "type": bounded(&problem.r#type, MAX_URI_BYTES),
                        "title": bounded(&problem.title, MAX_TITLE_BYTES),
                        "status": problem.status,
                        "detail": bounded(&problem.detail, MAX_MESSAGE_BYTES),
                        "instance": bounded(&problem.instance, MAX_URI_BYTES),
                        "correlationId": bounded(
                            &problem.correlation_id,
                            MAX_ID_OR_PATH_BYTES,
                        ),
                    }
                }));
            } else {
                body.message = format!("NOAA returned HTTP status {}.", response.status().as_u16());
            }
        }
        ClientError::Protocol(source) => {
            body.code = "protocol_error";
            body.message =
                "NOAA returned a response that violated the endpoint contract.".to_owned();
            body.details = Some(protocol_details(source));
        }
        ClientError::Invalid(source) => {
            body.code = "invalid_input";
            body.message = format!("The supplied {} was invalid.", source.kind().name());
            body.details = Some(json!({
                "kind": source.kind().name(),
                "input": bounded(source.input(), MAX_INPUT_BYTES),
                "reason": bounded(source.reason(), MAX_MESSAGE_BYTES),
            }));
        }
        _ => {}
    }

    body.message = bounded(&body.message, MAX_MESSAGE_BYTES);
    serialize_bounded(body)
}

fn serialize_bounded(mut body: FailureBody) -> String {
    let full = serde_json::to_string(&body)
        .unwrap_or_else(|_| r#"{"code":"client_error","message":"The NOAA client could not complete the request.","retryable":false,"attempts":1}"#.to_owned());
    if full.len() <= MAX_FAILURE_BYTES {
        return full;
    }

    body.message = generic_message(body.code).to_owned();
    body.correlation_id = None;
    body.request_id = None;
    body.details = None;
    serde_json::to_string(&body)
        .unwrap_or_else(|_| r#"{"code":"client_error","message":"The NOAA client could not complete the request.","retryable":false,"attempts":1}"#.to_owned())
}

fn generic_message(code: &str) -> &'static str {
    match code {
        "transport_error" => "The NOAA request could not be completed.",
        "invalid_json_response" => "NOAA returned JSON that could not be decoded as expected.",
        "invalid_xml_response" => "NOAA returned invalid XML.",
        "invalid_taf_response" => "NOAA returned an invalid TAF response.",
        "not_found" => "The requested NOAA resource was not found.",
        "rate_limited" => "NOAA rate-limited the request.",
        "upstream_http_error" => "NOAA returned an HTTP error.",
        "protocol_error" => "NOAA returned a response that violated the endpoint contract.",
        "invalid_input" => "A request value was invalid.",
        _ => "The NOAA client could not complete the request.",
    }
}

fn bounded(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_owned();
    }
    let mut end = max_bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_owned()
}

fn json_category(error: &serde_json::Error) -> &'static str {
    match error.classify() {
        serde_json::error::Category::Io => "io",
        serde_json::error::Category::Syntax => "syntax",
        serde_json::error::Category::Data => "data",
        serde_json::error::Category::Eof => "eof",
    }
}

fn taf_kind(kind: TafDecodeErrorKind) -> &'static str {
    match kind {
        TafDecodeErrorKind::MalformedXml => "malformed_xml",
        TafDecodeErrorKind::MissingRequiredField => "missing_required_field",
        TafDecodeErrorKind::InvalidTimestamp => "invalid_timestamp",
        TafDecodeErrorKind::InvalidPeriod => "invalid_period",
        TafDecodeErrorKind::InvalidNumber => "invalid_number",
        TafDecodeErrorKind::InvalidCoordinate => "invalid_coordinate",
        TafDecodeErrorKind::UnsupportedUnit => "unsupported_unit",
        TafDecodeErrorKind::InvalidCombination => "invalid_combination",
        TafDecodeErrorKind::InvalidValue => "invalid_value",
        _ => "other",
    }
}

fn protocol_details(error: &ProtocolError) -> Value {
    match error {
        ProtocolError::MissingContentType { expected, .. } => json!({
            "protocolSubtype": "missing_content_type",
            "expected": bounded(expected, MAX_TITLE_BYTES),
        }),
        ProtocolError::MalformedContentType {
            expected, actual, ..
        } => json!({
            "protocolSubtype": "malformed_content_type",
            "expected": bounded(expected, MAX_TITLE_BYTES),
            "actual": bounded(actual, MAX_TITLE_BYTES),
        }),
        ProtocolError::IncompatibleContentType {
            expected, actual, ..
        } => json!({
            "protocolSubtype": "incompatible_content_type",
            "expected": bounded(expected, MAX_TITLE_BYTES),
            "actual": bounded(actual.as_ref(), MAX_TITLE_BYTES),
        }),
        ProtocolError::Redirect { reason, .. } => json!({
            "protocolSubtype": "redirect",
            "redirectReason": redirect_reason(reason),
        }),
        ProtocolError::ResponseTooLarge { limit, .. } => json!({
            "protocolSubtype": "response_too_large",
            "limit": limit,
        }),
        _ => json!({ "protocolSubtype": "other" }),
    }
}

fn redirect_reason(reason: &RedirectReason) -> Value {
    match reason {
        RedirectReason::TooManyRedirects { limit } => {
            json!({ "kind": "too_many_redirects", "limit": limit })
        }
        RedirectReason::MissingLocation => json!({ "kind": "missing_location" }),
        RedirectReason::InvalidLocation { .. } => json!({ "kind": "invalid_location" }),
        RedirectReason::InsecureDowngrade { .. } => json!({ "kind": "insecure_downgrade" }),
        _ => json!({ "kind": "other" }),
    }
}

#[cfg(test)]
impl ToolFailure {
    pub(super) fn json(&self) -> &str {
        &self.0
    }
}
