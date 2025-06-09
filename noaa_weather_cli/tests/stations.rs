use assert_cmd::Command;

#[test]
fn test_no_args_stations_list_command_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("stations");
    cmd.arg("list");
    cmd.assert().success();
}

#[test]
fn test_stations_list_command_with_states_filter_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("stations");
    cmd.arg("list");
    cmd.arg("--state");
    cmd.arg("AZ");
    cmd.assert().success();
}

#[test]
fn test_stations_list_command_with_limit_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("stations");
    cmd.arg("list");
    cmd.arg("--state");
    cmd.arg("AZ");
    cmd.arg("--limit");
    cmd.arg("1");
    cmd.assert().success();
}

#[test]
fn test_stations_list_command_with_ids_filter_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("stations");
    cmd.arg("list");
    cmd.arg("--id");
    cmd.arg("KPHX");
    cmd.assert().success();
}

#[test]
fn test_latest_observation_command_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("stations");
    cmd.arg("latest-observation");
    cmd.arg("--station-id");
    cmd.arg("KPHX");
    cmd.assert().success();
}

#[test]
fn test_stations_tafs_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("stations");
    cmd.arg("terminal-aerodrome-forecasts");
    cmd.arg("--station-id");
    cmd.arg("KPHX");
    cmd.assert().success();
}

#[ignore = "Ignore this test for now since the data needs to be updated dynamically"]
#[test]
fn test_stations_taf_success() {
    let mut cmd = Command::cargo_bin("noaa-weather").unwrap();
    cmd.arg("stations");
    cmd.arg("terminal-aerodrome-forecast");
    cmd.arg("--station-id");
    cmd.arg("KPHX");
    cmd.arg("--date");
    cmd.arg("2025-05-03");
    cmd.arg("--time");
    cmd.arg("1800");
    cmd.assert().success();
}
