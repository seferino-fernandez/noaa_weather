use assert_cmd::Command;

#[test]
fn test_zones_list_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("zones");
    cmd.arg("list");
    cmd.assert().success();
}

#[test]
fn test_zones_list_with_area_state_filter_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("zones");
    cmd.arg("list");
    cmd.arg("--area");
    cmd.arg("AZ");
    cmd.assert().success();
}

#[test]
fn test_zones_get_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("zones");
    cmd.arg("get");
    cmd.arg("--zone-id");
    cmd.arg("AZZ543");
    cmd.arg("--type");
    cmd.arg("public");
    cmd.assert().success();
}

#[test]
fn test_zones_forecast_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("zones");
    cmd.arg("forecast");
    cmd.arg("--zone-id");
    cmd.arg("AZZ543");
    cmd.arg("--type");
    cmd.arg("public");
    cmd.assert().success();
}

#[test]
fn test_zones_observations_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("zones");
    cmd.arg("observations");
    cmd.arg("--zone-id");
    cmd.arg("AZZ543");
    cmd.assert().success();
}

#[test]
fn test_zones_stations_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("zones");
    cmd.arg("stations");
    cmd.arg("--zone-id");
    cmd.arg("AZZ543");
    cmd.assert().success();
}
