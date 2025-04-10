use assert_cmd::Command;

#[test]
fn test_points_command_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("points");
    cmd.arg("metadata");
    cmd.arg("39.7456,-97.0892");
    cmd.assert().success();
}

#[test]
fn test_points_command_failure_invalid_point() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("points");
    cmd.arg("metadata");
    cmd.arg("test");
    cmd.assert().failure();
}

#[test]
fn test_points_command_stations_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("points");
    cmd.arg("stations");
    cmd.arg("39.7456,-97.0892");
    cmd.assert().success();
}

#[test]
fn test_points_command_stations_failure_invalid_point() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("points");
    cmd.arg("stations");
    cmd.arg("test");
    cmd.assert().failure();
}
