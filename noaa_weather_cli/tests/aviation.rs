//! The `aviation` family, end to end.
//!
//! Two of the live tests here used to carry April-2025 dates NOAA stopped
//! serving, and were `#[ignore]`d rather than fixed. Both now resolve the
//! volatile half at run time by running the binary, the way `alerts alert`
//! does, so they check the route they are named after instead of nothing.

mod common;

use std::process::Output;

use common::noaa_weather;
use common::runner::{family, hermetic, live};
use serde_json::Value;

#[tokio::test]
async fn every_aviation_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("aviation")).await;
}

#[test]
fn test_aviation_live_noaa_answers_every_tabled_invocation() {
    live(family("aviation"));
}

/// Runs the binary and fails with its own message if it did not exit 0.
fn succeeding(arguments: &[&str]) -> Output {
    let output = noaa_weather()
        .args(arguments)
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        output.status.code(),
        Some(0),
        "`{}` failed: {}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn json(output: &Output, what: &str) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "`{what}` did not emit JSON: {error}\n{}",
            String::from_utf8_lossy(&output.stdout)
        )
    })
}

/// The feature collection a listing returned, or a failure naming the route.
///
/// NOAA legitimately has nothing to report for a quiet CWSU, and this is
/// what separates that from a route that stopped working: the envelope has
/// to be a well-formed, decoded `features` array either way, and only the
/// emptiness is allowed to vary.
fn features(document: &Value, what: &str) -> Vec<Value> {
    document["features"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` returned no `features` array: {document}"))
        .clone()
}

/// Every CWSU asked for a current advisory, most-active first.
///
/// One centre is often quiet, so asking several is what keeps the
/// single-CWA route covered on an ordinary day rather than only when the
/// weather cooperates.
const CWSUS: &[&str] = &["ZAB", "ZKC", "ZFW", "ZDC", "ZLA", "ZNY", "ZOA", "ZMA"];

/// Fetches one CWA by the date and sequence of a current one.
///
/// The predecessor carried `--date 2025-04-18 --sequence 101` and was
/// ignored, because NOAA keeps a CWA for a couple of hours.
#[test]
fn a_cwa_is_fetched_by_a_sequence_resolved_at_run_time() {
    let mut advisories = Vec::new();
    let mut asked = Vec::new();
    for cwsu in CWSUS {
        let what = format!("aviation cwas --cwsu-id {cwsu} --json");
        let listing = succeeding(&["aviation", "cwas", "--cwsu-id", cwsu, "--json"]);
        // Every listing is decoded and held to the envelope, whether or not
        // it is the one that supplies the sequence.
        advisories = features(&json(&listing, &what), &what);
        asked.push(*cwsu);
        if !advisories.is_empty() {
            break;
        }
    }

    let Some(first) = advisories.first() else {
        // Not a silent skip: every listing answered, decoded, and carried a
        // well-formed empty `features` array. That is a real answer about
        // quiet centres, and saying so is what stops this from reading as a
        // pass that checked the single-CWA route.
        eprintln!(
            "aviation cwas returned a well-formed empty collection for every \
             one of {asked:?}, so no centre had a current advisory and there \
             was no sequence to fetch. The listing endpoint was checked for \
             all {} of them; the single-CWA route was not.",
            asked.len()
        );
        return;
    };

    let properties = &first["properties"];
    let cwsu = properties["cwsu"]
        .as_str()
        .unwrap_or_else(|| panic!("the first CWA has no `properties.cwsu`: {first}"));
    let issue_time = properties["issueTime"]
        .as_str()
        .unwrap_or_else(|| panic!("the first CWA has no `properties.issueTime`: {first}"));
    let sequence = properties["sequence"]
        .as_u64()
        .unwrap_or_else(|| panic!("the first CWA has no `properties.sequence`: {first}"));
    let date = issue_time
        .split('T')
        .next()
        .unwrap_or_else(|| panic!("`issueTime` {issue_time:?} has no date part"));
    let sequence = sequence.to_string();

    let fetched = succeeding(&[
        "aviation",
        "cwa",
        "--cwsu-id",
        cwsu,
        "--date",
        date,
        "--sequence",
        &sequence,
        "--json",
    ]);
    let fetched = json(&fetched, "aviation cwa --json");
    assert_eq!(
        fetched["properties"]["sequence"]
            .as_u64()
            .map(|n| n.to_string()),
        Some(sequence.clone()),
        "NOAA answered `aviation cwa --sequence {sequence}` with a different \
         advisory"
    );
}

/// Fetches one SIGMET by the issue time of a current one.
///
/// The predecessor carried `--issued 2025-04-19T00:01:00Z` and was ignored.
/// NOAA addresses a SIGMET by its UTC date and HHMM minute, so the seconds
/// in the resolved timestamp are dropped by the client, not here.
#[test]
fn a_sigmet_is_fetched_by_an_issue_time_resolved_at_run_time() {
    let listing = succeeding(&["aviation", "sigmets", "--json"]);
    let listing = json(&listing, "aviation sigmets --json");
    let sigmets = features(&listing, "aviation sigmets --json");

    let Some(first) = sigmets.first() else {
        // As above: the listing answered and decoded, and its emptiness is
        // NOAA's answer rather than this test declining to check.
        eprintln!(
            "aviation sigmets returned a well-formed empty collection, so \
             there were no current SIGMETs and there was no issue time to \
             fetch. The listing endpoint was checked; the single-SIGMET \
             route was not."
        );
        return;
    };

    // Prefer a three-character unit when the listing has one. NOAA's
    // `ATSUIdentifier` is `^[A-Z]{3,4}$` and about one SIGMET in ten comes
    // from ANC, FAI, HNL or JNU; a 4-only parser rejected identifiers NOAA
    // had just handed us, and taking `features[0]` made that a one-in-ten
    // failure rather than a certain one. Preferring the short form drives
    // the branch that was broken instead of stepping around it.
    let first = sigmets
        .iter()
        .find(|sigmet| {
            sigmet["properties"]["atsu"]
                .as_str()
                .is_some_and(|a| a.len() == 3)
        })
        .unwrap_or(first);

    let properties = &first["properties"];
    let atsu = properties["atsu"]
        .as_str()
        .unwrap_or_else(|| panic!("the first SIGMET has no `properties.atsu`: {first}"));
    let issued = properties["issueTime"]
        .as_str()
        .unwrap_or_else(|| panic!("the first SIGMET has no `properties.issueTime`: {first}"));
    assert!(
        (3..=4).contains(&atsu.len()),
        "NOAA published an ATSU outside its own `^[A-Z]{{3,4}}$`: {atsu:?}"
    );

    let fetched = succeeding(&[
        "aviation", "sigmet", "--atsu", atsu, "--issued", issued, "--json",
    ]);
    let fetched = json(&fetched, "aviation sigmet --json");
    assert_eq!(
        fetched["properties"]["atsu"], properties["atsu"],
        "NOAA answered `aviation sigmet --atsu {atsu} --issued {issued}` with \
         a product from another unit"
    );
}

#[test]
fn test_aviation_cwa_rejects_malformed_date_and_cwsu() {
    let output = noaa_weather()
        .args([
            "aviation",
            "cwa",
            "--cwsu-id",
            "ZLA",
            "--date",
            "2025-13-40",
            "--sequence",
            "101",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("--date"), "{stderr}");

    let output = noaa_weather()
        .args(["aviation", "cwas", "--cwsu-id", "Z"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid CWSU id"), "{stderr}");
}

/// A sequence below 100 is not a CWA sequence, and clap says so before any
/// request is made.
#[test]
fn test_aviation_cwa_rejects_a_sequence_below_the_minimum() {
    let output = noaa_weather()
        .args([
            "aviation",
            "cwa",
            "--cwsu-id",
            "ZLA",
            "--date",
            "2026-09-02",
            "--sequence",
            "99",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("--sequence"), "{stderr}");
}

#[test]
fn test_aviation_sigmet_rejects_removed_date_and_time_flags() {
    let output = noaa_weather()
        .args([
            "aviation",
            "sigmet",
            "--atsu",
            "KKCI",
            "--date",
            "2025-04-19",
            "--time",
            "0001",
        ])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("unexpected argument '--date'"), "{stderr}");
}
