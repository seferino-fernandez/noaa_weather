//! The `offices` family, end to end.
//!
//! Three of these leaves return binary bytes and so refuse standard output;
//! the shared runners hand them a temporary file. Their live coverage is
//! conditional on the office having something published, which is what the
//! last test here reports on rather than skips over.

mod common;

use std::path::Path;
use std::process::Output;

use common::noaa_weather;
use common::runner::{family, hermetic, live};
use serde_json::Value;

#[tokio::test]
async fn every_offices_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("offices")).await;
}

#[test]
fn test_offices_live_noaa_answers_every_tabled_invocation() {
    live(family("offices"));
}

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

/// The last path segment of a download URL, which is the document id.
fn id_from_download(download: &str) -> &str {
    download
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .expect("document ID in download URL")
}

fn assert_nonempty_file(path: &Path) {
    let metadata = std::fs::metadata(path).expect("downloaded file metadata");
    assert!(metadata.len() > 0, "download should not be empty");
}

#[test]
fn test_offices_command_failure_invalid_office_id() {
    let output = noaa_weather()
        .args(["offices", "metadata", "--id", "invalid"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid office id"), "{stderr}");
}

#[test]
fn test_offices_help_lists_known_codes_without_restricting() {
    let output = succeeding(&["offices", "metadata", "--help"]);
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("Known forecast offices"), "{help}");
    assert!(help.contains("PSR"), "{help}");
}

/// Fetches one headline by an id resolved at run time.
#[test]
fn a_headline_is_fetched_by_an_id_resolved_at_run_time() {
    let what = "offices headlines --id PSR --json";
    let listing = succeeding(&["offices", "headlines", "--id", "PSR", "--json"]);
    let listing = json(&listing, what);
    let headlines = listing["@graph"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` returned no `@graph` array: {listing}"));

    let Some(first) = headlines.first() else {
        eprintln!(
            "`{what}` returned a well-formed empty `@graph`, so PSR has no \
             current news and there was no headline id to fetch. The listing \
             endpoint was checked; `offices headline` was not."
        );
        return;
    };

    let id = first["id"]
        .as_str()
        .unwrap_or_else(|| panic!("the first headline has no `id`: {first}"));

    let fetched = succeeding(&[
        "offices",
        "headline",
        "--id",
        "PSR",
        "--headline-id",
        id,
        "--json",
    ]);
    let fetched = json(&fetched, "offices headline --json");
    assert_eq!(
        fetched["id"], first["id"],
        "NOAA answered `offices headline --headline-id {id}` with a different \
         headline"
    );
}

/// Binary output has nowhere to go but a file, whatever the caller asked for.
///
/// Both refusals depend on argv alone: no request is made, no file is
/// touched, and `StdoutDestination::validate` never consults whether stdout
/// is a terminal, so they fail identically on every machine. That is a usage
/// error, exit 2. clap cannot express either — `--format` is global and the
/// conflict is with the subcommand — but "clap did not catch it" was never
/// the test; `Error::Invalid` is exit 2 on the same reasoning.
#[test]
fn test_binary_office_commands_require_output_and_reject_json() {
    for args in [
        vec![
            "offices",
            "briefing-download",
            "--id",
            "PSR",
            "--document-id",
            "not-requested",
        ],
        vec!["offices", "briefing-download-latest", "--id", "PSR"],
        vec![
            "offices",
            "weather-story-image",
            "--id",
            "PSR",
            "--story-id",
            "not-requested",
        ],
    ] {
        // Both refusals happen before any request, and a base URL nothing is
        // listening on is what proves it.
        let unreachable = ["--base-url", "http://127.0.0.1:1"];

        let mut missing_output = args.clone();
        missing_output.extend(unreachable);
        let output = noaa_weather().args(&missing_output).output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {stderr}");
        assert!(stderr.contains("requires --output <PATH>"), "{stderr}");

        let mut with_json = args;
        with_json.extend(["--output", "unused", "--json"]);
        with_json.extend(unreachable);
        let output = noaa_weather().args(&with_json).output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{with_json:?}: {stderr}");
        assert!(!Path::new("unused").exists());
    }
}

/// Downloads whatever media PSR has published, and reports it when there is
/// none.
///
/// NOAA answers `briefing/download/latest` with 404 whenever the office has
/// no active briefing, which is most of the time, so this cannot be driven
/// unconditionally. What it must not do is fall through in silence: the
/// listing endpoints are checked either way, and the branch that was not
/// taken is named.
#[test]
fn office_media_downloads_to_files_or_says_there_was_none() {
    let temporary = tempfile::tempdir().unwrap();

    let what = "offices briefing --id PSR --json";
    let briefing = json(
        &succeeding(&["offices", "briefing", "--id", "PSR", "--json"]),
        what,
    );
    assert!(
        briefing.get("briefing").is_some(),
        "`{what}` returned no `briefing` member at all: {briefing}"
    );
    match briefing["briefing"]
        .as_object()
        .and_then(|active| active.get("download"))
        .and_then(Value::as_str)
    {
        Some(download) => {
            let id = id_from_download(download);
            let document = temporary.path().join("briefing.pdf");
            succeeding(&[
                "offices",
                "briefing-download",
                "--id",
                "PSR",
                "--document-id",
                id,
                "--output",
                document.to_str().unwrap(),
            ]);
            assert_nonempty_file(&document);

            let latest = temporary.path().join("latest.pdf");
            succeeding(&[
                "offices",
                "briefing-download-latest",
                "--id",
                "PSR",
                "--output",
                latest.to_str().unwrap(),
            ]);
            assert_nonempty_file(&latest);
        }
        None => eprintln!(
            "`{what}` answered with a well-formed envelope whose `briefing` \
             is null, so PSR has no active briefing and NOAA answers both \
             download routes with 404. The briefing metadata endpoint was \
             checked; `briefing-download` and `briefing-download-latest` \
             were not."
        ),
    }

    let what = "offices weather-stories --id PSR --json";
    let stories = json(
        &succeeding(&["offices", "weather-stories", "--id", "PSR", "--json"]),
        what,
    );
    let stories = stories["stories"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` returned no `stories` array: {stories}"))
        .clone();

    match stories.first().and_then(|story| story["download"].as_str()) {
        Some(download) => {
            let id = id_from_download(download);
            let image = temporary.path().join("weather-story-image");
            succeeding(&[
                "offices",
                "weather-story-image",
                "--id",
                "PSR",
                "--story-id",
                id,
                "--output",
                image.to_str().unwrap(),
            ]);
            assert_nonempty_file(&image);
        }
        None => eprintln!(
            "`{what}` returned a well-formed empty `stories` array, so PSR \
             has published no weather story and there was no image to fetch. \
             The listing endpoint was checked; `weather-story-image` was not."
        ),
    }
}
