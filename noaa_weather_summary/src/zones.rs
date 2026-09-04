//! Human summaries for zone metadata and text forecasts.

use noaa_weather_client::models::{Zone, ZoneForecast};
use noaa_weather_client::{Feature, FeatureCollection};

use crate::{Cell, Column, Fact, Section, Summarize, Summary, SummaryOptions, Value};

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn state(zone: &Zone) -> Value {
    zone.state.as_ref().map_or(Value::Missing, |state| {
        Value::text(Some(&state.to_string()))
    })
}

fn time_zones(zone: &Zone) -> Value {
    Value::lines(
        zone.time_zone
            .iter()
            .map(|zone| Value::text(zone.iana_name()))
            .collect(),
    )
}

fn observation_stations(zone: &Zone) -> Value {
    Value::list(
        zone.observation_stations
            .iter()
            .map(|url| Value::identifier_from_url(url))
            .collect(),
    )
}

fn zone_row(zone: &Zone) -> Vec<Cell> {
    vec![
        Value::identifier(zone.id.to_string()).into(),
        Value::text(Some(&zone.name)).into(),
        Value::text(Some(&zone.zone_type.to_string())).into(),
        state(zone).into(),
        time_zones(zone).into(),
        Value::identifier_from_url(&zone.forecast_office).into(),
        observation_stations(zone).into(),
    ]
}

fn zone_columns() -> Vec<Column> {
    vec![
        Column::new("Zone ID", Some("id")),
        Column::new("Name", Some("name")),
        Column::new("Type", Some("type")),
        Column::new("State", Some("state")),
        Column::new("Time Zone(s)", Some("timeZone")),
        Column::new("Forecast Office", Some("forecastOffice")),
        Column::new("Observation Stations", Some("observationStations")),
    ]
}

impl Summarize for Feature<Zone> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let zone = &self.properties;
        Summary::new("Zone")
            .subtitle(format!("{} — {}", zone.id, zone.name))
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new(
                        "Zone ID",
                        Some("id"),
                        Value::identifier(zone.id.to_string()),
                    ),
                    Fact::new("Name", Some("name"), Value::text(Some(&zone.name))),
                    Fact::new(
                        "Type",
                        Some("type"),
                        Value::text(Some(&zone.zone_type.to_string())),
                    ),
                    Fact::new("State", Some("state"), state(zone)),
                    Fact::new(
                        "Effective",
                        Some("effectiveDate"),
                        Value::interval(zone.effective_date, Some(zone.expiration_date)),
                    )
                    .also(&["expirationDate"]),
                    Fact::new("Time Zone(s)", Some("timeZone"), time_zones(zone)),
                    Fact::new(
                        "Forecast Office",
                        Some("forecastOffice"),
                        Value::identifier_from_url(&zone.forecast_office),
                    ),
                    Fact::new(
                        "Grid Identifier",
                        Some("gridIdentifier"),
                        Value::identifier(zone.grid_identifier.to_string()),
                    ),
                    Fact::new(
                        "AWIPS Location",
                        Some("awipsLocationIdentifier"),
                        Value::identifier(zone.awips_location_identifier.to_string()),
                    ),
                    Fact::new(
                        "Radar Station",
                        Some("radarStation"),
                        zone.radar_station
                            .as_deref()
                            .map_or(Value::Missing, Value::identifier),
                    ),
                    Fact::new(
                        "Observation Stations",
                        Some("observationStations"),
                        observation_stations(zone),
                    ),
                ],
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        (
            "type",
            "the feature envelope is always Feature; the zone type is shown",
        ),
        (
            "geometry",
            "the zone name and identifier are more useful in text output",
        ),
        (
            "id",
            "the envelope URL duplicates the typed zone identifier",
        ),
        (
            "properties",
            "the zone; its keys are accounted for one by one",
        ),
        (
            "@id",
            "the typed zone identifier is enough for the next command",
        ),
        ("@type", "always wx:Zone"),
        (
            "cwa",
            "deprecated office identifiers duplicate forecastOffice",
        ),
        (
            "forecastOffices",
            "deprecated office URLs duplicate forecastOffice",
        ),
    ];
}

impl Summarize for FeatureCollection<Zone> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("Zones").subtitle(count_noun(self.len(), "zone", "zones"));
        if self.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("features"),
                message: "No zones matched the request".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: zone_columns(),
                rows: self
                    .iter()
                    .map(|feature| zone_row(&feature.properties))
                    .collect(),
            });
        }
        if self.pagination.is_some() {
            summary = summary.note("More zones available");
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        (
            "type",
            "always FeatureCollection or Feature; the zone type is shown",
        ),
        ("features", "each zone is one table row"),
        ("geometry", "catalog rows use the zone name and identifier"),
        ("id", "the feature URL duplicates the typed zone identifier"),
        (
            "properties",
            "each zone's keys are accounted for one by one",
        ),
        (
            "@id",
            "the typed zone identifier is enough for the next command",
        ),
        ("@type", "always wx:Zone"),
        ("effectiveDate", "catalog rows omit zone-definition history"),
        (
            "expirationDate",
            "catalog rows omit zone-definition history",
        ),
        (
            "cwa",
            "deprecated office identifiers duplicate forecastOffice",
        ),
        (
            "forecastOffices",
            "deprecated office URLs duplicate forecastOffice",
        ),
        (
            "gridIdentifier",
            "available from the single-zone metadata command",
        ),
        (
            "awipsLocationIdentifier",
            "available from the single-zone metadata command",
        ),
        (
            "radarStation",
            "available from the single-zone metadata command",
        ),
        ("title", "zone collections do not carry a title"),
        ("updated", "zone collections do not carry an update time"),
        ("pagination", "surfaced as the more-zones note"),
    ];
}

impl Summarize for Feature<ZoneForecast> {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let forecast = &self.properties;
        let mut summary = Summary::new("Zone forecast")
            .subtitle(
                forecast
                    .zone_id()
                    .map_or_else(|| forecast.zone.clone(), |id| id.to_string()),
            )
            .push(Section::Facts {
                heading: None,
                facts: vec![
                    Fact::new(
                        "Zone ID",
                        Some("zone"),
                        Value::identifier_from_url(&forecast.zone),
                    ),
                    Fact::new(
                        "Updated",
                        Some("updated"),
                        Value::timestamp(forecast.updated),
                    ),
                ],
            });

        if forecast.periods.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("periods"),
                message: "No forecast periods available".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("Day/Night", Some("name")).also(&["periods"]),
                    Column::new("Forecast", Some("detailedForecast")),
                ],
                rows: forecast
                    .periods
                    .iter()
                    .map(|period| {
                        vec![
                            Value::text(Some(&period.name)).into(),
                            Value::text(Some(&period.detailed_forecast)).into(),
                        ]
                    })
                    .collect(),
            });
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "the feature envelope is always Feature"),
        ("geometry", "the forecast text identifies the zone"),
        (
            "properties",
            "the forecast; its keys are accounted for one by one",
        ),
        ("number", "period order is already preserved by the table"),
    ];
}
