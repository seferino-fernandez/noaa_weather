use assert_cmd::Command;

#[test]
fn test_weather_command_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("weather");
    cmd.arg("city");
    cmd.arg("phoenix");
    cmd.arg("az");
    cmd.assert().success();
}

#[test]
fn test_weather_command_failure_invalid_city() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("weather");
    cmd.arg("city");
    cmd.arg("test");
    cmd.assert().failure();
}
