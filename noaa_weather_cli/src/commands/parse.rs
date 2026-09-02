//! Command-line value parsing the client leaves to callers.
//!
//! Typed identifiers, coordinates, and intervals parse straight into the
//! client's value types through `FromStr`, so clap reports a bad value as a
//! usage error (exit code 2) before any request is made. This module adds the
//! two CLI-only conveniences: relative times and the office completion hint.

use std::fmt::Write as _;

use jiff::{Span, Timestamp};
use noaa_weather_client::OfficeId;

/// Help text shared by every absolute-or-relative time flag.
pub const TIME_HELP: &str = "Accepts an RFC 3339 / ISO 8601 timestamp with an offset \
    (for example 2026-08-30T12:00:00Z or 2026-08-30T05:00:00-07:00) or a relative age \
    ending in m, h, or d that is resolved against the current time when the command \
    starts (for example 30m, 6h, or 2d for 30 minutes, 6 hours, or 2 days ago).";

/// Parses a time flag as an absolute timestamp or a relative age such as
/// `6h`, resolved against the current time.
pub fn time(text: &str) -> Result<Timestamp, String> {
    let text = text.trim();
    if let Some(span) = relative_span(text) {
        return Timestamp::now()
            .checked_sub(span)
            .map_err(|error| format!("relative time {text:?} is out of range: {error}"));
    }
    text.parse::<Timestamp>().map_err(|error| {
        format!(
            "{error}; expected an RFC 3339 timestamp such as 2026-08-30T12:00:00Z \
             or a relative age such as 6h"
        )
    })
}

fn relative_span(text: &str) -> Option<Span> {
    let (digits, unit) = text.split_at(text.len().checked_sub(1)?);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let amount: i64 = digits.parse().ok()?;
    match unit {
        "m" => Span::new().try_minutes(amount).ok(),
        "h" => Span::new().try_hours(amount).ok(),
        "d" => Span::new().try_hours(amount.checked_mul(24)?).ok(),
        _ => None,
    }
}

/// Long help for office arguments: the accepted shape plus the known
/// forecast office codes as a hint. Any structurally valid code is accepted
/// so product locations and headquarters outside this list still work.
pub fn office_long_help(role: &str) -> String {
    let mut help = format!(
        "{role} as a 3 or 4 character NWS code (case-insensitive). Any well-formed code \
         is accepted; regional headquarters (ARH, CRH, ERH, PRH, SRH, WRH) and the \
         national headquarters (NWS) work where NOAA serves them.\n\nKnown forecast \
         offices:"
    );
    for (index, code) in OfficeId::KNOWN.iter().enumerate() {
        let separator = if index % 12 == 0 { "\n  " } else { " " };
        let _ = write!(help, "{separator}{code}");
    }
    help
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_timestamps_parse_with_any_offset() {
        assert_eq!(
            time("2026-08-30T05:00:00-07:00").unwrap().to_string(),
            "2026-08-30T12:00:00Z"
        );
        assert_eq!(
            time(" 2026-08-30T12:00:00Z ").unwrap().to_string(),
            "2026-08-30T12:00:00Z"
        );
    }

    #[test]
    fn relative_ages_resolve_against_now() {
        let before = Timestamp::now();
        let six_hours_ago = time("6h").unwrap();
        let after = Timestamp::now();
        let expected_low = before.checked_sub(Span::new().hours(6)).unwrap();
        let expected_high = after.checked_sub(Span::new().hours(6)).unwrap();
        assert!(six_hours_ago >= expected_low && six_hours_ago <= expected_high);

        let two_days = time("2d").unwrap();
        let thirty_minutes = time("30m").unwrap();
        assert!(two_days < six_hours_ago);
        assert!(thirty_minutes > six_hours_ago);
    }

    #[test]
    fn malformed_times_are_rejected_with_guidance() {
        for text in ["", "6", "h", "6w", "-6h", "yesterday", "2026-08-30", "6.5h"] {
            let error = time(text).unwrap_err();
            assert!(
                error.contains("RFC 3339") || error.contains("relative"),
                "{text}: {error}"
            );
        }
    }

    #[test]
    fn office_help_lists_every_known_code() {
        let help = office_long_help("NWS office");
        for code in OfficeId::KNOWN {
            assert!(help.contains(code), "{code}");
        }
        assert!(help.contains("WRH"));
    }
}
