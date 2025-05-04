use assert_cmd::Command;

// Ignore this test for now since the date needs to be updated dynamically
#[test]
#[ignore]
fn test_aviation_cwa_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("cwa");
    cmd.arg("--cwsu-id");
    cmd.arg("ZLA");
    cmd.arg("--date");
    cmd.arg("2025-04-18");
    cmd.arg("--sequence");
    cmd.arg("101");
    cmd.assert().success();
}

#[test]
fn test_aviation_cwa_failure_sequence_too_low() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("cwa");
    cmd.arg("--cwsu-id");
    cmd.arg("ZLA");
    cmd.arg("--date");
    cmd.arg("2025-04-18");
    cmd.arg("--sequence");
    cmd.arg("99");
    cmd.assert().failure();
}

#[test]
fn test_aviation_cwas_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("cwas");
    cmd.arg("--cwsu-id");
    cmd.arg("ZLA");
    cmd.assert().success();
}

#[test]
fn test_aviation_cwsu_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("cwsu");
    cmd.arg("--cwsu-id");
    cmd.arg("ZLA");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_atsu_only_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.arg("--atsu");
    cmd.arg("KKCI");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_atsu_and_date_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.arg("--atsu");
    cmd.arg("KKCI");
    cmd.arg("--date");
    cmd.arg("2025-04-19");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_atsu_and_start_and_end_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.arg("--atsu");
    cmd.arg("KKCI");
    cmd.arg("--start");
    cmd.arg("2025-04-19T00:01:00+00:00");
    cmd.arg("--end");
    cmd.arg("2025-04-19T01:55:00+00:00");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_sequence_only_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.arg("--sequence");
    cmd.arg("52C");
    cmd.assert().success();
}

#[test]
#[ignore]
fn test_aviation_sigmet_success() {
    let mut cmd = Command::cargo_bin("noaa_weather_cli").unwrap();
    cmd.arg("aviation");
    cmd.arg("sigmet");
    cmd.arg("--date");
    cmd.arg("2025-04-19");
    cmd.arg("--atsu");
    cmd.arg("KKCI");
    cmd.arg("--time");
    cmd.arg("0001");
    cmd.assert().success();
}
