//! Holds the shared command table to clap's command tree.
//!
//! A subcommand is only covered if it is in `tests/common/table.rs`, and the
//! table is hand-written, so nothing but this file notices when a new command
//! ships without tests.

mod common;

use std::collections::BTreeSet;

use clap::{CommandFactory as _, builder::StyledStr};
use noaa_weather_cli::Cli;

use common::table::{FAMILIES, FAMILIES_AWAITING_COVERAGE};

/// Every leaf subcommand, as the argument path that reaches it.
///
/// A leaf is a command with no subcommands of its own. clap's generated
/// `help` command is not something a user drives for data, so it is skipped
/// wherever it appears.
fn leaves() -> BTreeSet<Vec<String>> {
    fn walk(command: &clap::Command, path: &mut Vec<String>, found: &mut BTreeSet<Vec<String>>) {
        let children: Vec<_> = command
            .get_subcommands()
            .filter(|child| child.get_name() != "help")
            .collect();
        if children.is_empty() {
            found.insert(path.clone());
            return;
        }
        for child in children {
            path.push(child.get_name().to_owned());
            walk(child, path, found);
            path.pop();
        }
    }

    let command = Cli::command();
    let mut found = BTreeSet::new();
    walk(&command, &mut Vec::new(), &mut found);
    found
}

/// Every subcommand path the table drives.
fn covered() -> BTreeSet<Vec<String>> {
    FAMILIES
        .iter()
        .flat_map(|family| family.invocations)
        .map(|invocation| {
            invocation
                .command
                .iter()
                .map(|&part| part.to_owned())
                .collect()
        })
        .collect()
}

/// The top-level subcommand names clap knows about.
fn families() -> BTreeSet<String> {
    Cli::command()
        .get_subcommands()
        .map(|child| child.get_name().to_owned())
        .filter(|name| name != "help")
        .collect()
}

fn render(paths: impl IntoIterator<Item = Vec<String>>) -> String {
    paths
        .into_iter()
        .map(|path| format!("\n  {}", path.join(" ")))
        .collect::<String>()
}

#[test]
fn every_leaf_subcommand_is_in_the_table_or_on_the_allow_list() {
    let covered = covered();
    let exempt: BTreeSet<&str> = FAMILIES_AWAITING_COVERAGE.iter().copied().collect();
    let missing: Vec<_> = leaves()
        .into_iter()
        .filter(|leaf| !covered.contains(leaf))
        .filter(|leaf| {
            leaf.first()
                .is_none_or(|family| !exempt.contains(family.as_str()))
        })
        .collect();

    assert!(
        missing.is_empty(),
        "these subcommands have no entry in tests/common/table.rs:{}",
        render(missing)
    );
}

#[test]
fn every_table_entry_names_a_subcommand_that_exists() {
    let leaves = leaves();
    let unknown: Vec<_> = covered()
        .into_iter()
        .filter(|path| !leaves.contains(path))
        .collect();

    assert!(
        unknown.is_empty(),
        "tests/common/table.rs drives commands clap does not define:{}",
        render(unknown)
    );
}

#[test]
fn the_allow_list_only_names_families_that_are_still_uncovered() {
    let families = families();
    let covered_families: BTreeSet<&str> = FAMILIES.iter().map(|family| family.name).collect();

    for name in FAMILIES_AWAITING_COVERAGE {
        assert!(
            families.contains(*name),
            "FAMILIES_AWAITING_COVERAGE names {name:?}, which is not a subcommand"
        );
        assert!(
            !covered_families.contains(name),
            "{name:?} is covered by the table, so it belongs out of \
             FAMILIES_AWAITING_COVERAGE"
        );
    }
}

/// The table is keyed by family name, so a name that drifts from clap's would
/// leave the family looking covered while its leaves went unchecked.
#[test]
fn every_covered_family_is_a_real_subcommand() {
    let families = families();
    for family in FAMILIES {
        assert!(
            families.contains(family.name),
            "the table covers {:?}, which is not a subcommand",
            family.name
        );
        for invocation in family.invocations {
            assert_eq!(
                invocation.command.first(),
                Some(&family.name),
                "{} is filed under the {:?} family",
                invocation.display(),
                family.name
            );
        }
    }
}

/// Nothing here reads help text, but rendering it proves the new global flags
/// did not leave the command tree in a state clap refuses to format.
#[test]
fn the_command_tree_renders() {
    let help: StyledStr = Cli::command().render_long_help();
    let help = help.to_string();
    for variable in [
        "NOAA_WEATHER_BASE_URL",
        "NOAA_WEATHER_USER_AGENT",
        "NOAA_WEATHER_TIMEOUT",
        "NOAA_WEATHER_RETRIES",
        "NOAA_WEATHER_API_KEY",
    ] {
        assert!(help.contains(variable), "--help never mentions {variable}");
    }
}
