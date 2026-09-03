//! Color policy against a real terminal.
//!
//! comfy-table decides styling from this process's stdout, so no in-process
//! test can tell "the destination is not a terminal" apart from "styling was
//! switched off": under a test harness stdout is always a pipe and both
//! answers look the same. The only honest check runs the built binary on a
//! pty and reads the bytes it wrote.
//!
//! The responses come from a `wiremock` server rather than NOAA, so these run
//! in the normal suite. They need util-linux `script` on the machine.

mod common;

use std::process::Command;

use assert_cmd::cargo::cargo_bin;
use common::fixtures::{ALERT_COUNT, GEO_JSON, JSON_LD, ZONE_LIST};
use common::strip_noaa_environment;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Runs the built binary under a pty and returns everything it wrote,
/// escapes included.
///
/// util-linux `script` is the portable-enough way to get a pty without a new
/// dependency: `-q` drops its own banner, `-e` returns the command's exit
/// status, and `/dev/null` throws away the typescript file.
fn on_a_pty(base_url: &str, arguments: &str, no_color: bool) -> String {
    let binary = cargo_bin!("noaa-weather");
    let mut command = Command::new("script");
    command
        .arg("-qec")
        .arg(format!(
            "{} {arguments} --base-url {base_url}",
            binary.display()
        ))
        .arg("/dev/null");
    strip_noaa_environment(&mut command);
    if no_color {
        command.env("NO_COLOR", "1");
    } else {
        command.env_remove("NO_COLOR");
    }

    let output = command
        .output()
        .expect("util-linux `script` must be installed to check terminal color policy");
    assert!(
        output.status.success(),
        "{arguments} failed on a pty: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn escapes(text: &str) -> usize {
    text.matches('\u{1b}').count()
}

/// Answers both commands below with captured NOAA responses.
async fn fixture_server() -> MockServer {
    let server = MockServer::start().await;
    for (route, body, media) in [
        ("/alerts/active/count", ALERT_COUNT, JSON_LD),
        ("/zones/land", ZONE_LIST, GEO_JSON),
    ] {
        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, media))
            .mount(&server)
            .await;
    }
    server
}

/// Counts the escapes one run wrote, keeping the blocking pty off the
/// runtime's worker thread.
async fn escapes_on_a_pty(server: &MockServer, arguments: &'static str, no_color: bool) -> usize {
    let base_url = server.uri();
    let written = tokio::task::spawn_blocking(move || on_a_pty(&base_url, arguments, no_color))
        .await
        .expect("the pty task must not panic");
    escapes(&written)
}

/// `alerts count` always renders a bold title, and `zones list` always
/// renders bold headers, so both directions of each assertion are real.
const PORTED: &str = "alerts count";
const UN_PORTED: &str = "zones list --type land --area MI";

#[tokio::test]
async fn no_color_silences_a_ported_family_on_a_terminal() {
    let server = fixture_server().await;
    assert!(
        escapes_on_a_pty(&server, PORTED, false).await > 0,
        "a terminal without NO_COLOR must be styled, or this test proves nothing"
    );
    assert_eq!(
        escapes_on_a_pty(&server, PORTED, true).await,
        0,
        "NO_COLOR must silence every escape, colors and attributes alike"
    );
}

/// The same policy has to reach the families that still build their own
/// tables; that is the bug this fix exists for.
#[tokio::test]
async fn no_color_silences_an_un_ported_family_on_a_terminal() {
    let server = fixture_server().await;
    assert!(
        escapes_on_a_pty(&server, UN_PORTED, false).await > 0,
        "a terminal without NO_COLOR must be styled, or this test proves nothing"
    );
    assert_eq!(
        escapes_on_a_pty(&server, UN_PORTED, true).await,
        0,
        "NO_COLOR must silence every escape, colors and attributes alike"
    );
}

/// `--color never` is the explicit form of the same request, and `--color
/// always` overrides NO_COLOR because the caller asked for it by name.
#[tokio::test]
async fn the_color_flag_overrides_both_the_terminal_and_no_color() {
    let server = fixture_server().await;
    assert_eq!(
        escapes_on_a_pty(&server, "alerts count --color never", false).await,
        0,
        "--color never must silence a terminal"
    );
    assert!(
        escapes_on_a_pty(&server, "alerts count --color always", true).await > 0,
        "--color always must beat NO_COLOR"
    );
}
