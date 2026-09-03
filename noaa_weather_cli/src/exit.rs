//! What the process returns to its caller, and what it writes when it fails.
//!
//! Two things have to agree and are therefore decided here: the exit code,
//! which a shell script branches on, and the `kind` in the JSON error line,
//! which a program branches on. [`ExitCode::kind`] is the only place either
//! is spelled, so the two taxonomies cannot drift apart.

use std::error::Error as StdError;

use noaa_weather_client::{BuildError, Error};
use serde::Serialize;
use serde_json::Value;

use crate::client_args::{ClientBuildError, Fault};
use crate::output::{OutputFailure, UsageFailure};

/// The status the program exits with.
///
/// The point of separating these is that a caller can act on them
/// differently: 4 is worth retrying later, 3 means NOAA answered and said no,
/// 2 means the command line was wrong and retrying it will not help.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ExitCode {
    /// The command produced its output.
    Ok = 0,
    /// The program failed for a reason none of the others describe,
    /// including a response body that did not decode.
    Internal = 1,
    /// The command line, or an environment variable standing in for one,
    /// carried a value the program rejected.
    ///
    /// Including the requests that argv alone makes impossible, such as
    /// `--json` on a command whose response is a PDF: no request is made,
    /// and it would fail the same way on every machine.
    Usage = 2,
    /// NOAA answered with a non-success HTTP status.
    Noaa = 3,
    /// The request never got a complete answer.
    Network = 4,
    /// The output destination could not take the bytes on this machine.
    ///
    /// Deliberately not "the answer arrived, then the write failed": the
    /// destination is validated before the request, so an unwritable
    /// `--output` path exits 5 without NOAA ever being asked. What the code
    /// means is "look at the filesystem", not "the fetch was wasted".
    Output = 5,
}

impl ExitCode {
    /// The number the process exits with.
    #[must_use]
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// The `kind` this code appears under in the JSON error line.
    ///
    /// [`ExitCode::Ok`] has none because there is no error.
    /// [`ExitCode::Usage`] has none by choice, not by accident: some usage
    /// errors this program detects itself, after `--format` has been parsed,
    /// so they *could* carry a line. Giving them one would make "exit 2
    /// never writes JSON" a sometimes rather than an always, and an absolute
    /// is worth more to a consumer than a rule with a footnote.
    ///
    /// The direction of that reasoning matters. A failure is classified by
    /// what went wrong, and only then asked whether its code carries a kind.
    /// Reasoning the other way — "there is no `usage` kind, so this must be
    /// something else" — is what once made `--json` on a PDF an `output`
    /// failure.
    ///
    /// Keeping the four kinds derived from the code rather than listed
    /// separately is what stops a caller's `$?` and its parsed `kind` from
    /// disagreeing.
    #[must_use]
    pub const fn kind(self) -> Option<&'static str> {
        match self {
            Self::Noaa => Some("noaa"),
            Self::Network => Some("network"),
            Self::Output => Some("output"),
            Self::Internal => Some("internal"),
            Self::Ok | Self::Usage => None,
        }
    }
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> Self {
        Self::from(code.code())
    }
}

/// Assigns an exit code to a failed command.
///
/// The error arrives as an `anyhow::Error` carrying whatever context the
/// command added, so the classification walks the chain and takes the first
/// cause it recognizes. Nothing recognizable means [`ExitCode::Internal`]:
/// silence is a bug in this function, and 1 is the code that says "no more
/// specific answer than failure".
#[must_use]
pub fn classify(error: &anyhow::Error) -> ExitCode {
    error
        .chain()
        .find_map(classify_cause)
        .unwrap_or(ExitCode::Internal)
}

fn classify_cause(cause: &(dyn StdError + 'static)) -> Option<ExitCode> {
    if let Some(error) = cause.downcast_ref::<Error>() {
        return Some(from_request(error));
    }
    // The wrapper answers first because it is what `client_args` produces;
    // the bare `BuildError` below is its source, and any other call site's.
    if let Some(error) = cause.downcast_ref::<ClientBuildError>() {
        return Some(match error.fault() {
            Fault::Usage => ExitCode::Usage,
            Fault::Internal => ExitCode::Internal,
        });
    }
    if let Some(error) = cause.downcast_ref::<BuildError>() {
        return Some(from_build(error));
    }
    if cause.is::<UsageFailure>() {
        return Some(ExitCode::Usage);
    }
    if cause.is::<OutputFailure>() {
        return Some(ExitCode::Output);
    }
    None
}

/// Classifies a failure the client returned from a request.
///
/// A decode failure is 1 rather than 3. A caller who sees 3 should be able to
/// read it as "NOAA refused this request", and act on the status; a body that
/// arrived and did not parse is not something a different argument would fix,
/// and folding it into 3 would make `[ $? -eq 3 ]` quietly wrong.
fn from_request(error: &Error) -> ExitCode {
    match error {
        Error::Response(_) => ExitCode::Noaa,
        Error::Transport { .. } => ExitCode::Network,
        Error::Invalid(_) => ExitCode::Usage,
        Error::Json(_)
        | Error::Xml(_)
        | Error::TerminalAerodromeForecast(_)
        | Error::Protocol(_) => ExitCode::Internal,
        // `Error` is `#[non_exhaustive]`. A variant added later is not
        // something this program knows how to describe, so it is a failure
        // with no more specific answer.
        _ => ExitCode::Internal,
    }
}

/// Classifies a failure to build the client.
fn from_build(error: &BuildError) -> ExitCode {
    match error {
        BuildError::InvalidUserAgent
        | BuildError::InvalidApiKey
        | BuildError::InvalidBaseUrl { .. } => ExitCode::Usage,
        BuildError::Http(_) => ExitCode::Internal,
        // As above: unclassified until somebody classifies it.
        _ => ExitCode::Internal,
    }
}

/// The one-line JSON document written to standard error under `--json`.
#[derive(Serialize)]
struct Envelope<'a> {
    error: Report<'a>,
}

/// Everything a program might branch on, with absent facts omitted rather
/// than written as `null`.
#[derive(Serialize)]
struct Report<'a> {
    kind: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_after: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<&'a str>,
    /// NOAA's RFC 7807 body, passed through byte for byte.
    ///
    /// Nested rather than flattened: its `status` and `title` are the
    /// server's account of the failure and the surrounding fields are the
    /// client's, and merging them would silently drop one of the two
    /// `status` values.
    ///
    /// Parsed from the response body rather than re-serialized from
    /// [`ProblemDetail`], which is the difference between "whole" and
    /// "whole except the parts we modelled". `ProblemDetail` has no
    /// `parameterErrors` field, so round-tripping through it dropped the
    /// four entries NOAA sends on a 400 naming the parameter and the
    /// patterns it failed — the only machine-readable account of *what was
    /// wrong with the value*. It also declares `status` as `f64`, so a `400`
    /// came back out as `400.0` while the sibling `status` above stayed an
    /// integer: one line, one key name, two JSON types.
    #[serde(skip_serializing_if = "Option::is_none")]
    problem: Option<Value>,
}

/// Renders the JSON error line for `error`, without its newline.
///
/// Returns `None` for an exit code that carries no `kind`; see
/// [`ExitCode::kind`].
#[must_use]
pub fn error_line(error: &anyhow::Error, code: ExitCode) -> Option<String> {
    let kind = code.kind()?;
    let response = error
        .chain()
        .find_map(|cause| cause.downcast_ref::<Error>());

    let report = Report {
        kind,
        // The same text the human line carries, causes included, so the two
        // forms describe one failure rather than two.
        message: format!("{error:#}"),
        status: response
            .and_then(|error| error.status())
            .map(|status| status.as_u16()),
        url: response.and_then(request_url),
        retry_after: response
            .and_then(noaa_weather_client::Error::retry_after)
            .map(|delay| delay.as_secs()),
        correlation_id: response.and_then(correlation_id),
        request_id: response.and_then(request_id),
        problem: response.and_then(problem_body),
    };

    // A `Report` has no map keys that could fail to serialize, so this
    // cannot fail; falling back keeps a bug here from costing the exit code.
    serde_json::to_string(&Envelope { error: report }).ok()
}

/// NOAA's problem document, as it arrived.
///
/// The client parses what it models and keeps the raw bytes; this reads the
/// bytes. Two guards keep it from embedding something that is not a problem
/// document: NOAA has to have said it was one, and it has to parse as a JSON
/// object. A body that is HTML, or a bare string, is left out rather than
/// nested under a key whose name promises RFC 7807.
///
/// Recognizing it by `Content-Type` *or* by the client having decoded a
/// [`ProblemDetail`] is deliberate: either alone would go quiet if NOAA
/// changed the other.
fn problem_body(error: &Error) -> Option<Value> {
    let Error::Response(response) = error else {
        return None;
    };
    let declared = response
        .content_type()
        .is_some_and(|media| media.essence_str() == "application/problem+json");
    if !declared && response.problem_detail().is_none() {
        return None;
    }
    match serde_json::from_slice(response.as_bytes()) {
        Ok(Value::Object(members)) => Some(Value::Object(members)),
        _ => None,
    }
}

/// The URL a failure names, for the two kinds of error that carry one.
fn request_url(error: &Error) -> Option<String> {
    match error {
        Error::Response(response) => Some(response.url().to_string()),
        Error::Protocol(protocol) => Some(protocol.url().to_string()),
        _ => None,
    }
}

fn correlation_id(error: &Error) -> Option<&str> {
    match error {
        Error::Response(response) => response.correlation_id(),
        _ => None,
    }
}

fn request_id(error: &Error) -> Option<&str> {
    match error {
        Error::Response(response) => response.request_id(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use noaa_weather_client::ids::{InvalidValue, ValueKind};
    use serde_json::Value;

    use super::*;

    /// A decode failure, which is the only [`Error`] variant this crate can
    /// build: `ResponseContent`'s fields are private to the client, so the
    /// `Response` half of this module is checked end to end against a mock
    /// server in `tests/exit_codes.rs` instead.
    fn decode_error() -> Error {
        Error::Json(serde_json::from_str::<Value>("{").unwrap_err())
    }

    #[test]
    fn every_code_that_carries_a_kind_carries_a_distinct_one() {
        let codes = [
            ExitCode::Ok,
            ExitCode::Internal,
            ExitCode::Usage,
            ExitCode::Noaa,
            ExitCode::Network,
            ExitCode::Output,
        ];
        let kinds: Vec<&str> = codes.iter().filter_map(|code| code.kind()).collect();
        assert_eq!(kinds, ["internal", "noaa", "network", "output"]);
        assert_eq!(ExitCode::Ok.kind(), None);
        assert_eq!(ExitCode::Usage.kind(), None);
        for (code, number) in codes.into_iter().zip([0, 1, 2, 3, 4, 5]) {
            assert_eq!(code.code(), number, "{code:?}");
        }
    }

    #[test]
    fn a_request_failure_is_classified_by_what_went_wrong() {
        assert_eq!(from_request(&decode_error()), ExitCode::Internal);
        assert_eq!(
            from_request(&Error::Invalid(InvalidValue::new(
                ValueKind::ZoneId,
                "CAZ 043",
                "zone ids have no spaces"
            ))),
            ExitCode::Usage
        );
    }

    #[test]
    fn a_build_failure_blames_the_caller_for_every_value_it_typed() {
        for error in [
            BuildError::InvalidUserAgent,
            BuildError::InvalidApiKey,
            BuildError::InvalidBaseUrl {
                url: "nope".to_owned(),
                source: None,
            },
        ] {
            assert_eq!(from_build(&error), ExitCode::Usage, "{error}");
        }
    }

    #[test]
    fn classification_reaches_through_the_context_a_command_added() {
        let buried = anyhow::Error::new(decode_error())
            .context("getting active alert counts")
            .context("and one more layer");
        assert_eq!(classify(&buried), ExitCode::Internal);

        // A wrapped output failure has to survive the same burial, or the
        // only thing separating 5 from 1 is which layer looked first.
        let buried = OutputFailure::wrap(anyhow!("permission denied"))
            .context("writing output to /root/out.json")
            .context("getting the NWS glossary");
        assert_eq!(classify(&buried), ExitCode::Output);
    }

    #[test]
    fn an_unrecognized_failure_is_internal_rather_than_silent() {
        assert_eq!(
            classify(&anyhow!("something went wrong")),
            ExitCode::Internal
        );
    }

    /// A failure that carries none of the optional facts writes none of the
    /// optional keys, rather than writing them as `null`.
    ///
    /// The populated direction needs a real NOAA response and lives in
    /// `tests/exit_codes.rs`.
    #[test]
    fn the_json_line_omits_the_facts_a_failure_does_not_carry() {
        let error = anyhow!("permission denied").context("writing output to /root/out.json");
        let line = error_line(&error, ExitCode::Output).expect("output carries a kind");
        assert!(!line.contains('\n'), "the line must be one line: {line}");

        let document: Value = serde_json::from_str(&line).expect("a parseable line");
        let report = document["error"].as_object().expect("an error object");
        assert_eq!(report["kind"], "output");
        assert_eq!(
            report["message"], "writing output to /root/out.json: permission denied",
            "the message must carry the causes the human line carries"
        );
        for absent in [
            "status",
            "url",
            "retry_after",
            "correlation_id",
            "request_id",
            "problem",
        ] {
            assert!(!report.contains_key(absent), "{absent} should be omitted");
        }
    }

    #[test]
    fn a_usage_failure_has_no_json_line_to_write() {
        let error = anyhow!("bad value").context("running a command");
        assert!(error_line(&error, ExitCode::Usage).is_none());
        assert!(error_line(&error, ExitCode::Ok).is_none());
    }
}
