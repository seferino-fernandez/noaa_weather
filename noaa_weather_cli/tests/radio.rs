use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_radio_station_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radio");
    cmd.arg("station");
    cmd.arg("KEC94");
    cmd.assert().success();
}

#[test]
fn test_radio_point_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radio");
    cmd.arg("point");
    cmd.arg("33.4484");
    cmd.arg("--");
    cmd.arg("-112.0740");
    cmd.assert().success();
}

#[test]
fn test_radio_station_failure_missing_arg() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radio");
    cmd.arg("station");
    cmd.assert().failure();
}
