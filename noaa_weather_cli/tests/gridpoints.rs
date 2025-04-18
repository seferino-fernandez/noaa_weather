use assert_cmd::Command;

#[test]
fn test_gridpoints_forecast_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("gridpoints");
    cmd.arg("forecast");
    cmd.arg("--office-id");
    cmd.arg("PSR");
    cmd.arg("--x");
    cmd.arg("159");
    cmd.arg("--y");
    cmd.arg("58");
    cmd.assert().success();
}

#[test]
fn test_gridpoints_stations_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("gridpoints");
    cmd.arg("stations");
    cmd.arg("--office-id");
    cmd.arg("PSR");
    cmd.arg("--x");
    cmd.arg("159");
    cmd.arg("--y");
    cmd.arg("58");
    cmd.arg("--limit");
    cmd.arg("10");
    cmd.assert().success();
}

#[test]
fn test_gridpoints_gridpoint_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("gridpoints");
    cmd.arg("gridpoint");
    cmd.arg("--office-id");
    cmd.arg("PSR");
    cmd.arg("--x");
    cmd.arg("159");
    cmd.arg("--y");
    cmd.arg("58");
    cmd.assert().success();
}

#[test]
fn test_gridpoints_hourly_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("gridpoints");
    cmd.arg("hourly");
    cmd.arg("--office-id");
    cmd.arg("PSR");
    cmd.arg("--x");
    cmd.arg("159");
    cmd.arg("--y");
    cmd.arg("58");
    cmd.assert().success();
}
