//! Radio summaries against every captured response shape.

use noaa_weather_client::models::{RadioBroadcast, RadioTransmitter, RadioTransmitterCollection};
use noaa_weather_summary::{Section, Summarize, SummaryOptions, coverage_gaps};

const TRANSMITTER: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/radio/transmitter.json");
const TRANSMITTERS: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/radio/transmitters.json");
const COUNTY: &str = include_str!("../../noaa_weather_client/tests/fixtures/radio/county.json");
const BROADCAST: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/radio/broadcast.xml");
const POINT: &str = include_str!("../../noaa_weather_client/tests/fixtures/radio/point.xml");

fn decode_json<T: serde::de::DeserializeOwned>(source: &str) -> T {
    serde_json::from_str(source).expect("radio JSON fixture decodes")
}

fn decode_xml(source: &str) -> RadioBroadcast {
    RadioBroadcast::from_ssml(source).expect("radio SSML fixture decodes")
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn transmitter_summary_snapshot() {
    insta::assert_yaml_snapshot!(
        decode_json::<RadioTransmitter>(TRANSMITTER).summarize(&SummaryOptions::default())
    );
}

#[test]
fn transmitter_collection_summary_snapshot() {
    let mut transmitters = decode_json::<RadioTransmitterCollection>(TRANSMITTERS);
    transmitters.transmitters = ["KUT404", "WXM25"]
        .into_iter()
        .map(|call_sign| {
            transmitters
                .transmitters
                .iter()
                .find(|transmitter| transmitter.call_sign.as_str() == call_sign)
                .expect("fixture carries snapshot transmitter")
                .clone()
        })
        .collect();
    insta::assert_yaml_snapshot!(transmitters.summarize(&SummaryOptions::default()));
}

#[test]
fn broadcast_summary_snapshot() {
    let mut broadcast = decode_xml(BROADCAST);
    broadcast.paragraphs.truncate(2);
    insta::assert_yaml_snapshot!(broadcast.summarize(&SummaryOptions::default()));
}

#[test]
fn every_radio_fixture_has_complete_summary_coverage() {
    assert_no_gaps(&decode_json::<RadioTransmitter>(TRANSMITTER));
    assert_no_gaps(&decode_json::<RadioTransmitterCollection>(TRANSMITTERS));
    assert_no_gaps(&decode_json::<RadioTransmitterCollection>(COUNTY));
    assert_no_gaps(&decode_xml(BROADCAST));
    assert_no_gaps(&decode_xml(POINT));
}

#[test]
fn empty_radio_responses_explain_why_there_is_no_content() {
    let transmitters: RadioTransmitterCollection = decode_json(r#"{"@graph":[]}"#);
    let broadcast = decode_xml(r#"<speak version="1.1" xml:lang="en-US"/>"#);

    for summary in [
        transmitters.summarize(&SummaryOptions::default()),
        broadcast.summarize(&SummaryOptions::default()),
    ] {
        assert!(
            summary
                .sections
                .iter()
                .any(|section| matches!(section, Section::Empty { .. }))
        );
    }
}

#[test]
fn pronunciation_hints_are_spoken_and_marks_remain_visible() {
    let summary = decode_xml(BROADCAST).summarize(&SummaryOptions::default());
    let transcript = summary
        .sections
        .iter()
        .find_map(|section| match section {
            Section::Prose { text, .. } => Some(text),
            _ => None,
        })
        .expect("broadcast has a transcript");
    assert!(transcript.contains("KEC94"));
    assert!(transcript.contains("[mark: {'requesterSameCode': '004013'}]"));
}
