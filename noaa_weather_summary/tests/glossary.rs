//! Glossary summary coverage against the captured NOAA fixture.

use noaa_weather_client::models::GlossaryResponse;
use noaa_weather_summary::{Section, Summarize, SummaryOptions, coverage_gaps};

const GLOSSARY: &str = include_str!("../../noaa_weather_client/tests/fixtures/glossary/terms.json");

fn glossary() -> GlossaryResponse {
    serde_json::from_str(GLOSSARY).expect("glossary fixture decodes")
}

#[test]
fn glossary_summary_snapshot() {
    insta::assert_yaml_snapshot!(glossary().summarize(&SummaryOptions::default()));
}

#[test]
fn glossary_summary_covers_every_property() {
    let gaps = coverage_gaps(&glossary());
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn empty_glossary_explains_why_there_are_no_rows() {
    let empty: GlossaryResponse = serde_json::from_str(r#"{"glossary":[]}"#).unwrap();
    let summary = empty.summarize(&SummaryOptions::default());

    assert!(summary.sections.iter().any(|section| matches!(
        section,
        Section::Empty {
            key: Some("glossary"),
            ..
        }
    )));
}
