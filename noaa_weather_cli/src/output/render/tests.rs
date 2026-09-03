//! Snapshots of everything the renderer can draw.
//!
//! One synthetic summary covers every [`Value`], [`Section`] and [`Emphasis`]
//! variant; it is rendered across the color and width settings so a change in
//! any of them shows up as a diff. The alerts fixtures then go through the
//! whole path — decode, [`Summarize`], render — because those three chosen
//! output changes are what a reviewer needs to see.

use noaa_weather_client::models::{ActiveAlertCounts, Alert, AlertEventTypes};
use noaa_weather_client::{Feature, FeatureCollection, OffsetDateTime};
use noaa_weather_summary::{
    Align, Cell, Column, Emphasis, Fact, Section, Summarize, Summary, SummaryOptions, Value,
};

use super::{ColorMode, RenderOptions, TimeZoneChoice};

const LIST: &str = include_str!("../../../../noaa_weather_client/tests/fixtures/alerts/list.json");
const SINGLE: &str =
    include_str!("../../../../noaa_weather_client/tests/fixtures/alerts/single.json");
const COUNT: &str =
    include_str!("../../../../noaa_weather_client/tests/fixtures/alerts/count.json");
const TYPES: &str =
    include_str!("../../../../noaa_weather_client/tests/fixtures/alerts/types.json");

/// Options with nothing left to the environment, so a snapshot means the same
/// thing on every machine.
fn options(color: ColorMode, width: u16) -> RenderOptions {
    RenderOptions::new(color, Some(width), &TimeZoneChoice::Source, false)
}

fn at(text: &str) -> OffsetDateTime {
    text.parse().expect("valid RFC 3339 timestamp with offset")
}

/// Every variant the renderer has to have an answer for.
fn synthetic_summary() -> Summary {
    let sent = at("2026-09-02T03:48:00-04:00");
    let expires = at("2026-09-02T07:00:00-04:00");

    Summary::new("Severe Thunderstorm Warning")
        .subtitle("Severe Thunderstorm Warning issued for Wayne County until 7:00 AM EDT")
        .emphasis(Emphasis::Danger)
        .push(Section::Facts {
            heading: None,
            facts: vec![
                Fact::new(
                    "ID",
                    Some("id"),
                    Value::identifier("urn:oid:2.49.0.1.840.0.abcdef0123456789.001.1"),
                ),
                Fact::new("Area", Some("areaDesc"), Value::text(Some("Wayne, MI"))),
                Fact::new("Sent", Some("sent"), Value::timestamp(sent)),
                Fact::new(
                    "Effective",
                    Some("effective"),
                    Value::interval(sent, Some(expires)),
                ),
                Fact::new("Ends", Some("ends"), Value::interval(sent, None)),
                Fact::new("Severity", Some("severity"), Value::text(Some("Severe")))
                    .with_emphasis(Emphasis::Danger),
                Fact::new("Urgency", Some("urgency"), Value::text(Some("Expected")))
                    .with_emphasis(Emphasis::Warning),
                Fact::new("Watch", None, Value::text(Some("Under review")))
                    .with_emphasis(Emphasis::Notice),
                Fact::new("Certainty", Some("certainty"), Value::text(Some("Likely")))
                    .with_emphasis(Emphasis::Info),
                Fact::new("Headline", Some("headline"), Value::text(None)),
                Fact::new(
                    "Wind gust",
                    Some("maxWindGust"),
                    Value::number(Some(60.0), 0, Some("mph")),
                ),
                Fact::new(
                    "Hail size",
                    Some("maxHailSize"),
                    Value::number(Some(f64::NAN), 2, Some("in")),
                ),
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
                        Value::identifier("MIZ044"),
                        Value::identifier("MIC163"),
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
                Column::new("Area", Some("areas")),
                Column::new("Alerts", None).align(Align::Right),
                Column::new("Marine", None).align(Align::Center),
                Column::new("Zone", None),
            ],
            rows: vec![
                vec![
                    Cell::new(
                        Value::lines(vec![
                            Value::text(Some("Michigan")),
                            Value::text(Some("severe storms across the southeast counties")),
                        ]),
                        Emphasis::Notice,
                    ),
                    Cell::new(Value::count(12), Emphasis::Danger),
                    Value::yes_no(Some(true)).into(),
                    Value::identifier("MIZ044").into(),
                ],
                vec![
                    Cell::new(Value::text(Some("Ohio")), Emphasis::Warning),
                    Cell::new(Value::count(3), Emphasis::Info),
                    Value::yes_no(Some(false)).into(),
                    Value::identifier("OHZ001").into(),
                ],
                vec![
                    Value::text(None).into(),
                    Value::Invalid.into(),
                    Value::Missing.into(),
                    Value::identifier("PZZ800").into(),
                ],
            ],
        })
        .push(Section::Prose {
            heading: Some("Description".to_owned()),
            key: Some("description"),
            text: "At 348 AM EDT, a severe thunderstorm was located over Detroit, moving east at 40 mph. Hail damage to vehicles is expected."
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
fn every_variant_without_color() {
    insta::assert_snapshot!(options(ColorMode::Never, 100).render(&synthetic_summary()));
}

#[test]
fn every_variant_narrow() {
    insta::assert_snapshot!(options(ColorMode::Never, 40).render(&synthetic_summary()));
}

/// `--width 0` means never wrap, for piping into a pager.
#[test]
fn every_variant_unlimited() {
    insta::assert_snapshot!(options(ColorMode::Never, 0).render(&synthetic_summary()));
}

/// A width below the floor renders as the floor, not as an unreadable sliver.
#[test]
fn narrower_than_the_floor_is_the_floor() {
    assert_eq!(
        options(ColorMode::Never, 10).render(&synthetic_summary()),
        options(ColorMode::Never, super::MIN_WIDTH).render(&synthetic_summary())
    );
}

/// The destination here is not a terminal, so this locks down that escapes
/// still reach a pipe when a caller asks for them explicitly.
#[test]
fn every_variant_with_color_into_a_pipe() {
    insta::assert_snapshot!(options(ColorMode::Always, 100).render(&synthetic_summary()));
}

#[test]
fn color_never_writes_no_escapes() {
    let rendered = options(ColorMode::Never, 100).render(&synthetic_summary());
    assert!(
        !rendered.contains('\u{1b}'),
        "--color never must not emit escapes"
    );
}

/// `Auto` looks at the destination: a pipe gets no escapes without being told.
#[test]
fn color_auto_into_a_pipe_writes_no_escapes() {
    let options = RenderOptions::new(ColorMode::Auto, Some(100), &TimeZoneChoice::Source, false);
    assert!(!options.render(&synthetic_summary()).contains('\u{1b}'));
}

/// The choke point decides styling from the resolved color mode, never from
/// this process's stdout, and that decision reaches the tables the un-ported
/// families still build by hand.
#[test]
fn applying_options_to_a_hand_built_table_follows_the_color_mode() {
    for (color, expected) in [(ColorMode::Never, false), (ColorMode::Always, true)] {
        let mut table = comfy_table::Table::new();
        table.set_header(["Column"]);
        table.add_row([comfy_table::Cell::new("Value").fg(comfy_table::Color::Red)]);
        options(color, 100).apply(&mut table);

        assert_eq!(
            table.should_style(),
            expected,
            "{color:?} must decide styling for an un-ported table"
        );
        assert_eq!(
            table.to_string().contains('\u{1b}'),
            expected,
            "{color:?} must decide whether escapes are written"
        );
    }
}

#[test]
fn unlimited_width_disables_arrangement() {
    let mut table = comfy_table::Table::new();
    options(ColorMode::Never, 0).apply(&mut table);
    assert!(matches!(
        table.content_arrangement(),
        comfy_table::ContentArrangement::Disabled
    ));

    options(ColorMode::Never, 100).apply(&mut table);
    assert!(matches!(
        table.content_arrangement(),
        comfy_table::ContentArrangement::Dynamic
    ));
    assert_eq!(table.width(), Some(100));
}

#[test]
fn list_of_alerts() {
    let alerts: FeatureCollection<Alert> = serde_json::from_str(LIST).expect("list.json decodes");
    insta::assert_snapshot!(
        options(ColorMode::Never, 100).render(&alerts.summarize(&SummaryOptions::default()))
    );
}

/// The threshold in [`super::table`]: a 69-character alert URN, five other
/// columns needing eight each, and the borders come to exactly 128 columns.
/// At 128 the identifier is whole and copyable; one column narrower and it
/// breaks into fragments so the other five keep something to say.
#[test]
fn list_of_alerts_at_the_identifier_threshold() {
    let alerts: FeatureCollection<Alert> = serde_json::from_str(LIST).expect("list.json decodes");
    let rendered =
        options(ColorMode::Never, 128).render(&alerts.summarize(&SummaryOptions::default()));
    assert!(
        rendered.contains("urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1"),
        "at the threshold the URN must appear whole on one line"
    );
    insta::assert_snapshot!(rendered);
}

#[test]
fn list_of_alerts_one_column_below_the_identifier_threshold() {
    let alerts: FeatureCollection<Alert> = serde_json::from_str(LIST).expect("list.json decodes");
    let rendered =
        options(ColorMode::Never, 127).render(&alerts.summarize(&SummaryOptions::default()));
    assert!(
        !rendered.contains("urn:oid:2.49.0.1.840.0.af7e0442df8bfed7953cb5cae6e4661304cc1f49.001.1"),
        "one column below the threshold the URN must wrap instead of starving the row"
    );
    insta::assert_snapshot!(rendered);
}

#[test]
fn single_alert() {
    let alert: Feature<Alert> = serde_json::from_str(SINGLE).expect("single.json decodes");
    insta::assert_snapshot!(
        options(ColorMode::Never, 100).render(&alert.summarize(&SummaryOptions::default()))
    );
}

#[test]
fn active_alert_counts() {
    let counts: ActiveAlertCounts = serde_json::from_str(COUNT).expect("count.json decodes");
    insta::assert_snapshot!(
        options(ColorMode::Never, 100).render(&counts.summarize(&SummaryOptions::default()))
    );
}

#[test]
fn alert_event_types() {
    let types: AlertEventTypes = serde_json::from_str(TYPES).expect("types.json decodes");
    insta::assert_snapshot!(
        options(ColorMode::Never, 100).render(&types.summarize(&SummaryOptions::default()))
    );
}

#[test]
fn a_named_zone_moves_every_timestamp() {
    let zone = jiff::tz::TimeZone::get("UTC").expect("UTC exists");
    let options = RenderOptions::new(
        ColorMode::Never,
        Some(100),
        &TimeZoneChoice::Named(zone),
        false,
    );
    insta::assert_snapshot!(options.render(&synthetic_summary()));
}
