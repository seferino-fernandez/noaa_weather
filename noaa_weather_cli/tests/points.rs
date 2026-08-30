use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_points_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("points");
    cmd.arg("metadata");
    cmd.arg("39.7456");
    cmd.arg("--");
    cmd.arg("-97.0892");
    cmd.assert().success();
}

#[test]
fn test_points_command_failure_invalid_point() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("points");
    cmd.arg("metadata");
    cmd.arg("test");
    cmd.assert().failure();
}

#[test]
fn test_points_command_rejects_removed_stations_subcommand() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["points", "stations", "33.4484", "--", "-112.0740"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unrecognized subcommand 'stations'"),
        "{stderr}"
    );
}
