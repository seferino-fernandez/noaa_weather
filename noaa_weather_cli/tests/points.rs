use assert_cmd::cargo::*;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_points_command_success() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["points", "metadata", "39.7456,-97.0892"]);
    cmd.assert().success();
}

#[test]
fn test_points_command_failure_invalid_point() {
    for value in ["test", "39.7456", "91,-97.0892", "39.7456,-181"] {
        let mut cmd = Command::new(cargo_bin!("noaa-weather"));
        cmd.args(["points", "metadata", value]);
        let output = cmd.output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{value}: {stderr}");
        assert!(stderr.contains("invalid coordinates"), "{value}: {stderr}");
    }
}

#[test]
fn test_points_command_rejects_two_positional_values() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["points", "metadata", "39.7456", "--", "-97.0892"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unexpected argument '-97.0892'"),
        "{stderr}"
    );
}

#[test]
fn test_points_command_rejects_removed_stations_subcommand() {
    let mut cmd = Command::new(cargo_bin!("noaa-weather"));
    cmd.args(["points", "stations", "33.4484,-112.0740"]);
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(
        stderr.contains("unrecognized subcommand 'stations'"),
        "{stderr}"
    );
}
