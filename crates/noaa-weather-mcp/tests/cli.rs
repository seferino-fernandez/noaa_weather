//! Artifact-generation smoke tests for the MCP binary.

use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt as _;

fn assert_version(flag: &str) {
    let expected = format!(
        "noaa-weather-mcp {} (api.weather.gov spec {})\n",
        env!("CARGO_PKG_VERSION"),
        noaa_weather_client::API_SPEC_VERSION
    );

    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .arg(flag)
        .assert()
        .success()
        .stdout(expected)
        .stderr("");
}

#[test]
fn long_version_reports_package_and_api_spec_versions_without_starting_stdio() {
    assert_version("--version");
}

#[test]
fn short_version_reports_package_and_api_spec_versions_without_starting_stdio() {
    assert_version("-V");
}

#[test]
fn help_describes_the_stdio_only_server_without_oauth() {
    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("stdio").and(predicates::str::contains("OAuth").not()));
}

#[test]
fn static_bash_completions_are_generated() {
    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("noaa-weather-mcp"));
}

#[test]
fn static_nushell_completions_are_generated() {
    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .args(["completions", "nushell"])
        .assert()
        .success()
        .stdout(predicates::str::contains("export extern noaa-weather-mcp"));
}

#[test]
fn dynamic_completion_registration_is_generated() {
    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .env("COMPLETE", "zsh")
        .assert()
        .success()
        .stdout(predicates::str::contains("COMPLETE"));
}

#[test]
fn man_pages_are_generated() {
    let directory = tempfile::tempdir().expect("temporary directory must be created");

    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .arg("man")
        .arg(directory.path())
        .assert()
        .success();

    assert!(directory.path().join("noaa-weather-mcp.1").exists());
}
