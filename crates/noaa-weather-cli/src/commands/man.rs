//! `noaa-weather man`: generate man pages for the CLI and its subcommands.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::Command;

/// Generates man pages under `output_directory`, creating it when needed.
pub fn run(command: Command, output_directory: &Path) -> Result<()> {
    fs::create_dir_all(output_directory)?;
    clap_mangen::generate_to(command, output_directory)?;
    Ok(())
}
