use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
#[ignore = "Ignore this test for now since the date needs to be updated dynamically"]
fn test_aviation_cwa_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
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
fn test_aviation_cwa_rejects_malformed_date_and_cwsu() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "aviation",
        "cwa",
        "--cwsu-id",
        "ZLA",
        "--date",
        "2025-13-40",
        "--sequence",
        "101",
    ]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("--date"), "{stderr}");

    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["aviation", "cwas", "--cwsu-id", "Z"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid CWSU id"), "{stderr}");
}

#[test]
fn test_aviation_cwa_failure_sequence_too_low() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
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
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("aviation");
    cmd.arg("cwas");
    cmd.arg("--cwsu-id");
    cmd.arg("ZLA");
    cmd.assert().success();
}

#[test]
fn test_aviation_cwsu_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("aviation");
    cmd.arg("cwsu");
    cmd.arg("--cwsu-id");
    cmd.arg("ZLA");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_atsu_only_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.arg("--atsu");
    cmd.arg("KKCI");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_atsu_and_date_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
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
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
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
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("aviation");
    cmd.arg("sigmets");
    cmd.arg("--sequence");
    cmd.arg("52C");
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmets_relative_start_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["aviation", "sigmets", "--atsu", "KKCI", "--start", "6h"]);
    cmd.assert().success();
}

#[test]
fn test_aviation_sigmet_rejects_removed_date_and_time_flags() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "aviation",
        "sigmet",
        "--atsu",
        "KKCI",
        "--date",
        "2025-04-19",
        "--time",
        "0001",
    ]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("unexpected argument '--date'"), "{stderr}");
}

#[test]
#[ignore = "Ignore this test for now since the issue time needs to be updated dynamically"]
fn test_aviation_sigmet_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args([
        "aviation",
        "sigmet",
        "--atsu",
        "KKCI",
        "--issued",
        "2025-04-19T00:01:00Z",
    ]);
    cmd.assert().success();
}
