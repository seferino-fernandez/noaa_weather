//! The `products` family, end to end.

mod common;

use std::process::Output;

use common::noaa_weather;
use common::runner::{family, hermetic, live};
use serde_json::Value;

#[tokio::test]
async fn every_products_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("products")).await;
}

#[test]
fn test_products_live_noaa_answers_every_tabled_invocation() {
    live(family("products"));
}

fn succeeding(arguments: &[&str]) -> Output {
    let output = noaa_weather()
        .args(arguments)
        .output()
        .expect("the built binary must be runnable");
    assert_eq!(
        output.status.code(),
        Some(0),
        "`{}` failed: {}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn json(output: &Output, what: &str) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "`{what}` did not emit JSON: {error}\n{}",
            String::from_utf8_lossy(&output.stdout)
        )
    })
}

/// Fetches one text product by an id resolved at run time.
///
/// The predecessor carried `--id a4791428-298e-473c-8e6f-5796701c9e4a` and
/// was ignored, because a product id ages out of NOAA's window. Resolving
/// through `products list --json` keeps the whole test in the CLI's own
/// vocabulary: a `--json` that stopped carrying ids breaks resolution here
/// rather than passing unnoticed.
#[test]
fn a_product_is_fetched_by_an_id_resolved_at_run_time() {
    let what = "products list --limit 1 --json";
    let listing = succeeding(&["products", "list", "--limit", "1", "--json"]);
    let listing = json(&listing, what);
    let products = listing["@graph"]
        .as_array()
        .unwrap_or_else(|| panic!("`{what}` returned no `@graph` array: {listing}"));

    let Some(first) = products.first() else {
        // The listing answered and decoded; NOAA simply had nothing in the
        // window. That is a fact about the data, not a test declining to
        // run, and this is where it gets said.
        eprintln!(
            "`{what}` returned a well-formed empty `@graph`, so NOAA had no \
             recent text product and there was no id to fetch. The listing \
             endpoint was checked; `products metadata` was not."
        );
        return;
    };

    let id = first["id"]
        .as_str()
        .unwrap_or_else(|| panic!("the first listed product has no `id`: {first}"));

    let fetched = succeeding(&["products", "metadata", "--id", id, "--json"]);
    let fetched = json(&fetched, "products metadata --json");
    assert_eq!(
        fetched["id"], first["id"],
        "NOAA answered `products metadata --id {id}` with a different product"
    );
    assert!(
        fetched["productText"]
            .as_str()
            .is_some_and(|text| !text.is_empty()),
        "`products metadata --id {id}` carried no product text: {fetched}"
    );
}

#[test]
fn test_products_reject_malformed_type_code() {
    let output = noaa_weather()
        .args(["products", "type", "--type-id", "AFDX"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid product type code"), "{stderr}");
}

/// The list command's limit is bounded by clap, before any request is made.
#[test]
fn test_products_list_rejects_a_limit_outside_the_accepted_range() {
    for value in ["0", "501"] {
        let output = noaa_weather()
            .args(["products", "list", "--limit", value])
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{value}: {stderr}");
        assert!(stderr.contains("--limit"), "{value}: {stderr}");
    }
}
