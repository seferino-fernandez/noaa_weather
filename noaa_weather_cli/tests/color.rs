//! Color policy against a real terminal.
//!
//! comfy-table decides styling from this process's stdout, so no in-process
//! test can tell "the destination is not a terminal" apart from "styling was
//! switched off": under a test harness stdout is always a pipe and both
//! answers look the same. The only honest check runs the built binary on a
//! pty and reads the bytes it wrote.
//!
//! These are `#[ignore]`d because they reach live NOAA — the binary has no
//! base-URL override to point at a fixture server. Run them with
//! `just test-live`.

use std::process::Command;

/// Runs the built binary under a pty and returns everything it wrote,
/// escapes included.
///
/// util-linux `script` is the portable-enough way to get a pty without a new
/// dependency: `-q` drops its own banner, `-e` returns the command's exit
/// status, and `/dev/null` throws away the typescript file.
fn on_a_pty(arguments: &str, no_color: bool) -> String {
    let binary = assert_cmd::cargo::cargo_bin("noaa-weather");
    let mut command = Command::new("script");
    command
        .arg("-qec")
        .arg(format!("{} {arguments}", binary.display()))
        .arg("/dev/null");
    if no_color {
        command.env("NO_COLOR", "1");
    } else {
        command.env_remove("NO_COLOR");
    }

    let output = command.output().expect(
        "util-linux `script` must be installed to check terminal color policy; \
         run `just test-live` on a machine that has it",
    );
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

/// `alerts count` always renders a bold title, and `zones list` always
/// renders bold headers, so both directions of each assertion are real.
const PORTED: &str = "alerts count";
const UN_PORTED: &str = "zones list --type land --area MI";

#[test]
#[ignore = "reaches live NOAA and needs a pty; run with `just test-live`"]
fn no_color_silences_a_ported_family_on_a_terminal() {
    assert!(
        escapes(&on_a_pty(PORTED, false)) > 0,
        "a terminal without NO_COLOR must be styled, or this test proves nothing"
    );
    assert_eq!(
        escapes(&on_a_pty(PORTED, true)),
        0,
        "NO_COLOR must silence every escape, colors and attributes alike"
    );
}

/// The same policy has to reach the families that still build their own
/// tables; that is the bug this fix exists for.
#[test]
#[ignore = "reaches live NOAA and needs a pty; run with `just test-live`"]
fn no_color_silences_an_un_ported_family_on_a_terminal() {
    assert!(
        escapes(&on_a_pty(UN_PORTED, false)) > 0,
        "a terminal without NO_COLOR must be styled, or this test proves nothing"
    );
    assert_eq!(
        escapes(&on_a_pty(UN_PORTED, true)),
        0,
        "NO_COLOR must silence every escape, colors and attributes alike"
    );
}

/// `--color never` is the explicit form of the same request, and `--color
/// always` overrides NO_COLOR because the caller asked for it by name.
#[test]
#[ignore = "reaches live NOAA and needs a pty; run with `just test-live`"]
fn the_color_flag_overrides_both_the_terminal_and_no_color() {
    assert_eq!(
        escapes(&on_a_pty(&format!("{PORTED} --color never"), false)),
        0,
        "--color never must silence a terminal"
    );
    assert!(
        escapes(&on_a_pty(&format!("{PORTED} --color always"), true)) > 0,
        "--color always must beat NO_COLOR"
    );
}
