//! Human summary for the NWS glossary.

use noaa_weather_client::models::GlossaryResponse;

use crate::{Column, Section, Summarize, Summary, SummaryOptions, Value};

fn count_noun(count: usize) -> String {
    if count == 1 {
        "1 term".to_owned()
    } else {
        format!("{count} terms")
    }
}

impl Summarize for GlossaryResponse {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NWS glossary").subtitle(count_noun(self.glossary.len()));
        if self.glossary.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("glossary"),
                message: "No glossary terms available".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("Term", Some("term")),
                    Column::new("Definition", Some("definition")),
                ],
                rows: self
                    .glossary
                    .iter()
                    .map(|entry| {
                        vec![
                            Value::text(Some(&entry.term)).into(),
                            Value::text(Some(&entry.definition)).into(),
                        ]
                    })
                    .collect(),
            });
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@context", "empty JSON-LD vocabulary metadata"),
        ("glossary", "each glossary entry is one table row"),
    ];
}
