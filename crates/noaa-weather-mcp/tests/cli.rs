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
        .stdout(
            predicates::str::contains("stdio")
                .and(predicates::str::contains("OAuth").not())
                .and(predicates::str::contains("--max-response-bytes <BYTES>"))
                .and(predicates::str::contains(
                    "NOAA_WEATHER_MCP_MAX_RESPONSE_BYTES",
                ))
                .and(predicates::str::contains("[default: 10485760]")),
        );
}

#[test]
fn environment_response_limit_is_accepted_without_starting_stdio() {
    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .env("NOAA_WEATHER_MCP_MAX_RESPONSE_BYTES", "2048")
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("noaa-weather-mcp"));
}

#[test]
fn command_line_response_limit_overrides_an_invalid_environment_value() {
    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .env("NOAA_WEATHER_MCP_MAX_RESPONSE_BYTES", "invalid")
        .args(["--max-response-bytes", "2048", "completions", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("noaa-weather-mcp"));
}

#[test]
fn invalid_response_limits_are_usage_errors() {
    for invalid in ["0", "-1", "invalid", "999999999999999999999999999999999"] {
        Command::cargo_bin("noaa-weather-mcp")
            .expect("binary must build")
            .args(["--max-response-bytes", invalid, "completions", "bash"])
            .assert()
            .code(2)
            .stderr(predicates::str::contains("invalid value"));
    }
}

#[test]
fn invalid_environment_response_limit_is_a_usage_error() {
    Command::cargo_bin("noaa-weather-mcp")
        .expect("binary must build")
        .env("NOAA_WEATHER_MCP_MAX_RESPONSE_BYTES", "0")
        .args(["completions", "bash"])
        .assert()
        .code(2)
        .stderr(predicates::str::contains("invalid value"));
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

#[test]
fn manifest_has_no_summary_crate_dependency() {
    let manifest = std::fs::read_to_string(format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")))
        .expect("MCP manifest must be readable");
    assert!(!manifest.contains("noaa_weather_summary"));
}
