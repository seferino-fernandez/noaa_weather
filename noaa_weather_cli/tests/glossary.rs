use assert_cmd::cargo::cargo_bin;
use std::fs;
use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(cargo_bin!("noaa-weather"))
        .args(args)
        .output()
        .expect("run noaa-weather")
}

#[test]
fn glossary_supports_table_and_json_output() {
    let table = run(&["glossary"]);
    assert!(
        table.status.success(),
        "{}",
        String::from_utf8_lossy(&table.stderr)
    );
    assert!(String::from_utf8_lossy(&table.stdout).contains("Definition"));

    let json = run(&["glossary", "--json"]);
    assert!(
        json.status.success(),
        "{}",
        String::from_utf8_lossy(&json.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).expect("glossary JSON");
    let glossary = value["glossary"].as_array().expect("glossary array");
    assert!(
        glossary.is_empty() || glossary[0]["term"].is_string(),
        "unexpected glossary item: {:?}",
        glossary.first()
    );
}

#[test]
fn glossary_supports_file_and_explicit_stdout_destinations() {
    let directory = tempfile::tempdir().expect("temporary output directory");
    let table_path = directory.path().join("glossary.txt");
    let table_path_arg = table_path.to_str().expect("UTF-8 temporary path");

    let table = run(&["glossary", "--output", table_path_arg]);
    assert!(
        table.status.success(),
        "{}",
        String::from_utf8_lossy(&table.stderr)
    );
    assert!(table.stdout.is_empty());
    let table_text = fs::read_to_string(&table_path).expect("table output file");
    assert!(table_text.contains("Definition"));
    assert!(table_text.ends_with('\n'));
    assert!(!table_text.ends_with("\n\n"));

    let json = run(&["glossary", "--json", "--output", "-"]);
    assert!(
        json.status.success(),
        "{}",
        String::from_utf8_lossy(&json.stderr)
    );
    assert!(json.stdout.ends_with(b"\n"));
    assert!(!json.stdout.ends_with(b"\n\n"));
    let value: serde_json::Value =
        serde_json::from_slice(&json.stdout).expect("explicit stdout JSON");
    assert!(value["glossary"].is_array());
}
