use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::path::Path;
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(cargo_bin!("noaa-weather"))
        .args(args)
        .output()
        .expect("run noaa-weather")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

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
fn test_offices_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("metadata");
    cmd.arg("--id");
    cmd.arg("PSR");
    cmd.assert().success();
}

#[test]
fn test_regional_headquarters_office_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["offices", "metadata", "--id", "WRH"]);
    cmd.assert().success();
}

#[test]
fn test_offices_command_failure_invalid_office_id() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("metadata");
    cmd.arg("--id");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_offices_command_headlines_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("headlines");
    cmd.arg("--id");
    cmd.arg("PSR");
    cmd.assert().success();
}

#[test]
fn test_offices_command_single_headline_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("headline");
    cmd.arg("--id");
    cmd.arg("PSR");
    cmd.arg("--headline-id");
    cmd.arg("593627f70073a49e2483c3e0bf4f8221");
    cmd.assert().success();
}

#[test]
fn test_office_briefing_supports_table_and_json() {
    let table = run(&["offices", "briefing", "--id", "PSR"]);
    assert_success(&table);
    assert!(String::from_utf8_lossy(&table.stdout).contains("Download"));

    let json = run(&["offices", "briefing", "--id", "PSR", "--json"]);
    assert_success(&json);
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert!(value.get("briefing").is_some());
}

#[test]
fn test_new_office_commands_accept_regional_headquarters_ids() {
    let json = run(&["offices", "briefing", "--id", "WRH", "--json"]);
    assert_success(&json);
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert!(value.get("briefing").is_some());
}

#[test]
fn test_office_weather_stories_support_table_and_json() {
    let table = run(&["offices", "weather-stories", "--id", "PSR"]);
    assert_success(&table);
    assert!(String::from_utf8_lossy(&table.stdout).contains("Alt Text"));

    let json = run(&["offices", "weather-stories", "--id", "PSR", "--json"]);
    assert_success(&json);
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert!(value["stories"].is_array());
}

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
        let missing_output = run(&args);
        assert_eq!(missing_output.status.code(), Some(1));
        assert!(
            String::from_utf8_lossy(&missing_output.stderr).contains("requires --output <PATH>")
        );

        let mut with_json = args;
        with_json.extend(["--output", "unused", "--json"]);
        let json = run(&with_json);
        assert_eq!(json.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&json.stderr).contains("--json cannot be used"));
        assert!(!Path::new("unused").exists());
    }
}

#[test]
fn test_active_office_media_downloads_to_files_when_available() {
    let temp = tempfile::tempdir().unwrap();

    let briefing_json = run(&["offices", "briefing", "--id", "PSR", "--json"]);
    assert_success(&briefing_json);
    let briefing: serde_json::Value = serde_json::from_slice(&briefing_json.stdout).unwrap();
    if let Some(active) = briefing["briefing"].as_object()
        && let Some(download) = active.get("download").and_then(|value| value.as_str())
    {
        let id = id_from_download(download);
        let document_path = temp.path().join("briefing.pdf");
        let document_path_str = document_path.to_str().unwrap();
        let output = run(&[
            "offices",
            "briefing-download",
            "--id",
            "PSR",
            "--document-id",
            id,
            "--output",
            document_path_str,
        ]);
        assert_success(&output);
        assert_nonempty_file(&document_path);

        let latest_path = temp.path().join("latest.pdf");
        let output = run(&[
            "offices",
            "briefing-download-latest",
            "--id",
            "PSR",
            "--output",
            latest_path.to_str().unwrap(),
        ]);
        assert_success(&output);
        assert_nonempty_file(&latest_path);
    }

    let stories_json = run(&["offices", "weather-stories", "--id", "PSR", "--json"]);
    assert_success(&stories_json);
    let stories: serde_json::Value = serde_json::from_slice(&stories_json.stdout).unwrap();
    if let Some(download) = stories["stories"]
        .as_array()
        .and_then(|stories| stories.first())
        .and_then(|story| story["download"].as_str())
    {
        let id = id_from_download(download);
        let image_path = temp.path().join("weather-story-image");
        let output = run(&[
            "offices",
            "weather-story-image",
            "--id",
            "PSR",
            "--story-id",
            id,
            "--output",
            image_path.to_str().unwrap(),
        ]);
        assert_success(&output);
        assert_nonempty_file(&image_path);
    }
}
