//! Summaries for the `/alerts` family.
//!
//! These mirror what the CLI shows today: the list is one table with a row
//! per alert, a single alert is a fact sheet followed by its description and
//! instruction, the count is three totals plus per-area and per-region
//! tables, and the event types are a one-column list.
//!
//! Severity drives emphasis everywhere: `Extreme` and `Severe` read as
//! [`Emphasis::Danger`], `Moderate` as [`Emphasis::Warning`], `Minor` as
//! [`Emphasis::Info`], and `Unknown` as [`Emphasis::None`].

use std::collections::BTreeMap;

use noaa_weather_client::models::{ActiveAlertCounts, Alert, AlertEventTypes, AlertSeverity};
use noaa_weather_client::{Feature, FeatureCollection};

use crate::{
    Align, Cell, Column, Emphasis, Fact, Section, Summarize, Summary, SummaryOptions, Value,
};

/// How urgently an alert of this severity should read.
fn severity_emphasis(severity: AlertSeverity) -> Emphasis {
    match severity {
        AlertSeverity::Extreme | AlertSeverity::Severe => Emphasis::Danger,
        AlertSeverity::Moderate => Emphasis::Warning,
        AlertSeverity::Minor => Emphasis::Info,
        AlertSeverity::Unknown => Emphasis::None,
    }
}

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

/// Property keys every alert carries that neither the list nor the single
/// alert shows, each with the reason.
const ALERT_PROVENANCE: [(&str, &str); 12] = [
    (
        "@id",
        "the alert's own URL; the id is enough to fetch it again",
    ),
    ("@type", "always wx:Alert"),
    (
        "geocode",
        "UGC codes duplicate affectedZones; SAME codes are FIPS county codes for broadcast receivers",
    ),
    (
        "references",
        "ids of the alerts this one updates; the current alert is what matters",
    ),
    ("sender", "always the NWS webmaster mailbox"),
    (
        "parameters",
        "CAP system parameters (VTEC, AWIPS and WMO ids, threat codes); NWSheadline and the hazard values are candidates for future facts",
    ),
    ("scope", "always Public for anything the API serves"),
    ("code", "always IPAWSv1.0"),
    ("language", "always en-US"),
    ("web", "always the generic weather.gov home page"),
    (
        "eventCode",
        "SAME and NWS codes for the event; the event name says the same",
    ),
    (
        "geometry",
        "polygon coordinates are unreadable in text; areaDesc names the area",
    ),
];

/// Builds an `OMITTED` slice from the given entries followed by every entry
/// of [`ALERT_PROVENANCE`].
// The splice below indexes every entry by hand because const slices cannot
// be concatenated on stable; this assertion fails to compile if an entry is
// added to `ALERT_PROVENANCE` without extending the macro.
const _: () = assert!(ALERT_PROVENANCE.len() == 12);

macro_rules! with_provenance {
    ($($entry:expr),* $(,)?) => {
        &[
            $($entry,)*
            ALERT_PROVENANCE[0],
            ALERT_PROVENANCE[1],
            ALERT_PROVENANCE[2],
            ALERT_PROVENANCE[3],
            ALERT_PROVENANCE[4],
            ALERT_PROVENANCE[5],
            ALERT_PROVENANCE[6],
            ALERT_PROVENANCE[7],
            ALERT_PROVENANCE[8],
            ALERT_PROVENANCE[9],
            ALERT_PROVENANCE[10],
            ALERT_PROVENANCE[11],
        ]
    };
}

impl Summarize for FeatureCollection<Alert> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let title = self.title.clone().unwrap_or_else(|| "Alerts".to_owned());
        let mut summary = Summary::new(title).subtitle(count_noun(self.len(), "alert", "alerts"));

        if self.is_empty() {
            summary = summary.push(Section::Empty {
                key: None,
                message: "No alerts".to_owned(),
            });
        } else {
            let rows = self
                .iter()
                .map(|feature| {
                    let alert = &feature.properties;
                    // Two independent statements, not one sentence: who issued
                    // the alert, then what they said.
                    let who_and_what = Value::lines(vec![
                        Value::text(Some(&alert.sender_name)),
                        Value::text(alert.headline.as_deref()),
                    ]);
                    vec![
                        who_and_what.into(),
                        Value::text(Some(&alert.area_desc)).into(),
                        Value::interval(alert.effective, Some(alert.expires)).into(),
                        Cell::new(
                            Value::text(Some(&alert.severity.to_string())),
                            severity_emphasis(alert.severity),
                        ),
                        Value::text(alert.instruction.as_deref()).into(),
                        Value::identifier(alert.id.as_str()).into(),
                    ]
                })
                .collect();
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("Alert", Some("senderName")).also(&["headline"]),
                    Column::new("Areas Affected", Some("areaDesc")),
                    Column::new("Effective", Some("effective")).also(&["expires"]),
                    Column::new("Severity", Some("severity")),
                    Column::new("Instructions", Some("instruction")),
                    Column::new("ID", Some("id")),
                ],
                rows,
            });
        }

        if self.pagination.is_some() {
            summary = summary.note("More alerts available");
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = with_provenance![
        // Envelope.
        ("type", "always FeatureCollection"),
        ("title", "shown as the summary title"),
        ("features", "each feature's properties is one table row"),
        (
            "updated",
            "when NOAA built the page; the alerts carry their own times"
        ),
        ("pagination", "surfaced as the 'More alerts available' note"),
        // Per-alert properties not in the table.
        (
            "event",
            "the headline names the event and when it was issued"
        ),
        (
            "sent",
            "effective is the time that matters and is usually the same"
        ),
        (
            "onset",
            "usually equals effective; the single-alert view shows it"
        ),
        (
            "ends",
            "usually null; expires bounds the row and the single-alert view shows ends"
        ),
        (
            "status",
            "list callers filter by status; the single-alert view shows it"
        ),
        ("messageType", "the single-alert view shows it"),
        (
            "category",
            "almost always Met; the single-alert view shows it"
        ),
        ("certainty", "the single-alert view shows it"),
        ("urgency", "the single-alert view shows it"),
        ("response", "the instruction column says what to do"),
        (
            "description",
            "paragraphs of text; the single-alert view shows it"
        ),
        ("note", "usually null; the single-alert view shows it"),
        ("affectedZones", "zone ids; areaDesc names the same places"),
        ("replacedBy", "the single-alert view shows it"),
        ("replacedAt", "the single-alert view shows it"),
    ];
}

impl Summarize for Feature<Alert> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let alert = &self.properties;
        let emphasis = severity_emphasis(alert.severity);
        let optional_time = |time: Option<_>| time.map_or(Value::Missing, Value::timestamp);

        let mut summary = Summary::new(alert.event.clone()).emphasis(emphasis);
        if let Some(headline) = alert
            .headline
            .as_deref()
            .filter(|text| !text.trim().is_empty())
        {
            summary = summary.subtitle(headline);
        }

        // The feature-level `id` is this alert's URL; the property `id` fact
        // covers both key names.
        summary = summary.push(Section::Facts {
            heading: None,
            facts: vec![
                Fact::new("ID", Some("id"), Value::identifier(alert.id.as_str())),
                Fact::new(
                    "Area",
                    Some("areaDesc"),
                    Value::text(Some(&alert.area_desc)),
                ),
                Fact::new(
                    "Sender name",
                    Some("senderName"),
                    Value::text(Some(&alert.sender_name)),
                ),
                Fact::new("Sent", Some("sent"), Value::timestamp(alert.sent)),
                Fact::new(
                    "Effective",
                    Some("effective"),
                    Value::timestamp(alert.effective),
                ),
                Fact::new("Onset", Some("onset"), optional_time(alert.onset)),
                Fact::new("Expires", Some("expires"), Value::timestamp(alert.expires)),
                Fact::new("Ends", Some("ends"), optional_time(alert.ends)),
                Fact::new(
                    "Status",
                    Some("status"),
                    Value::text(Some(&alert.status.to_string())),
                ),
                Fact::new(
                    "Message type",
                    Some("messageType"),
                    Value::text(Some(&alert.message_type.to_string())),
                ),
                Fact::new(
                    "Category",
                    Some("category"),
                    Value::text(Some(&alert.category.to_string())),
                ),
                Fact::new(
                    "Severity",
                    Some("severity"),
                    Value::text(Some(&alert.severity.to_string())),
                )
                .with_emphasis(emphasis),
                Fact::new(
                    "Certainty",
                    Some("certainty"),
                    Value::text(Some(&alert.certainty.to_string())),
                ),
                Fact::new(
                    "Urgency",
                    Some("urgency"),
                    Value::text(Some(&alert.urgency.to_string())),
                ),
                Fact::new(
                    "Response",
                    Some("response"),
                    Value::text(
                        alert
                            .response
                            .map(|response| response.to_string())
                            .as_deref(),
                    ),
                ),
                Fact::new(
                    "Affected zones",
                    Some("affectedZones"),
                    Value::list(
                        alert
                            .affected_zone_ids()
                            .map(|zone| Value::identifier(zone.as_str()))
                            .collect(),
                    ),
                ),
                Fact::new("Note", Some("note"), Value::text(alert.note.as_deref())),
                Fact::new(
                    "Replaced by",
                    Some("replacedBy"),
                    alert
                        .replaced_by
                        .as_deref()
                        .map_or(Value::Missing, Value::identifier_from_url),
                ),
                Fact::new(
                    "Replaced at",
                    Some("replacedAt"),
                    optional_time(alert.replaced_at),
                ),
            ],
        });

        summary = summary.push(prose_or_empty(
            "Description",
            "description",
            alert.description.as_deref(),
            "No description",
        ));
        summary.push(prose_or_empty(
            "Instruction",
            "instruction",
            alert.instruction.as_deref(),
            "No instructions",
        ))
    }

    const OMITTED: &'static [(&'static str, &'static str)] = with_provenance![
        // Envelope.
        ("type", "always Feature"),
        (
            "properties",
            "the alert itself; its keys are accounted for one by one"
        ),
        // Shown outside the facts.
        ("event", "shown as the title"),
        ("headline", "shown as the subtitle"),
    ];
}

/// A prose section for `text`, or an empty section carrying the same key
/// when there is no text.
fn prose_or_empty(
    heading: &str,
    key: &'static str,
    text: Option<&str>,
    empty_message: &str,
) -> Section {
    match text.map(str::trim).filter(|text| !text.is_empty()) {
        Some(text) => Section::Prose {
            heading: Some(heading.to_owned()),
            key: Some(key),
            text: text.to_owned(),
        },
        None => Section::Empty {
            key: Some(key),
            message: empty_message.to_owned(),
        },
    }
}

impl Summarize for ActiveAlertCounts {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let count_table = |heading: &str,
                           title: &'static str,
                           key: &'static str,
                           counts: &BTreeMap<String, u32>| {
            Section::Table {
                heading: Some(heading.to_owned()),
                columns: vec![
                    Column::new(title, Some(key)),
                    Column::new("Alerts", None).align(Align::Right),
                ],
                rows: counts
                    .iter()
                    .map(|(name, count)| {
                        vec![
                            Value::text(Some(name)).into(),
                            Value::count(u64::from(*count)).into(),
                        ]
                    })
                    .collect(),
            }
        };

        Summary::new("Active alerts")
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new("Total", Some("total"), Value::count(u64::from(self.total))),
                    Fact::new("Land", Some("land"), Value::count(u64::from(self.land))),
                    Fact::new(
                        "Marine",
                        Some("marine"),
                        Value::count(u64::from(self.marine)),
                    ),
                ],
            })
            .push(count_table("By area", "Area", "areas", &self.areas))
            .push(count_table("By region", "Region", "regions", &self.regions))
            .note(format!(
                "{} active alerts",
                count_noun(self.zones.len(), "zone has", "zones have")
            ))
    }

    const OMITTED: &'static [(&'static str, &'static str)] =
        &[("zones", "about 2,900 rows; per-zone counts are in --json")];
}

impl Summarize for AlertEventTypes {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let summary = Summary::new("Alert event types").subtitle(count_noun(
            self.event_types.len(),
            "type",
            "types",
        ));
        if self.event_types.is_empty() {
            return summary.push(Section::Empty {
                key: Some("eventTypes"),
                message: "No event types".to_owned(),
            });
        }
        summary.push(Section::Table {
            heading: None,
            columns: vec![Column::new("Event type", Some("eventTypes"))],
            rows: self
                .event_types
                .iter()
                .map(|name| vec![Value::text(Some(name)).into()])
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_maps_to_emphasis_like_the_cli_colors() {
        assert_eq!(severity_emphasis(AlertSeverity::Extreme), Emphasis::Danger);
        assert_eq!(severity_emphasis(AlertSeverity::Severe), Emphasis::Danger);
        assert_eq!(
            severity_emphasis(AlertSeverity::Moderate),
            Emphasis::Warning
        );
        assert_eq!(severity_emphasis(AlertSeverity::Minor), Emphasis::Info);
        assert_eq!(severity_emphasis(AlertSeverity::Unknown), Emphasis::None);
    }

    #[test]
    fn count_noun_picks_singular_for_one() {
        assert_eq!(count_noun(1, "alert", "alerts"), "1 alert");
        assert_eq!(count_noun(0, "alert", "alerts"), "0 alerts");
        assert_eq!(count_noun(5, "alert", "alerts"), "5 alerts");
    }

    #[test]
    fn prose_or_empty_treats_blank_text_as_empty() {
        assert_eq!(
            prose_or_empty("Description", "description", Some("   "), "No description"),
            Section::Empty {
                key: Some("description"),
                message: "No description".to_owned(),
            }
        );
        assert_eq!(
            prose_or_empty(
                "Description",
                "description",
                Some(" Text "),
                "No description"
            ),
            Section::Prose {
                heading: Some("Description".to_owned()),
                key: Some("description"),
                text: "Text".to_owned(),
            }
        );
    }
}
