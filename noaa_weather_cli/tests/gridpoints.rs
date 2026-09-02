use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_gridpoints_forecast_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["gridpoints", "forecast", "PSR/159,58"]);
    cmd.assert().success();
}

#[test]
fn test_gridpoints_stations_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["gridpoints", "stations", "PSR/159,58", "--limit", "10"]);
    cmd.assert().success();
}

#[test]
fn test_gridpoints_gridpoint_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["gridpoints", "gridpoint", "PSR/159,58"]);
    cmd.assert().success();
}

#[test]
fn test_gridpoints_hourly_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "gridpoints",
        "forecast-hourly",
        "psr/159,58",
        "--units",
        "si",
    ]);
    cmd.assert().success();
}

#[test]
fn test_gridpoints_rejects_malformed_gridpoint_as_usage_error() {
    for (value, reason) in [
        ("PSR/159", "must be OFFICE/x,y"),
        ("PSR/-1,58", "grid x and y must be whole numbers"),
        ("P/159,58", "office code must be"),
    ] {
        let mut cmd = Command::new(cargo_bin!("noaa-weather"));
        cmd.args(["gridpoints", "forecast", value]);
        let output = cmd.output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{value}: {stderr}");
        assert!(stderr.contains("invalid gridpoint"), "{value}: {stderr}");
        assert!(stderr.contains(reason), "{value}: {stderr}");
    }
}

#[test]
fn test_gridpoints_rejects_removed_office_and_coordinate_flags() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "gridpoints",
        "forecast",
        "--forecast-office-id",
        "PSR",
        "--x",
        "159",
        "--y",
        "58",
    ]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unexpected argument '--forecast-office-id'"),
        "{stderr}"
    );
}
