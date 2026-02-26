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
fn test_points_command_stations_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("points");
    cmd.arg("stations");
    cmd.arg("39.7456");
    cmd.arg("--");
    cmd.arg("-97.0892");
    cmd.assert().success();
}

#[test]
fn test_points_command_stations_failure_invalid_point() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("points");
    cmd.arg("stations");
    cmd.arg("test");
    cmd.assert().failure();
}
