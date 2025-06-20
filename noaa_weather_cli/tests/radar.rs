use assert_cmd::Command;

#[test]
fn test_radar_data_queue_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("radar")
        .arg("data-queue")
        .arg("--host")
        .arg("rds")
        .arg("--station")
        .arg("KIWA");
    cmd.assert().success();
}

#[test]
fn test_radar_server_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("radar").arg("server").arg("--id").arg("ldm1");
    cmd.assert().success();
}

#[test]
fn test_radar_servers_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("radar").arg("servers");
    cmd.assert().success();
}

#[test]
fn test_radar_station_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("radar")
        .arg("station")
        .arg("--station-id")
        .arg("HWPA2");
    cmd.assert().success();
}

#[test]
fn test_radar_station_alarms_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("radar")
        .arg("station-alarms")
        .arg("--station-id")
        .arg("KABQ");
    cmd.assert().success();
}

#[test]
fn test_radar_stations_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("radar").arg("stations");
    cmd.assert().success();
}

#[test]
fn test_radar_stations_with_type_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("radar")
        .arg("stations")
        .arg("--station-type")
        .arg("WSR-88D");
    cmd.assert().success();
}
