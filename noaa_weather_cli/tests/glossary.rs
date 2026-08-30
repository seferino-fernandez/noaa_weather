use assert_cmd::cargo::cargo_bin;
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
