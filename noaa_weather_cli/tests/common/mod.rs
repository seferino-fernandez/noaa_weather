//! Scaffolding shared by the CLI's integration tests.
//!
//! Each test binary pulls this in with `mod common;`, so anything it does not
//! use looks dead to that binary; the allow below is what keeps one suite's
//! unused helper from failing the lint gate for all of them.

#![allow(dead_code)]

pub mod fixtures;
pub mod table;

use std::ffi::OsString;
use std::process::Command;

use assert_cmd::cargo::cargo_bin;

/// Builds a command for the built binary with every `NOAA_WEATHER_*` variable
/// removed from the child's environment.
///
/// The child inherits this process's environment, so a developer with
/// `NOAA_WEATHER_BASE_URL` exported would silently redirect the whole suite,
/// and one with `NOAA_WEATHER_API_KEY` set would hand the key to a mock
/// server. `env_clear` would cover both, and would also take everything else
/// the child inherits — `PATH`, `TMPDIR`, `TERM` — with it; removing only
/// what this program reads is the smaller and more predictable change.
pub fn noaa_weather() -> Command {
    let mut command = Command::new(cargo_bin!("noaa-weather"));
    strip_noaa_environment(&mut command);
    command
}

/// Removes every `NOAA_WEATHER_*` variable from `command`'s environment.
///
/// Separate from [`noaa_weather`] because the pty tests run the binary
/// through `script`, and that wrapper inherits the same variables.
pub fn strip_noaa_environment(command: &mut Command) {
    for name in inherited_noaa_variables() {
        command.env_remove(name);
    }
}

/// Names every `NOAA_WEATHER_*` variable set in this process.
fn inherited_noaa_variables() -> Vec<OsString> {
    std::env::vars_os()
        .map(|(name, _)| name)
        .filter(|name| name.to_string_lossy().starts_with("NOAA_WEATHER_"))
        .collect()
}
