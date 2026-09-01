//! Retry policy, backoff schedule, and `Retry-After` interpretation.

use std::{
    hash::{DefaultHasher, Hash as _, Hasher as _},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use jiff::{Timestamp, fmt::rfc2822};
use reqwest::{StatusCode, header::HeaderValue};

/// When and how often a [`Client`](super::Client) retries a failed request.
///
/// Retries apply to transient failures only: HTTP 429, 500, 502, 503, and
/// 504 responses, plus connection, timeout, and body-read transport errors.
/// Request construction, redirect, decode, and size-cap failures are never
/// retried. Redirects are resolved inside each attempt, so one attempt may
/// span several hops.
///
/// Delays grow exponentially from `base_delay`, are capped at `max_delay`,
/// and carry deterministic ±20% jitter. A `Retry-After` response header
/// replaces the computed delay; when it exceeds `max_delay` the client gives
/// up immediately and reports the header through
/// [`Error::retry_after`](crate::Error::retry_after).
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// use noaa_weather_client::RetryPolicy;
///
/// let patient = RetryPolicy::default()
///     .max_attempts(5)
///     .base_delay(Duration::from_secs(1))
///     .max_delay(Duration::from_secs(60));
/// let single_shot = RetryPolicy::none();
/// assert_ne!(patient, single_shot);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RetryPolicy {
    max_attempts: u8,
    base_delay: Duration,
    max_delay: Duration,
}

impl Default for RetryPolicy {
    /// Three attempts, starting at 500 ms and capped at 20 s.
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(20),
        }
    }
}

impl RetryPolicy {
    /// A policy that performs exactly one attempt and never retries.
    #[must_use]
    pub fn none() -> Self {
        Self {
            max_attempts: 1,
            ..Self::default()
        }
    }

    /// Sets the total number of attempts, including the first request.
    ///
    /// Values below 1 are treated as 1.
    #[must_use]
    pub const fn max_attempts(mut self, attempts: u8) -> Self {
        self.max_attempts = if attempts == 0 { 1 } else { attempts };
        self
    }

    /// Sets the delay before the first retry; later delays double each time.
    #[must_use]
    pub const fn base_delay(mut self, delay: Duration) -> Self {
        self.base_delay = delay;
        self
    }

    /// Sets the longest delay the client will wait between attempts.
    ///
    /// A `Retry-After` header requesting a longer wait stops the retry loop.
    #[must_use]
    pub const fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Returns how long to wait before the attempt after `attempt`, or
    /// `None` when the client must stop retrying.
    ///
    /// `attempt` is one-based. A server-supplied `Retry-After` wins over the
    /// backoff schedule but cannot exceed `max_delay`.
    pub(crate) fn delay_before_retry(
        &self,
        attempt: u8,
        retry_after: Option<Duration>,
    ) -> Option<Duration> {
        if attempt >= self.max_attempts {
            return None;
        }
        match retry_after {
            Some(delay) if delay > self.max_delay => None,
            Some(delay) => Some(delay),
            None => Some(self.backoff(attempt, jitter_seed())),
        }
    }

    /// Computes the exponential backoff for a one-based attempt number with
    /// deterministic ±20% jitter derived from `seed`.
    fn backoff(&self, attempt: u8, seed: u64) -> Duration {
        let exponent = u32::from(attempt.saturating_sub(1)).min(31);
        let scaled = self
            .base_delay
            .checked_mul(1_u32 << exponent)
            .unwrap_or(self.max_delay)
            .min(self.max_delay);
        // Map the seed onto [0.8, 1.2) without floating-point randomness
        // sources: the seed already comes from a hash.
        let permille = seed % 1_000;
        let factor = 0.8 + (permille as f64) / 2_500.0;
        scaled.mul_f64(factor).min(self.max_delay)
    }
}

fn jitter_seed() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    let mut hasher = DefaultHasher::new();
    nanos.hash(&mut hasher);
    hasher.finish()
}

/// Returns whether a final response status is worth another attempt.
pub(crate) const fn retryable_status(status: StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 500 | 502 | 503 | 504)
}

/// Returns whether a transport failure is transient enough to retry.
pub(crate) fn retryable_transport(error: &reqwest::Error) -> bool {
    error.is_timeout() || error.is_connect() || error.is_body()
}

/// Parses a `Retry-After` header as delay seconds or an HTTP-date.
///
/// Dates in the past yield a zero delay. Unparseable values yield `None`.
pub(crate) fn parse_retry_after(value: &HeaderValue, now: Timestamp) -> Option<Duration> {
    let text = value.to_str().ok()?.trim();
    if let Ok(seconds) = text.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let at = rfc2822::DateTimeParser::new().parse_timestamp(text).ok()?;
    let delay = at.duration_since(now);
    Some(Duration::try_from(delay).unwrap_or(Duration::ZERO))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use jiff::Timestamp;
    use reqwest::{StatusCode, header::HeaderValue};

    use super::{RetryPolicy, parse_retry_after, retryable_status};

    #[test]
    fn default_policy_and_none_have_the_documented_shape() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_attempts, 3);
        assert_eq!(policy.base_delay, Duration::from_millis(500));
        assert_eq!(policy.max_delay, Duration::from_secs(20));
        assert_eq!(RetryPolicy::none().max_attempts, 1);
        assert_eq!(RetryPolicy::default().max_attempts(0).max_attempts, 1);
    }

    #[test]
    fn backoff_doubles_from_base_delay_with_bounded_jitter() {
        let policy = RetryPolicy::default()
            .base_delay(Duration::from_millis(100))
            .max_delay(Duration::from_secs(20));
        for seed in [0, 1, 499, 999, u64::MAX] {
            let first = policy.backoff(1, seed);
            let second = policy.backoff(2, seed);
            let third = policy.backoff(3, seed);
            assert!((80..120).contains(&first.as_millis()), "{first:?}");
            assert!((160..240).contains(&second.as_millis()), "{second:?}");
            assert!((320..480).contains(&third.as_millis()), "{third:?}");
        }
        assert_eq!(policy.backoff(1, 0), Duration::from_millis(80));
        assert_eq!(policy.backoff(1, 500), Duration::from_millis(100));
    }

    #[test]
    fn backoff_never_exceeds_max_delay_even_for_large_attempts() {
        let policy = RetryPolicy::default()
            .base_delay(Duration::from_secs(1))
            .max_delay(Duration::from_secs(5));
        assert_eq!(policy.backoff(u8::MAX, 999), Duration::from_secs(5));
        assert_eq!(policy.backoff(4, 999), Duration::from_secs(5));
    }

    #[test]
    fn delay_before_retry_stops_at_max_attempts_and_honors_retry_after() {
        let policy = RetryPolicy::default()
            .max_attempts(2)
            .max_delay(Duration::from_secs(10));
        assert!(policy.delay_before_retry(1, None).is_some());
        assert!(policy.delay_before_retry(2, None).is_none());
        assert_eq!(
            policy.delay_before_retry(1, Some(Duration::from_secs(3))),
            Some(Duration::from_secs(3))
        );
        assert_eq!(
            policy.delay_before_retry(1, Some(Duration::from_secs(11))),
            None
        );
        assert_eq!(
            RetryPolicy::none().delay_before_retry(1, Some(Duration::ZERO)),
            None
        );
    }

    #[test]
    fn retryable_statuses_are_the_transient_server_and_rate_limit_codes() {
        for code in [429, 500, 502, 503, 504] {
            assert!(retryable_status(StatusCode::from_u16(code).unwrap()));
        }
        for code in [200, 301, 400, 401, 403, 404, 422, 501] {
            assert!(!retryable_status(StatusCode::from_u16(code).unwrap()));
        }
    }

    #[test]
    fn retry_after_accepts_seconds_and_http_dates() {
        let now = Timestamp::from_second(1_700_000_000).unwrap();
        assert_eq!(
            parse_retry_after(&HeaderValue::from_static("120"), now),
            Some(Duration::from_secs(120))
        );
        assert_eq!(
            parse_retry_after(&HeaderValue::from_static(" 0 "), now),
            Some(Duration::ZERO)
        );
        // 2023-11-14T22:13:20Z is 1_700_000_000; ask for 90 seconds later.
        assert_eq!(
            parse_retry_after(
                &HeaderValue::from_static("Tue, 14 Nov 2023 22:14:50 GMT"),
                now,
            ),
            Some(Duration::from_secs(90))
        );
        assert_eq!(
            parse_retry_after(
                &HeaderValue::from_static("Tue, 14 Nov 2023 22:00:00 GMT"),
                now,
            ),
            Some(Duration::ZERO)
        );
        assert_eq!(
            parse_retry_after(&HeaderValue::from_static("soon"), now),
            None
        );
        assert_eq!(
            parse_retry_after(&HeaderValue::from_static("-5"), now),
            None
        );
    }
}
