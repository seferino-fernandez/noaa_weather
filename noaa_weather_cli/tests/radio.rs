use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(cargo_bin!("noaa-weather"))
        .args(args)
        .output()
        .expect("run noaa-weather")
}

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
    cmd.args(["radio", "point", "33.4484,-112.0740"]);
    cmd.assert().success();
}

#[test]
fn test_radio_station_rejects_malformed_call_sign() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["radio", "station", "KE C94"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid call sign"), "{stderr}");
}

#[test]
fn test_radio_station_failure_missing_arg() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radio");
    cmd.arg("station");
    cmd.assert().failure();
}

#[test]
fn test_radio_transmitters_support_table_json_and_cursor() {
    let table = run(&["radio", "transmitters"]);
    assert!(
        table.status.success(),
        "{}",
        String::from_utf8_lossy(&table.stderr)
    );
    assert!(String::from_utf8_lossy(&table.stdout).contains("Call Sign"));

    let json = run(&["radio", "transmitters", "--json"]);
    assert!(
        json.status.success(),
        "{}",
        String::from_utf8_lossy(&json.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    let transmitters = value["@graph"].as_array().expect("transmitter graph");
    assert!(transmitters.is_empty() || transmitters[0]["callSign"].is_string());

    if let Some(next) = value["pagination"]["next"].as_str()
        && let Some(cursor) = next.split("cursor=").nth(1)
    {
        let page = run(&["radio", "transmitters", "--cursor", cursor, "--json"]);
        assert!(
            page.status.success(),
            "{}",
            String::from_utf8_lossy(&page.stderr)
        );
        let value: serde_json::Value = serde_json::from_slice(&page.stdout).unwrap();
        assert!(value["@graph"].is_array());
    }
}

#[test]
fn test_radio_transmitter_supports_table_and_json() {
    let table = run(&["radio", "transmitter", "KEC94"]);
    assert!(
        table.status.success(),
        "{}",
        String::from_utf8_lossy(&table.stderr)
    );
    assert!(String::from_utf8_lossy(&table.stdout).contains("KEC94"));

    let json = run(&["radio", "transmitter", "KEC94", "--json"]);
    assert!(
        json.status.success(),
        "{}",
        String::from_utf8_lossy(&json.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(value["callSign"], "KEC94");
}

#[test]
fn test_radio_county_zone_supports_table_and_json() {
    let table = run(&["radio", "zone", "AZC013"]);
    assert!(
        table.status.success(),
        "{}",
        String::from_utf8_lossy(&table.stderr)
    );
    assert!(String::from_utf8_lossy(&table.stdout).contains("Call Sign"));

    let json = run(&["radio", "zone", "AZC013", "--json"]);
    assert!(
        json.status.success(),
        "{}",
        String::from_utf8_lossy(&json.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert!(value["@graph"].is_array());
}
