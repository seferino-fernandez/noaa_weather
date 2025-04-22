use assert_cmd::Command;

#[test]
fn test_alerts_command_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("active");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_failure_invalid_command() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_command_area_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("area");
    cmd.arg("AZ");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_area_failure_invalid_area() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("area");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_alerts_command_count_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("count");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_region_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("region");
    cmd.arg("PI");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_region_failure_invalid_region() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("region");
    cmd.arg("invalid");
    cmd.assert().failure();
}

// Ignore this test for now since the alert id needs to be updated
#[test]
#[ignore]
fn test_alerts_command_get_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("alert");
    cmd.arg("urn:oid:2.49.0.1.840.0.eb79fad94f63c186cdfb1678251f96b5c628af14.001.1");
    cmd.assert().success();
}

#[test]
fn test_alerts_command_types_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("alerts");
    cmd.arg("types");
    cmd.assert().success();
}
