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
fn test_stations_observations_accept_relative_times() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "stations",
        "observations",
        "--station-id",
        "KPHX",
        "--start",
        "6h",
        "--end",
        "1h",
        "--limit",
        "3",
    ]);
    cmd.assert().success();
}

#[test]
fn test_stations_reject_malformed_station_id_and_time() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["stations", "metadata", "--id", "K PHX"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid station id"), "{stderr}");

    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "stations",
        "observations",
        "--station-id",
        "KPHX",
        "--start",
        "yesterday",
    ]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("RFC 3339"), "{stderr}");
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

#[test]
fn test_stations_tafs_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("stations");
    cmd.arg("terminal-aerodrome-forecasts");
    cmd.arg("--station-id");
    cmd.arg("KPHX");
    cmd.assert().success();
}

#[test]
fn test_stations_taf_success() {
    let metadata = Command::new(cargo_bin!("noaa-weather"))
        .args([
            "stations",
            "terminal-aerodrome-forecasts",
            "--station-id",
            "KPHX",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        metadata.status.success(),
        "{}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&metadata.stdout).unwrap();
    let id = metadata["@graph"]
        .as_array()
        .and_then(|forecasts| forecasts.first())
        .and_then(|forecast| forecast["id"].as_str())
        .expect("NOAA returned at least one current KPHX TAF identifier");
    let mut segments = id.trim_end_matches('/').rsplit('/');
    let time = segments.next().expect("TAF identifier time segment");
    let date = segments.next().expect("TAF identifier date segment");
    let (hours, minutes) = time.split_at(2);
    let issued = format!("{date}T{hours}:{minutes}:00Z");

    let output = Command::new(cargo_bin!("noaa-weather"))
        .args([
            "stations",
            "terminal-aerodrome-forecast",
            "--station-id",
            "KPHX",
            "--issued",
            &issued,
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let table = String::from_utf8(output.stdout).unwrap();
    assert!(table.contains("KPHX"));
    assert!(table.contains("Report state"));
}
