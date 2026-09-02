use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_alerts_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("active");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_list_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("list");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_list_status_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("list");
    cmd.arg("--status");
    cmd.arg("actual");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_list_rejects_removed_active_option() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["alerts", "list", "--active", "true"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unexpected argument '--active'"),
        "{stderr}"
    );
}

#[test]
fn test_alerts_command_failure_invalid_command() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_command_area_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("area");
    cmd.arg("--area");
    cmd.arg("AZ");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_area_failure_invalid_area() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("area");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_active_accepts_point_and_zone_filters() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["alerts", "active", "--point", "39.7456,-97.0892"]);
    cmd.assert().success();

    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["alerts", "active", "--zone", "AZC013,azz540"]);
    cmd.assert().success();
}

#[test]
fn test_alerts_list_accepts_relative_times_and_rejects_malformed_zone() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "alerts", "list", "--start", "6h", "--end", "1h", "--limit", "5",
    ]);
    cmd.assert().success();

    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["alerts", "zone", "--zone-id", "CAZ 043"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid zone id"), "{stderr}");
}

#[test]
fn test_alerts_command_count_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("count");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_marine_region_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("marine-region");
    cmd.arg("--marine-region");
    cmd.arg("PI");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_marine_region_failure_invalid_region() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("marine-region");
    cmd.arg("--marine-region");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
#[ignore = "Ignore this test for now since the alert id needs to be updated"]
fn test_alerts_command_get_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("alert");
    cmd.arg("--id");
    cmd.arg("urn:oid:2.49.0.1.840.0.dcc6cd9527d1f8732519ea87f13d3810e9ef672c.001.1");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_types_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("types");
    cmd.assert().success();
}

#[test]
fn test_alerts_zone_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("alerts");
    cmd.arg("zone");
    cmd.arg("--zone-id");
    cmd.arg("AZC013");
    cmd.assert().success();
}
