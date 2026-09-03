//! Snapshot tests over one synthetic summary that touches every variant.

use noaa_weather_client::OffsetDateTime;
use noaa_weather_summary::render::{RenderOptions, markdown, plain};
use noaa_weather_summary::{Align, Cell, Column, Emphasis, Fact, Section, Summary, Value};

fn at(text: &str) -> OffsetDateTime {
    text.parse().expect("valid RFC 3339 timestamp with offset")
}

fn synthetic_summary() -> Summary {
    let sent = at("2026-09-02T03:48:00-04:00");
    let expires = at("2026-09-02T07:00:00-04:00");

    Summary::new("Severe Thunderstorm Warning")
        .subtitle("Severe Thunderstorm Warning issued for Wayne County until 7:00 AM EDT")
        .emphasis(Emphasis::Danger)
        .push(Section::Facts {
            heading: None,
            facts: vec![
                Fact::new("ID", Some("id"), Value::identifier("urn:oid:2.49.0.1.840.0.abc")),
                Fact::new("Area", Some("areaDesc"), Value::text(Some("  Wayne, MI  "))),
                Fact::new("Sent", Some("sent"), Value::timestamp(sent)),
                Fact::new("Effective", Some("effective"), Value::interval(sent, Some(expires)))
                    .also(&["expires"]),
                Fact::new("Ends", Some("ends"), Value::interval(sent, None)),
                Fact::new("Severity", Some("severity"), Value::text(Some("Severe")))
                    .with_emphasis(Emphasis::Danger),
                Fact::new("Urgency", Some("urgency"), Value::text(Some("Expected")))
                    .with_emphasis(Emphasis::Warning),
                Fact::new("Certainty", Some("certainty"), Value::text(Some("Likely")))
                    .with_emphasis(Emphasis::Info),
                Fact::new("Headline", Some("headline"), Value::text(None)),
                Fact::new("Wind gust", Some("maxWindGust"), Value::number(Some(60.0), 0, Some("mph"))),
                Fact::new("Hail size", Some("maxHailSize"), Value::number(Some(f64::NAN), 2, Some("in"))),
                Fact::new("Ratio", None, Value::number(Some(0.5), 2, None)),
                Fact::new(
                    "Wind",
                    Some("windSpeed"),
                    Value::range(Some(10.0), Some(20.0), 0, Some("mph")),
                ),
                Fact::new(
                    "Wind chill",
                    None,
                    Value::range(Some(-5.0), Some(3.0), 0, Some("\u{b0}C")),
                ),
                Fact::new("Chance of rain", None, Value::percent(Some(39.6))),
                Fact::new("Zones", None, Value::count(3)),
                Fact::new("Payload", None, Value::bytes(1_536_000)),
                Fact::new("Replaced", None, Value::yes_no(Some(false))),
                Fact::new("Location", None, Value::coordinates(42.331_427, -83.045_754)),
                Fact::new(
                    "Affected zones",
                    Some("affectedZones"),
                    Value::list(vec![
                        Value::identifier_from_url("https://api.weather.gov/zones/forecast/MIZ044"),
                        Value::identifier_from_url("https://api.weather.gov/zones/county/MIC163/"),
                    ]),
                ),
                Fact::new(
                    "Issued by",
                    Some("senderName"),
                    Value::lines(vec![
                        Value::text(Some("NWS Detroit/Pontiac MI")),
                        Value::text(None),
                    ]),
                ),
            ],
        })
        .push(Section::Table {
            heading: Some("By area".to_owned()),
            columns: vec![
                Column::new("Area", Some("areas")).also(&["areaNames"]),
                Column::new("Alerts", None).align(Align::Right),
                Column::new("Marine", None).align(Align::Center),
            ],
            rows: vec![
                vec![
                    Cell::new(
                        Value::lines(vec![
                            Value::text(Some("MI")),
                            Value::text(Some("two lines in one cell")),
                        ]),
                        Emphasis::Notice,
                    ),
                    Cell::new(Value::count(12), Emphasis::Danger),
                    Value::yes_no(Some(true)).into(),
                ],
                vec![
                    Value::text(Some("OH | WV")).into(),
                    Cell::new(Value::count(3), Emphasis::Info),
                    Value::yes_no(Some(false)).into(),
                ],
                vec![Value::text(None).into(), Value::Invalid.into(), Value::Missing.into()],
            ],
        })
        .push(Section::Prose {
            heading: Some("Description".to_owned()),
            key: Some("description"),
            text: "At 348 AM EDT, a severe thunderstorm was located over Detroit, moving east at 40 mph."
                .to_owned(),
        })
        .push(Section::Empty {
            key: Some("instruction"),
            message: "No instructions".to_owned(),
        })
        .note("More alerts available")
        .note("Zone counts are in --json")
}

#[test]
fn markdown_snapshot() {
    let summary = synthetic_summary();
    insta::assert_snapshot!(markdown::render(&summary, &RenderOptions::default()));
}

#[test]
fn plain_snapshot() {
    let summary = synthetic_summary();
    insta::assert_snapshot!(plain::render(&summary, &RenderOptions::default()));
}

#[test]
fn summary_snapshot() {
    insta::assert_yaml_snapshot!(synthetic_summary());
}

#[test]
fn keys_collect_facts_columns_prose_and_empty_with_extras() {
    let keys = synthetic_summary().keys();
    assert!(keys.contains("areaDesc"));
    assert!(keys.contains("areas"));
    assert!(keys.contains("expires"), "fact `also` keys count");
    assert!(keys.contains("areaNames"), "column `also` keys count");
    assert!(keys.contains("description"), "prose keys count");
    assert!(keys.contains("instruction"), "empty-section keys count");
    assert!(!keys.contains("Ratio"));
}
