use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_zones_list_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("zones");
    cmd.arg("list");
    cmd.assert().success();
}

#[test]
fn test_zones_list_with_area_state_filter_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("zones");
    cmd.arg("list");
    cmd.arg("--area");
    cmd.arg("AZ");
    cmd.assert().success();
}

#[test]
fn test_zones_metadata_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("zones");
    cmd.arg("metadata");
    cmd.arg("--id");
    cmd.arg("AZZ543");
    cmd.arg("--type");
    cmd.arg("public");
    cmd.assert().success();
}

#[test]
fn test_zones_forecast_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("zones");
    cmd.arg("forecast");
    cmd.arg("--id");
    cmd.arg("AZZ543");
    cmd.arg("--type");
    cmd.arg("public");
    cmd.assert().success();
}

#[test]
fn test_zones_observations_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("zones");
    cmd.arg("observations");
    cmd.arg("--id");
    cmd.arg("AZZ543");
    cmd.assert().success();
}

#[test]
fn test_zones_stations_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("zones");
    cmd.arg("stations");
    cmd.arg("--id");
    cmd.arg("AZZ543");
    cmd.assert().success();
}
