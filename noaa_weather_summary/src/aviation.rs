//! Summaries for Center Weather Service Unit offices, Center Weather
//! Advisories, SIGMETs, and AIRMETs.
//!
//! The single-product summaries read as fact sheets. Collection responses use
//! the same facts as columns, so a caller can scan products without losing the
//! identifiers and validity windows needed for a follow-up request.

use noaa_weather_client::models::{CenterWeatherAdvisory, CwsuOffice, Sigmet};
use noaa_weather_client::{Feature, FeatureCollection};

use crate::{Align, Column, Fact, Section, Summarize, Summary, SummaryOptions, Value};

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn address(office: &CwsuOffice) -> Value {
    let address = &office.address;
    let city = address.address_locality.trim();
    let region = address.address_region.trim();
    let postal = address.postal_code.trim();
    let city_and_region = match (city.is_empty(), region.is_empty()) {
        (false, false) => format!("{city}, {region}"),
        (false, true) => city.to_owned(),
        (true, false) => region.to_owned(),
        (true, true) => String::new(),
    };
    let locality = [city_and_region, postal.to_owned()]
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

fn optional_identifier_from_url(url: Option<&str>) -> Value {
    url.map_or(Value::Missing, Value::identifier_from_url)
}

fn cwa_row(advisory: &CenterWeatherAdvisory) -> Vec<crate::Cell> {
    vec![
        Value::identifier_from_url(&advisory.id).into(),
        Value::timestamp(advisory.issue_time).into(),
        Value::identifier(advisory.cwsu.to_string()).into(),
        Value::count(u64::from(advisory.sequence)).into(),
        Value::interval(advisory.start, Some(advisory.end)).into(),
        optional_identifier_from_url(advisory.observed_property.as_deref()).into(),
        Value::text(Some(&advisory.text)).into(),
    ]
}

fn sigmet_row(sigmet: &Sigmet) -> Vec<crate::Cell> {
    vec![
        Value::identifier_from_url(&sigmet.id).into(),
        Value::timestamp(sigmet.issue_time).into(),
        Value::identifier(sigmet.atsu.to_string()).into(),
        Value::text(sigmet.fir.as_deref()).into(),
        Value::text(sigmet.sequence.as_deref()).into(),
        optional_identifier_from_url(sigmet.phenomenon.as_deref()).into(),
        Value::interval(sigmet.start, Some(sigmet.end)).into(),
    ]
}

impl Summarize for CwsuOffice {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        Summary::new(self.name.clone())
            .subtitle(format!("Center Weather Service Unit {}", self.id))
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new("ID", Some("id"), Value::identifier(self.id.to_string())),
                    Fact::new("Address", Some("address"), address(self)),
                    Fact::new(
                        "Phone",
                        Some("telephone"),
                        Value::text(Some(&self.telephone)),
                    ),
                    Fact::new(
                        "Fax",
                        Some("faxNumber"),
                        Value::text(Some(&self.fax_number)),
                    ),
                    Fact::new("Email", Some("email"), Value::text(Some(&self.email))),
                    Fact::new("Website", Some("sameAs"), Value::text(Some(&self.same_as))),
                    Fact::new(
                        "Region",
                        Some("nwsRegion"),
                        Value::text(Some(&self.nws_region)),
                    ),
                ],
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@context", "fixed JSON-LD vocabulary metadata"),
        ("@id", "the API URL; the CWSU id identifies the same office"),
        ("@type", "always GovernmentOrganization"),
        ("name", "shown as the summary title"),
    ];
}

impl Summarize for Feature<CenterWeatherAdvisory> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let advisory = &self.properties;
        Summary::new("Center Weather Advisory")
            .subtitle(format!("{} sequence {}", advisory.cwsu, advisory.sequence))
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new("ID", Some("id"), Value::identifier_from_url(&advisory.id)),
                    Fact::new(
                        "Issue time",
                        Some("issueTime"),
                        Value::timestamp(advisory.issue_time),
                    ),
                    Fact::new(
                        "CWSU",
                        Some("cwsu"),
                        Value::identifier(advisory.cwsu.to_string()),
                    ),
                    Fact::new(
                        "Sequence",
                        Some("sequence"),
                        Value::count(u64::from(advisory.sequence)),
                    ),
                    Fact::new(
                        "Valid",
                        Some("start"),
                        Value::interval(advisory.start, Some(advisory.end)),
                    )
                    .also(&["end"]),
                    Fact::new(
                        "Basis",
                        Some("observedProperty"),
                        optional_identifier_from_url(advisory.observed_property.as_deref()),
                    ),
                ],
            })
            .push(Section::Prose {
                heading: Some("Advisory".to_owned()),
                key: Some("text"),
                text: advisory.text.clone(),
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always Feature"),
        (
            "geometry",
            "polygon coordinates are not useful in a text summary",
        ),
        (
            "properties",
            "the advisory; its keys are accounted for one by one",
        ),
    ];
}

impl Summarize for FeatureCollection<CenterWeatherAdvisory> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new(
            self.title
                .clone()
                .unwrap_or_else(|| "Center Weather Advisories".to_owned()),
        )
        .subtitle(count_noun(self.len(), "advisory", "advisories"));
        if self.is_empty() {
            summary = summary.push(Section::Empty {
                key: None,
                message: "No current Center Weather Advisories".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("ID", Some("id")),
                    Column::new("Issue time", Some("issueTime")),
                    Column::new("CWSU", Some("cwsu")),
                    Column::new("Sequence", Some("sequence")).align(Align::Right),
                    Column::new("Valid", Some("start")).also(&["end"]),
                    Column::new("Basis", Some("observedProperty")),
                    Column::new("Advisory", Some("text")),
                ],
                rows: self
                    .iter()
                    .map(|feature| cwa_row(&feature.properties))
                    .collect(),
            });
        }
        if self.pagination.is_some() {
            summary = summary.note("More advisories available");
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always FeatureCollection or Feature"),
        ("features", "each advisory is one table row"),
        (
            "geometry",
            "polygon coordinates are not useful in a text summary",
        ),
        ("title", "shown as the summary title"),
        ("updated", "the products carry their own issue times"),
        ("pagination", "surfaced as the more-advisories note"),
    ];
}

impl Summarize for Feature<Sigmet> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let sigmet = &self.properties;
        let subtitle = sigmet.sequence.as_deref().map_or_else(
            || sigmet.atsu.to_string(),
            |sequence| format!("{} sequence {sequence}", sigmet.atsu),
        );
        Summary::new("SIGMET or AIRMET")
            .subtitle(subtitle)
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new("ID", Some("id"), Value::identifier_from_url(&sigmet.id)),
                    Fact::new(
                        "Issue time",
                        Some("issueTime"),
                        Value::timestamp(sigmet.issue_time),
                    ),
                    Fact::new(
                        "ATSU",
                        Some("atsu"),
                        Value::identifier(sigmet.atsu.to_string()),
                    ),
                    Fact::new("FIR", Some("fir"), Value::text(sigmet.fir.as_deref())),
                    Fact::new(
                        "Sequence",
                        Some("sequence"),
                        Value::text(sigmet.sequence.as_deref()),
                    ),
                    Fact::new(
                        "Phenomenon",
                        Some("phenomenon"),
                        optional_identifier_from_url(sigmet.phenomenon.as_deref()),
                    ),
                    Fact::new(
                        "Valid",
                        Some("start"),
                        Value::interval(sigmet.start, Some(sigmet.end)),
                    )
                    .also(&["end"]),
                ],
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always Feature"),
        (
            "geometry",
            "polygon coordinates are not useful in a text summary",
        ),
        (
            "properties",
            "the product; its keys are accounted for one by one",
        ),
    ];
}

impl Summarize for FeatureCollection<Sigmet> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new(
            self.title
                .clone()
                .unwrap_or_else(|| "SIGMETs and AIRMETs".to_owned()),
        )
        .subtitle(count_noun(self.len(), "product", "products"));
        if self.is_empty() {
            summary = summary.push(Section::Empty {
                key: None,
                message: "No current SIGMETs or AIRMETs".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("ID", Some("id")),
                    Column::new("Issue time", Some("issueTime")),
                    Column::new("ATSU", Some("atsu")),
                    Column::new("FIR", Some("fir")),
                    Column::new("Sequence", Some("sequence")),
                    Column::new("Phenomenon", Some("phenomenon")),
                    Column::new("Valid", Some("start")).also(&["end"]),
                ],
                rows: self
                    .iter()
                    .map(|feature| sigmet_row(&feature.properties))
                    .collect(),
            });
        }
        if self.pagination.is_some() {
            summary = summary.note("More products available");
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always FeatureCollection or Feature"),
        ("features", "each product is one table row"),
        (
            "geometry",
            "polygon coordinates are not useful in a text summary",
        ),
        ("title", "shown as the summary title"),
        ("updated", "the products carry their own issue times"),
        ("pagination", "surfaced as the more-products note"),
    ];
}
