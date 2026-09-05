//! Completion and manpage generation through the built CLI.

use assert_cmd::Command;

#[test]
fn static_bash_completions_are_generated_without_building_a_client() {
    Command::cargo_bin("noaa-weather")
        .expect("binary must build")
        .env("NOAA_WEATHER_USER_AGENT", "")
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicates::str::contains("noaa-weather"));
}

#[test]
fn static_nushell_completions_are_generated() {
    Command::cargo_bin("noaa-weather")
        .expect("binary must build")
        .args(["completions", "nushell"])
        .assert()
        .success()
        .stdout(predicates::str::contains("export extern noaa-weather"));
}

#[test]
fn dynamic_completion_registration_is_generated() {
    Command::cargo_bin("noaa-weather")
        .expect("binary must build")
        .env("COMPLETE", "zsh")
        .assert()
        .success()
        .stdout(predicates::str::contains("COMPLETE"));
}

#[test]
fn man_pages_are_generated_without_building_a_client() {
    let directory = tempfile::tempdir().expect("temporary directory must be created");

    Command::cargo_bin("noaa-weather")
        .expect("binary must build")
        .env("NOAA_WEATHER_USER_AGENT", "")
        .arg("man")
        .arg(directory.path())
        .assert()
        .success();

    assert!(directory.path().join("noaa-weather.1").exists());
    assert!(directory.path().join("noaa-weather-alerts.1").exists());
}
