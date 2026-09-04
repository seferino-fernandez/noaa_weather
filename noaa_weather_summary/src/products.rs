//! Human summaries for NWS text products and their catalogs.

use noaa_weather_client::models::{
    TextProduct, TextProductCollection, TextProductLocationCollection, TextProductTypeCollection,
};

use crate::{Column, Fact, Section, Summarize, Summary, SummaryOptions, Value};

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("1 {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn product_columns() -> Vec<Column> {
    vec![
        Column::new("ID", Some("id")),
        Column::new("Product Code", Some("productCode")),
        Column::new("Issuing Office", Some("issuingOffice")),
        Column::new("Issuance Time", Some("issuanceTime")),
    ]
}

impl Summarize for TextProduct {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NWS text product")
            .subtitle(format!("{} — {}", self.product_code, self.product_name))
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new("ID", Some("id"), Value::identifier(self.id.to_string())),
                    Fact::new(
                        "WMO Collective ID",
                        Some("wmoCollectiveId"),
                        Value::identifier(&self.wmo_collective_id),
                    ),
                    Fact::new(
                        "Issuing Office",
                        Some("issuingOffice"),
                        Value::identifier(self.issuing_office.to_string()),
                    ),
                    Fact::new(
                        "Issuance Time",
                        Some("issuanceTime"),
                        Value::timestamp(self.issuance_time),
                    ),
                    Fact::new(
                        "Product Code",
                        Some("productCode"),
                        Value::identifier(self.product_code.to_string()),
                    ),
                    Fact::new(
                        "Product Name",
                        Some("productName"),
                        Value::text(Some(&self.product_name)),
                    ),
                ],
            });

        summary = match self.product_text.as_deref().map(str::trim) {
            Some(text) if !text.is_empty() => summary.push(Section::Prose {
                heading: Some("Product text".to_owned()),
                key: Some("productText"),
                text: text.to_owned(),
            }),
            _ => summary.push(Section::Empty {
                key: Some("productText"),
                message: "No product text available".to_owned(),
            }),
        };
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] =
        &[("@id", "the server-issued product identifier is shown")];
}

impl Summarize for TextProductCollection {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NWS text products").subtitle(count_noun(
            self.at_graph.len(),
            "product",
            "products",
        ));
        if self.at_graph.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No text products matched the request".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: product_columns(),
                rows: self
                    .at_graph
                    .iter()
                    .map(|product| {
                        vec![
                            Value::identifier(product.id.to_string()).into(),
                            Value::identifier(product.product_code.to_string()).into(),
                            Value::identifier(product.issuing_office.to_string()).into(),
                            Value::timestamp(product.issuance_time).into(),
                        ]
                    })
                    .collect(),
            });
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("@graph", "each product is one table row"),
        ("@id", "the server-issued product identifier is shown"),
        (
            "wmoCollectiveId",
            "the product code, office, and issuance time identify catalog rows",
        ),
        (
            "productName",
            "the product code is compact; names are available from the product-types command",
        ),
        (
            "productText",
            "catalog endpoints omit the full product text",
        ),
    ];
}

impl Summarize for TextProductLocationCollection {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NWS product locations").subtitle(count_noun(
            self.locations.len(),
            "location",
            "locations",
        ));
        if self.locations.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("locations"),
                message: "No product locations available".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("Location ID", Some("locations")),
                    Column::new("Location Name", None),
                ],
                rows: self
                    .locations
                    .iter()
                    .map(|(id, name)| {
                        vec![
                            Value::identifier(id).into(),
                            Value::text(name.as_deref()).into(),
                        ]
                    })
                    .collect(),
            });
        }
        summary
    }
}

impl Summarize for TextProductTypeCollection {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("NWS product types").subtitle(count_noun(
            self.at_graph.len(),
            "type",
            "types",
        ));
        if self.at_graph.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No product types available".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("Product Code", Some("productCode")).also(&["@graph"]),
                    Column::new("Product Name", Some("productName")),
                ],
                rows: self
                    .at_graph
                    .iter()
                    .map(|product_type| {
                        vec![
                            Value::identifier(product_type.product_code.to_string()).into(),
                            Value::text(Some(&product_type.product_name)).into(),
                        ]
                    })
                    .collect(),
            });
        }
        summary
    }
}
