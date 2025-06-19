use assert_cmd::Command;

#[test]
fn test_alerts_command_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("active");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_list_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("list");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_list_status_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("list");
    cmd.arg("--status");
    cmd.arg("actual");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_failure_invalid_command() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_command_area_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("area");
    cmd.arg("--area");
    cmd.arg("AZ");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_area_failure_invalid_area() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("area");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_command_count_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("count");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_marine_region_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("marine-region");
    cmd.arg("--marine-region");
    cmd.arg("PI");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_marine_region_failure_invalid_region() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("marine-region");
    cmd.arg("--marine-region");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
#[ignore = "Ignore this test for now since the alert id needs to be updated"]
fn test_alerts_command_get_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("alert");
    cmd.arg("--id");
    cmd.arg("urn:oid:2.49.0.1.840.0.dcc6cd9527d1f8732519ea87f13d3810e9ef672c.001.1");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_types_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("types");
    cmd.assert().success();
}

#[test]
fn test_alerts_zone_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("alerts");
    cmd.arg("zone");
    cmd.arg("--zone-id");
    cmd.arg("AZC013");
    cmd.assert().success();
}
