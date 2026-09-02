use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_radar_data_queue_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radar")
        .arg("data-queue")
        .arg("--host")
        .arg("rds")
        .arg("--station")
        .arg("KIWA");
    cmd.assert().success();
}

#[test]
fn test_radar_data_queue_rejects_zero_limit() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["radar", "data-queue", "--host", "rds", "--limit", "0"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("invalid value '0' for '--limit <LIMIT>'")
            && stderr.contains("0 is not in 1..=50000"),
        "{stderr}"
    );
}

#[test]
fn test_radar_data_queue_rejects_limit_above_maximum() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["radar", "data-queue", "--host", "rds", "--limit", "50001"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("invalid value '50001' for '--limit <LIMIT>'")
            && stderr.contains("50001 is not in 1..=50000"),
        "{stderr}"
    );
}

#[test]
fn test_radar_server_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radar").arg("server").arg("--id").arg("ldm1");
    cmd.assert().success();
}

#[test]
fn test_radar_servers_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radar").arg("servers");
    cmd.assert().success();
}

#[test]
fn test_radar_station_success() {
    // NOAA's station list mixes 4-character NEXRAD sites with 5-character
    // profilers such as HWPA2; both shapes must reach the API.
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["radar", "station", "--station-id", "hwpa2"]);
    cmd.assert().success();
}

#[test]
fn test_radar_station_alarms_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radar")
        .arg("station-alarms")
        .arg("--station-id")
        .arg("KABQ");
    cmd.assert().success();
}

#[test]
fn test_radar_stations_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radar").arg("stations");
    cmd.assert().success();
}

#[test]
fn test_radar_stations_with_type_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("radar")
        .arg("stations")
        .arg("--station-type")
        .arg("WSR-88D");
    cmd.assert().success();
}

#[test]
fn test_radar_spgds_supports_table_json_and_published_filter() {
    let mut table = Command::new(cargo_bin!("noaa-weather"));
    let output = table.args(["radar", "spgds"]).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Data Flow"));

    let mut json = Command::new(cargo_bin!("noaa-weather"));
    let output = json
        .args([
            "radar",
            "spgds",
            "--published",
            "2026-08-30T00:00:00Z/2026-08-30T01:00:00Z",
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(value["@graph"].is_array());
}

#[test]
fn test_radar_spgds_error_preserves_operation_and_http_detail() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    let output = cmd
        .args(["radar", "spgds", "--published", "PT1H"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("getting radar SPGDS telemetry"), "{stderr}");
    assert!(stderr.contains("HTTP 400 Bad Request"), "{stderr}");
    assert!(stderr.contains("query.published"), "{stderr}");
}

#[test]
fn test_radar_spgds_rejects_malformed_interval_as_usage_error() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    let output = cmd
        .args(["radar", "spgds", "--published", "PT1H/NOW"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid interval"), "{stderr}");
}

#[test]
fn test_radar_station_rejects_malformed_station_id() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    let output = cmd
        .args(["radar", "station", "--station-id", "KAB"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid radar station id"), "{stderr}");
}
