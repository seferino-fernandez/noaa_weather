use assert_cmd::Command;

#[test]
fn test_radar_queue_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("radar")
        .arg("queue")
        .arg("rds")
        .arg("--station")
        .arg("KIWA");
    cmd.assert().success();
}

#[test]
fn test_radar_server_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("radar").arg("server").arg("ldm1");
    cmd.assert().success();
}

#[test]
fn test_radar_servers_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("radar").arg("servers");
    cmd.assert().success();
}

#[test]
fn test_radar_station_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("radar").arg("station").arg("HWPA2");
    cmd.assert().success();
}

#[test]
fn test_radar_station_alarms_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("radar").arg("station-alarms").arg("KABQ");
    cmd.assert().success();
}

#[test]
fn test_radar_stations_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("radar").arg("stations");
    cmd.assert().success();
}

#[test]
fn test_radar_stations_with_type_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("radar")
        .arg("stations")
        .arg("--stationType")
        .arg("WSR-88D");
    cmd.assert().success();
}
