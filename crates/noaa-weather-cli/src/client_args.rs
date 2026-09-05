//! Global arguments that configure the HTTP client.
//!
//! This module owns the only call to [`Client::builder`] the CLI makes at run
//! time, so every knob the program exposes over the transport is visible in
//! one place. The other call sites are `#[cfg(test)]`.

use std::env;
use std::fmt;
use std::time::Duration;

use clap::Args;
use jiff::SignedDuration;
use noaa_weather_client::{BuildError, Client, RetryPolicy};

/// The `User-Agent` sent when nothing overrides it.
const DEFAULT_USER_AGENT: &str = concat!(
    "noaa-weather/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/seferino-fernandez/noaa_weather)"
);

/// The environment variable holding an optional NOAA API key.
const API_KEY_VARIABLE: &str = "NOAA_WEATHER_API_KEY";

/// The trailing section of `--help`, listing the variables the program reads.
///
/// `NOAA_WEATHER_API_KEY` has no flag of its own, so this is the only place
/// it is documented.
pub(crate) const ENVIRONMENT_HELP: &str = "\
Environment variables:
  NOAA_WEATHER_BASE_URL    Same as --base-url
  NOAA_WEATHER_USER_AGENT  Same as --user-agent
  NOAA_WEATHER_TIMEOUT     Same as --timeout
  NOAA_WEATHER_RETRIES     Same as --retries
  NOAA_WEATHER_API_KEY     Sent as the X-Api-Key header to the base URL's
                           origin. There is deliberately no flag for it:
                           a process's arguments are readable by other
                           users on the machine. NOAA's API is free and
                           does not normally need a key.

A flag always overrides the matching variable.";

/// Global command-line arguments that configure the HTTP client.
#[derive(Args, Debug)]
pub(crate) struct ClientArgs {
    /// Send requests to this API root instead of NOAA's.
    ///
    /// The program has one real destination, so this exists for testing
    /// against a fixture server and for pointing at a local proxy.
    #[arg(
        long,
        global = true,
        value_name = "URL",
        env = "NOAA_WEATHER_BASE_URL",
        hide_env_values = true
    )]
    base_url: Option<String>,

    /// Identify this program to NOAA with this `User-Agent`.
    #[arg(
        long,
        global = true,
        value_name = "UA",
        env = "NOAA_WEATHER_USER_AGENT",
        hide_env_values = true,
        default_value = DEFAULT_USER_AGENT
    )]
    user_agent: String,

    /// Give up on one request attempt after this long, for example `30s` or
    /// `1m30s`.
    #[arg(
        long,
        global = true,
        value_name = "DURATION",
        env = "NOAA_WEATHER_TIMEOUT",
        hide_env_values = true,
        // Without this a negative duration is rejected as an unknown flag,
        // which hides the real complaint.
        allow_hyphen_values = true,
        value_parser = parse_timeout
    )]
    timeout: Option<Duration>,

    /// Attempt a retryable request this many times; `0` and `1` both mean one
    /// attempt and no retry.
    #[arg(
        long,
        global = true,
        value_name = "N",
        env = "NOAA_WEATHER_RETRIES",
        hide_env_values = true
    )]
    retries: Option<u8>,
}

impl ClientArgs {
    /// Builds the client these arguments describe.
    ///
    /// # Errors
    ///
    /// Returns a [`ClientBuildError`] when the library rejects the user
    /// agent, the API key, or the base URL, or cannot initialize its HTTP
    /// stack. Downcast to it to recover the [`Fault`].
    pub(crate) fn build(&self) -> anyhow::Result<Client> {
        let mut builder = Client::builder(&self.user_agent);
        if let Some(base_url) = &self.base_url {
            builder = builder.base_url(base_url);
        }
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(retries) = self.retries {
            builder = builder.retry(retry_policy(retries));
        }
        if let Ok(key) = env::var(API_KEY_VARIABLE)
            && !key.is_empty()
        {
            builder = builder.api_key(key);
        }
        Ok(builder.build().map_err(ClientBuildError::new)?)
    }
}

/// Turns a requested attempt count into a policy.
///
/// `RetryPolicy::max_attempts` treats 0 as 1, so 0 needs its own branch to
/// mean "one attempt, no retries" rather than falling through to the default
/// schedule.
fn retry_policy(retries: u8) -> RetryPolicy {
    if retries == 0 {
        RetryPolicy::none()
    } else {
        RetryPolicy::default().max_attempts(retries)
    }
}

/// Parses a timeout written in jiff's friendly duration format.
///
/// The library has no way to express "wait forever", so a zero or negative
/// duration is a usage error rather than something to translate.
fn parse_timeout(text: &str) -> Result<Duration, String> {
    let parsed: SignedDuration = text
        .parse()
        .map_err(|error| format!("{text:?} is not a duration such as `30s` or `1m30s`: {error}"))?;
    Duration::try_from(parsed)
        .ok()
        .filter(|duration| !duration.is_zero())
        .ok_or_else(|| format!("timeout must be greater than zero, but {text:?} is not"))
}

/// Who has to fix a failed client build.
///
/// The distinction is between a value the caller typed or exported and a
/// failure inside the HTTP stack that no argument would have changed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Fault {
    /// A flag or environment variable held a value the library rejected.
    Usage,
    /// The HTTP client could not be initialized.
    Internal,
}

/// A [`BuildError`] with the fault assigned and the source of the bad value
/// named.
///
/// `BuildError` says what was wrong; it cannot say which flag or variable
/// carried the value, and for the API key there is no flag to point at.
#[derive(Debug)]
pub struct ClientBuildError {
    fault: Fault,
    origin: &'static str,
    source: BuildError,
}

impl ClientBuildError {
    fn new(source: BuildError) -> Self {
        let (fault, origin) = match &source {
            BuildError::InvalidUserAgent => {
                (Fault::Usage, "--user-agent or NOAA_WEATHER_USER_AGENT")
            }
            BuildError::InvalidApiKey => (Fault::Usage, API_KEY_VARIABLE),
            BuildError::InvalidBaseUrl { .. } => {
                (Fault::Usage, "--base-url or NOAA_WEATHER_BASE_URL")
            }
            BuildError::Http(_) => (Fault::Internal, ""),
            // `BuildError` is `#[non_exhaustive]`; a variant added later has
            // no flag to name, so treat it as the process's problem until
            // someone classifies it.
            _ => (Fault::Internal, ""),
        };
        Self {
            fault,
            origin,
            source,
        }
    }

    /// Returns whether the caller or the process is to blame.
    #[must_use]
    pub const fn fault(&self) -> Fault {
        self.fault
    }
}

impl fmt::Display for ClientBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.source)?;
        if !self.origin.is_empty() {
            write!(formatter, " (set by {})", self.origin)?;
        }
        Ok(())
    }
}

impl std::error::Error for ClientBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friendly_durations_parse_and_zero_or_negative_do_not() {
        assert_eq!(parse_timeout("30s"), Ok(Duration::from_secs(30)));
        assert_eq!(parse_timeout("1m30s"), Ok(Duration::from_secs(90)));
        assert_eq!(parse_timeout("250ms"), Ok(Duration::from_millis(250)));
        assert!(parse_timeout("0s").is_err());
        assert!(parse_timeout("-5s").is_err());
        assert!(parse_timeout("soon").is_err());
    }

    #[test]
    fn zero_retries_is_a_single_attempt_and_other_counts_keep_the_schedule() {
        assert_eq!(retry_policy(0), RetryPolicy::none());
        assert_eq!(retry_policy(5), RetryPolicy::default().max_attempts(5));
        assert_ne!(retry_policy(5), RetryPolicy::none());
    }

    #[test]
    fn a_bad_api_key_names_the_variable_that_carried_it_and_blames_the_caller() {
        let error = ClientBuildError::new(BuildError::InvalidApiKey);
        assert_eq!(error.fault(), Fault::Usage);
        assert!(error.to_string().contains(API_KEY_VARIABLE), "{error}");
    }

    #[test]
    fn a_bad_base_url_names_both_ways_of_setting_it() {
        let error = ClientBuildError::new(BuildError::InvalidBaseUrl {
            url: "nope".to_owned(),
            source: None,
        });
        assert_eq!(error.fault(), Fault::Usage);
        let message = error.to_string();
        assert!(message.contains("--base-url"), "{message}");
        assert!(message.contains("NOAA_WEATHER_BASE_URL"), "{message}");
    }
}
