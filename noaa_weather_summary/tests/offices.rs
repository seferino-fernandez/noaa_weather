//! Office summaries against every captured response shape.

use noaa_weather_client::models::{
    Office, OfficeBriefingResponse, OfficeHeadline, OfficeHeadlineCollection,
    OfficeWeatherStoryCollection,
};
use noaa_weather_summary::{Section, Summarize, SummaryOptions, coverage_gaps};

const OFFICE: &str = include_str!("../../noaa_weather_client/tests/fixtures/offices/office.json");
const HEADLINE: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/offices/headline.json");
const HEADLINES: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/offices/headlines.json");
const BRIEFING: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/offices/briefing.json");
const STORIES: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/offices/weather_stories.json");

fn decode<T: serde::de::DeserializeOwned>(source: &str) -> T {
    serde_json::from_str(source).expect("office fixture decodes")
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn office_summary_snapshot() {
    insta::assert_yaml_snapshot!(decode::<Office>(OFFICE).summarize(&SummaryOptions::default()));
}

#[test]
fn headline_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<OfficeHeadline>(HEADLINE).summarize(&SummaryOptions::default())
    );
}

#[test]
fn headline_collection_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode::<OfficeHeadlineCollection>(HEADLINES).summarize(&SummaryOptions::default())
    );
}

#[test]
fn active_briefing_summary_snapshot() {
    let briefing: OfficeBriefingResponse = decode(
        r#"{
            "briefing": {
                "id": "f25c1906-57d1-4f86-80cf-a20dce3bddaa",
                "officeId": "TOP",
                "startTime": "2026-09-04T08:02:00+00:00",
                "endTime": "2026-09-05T08:02:00+00:00",
                "updateTime": "2026-09-04T18:55:11+00:00",
                "title": "Click to view briefing",
                "description": "Extreme Heat Warning Through Tuesday Evening",
                "priority": false,
                "download": "https://api.weather.gov/offices/TOP/briefing/download/f25c1906-57d1-4f86-80cf-a20dce3bddaa"
            }
        }"#,
    );
    assert_no_gaps(&briefing);
    insta::assert_yaml_snapshot!(briefing.summarize(&SummaryOptions::default()));
}

#[test]
fn weather_story_collection_summary_snapshot() {
    let mut stories = decode::<OfficeWeatherStoryCollection>(STORIES);
    stories.stories.truncate(2);
    insta::assert_yaml_snapshot!(stories.summarize(&SummaryOptions::default()));
}

#[test]
fn every_office_fixture_has_complete_summary_coverage() {
    assert_no_gaps(&decode::<Office>(OFFICE));
    assert_no_gaps(&decode::<OfficeHeadline>(HEADLINE));
    assert_no_gaps(&decode::<OfficeHeadlineCollection>(HEADLINES));
    assert_no_gaps(&decode::<OfficeBriefingResponse>(BRIEFING));
    assert_no_gaps(&decode::<OfficeWeatherStoryCollection>(STORIES));
}

#[test]
fn empty_office_publications_explain_why_there_is_no_content() {
    for summary in [
        decode::<OfficeHeadlineCollection>(r#"{"@graph":[]}"#)
            .summarize(&SummaryOptions::default()),
        decode::<OfficeBriefingResponse>(r#"{"briefing":null}"#)
            .summarize(&SummaryOptions::default()),
        decode::<OfficeWeatherStoryCollection>(r#"{"stories":[]}"#)
            .summarize(&SummaryOptions::default()),
    ] {
        assert!(
            summary
                .sections
                .iter()
                .any(|section| matches!(section, Section::Empty { .. }))
        );
    }
}
