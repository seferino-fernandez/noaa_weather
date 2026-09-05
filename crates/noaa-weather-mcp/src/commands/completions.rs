//! `noaa-weather-mcp completions`: generate a static shell completion script.

use std::io;

use clap::{Command, ValueEnum};
use clap_complete::{Shell, generate};
use clap_complete_nushell::Nushell;

/// Shells a static completion script can be generated for.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CompletionShell {
    Bash,
    Elvish,
    Fish,
    Nushell,
    #[value(name = "powershell")]
    PowerShell,
    Zsh,
}

/// Writes a static shell completion script for `command` to standard output.
pub fn run(shell: CompletionShell, mut command: Command) {
    let name = command.get_name().to_owned();
    let mut output = io::stdout();
    match shell {
        CompletionShell::Bash => generate(Shell::Bash, &mut command, name, &mut output),
        CompletionShell::Elvish => generate(Shell::Elvish, &mut command, name, &mut output),
        CompletionShell::Fish => generate(Shell::Fish, &mut command, name, &mut output),
        CompletionShell::Nushell => generate(Nushell, &mut command, name, &mut output),
        CompletionShell::PowerShell => {
            generate(Shell::PowerShell, &mut command, name, &mut output);
        }
        CompletionShell::Zsh => generate(Shell::Zsh, &mut command, name, &mut output),
    }
}
