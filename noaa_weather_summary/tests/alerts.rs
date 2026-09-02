//! The alerts summaries against the captured NOAA fixtures.
//!
//! Each fixture is decoded through the client types, summarized, rendered as
//! markdown with the source offsets, and checked for coverage gaps.

use noaa_weather_client::models::{ActiveAlertCounts, Alert, AlertEventTypes};
use noaa_weather_client::{Feature, FeatureCollection, Pagination};
use noaa_weather_summary::render::{RenderOptions, markdown};
use noaa_weather_summary::{Section, Summarize, coverage_gaps};

const LIST: &str = include_str!("../../noaa_weather_client/tests/fixtures/alerts/list.json");
const SINGLE: &str = include_str!("../../noaa_weather_client/tests/fixtures/alerts/single.json");
const COUNT: &str = include_str!("../../noaa_weather_client/tests/fixtures/alerts/count.json");
const TYPES: &str = include_str!("../../noaa_weather_client/tests/fixtures/alerts/types.json");

fn list() -> FeatureCollection<Alert> {
    serde_json::from_str(LIST).expect("list.json decodes")
}

fn single() -> Feature<Alert> {
    serde_json::from_str(SINGLE).expect("single.json decodes")
}

fn count() -> ActiveAlertCounts {
    serde_json::from_str(COUNT).expect("count.json decodes")
}

fn types() -> AlertEventTypes {
    serde_json::from_str(TYPES).expect("types.json decodes")
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn list_summary_snapshot() {
    insta::assert_yaml_snapshot!(list().summarize());
}

#[test]
fn list_markdown_snapshot() {
    let summary = list().summarize();
    insta::assert_snapshot!(markdown::render(&summary, &RenderOptions::default()));
}

#[test]
fn list_covers_every_property() {
    assert_no_gaps(&list());
}

#[test]
fn single_summary_snapshot() {
    insta::assert_yaml_snapshot!(single().summarize());
}

#[test]
fn single_markdown_snapshot() {
    let summary = single().summarize();
    insta::assert_snapshot!(markdown::render(&summary, &RenderOptions::default()));
}

#[test]
fn single_covers_every_property() {
    assert_no_gaps(&single());
}

#[test]
fn count_summary_snapshot() {
    insta::assert_yaml_snapshot!(count().summarize());
}

#[test]
fn count_markdown_snapshot() {
    let summary = count().summarize();
    insta::assert_snapshot!(markdown::render(&summary, &RenderOptions::default()));
}

#[test]
fn count_covers_every_property() {
    assert_no_gaps(&count());
}

#[test]
fn types_summary_snapshot() {
    insta::assert_yaml_snapshot!(types().summarize());
}

#[test]
fn types_markdown_snapshot() {
    let summary = types().summarize();
    insta::assert_snapshot!(markdown::render(&summary, &RenderOptions::default()));
}

#[test]
fn types_covers_every_property() {
    assert_no_gaps(&types());
}

#[test]
fn empty_list_shows_an_empty_section_and_no_note() {
    let empty = FeatureCollection::<Alert> {
        features: Vec::new(),
        title: None,
        updated: None,
        pagination: None,
    };
    let summary = empty.summarize();
    assert_eq!(summary.title, "Alerts");
    assert_eq!(summary.subtitle.as_deref(), Some("0 alerts"));
    assert_eq!(
        summary.sections,
        vec![Section::Empty {
            key: None,
            message: "No alerts".to_owned(),
        }]
    );
    assert!(summary.notes.is_empty());
    assert_no_gaps(&empty);
}

#[test]
fn paginated_list_notes_more_alerts() {
    let mut paginated = list();
    paginated.pagination = Some(Pagination {
        next: "https://api.weather.gov/alerts?cursor=abc".to_owned(),
    });
    assert_eq!(
        paginated.summarize().notes,
        vec!["More alerts available".to_owned()]
    );

    paginated.pagination = None;
    assert!(paginated.summarize().notes.is_empty());
}

#[test]
fn single_alert_uses_singular_subtitle_in_a_list_of_one() {
    let mut one = list();
    one.features.truncate(1);
    one.pagination = None;
    assert_eq!(one.summarize().subtitle.as_deref(), Some("1 alert"));
}
