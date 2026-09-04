//! Human summaries for NOAA Weather Radio transmitters and broadcasts.

use noaa_weather_client::models::{
    Paragraph, RadioBroadcast, RadioTransmitter, RadioTransmitterCollection,
};

use crate::{Align, Column, Fact, Section, Summarize, Summary, SummaryOptions, Value};

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn transmitter_columns() -> Vec<Column> {
    vec![
        Column::new("Call Sign", Some("callSign")),
        Column::new("Frequency", Some("transmitterFrequency")),
        Column::new("Site", Some("siteName")),
        Column::new("City", Some("siteCity")),
        Column::new("State", Some("siteState")),
        Column::new("SAME Codes", Some("sameCodes")).align(Align::Right),
        Column::new("Counties", Some("counties")).align(Align::Right),
    ]
}

fn transmitter_row(transmitter: &RadioTransmitter) -> Vec<crate::Cell> {
    vec![
        Value::identifier(transmitter.call_sign.to_string()).into(),
        Value::text(Some(&transmitter.frequency)).into(),
        Value::text(Some(&transmitter.site_name)).into(),
        Value::text(Some(&transmitter.city)).into(),
        Value::identifier(transmitter.state.to_string()).into(),
        Value::count(transmitter.same_codes.len() as u64).into(),
        Value::count(transmitter.counties.len() as u64).into(),
    ]
}

fn paragraph_text(paragraph: &Paragraph) -> String {
    let spoken = paragraph
        .sentences
        .iter()
        .map(|sentence| sentence.full_text())
        .map(|sentence| sentence.trim().to_owned())
        .filter(|sentence| !sentence.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let marks = paragraph
        .marks
        .iter()
        .map(|mark| format!("[mark: {}]", mark.name))
        .collect::<Vec<_>>()
        .join("\n");

    match (spoken.is_empty(), marks.is_empty()) {
        (false, false) => format!("{spoken}\n{marks}"),
        (false, true) => spoken,
        (true, false) => marks,
        (true, true) => String::new(),
    }
}

impl Summarize for RadioTransmitter {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        Summary::new("NOAA Weather Radio transmitter")
            .subtitle(self.call_sign.to_string())
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new(
                        "Call Sign",
                        Some("callSign"),
                        Value::identifier(self.call_sign.to_string()),
                    ),
                    Fact::new(
                        "Frequency",
                        Some("transmitterFrequency"),
                        Value::text(Some(&self.frequency)),
                    ),
                    Fact::new("Site", Some("siteName"), Value::text(Some(&self.site_name))),
                    Fact::new("City", Some("siteCity"), Value::text(Some(&self.city))),
                    Fact::new(
                        "State",
                        Some("siteState"),
                        Value::identifier(self.state.to_string()),
                    ),
                    Fact::new(
                        "SAME Codes",
                        Some("sameCodes"),
                        Value::list(self.same_codes.iter().map(Value::identifier).collect()),
                    ),
                    Fact::new(
                        "Counties",
                        Some("counties"),
                        Value::list(
                            self.counties
                                .iter()
                                .map(ToString::to_string)
                                .map(Value::identifier)
                                .collect(),
                        ),
                    ),
                ],
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@id", "the call sign identifies the same transmitter"),
        ("@type", "always wx:Transmitter"),
        ("setId", "internal transmitter-dataset revision"),
    ];
}

impl Summarize for RadioTransmitterCollection {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NOAA Weather Radio transmitters").subtitle(count_noun(
            self.len(),
            "transmitter",
            "transmitters",
        ));
        if self.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No radio transmitters matched the request".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: transmitter_columns(),
                rows: self.iter().map(transmitter_row).collect(),
            });
        }
        if self.pagination.is_some() {
            summary = summary.note("More transmitters available");
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@graph", "each transmitter is one table row"),
        ("@id", "the call sign identifies the same transmitter"),
        ("@type", "always wx:Transmitter"),
        ("setId", "internal transmitter-dataset revision"),
        ("pagination", "surfaced as the more-transmitters note"),
    ];
}

impl Summarize for RadioBroadcast {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let transcript = self
            .paragraphs
            .iter()
            .map(paragraph_text)
            .filter(|paragraph| !paragraph.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");
        let summary = Summary::new("NOAA Weather Radio Broadcast").push(Section::Facts {
            heading: None,
            facts: vec![
                Fact::new("Language", Some("@xml:lang"), Value::text(Some(&self.lang))),
                Fact::new(
                    "SSML Version",
                    Some("@version"),
                    Value::text(Some(&self.version)),
                ),
            ],
        });
        if transcript.is_empty() {
            summary.push(Section::Empty {
                key: Some("p"),
                message: "This radio broadcast contains no spoken text".to_owned(),
            })
        } else {
            summary.push(Section::Prose {
                heading: Some("Transcript".to_owned()),
                key: Some("p"),
                text: transcript,
            })
        }
    }
}
