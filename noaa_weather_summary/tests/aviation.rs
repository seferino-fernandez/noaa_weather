//! Aviation summaries against the captured NOAA fixtures.
//!
//! Each public response shape is decoded through the client model,
//! snapshotted as a format-independent summary, and audited so a newly
//! arriving NOAA key cannot disappear from human output unnoticed.

use noaa_weather_client::models::{CenterWeatherAdvisory, CwsuOffice, Sigmet};
use noaa_weather_client::{Feature, FeatureCollection};
use noaa_weather_summary::{Section, Summarize, SummaryOptions, coverage_gaps};

const CWSU: &str = include_str!("../../noaa_weather_client/tests/fixtures/aviation/cwsu.json");
const CWA: &str = include_str!("../../noaa_weather_client/tests/fixtures/aviation/cwa.json");
const CWAS: &str = include_str!("../../noaa_weather_client/tests/fixtures/aviation/cwas.json");
const SIGMET: &str = include_str!("../../noaa_weather_client/tests/fixtures/aviation/sigmet.json");
const SIGMETS: &str =
    include_str!("../../noaa_weather_client/tests/fixtures/aviation/sigmets.json");

fn cwsu() -> CwsuOffice {
    serde_json::from_str(CWSU).expect("cwsu.json decodes")
}

fn cwa() -> Feature<CenterWeatherAdvisory> {
    serde_json::from_str(CWA).expect("cwa.json decodes")
}

fn cwas() -> FeatureCollection<CenterWeatherAdvisory> {
    serde_json::from_str(CWAS).expect("cwas.json decodes")
}

fn sigmet() -> Feature<Sigmet> {
    serde_json::from_str(SIGMET).expect("sigmet.json decodes")
}

fn sigmets() -> FeatureCollection<Sigmet> {
    serde_json::from_str(SIGMETS).expect("sigmets.json decodes")
}

fn assert_no_gaps<T: Summarize>(value: &T) {
    let gaps = coverage_gaps(value);
    assert!(
        gaps.is_empty(),
        "properties neither shown nor omitted: {gaps:?}"
    );
}

#[test]
fn cwsu_summary_snapshot() {
    insta::assert_yaml_snapshot!(cwsu().summarize(&SummaryOptions::default()));
}

#[test]
fn single_cwa_summary_snapshot() {
    insta::assert_yaml_snapshot!(cwa().summarize(&SummaryOptions::default()));
}

#[test]
fn cwa_collection_summary_snapshot() {
    let mut advisories = cwas();
    advisories.features.truncate(2);
    insta::assert_yaml_snapshot!(advisories.summarize(&SummaryOptions::default()));
}

#[test]
fn single_sigmet_summary_snapshot() {
    insta::assert_yaml_snapshot!(sigmet().summarize(&SummaryOptions::default()));
}

#[test]
fn sigmet_collection_summary_snapshot() {
    let mut products = sigmets();
    products.features.truncate(2);
    insta::assert_yaml_snapshot!(products.summarize(&SummaryOptions::default()));
}

#[test]
fn every_aviation_shape_covers_every_property() {
    assert_no_gaps(&cwsu());
    assert_no_gaps(&cwa());
    assert_no_gaps(&cwas());
    assert_no_gaps(&sigmet());
    assert_no_gaps(&sigmets());
}

#[test]
fn empty_collections_explain_that_conditions_are_quiet() {
    let empty_cwas = FeatureCollection::<CenterWeatherAdvisory> {
        features: Vec::new(),
        title: None,
        updated: None,
        pagination: None,
    };
    let empty_sigmets = FeatureCollection::<Sigmet> {
        features: Vec::new(),
        title: None,
        updated: None,
        pagination: None,
    };

    for (summary, subtitle, expected) in [
        (
            empty_cwas.summarize(&SummaryOptions::default()),
            "0 advisories",
            "No current Center Weather Advisories",
        ),
        (
            empty_sigmets.summarize(&SummaryOptions::default()),
            "0 products",
            "No current SIGMETs or AIRMETs",
        ),
    ] {
        assert_eq!(summary.subtitle.as_deref(), Some(subtitle));
        assert!(matches!(
            summary.sections.as_slice(),
            [Section::Empty { message, .. }] if message == expected
        ));
    }
}
