use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_no_args_stations_list_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("list");
    cmd.assert().success();
}

#[test]
fn test_stations_list_command_with_states_filter_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("list");
    cmd.arg("--state");
    cmd.arg("AZ");
    cmd.assert().success();
}

#[test]
fn test_stations_list_command_with_limit_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("list");
    cmd.arg("--state");
    cmd.arg("AZ");
    cmd.arg("--limit");
    cmd.arg("1");
    cmd.assert().success();
}

#[test]
fn test_stations_list_command_with_ids_filter_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("list");
    cmd.arg("--id");
    cmd.arg("KPHX");
    cmd.assert().success();
}

#[test]
fn test_latest_observation_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("latest-observation");
    cmd.arg("--station-id");
    cmd.arg("KPHX");
    cmd.assert().success();
}

#[cfg(feature = "xml")]
#[test]
fn test_stations_tafs_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("terminal-aerodrome-forecasts");
    cmd.arg("--station-id");
    cmd.arg("KPHX");
    cmd.assert().success();
}

#[ignore = "Ignore this test for now since the data needs to be updated dynamically"]
#[cfg(feature = "xml")]
#[test]
fn test_stations_taf_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("terminal-aerodrome-forecast");
    cmd.arg("--station-id");
    cmd.arg("KPHX");
    cmd.arg("--date");
    cmd.arg("2025-05-03");
    cmd.arg("--time");
    cmd.arg("1800");
    cmd.assert().success();
}

#[cfg(not(feature = "xml"))]
#[test]
fn terminal_aerodrome_forecasts_is_rejected_without_xml() {
    let output = Command::new(cargo_bin!("noaa-weather"))
        .args([
            "stations",
            "terminal-aerodrome-forecasts",
            "--station-id",
            "KPHX",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("unrecognized subcommand 'terminal-aerodrome-forecasts'")
    );
}

#[cfg(not(feature = "xml"))]
#[test]
fn terminal_aerodrome_forecast_is_rejected_without_xml() {
    let output = Command::new(cargo_bin!("noaa-weather"))
        .args([
            "stations",
            "terminal-aerodrome-forecast",
            "--station-id",
            "KPHX",
            "--date",
            "2026-08-30",
            "--time",
            "1800",
        ])
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("unrecognized subcommand 'terminal-aerodrome-forecast'")
    );
}
