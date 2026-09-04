//! Human summaries for NWS office metadata, news, briefings, and weather stories.

use noaa_weather_client::models::{
    NwsConnectDocumentMetadata, Office, OfficeAddress, OfficeBriefingResponse, OfficeHeadline,
    OfficeHeadlineCollection, OfficeWeatherStory, OfficeWeatherStoryCollection,
};

use crate::{
    Align, Cell, Column, Emphasis, Fact, Section, Summarize, Summary, SummaryOptions, Value,
};

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn address(address: &OfficeAddress) -> Value {
    let city = address.city.trim();
    let state = address.state.to_string();
    let postal = address.postal_code.trim();
    let locality = [
        match (city.is_empty(), state.is_empty()) {
            (false, false) => format!("{city}, {state}"),
            (false, true) => city.to_owned(),
            (true, false) => state,
            (true, true) => String::new(),
        },
        postal.to_owned(),
    ]
    .into_iter()
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    Value::lines(
        [address.street_address.trim().to_owned(), locality]
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| Value::text(Some(&line)))
            .collect(),
    )
}

fn optional_count(values: Option<&[String]>) -> Value {
    values.map_or(Value::Missing, |values| Value::count(values.len() as u64))
}

fn optional_identifier_from_url(url: Option<&str>) -> Value {
    url.map_or(Value::Missing, Value::identifier_from_url)
}

fn headline_columns() -> Vec<Column> {
    vec![
        Column::new("ID", Some("id")).also(&["@graph"]),
        Column::new("Title", Some("title")),
        Column::new("Summary", Some("summary")),
        Column::new("Important", Some("important")),
        Column::new("Issued", Some("issuanceTime")),
        Column::new("Link", Some("link")),
    ]
}

fn headline_row(headline: &OfficeHeadline) -> Vec<Cell> {
    vec![
        Value::identifier(&headline.id).into(),
        Value::text(Some(&headline.title)).into(),
        Value::text(headline.summary.as_deref()).into(),
        Cell::new(
            Value::yes_no(Some(headline.important)),
            if headline.important {
                Emphasis::Notice
            } else {
                Emphasis::None
            },
        ),
        Value::timestamp(headline.issuance_time).into(),
        Value::text(Some(&headline.link)).into(),
    ]
}

fn briefing_facts(document: &NwsConnectDocumentMetadata) -> Vec<Fact> {
    vec![
        Fact::new("ID", Some("id"), Value::identifier(&document.id)).also(&["briefing"]),
        Fact::new(
            "Office",
            Some("officeId"),
            Value::identifier(document.office_id.to_string()),
        ),
        Fact::new(
            "Active",
            Some("startTime"),
            Value::interval(document.start_time, Some(document.end_time)),
        )
        .also(&["endTime"]),
        Fact::new(
            "Updated",
            Some("updateTime"),
            Value::timestamp(document.update_time),
        ),
        Fact::new("Title", Some("title"), Value::text(Some(&document.title))),
        Fact::new(
            "Description",
            Some("description"),
            Value::text(Some(&document.description)),
        ),
        Fact::new(
            "Priority",
            Some("priority"),
            Value::yes_no(Some(document.priority)),
        )
        .with_emphasis(if document.priority {
            Emphasis::Notice
        } else {
            Emphasis::None
        }),
        Fact::new(
            "Download",
            Some("download"),
            Value::text(Some(&document.download)),
        ),
    ]
}

fn story_columns() -> Vec<Column> {
    vec![
        Column::new("Story ID", Some("download")).also(&["stories"]),
        Column::new("Title", Some("title")).also(&["priority"]),
        Column::new("Description", Some("description")),
        Column::new("Alt Text", Some("altText")),
        Column::new("Order", Some("order")).align(Align::Right),
    ]
}

fn story_row(story: &OfficeWeatherStory) -> Vec<Cell> {
    vec![
        Value::identifier_from_url(&story.download).into(),
        Cell::new(
            Value::text(Some(&story.title)),
            if story.priority {
                Emphasis::Notice
            } else {
                Emphasis::None
            },
        ),
        Value::text(Some(&story.description)).into(),
        Value::text(Some(&story.alt_text)).into(),
        Value::count(u64::from(story.order)).into(),
    ]
}

impl Summarize for Office {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        Summary::new(self.name.clone())
            .subtitle(format!("NWS office {}", self.id))
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new("ID", Some("id"), Value::identifier(self.id.to_string())),
                    Fact::new("Address", Some("address"), address(&self.address)).also(&[
                        "streetAddress",
                        "addressLocality",
                        "addressRegion",
                        "postalCode",
                    ]),
                    Fact::new(
                        "Phone",
                        Some("telephone"),
                        Value::text(Some(&self.phone_number)),
                    ),
                    Fact::new(
                        "Fax",
                        Some("faxNumber"),
                        Value::text(Some(&self.fax_number)),
                    ),
                    Fact::new("Email", Some("email"), Value::text(Some(&self.email))),
                    Fact::new(
                        "Website",
                        Some("sameAs"),
                        Value::text(Some(&self.website_url)),
                    ),
                    Fact::new(
                        "Region",
                        Some("nwsRegion"),
                        Value::text(self.nws_region.as_deref()),
                    ),
                    Fact::new(
                        "Parent Organization",
                        Some("parentOrganization"),
                        optional_identifier_from_url(self.parent_organization.as_deref()),
                    ),
                    Fact::new(
                        "Responsible Counties",
                        Some("responsibleCounties"),
                        optional_count(self.responsible_counties.as_deref()),
                    ),
                    Fact::new(
                        "Forecast Zones",
                        Some("responsibleForecastZones"),
                        optional_count(self.responsible_forecast_zones.as_deref()),
                    ),
                    Fact::new(
                        "Fire Zones",
                        Some("responsibleFireZones"),
                        optional_count(self.responsible_fire_zones.as_deref()),
                    ),
                    Fact::new(
                        "Observation Stations",
                        Some("approvedObservationStations"),
                        optional_count(self.approved_observation_stations.as_deref()),
                    ),
                ],
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@id", "the office identifier addresses the same resource"),
        ("@type", "fixed organization and postal-address types"),
        ("name", "shown as the summary title"),
    ];
}

impl Summarize for OfficeHeadline {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new(self.title.clone())
            .subtitle(self.office_id().map_or_else(
                || "NWS office headline".to_owned(),
                |id| format!("NWS {id} headline"),
            ))
            .emphasis(if self.important {
                Emphasis::Notice
            } else {
                Emphasis::None
            })
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new("ID", Some("id"), Value::identifier(&self.id)),
                    Fact::new(
                        "Office",
                        Some("office"),
                        Value::identifier_from_url(&self.office),
                    ),
                    Fact::new(
                        "Important",
                        Some("important"),
                        Value::yes_no(Some(self.important)),
                    ),
                    Fact::new(
                        "Issued",
                        Some("issuanceTime"),
                        Value::timestamp(self.issuance_time),
                    ),
                    Fact::new("Link", Some("link"), Value::text(Some(&self.link))),
                    Fact::new("Name", Some("name"), Value::identifier(&self.name)),
                ],
            });

        summary = match self.summary.as_deref().map(str::trim) {
            Some(text) if !text.is_empty() => summary.push(Section::Prose {
                heading: Some("Summary".to_owned()),
                key: Some("summary"),
                text: text.to_owned(),
            }),
            _ => summary.push(Section::Empty {
                key: Some("summary"),
                message: "No short summary supplied".to_owned(),
            }),
        };
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@id", "the server-issued headline identifier is shown"),
        ("title", "shown as the summary title"),
        ("content", "raw HTML is available with JSON output"),
    ];
}

impl Summarize for OfficeHeadlineCollection {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NWS office headlines").subtitle(count_noun(
            self.at_graph.len(),
            "headline",
            "headlines",
        ));
        if self.at_graph.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("@graph"),
                message: "This office has no current headlines".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: headline_columns(),
                rows: self.at_graph.iter().map(headline_row).collect(),
            });
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@id", "the server-issued headline identifier is shown"),
        (
            "office",
            "the command already identifies the publishing office",
        ),
        ("name", "the title is more descriptive in a headline list"),
        ("content", "raw HTML is available with JSON output"),
    ];
}

impl Summarize for OfficeBriefingResponse {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        match &self.briefing {
            Some(document) => Summary::new("NWS office briefing")
                .subtitle(document.title.clone())
                .push(Section::Facts {
                    heading: None,
                    facts: briefing_facts(document),
                }),
            None => Summary::new("NWS office briefing").push(Section::Empty {
                key: Some("briefing"),
                message: "This office has no active briefing".to_owned(),
            }),
        }
    }
}

impl Summarize for OfficeWeatherStoryCollection {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NWS office weather stories").subtitle(count_noun(
            self.stories.len(),
            "story",
            "stories",
        ));
        if self.stories.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("stories"),
                message: "This office has no active weather stories".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: story_columns(),
                rows: self.stories.iter().map(story_row).collect(),
            });
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        (
            "officeId",
            "the command already identifies the publishing office",
        ),
        (
            "startTime",
            "publication scheduling metadata is available with JSON output",
        ),
        (
            "endTime",
            "publication scheduling metadata is available with JSON output",
        ),
        (
            "updateTime",
            "publication scheduling metadata is available with JSON output",
        ),
    ];
}
