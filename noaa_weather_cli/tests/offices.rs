use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_offices_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("metadata");
    cmd.arg("--id");
    cmd.arg("PSR");
    cmd.assert().success();
}

#[test]
fn test_offices_command_failure_invalid_office_id() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("metadata");
    cmd.arg("--id");
    cmd.arg("invalid");
    cmd.assert().failure();
}

#[test]
fn test_offices_command_headlines_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("headlines");
    cmd.arg("--id");
    cmd.arg("PSR");
    cmd.assert().success();
}

#[test]
fn test_offices_command_single_headline_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.arg("offices");
    cmd.arg("headline");
    cmd.arg("--id");
    cmd.arg("PSR");
    cmd.arg("--headline-id");
    cmd.arg("593627f70073a49e2483c3e0bf4f8221");
    cmd.assert().success();
}
