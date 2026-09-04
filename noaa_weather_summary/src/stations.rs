//! Human summaries for observation stations, surface observations, and TAFs.

use noaa_weather_client::geo::Geometry;
use noaa_weather_client::models::terminal_aerodrome_forecast::{
    CloudAmount, CloudLayer, CloudType, Comparison, ForecastClouds, ForecastConditions,
    ForecastElement, ForecastGroup, ForecastGroupKind, ForecastReport, ForecastValue,
    ForecastWeather, ForecastWind, MissingForecastReason, MissingReason, PermissibleUsage,
    PermissibleUsageReason, ReportStatus, SurfaceWind, TemperatureForecast, TimeRange, Weather,
    WeatherDescriptor, WeatherIntensity, WeatherPhenomenon, WindDirection, WindSpeed,
};
use noaa_weather_client::models::{
    Observation, ObservationCloudLayer, ObservationStation, TerminalAerodromeForecast,
    TerminalAerodromeForecastsResponse,
};
use noaa_weather_client::{Feature, FeatureCollection};
use serde::Serialize;

use crate::{
    Align, Cell, Column, Fact, QuantityKind, Section, Summarize, Summary, SummaryOptions, Value,
};

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

fn identifier_from_optional_url(url: Option<&str>) -> Value {
    url.map_or(Value::Missing, Value::identifier_from_url)
}

fn quantity_or_missing(
    quantity: Option<&noaa_weather_client::models::Quantity>,
    kind: QuantityKind,
    options: &SummaryOptions,
) -> Value {
    quantity.map_or(Value::Missing, |quantity| {
        Value::quantity(quantity, kind, options)
    })
}

fn point(geometry: Option<&Geometry>) -> Value {
    match geometry {
        Some(Geometry::Point(position)) => Value::coordinates(position.lat(), position.lon()),
        _ => Value::Missing,
    }
}

fn provider(station: &ObservationStation) -> Value {
    Value::lines(
        [&station.provider, &station.sub_provider]
            .into_iter()
            .filter(|value| !value.trim().is_empty())
            .map(|value| Value::text(Some(value)))
            .collect(),
    )
}

fn station_zones(station: &ObservationStation) -> Value {
    Value::lines(vec![
        identifier_from_optional_url(station.forecast.as_deref()),
        identifier_from_optional_url(station.county.as_deref()),
        identifier_from_optional_url(station.fire_weather_zone.as_deref()),
    ])
}

fn station_feature_row(
    station: &Feature<ObservationStation>,
    options: &SummaryOptions,
) -> Vec<Cell> {
    let geometry = station.geometry.as_ref();
    let station = &station.properties;
    vec![
        Value::identifier(station.station_identifier.to_string()).into(),
        Value::text(Some(&station.name)).into(),
        Value::quantity(&station.elevation, QuantityKind::Height, options).into(),
        Value::text(station.time_zone.iana_name()).into(),
        point(geometry).into(),
        provider(station).into(),
        station_zones(station).into(),
        quantity_or_missing(station.distance.as_ref(), QuantityKind::Distance, options).into(),
        quantity_or_missing(station.bearing.as_ref(), QuantityKind::Angle, options).into(),
    ]
}

fn station_columns() -> Vec<Column> {
    vec![
        Column::new("Station ID", Some("stationIdentifier")),
        Column::new("Name", Some("name")),
        Column::new("Elevation", Some("elevation")).align(Align::Right),
        Column::new("Time Zone", Some("timeZone")),
        Column::new("Coordinates", Some("geometry")),
        Column::new("Provider", Some("provider")).also(&["subProvider"]),
        Column::new("Zones", Some("forecast")).also(&["county", "fireWeatherZone"]),
        Column::new("Distance", Some("distance")).align(Align::Right),
        Column::new("Bearing", Some("bearing")).align(Align::Right),
    ]
}

impl Summarize for Feature<ObservationStation> {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        Summary::new("Observation station")
            .subtitle(self.properties.name.clone())
            .push(Section::Table {
                heading: None,
                columns: station_columns(),
                rows: vec![station_feature_row(self, options)],
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always Feature"),
        ("id", "the envelope URL duplicates the station property URL"),
        (
            "properties",
            "the station; its keys are accounted for one by one",
        ),
        (
            "@id",
            "the station identifier is enough for the next command",
        ),
        ("@type", "always wx:ObservationStation"),
    ];
}

impl Summarize for FeatureCollection<ObservationStation> {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("Observation stations").subtitle(count_noun(
            self.len(),
            "station",
            "stations",
        ));
        if self.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("features"),
                message: "No observation stations".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: station_columns(),
                rows: self
                    .iter()
                    .map(|station| station_feature_row(station, options))
                    .collect(),
            });
        }
        if self.pagination.is_some() {
            summary = summary.note("More stations available");
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always FeatureCollection or Feature"),
        ("features", "each station is one table row"),
        ("id", "the envelope URL duplicates the station property URL"),
        (
            "properties",
            "each station's keys are accounted for one by one",
        ),
        (
            "@id",
            "the station identifier is enough for the next command",
        ),
        ("@type", "always wx:ObservationStation"),
        ("title", "station collections do not carry a title"),
        ("updated", "station collections do not carry an update time"),
        ("pagination", "surfaced as the more-stations note"),
    ];
}

fn present_weather(observation: &Observation) -> Value {
    Value::lines(
        observation
            .present_weather
            .iter()
            .map(|weather| Value::text(Some(&weather.raw_string)))
            .collect(),
    )
}

fn cloud_facts(clouds: &[ObservationCloudLayer], options: &SummaryOptions) -> Vec<Fact> {
    if clouds.is_empty() {
        return vec![Fact::new(
            "Clouds",
            Some("cloudLayers"),
            Value::text(Some("Clear")),
        )];
    }
    clouds
        .iter()
        .enumerate()
        .map(|(index, layer)| {
            Fact::new(
                format!("Cloud {} ({})", index + 1, layer.amount),
                (index == 0).then_some("cloudLayers"),
                Value::quantity(&layer.base, QuantityKind::Height, options),
            )
        })
        .collect()
}

impl Summarize for Feature<Observation> {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let observation = &self.properties;
        let mut facts = vec![
            Fact::new(
                "Observed",
                Some("timestamp"),
                Value::timestamp(observation.timestamp),
            ),
            Fact::new(
                "Conditions",
                Some("textDescription"),
                Value::text(Some(&observation.text_description)),
            ),
            Fact::new(
                "Present weather",
                Some("presentWeather"),
                present_weather(observation),
            ),
            Fact::new(
                "Elevation",
                Some("elevation"),
                Value::quantity(&observation.elevation, QuantityKind::Height, options),
            ),
            Fact::new(
                "Temperature",
                Some("temperature"),
                Value::quantity(&observation.temperature, QuantityKind::Temperature, options),
            ),
            Fact::new(
                "Dewpoint",
                Some("dewpoint"),
                Value::quantity(&observation.dewpoint, QuantityKind::Temperature, options),
            ),
            Fact::new(
                "Wind direction",
                Some("windDirection"),
                Value::quantity(&observation.wind_direction, QuantityKind::Angle, options),
            ),
            Fact::new(
                "Wind speed",
                Some("windSpeed"),
                Value::quantity(&observation.wind_speed, QuantityKind::Speed, options),
            ),
            Fact::new(
                "Wind gust",
                Some("windGust"),
                Value::quantity(&observation.wind_gust, QuantityKind::Speed, options),
            ),
            Fact::new(
                "Barometric pressure",
                Some("barometricPressure"),
                Value::quantity(
                    &observation.barometric_pressure,
                    QuantityKind::Pressure,
                    options,
                ),
            ),
            Fact::new(
                "Sea-level pressure",
                Some("seaLevelPressure"),
                Value::quantity(
                    &observation.sea_level_pressure,
                    QuantityKind::Pressure,
                    options,
                ),
            ),
            Fact::new(
                "Visibility",
                Some("visibility"),
                Value::quantity(&observation.visibility, QuantityKind::Distance, options),
            ),
            Fact::new(
                "Relative humidity",
                Some("relativeHumidity"),
                Value::quantity(
                    &observation.relative_humidity,
                    QuantityKind::Percent,
                    options,
                ),
            ),
            Fact::new(
                "Wind chill",
                Some("windChill"),
                Value::quantity(&observation.wind_chill, QuantityKind::Temperature, options),
            ),
            Fact::new(
                "Heat index",
                Some("heatIndex"),
                Value::quantity(&observation.heat_index, QuantityKind::Temperature, options),
            ),
            Fact::new(
                "24-hour high",
                Some("maxTemperatureLast24Hours"),
                Value::quantity(
                    &observation.max_temperature_last24_hours,
                    QuantityKind::Temperature,
                    options,
                ),
            ),
            Fact::new(
                "24-hour low",
                Some("minTemperatureLast24Hours"),
                Value::quantity(
                    &observation.min_temperature_last24_hours,
                    QuantityKind::Temperature,
                    options,
                ),
            ),
            Fact::new(
                "Precipitation, 1 hour",
                Some("precipitationLastHour"),
                quantity_or_missing(
                    observation.precipitation_last_hour.as_ref(),
                    QuantityKind::Depth,
                    options,
                ),
            ),
            Fact::new(
                "Precipitation, 3 hours",
                Some("precipitationLast3Hours"),
                Value::quantity(
                    &observation.precipitation_last3_hours,
                    QuantityKind::Depth,
                    options,
                ),
            ),
            Fact::new(
                "Precipitation, 6 hours",
                Some("precipitationLast6Hours"),
                quantity_or_missing(
                    observation.precipitation_last6_hours.as_ref(),
                    QuantityKind::Depth,
                    options,
                ),
            ),
        ];
        facts.extend(cloud_facts(&observation.cloud_layers, options));
        Summary::new(format!("Station: {} - Observation", observation.station_id))
            .subtitle(observation.station_name.clone())
            .push(Section::Facts {
                heading: None,
                facts,
            })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always Feature"),
        ("id", "the observation URL duplicates the property URL"),
        (
            "geometry",
            "station metadata provides the fixed station coordinates",
        ),
        (
            "properties",
            "the observation; its keys are accounted for one by one",
        ),
        ("@id", "the timestamp and station identify the observation"),
        ("@type", "always wx:ObservationStation"),
        ("station", "the station id is shown in the title"),
        ("stationId", "shown in the title"),
        ("stationName", "shown as the subtitle"),
        (
            "rawMessage",
            "decoded measurements and weather are shown instead",
        ),
        (
            "icon",
            "the text description carries the same condition meaning",
        ),
    ];
}

fn observation_row(observation: &Observation, options: &SummaryOptions) -> Vec<Cell> {
    vec![
        Value::timestamp(observation.timestamp).into(),
        Value::quantity(&observation.temperature, QuantityKind::Temperature, options).into(),
        Value::quantity(&observation.dewpoint, QuantityKind::Temperature, options).into(),
        Value::quantity(&observation.wind_direction, QuantityKind::Angle, options).into(),
        Value::quantity(&observation.wind_speed, QuantityKind::Speed, options).into(),
        Value::quantity(&observation.wind_gust, QuantityKind::Speed, options).into(),
        Value::quantity(
            &observation.barometric_pressure,
            QuantityKind::Pressure,
            options,
        )
        .into(),
        Value::quantity(
            &observation.sea_level_pressure,
            QuantityKind::Pressure,
            options,
        )
        .into(),
        Value::quantity(&observation.visibility, QuantityKind::Distance, options).into(),
        Value::quantity(
            &observation.relative_humidity,
            QuantityKind::Percent,
            options,
        )
        .into(),
        Value::quantity(&observation.wind_chill, QuantityKind::Temperature, options).into(),
        Value::quantity(&observation.heat_index, QuantityKind::Temperature, options).into(),
    ]
}

impl Summarize for FeatureCollection<Observation> {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("Station observations").subtitle(count_noun(
            self.len(),
            "observation",
            "observations",
        ));
        if self.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("features"),
                message: "No observations".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("Timestamp", Some("timestamp")),
                    Column::new("Temperature", Some("temperature")),
                    Column::new("Dewpoint", Some("dewpoint")),
                    Column::new("Wind direction", Some("windDirection")),
                    Column::new("Wind speed", Some("windSpeed")),
                    Column::new("Wind gust", Some("windGust")),
                    Column::new("Barometric pressure", Some("barometricPressure")),
                    Column::new("Sea-level pressure", Some("seaLevelPressure")),
                    Column::new("Visibility", Some("visibility")),
                    Column::new("Relative humidity", Some("relativeHumidity")),
                    Column::new("Wind chill", Some("windChill")),
                    Column::new("Heat index", Some("heatIndex")),
                ],
                rows: self
                    .iter()
                    .map(|item| observation_row(&item.properties, options))
                    .collect(),
            });
        }
        if self.pagination.is_some() {
            summary = summary.note("More observations available");
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always FeatureCollection or Feature"),
        ("features", "each observation is one table row"),
        ("id", "the timestamp and station identify each observation"),
        (
            "geometry",
            "station metadata provides the fixed coordinates",
        ),
        (
            "properties",
            "each observation's keys are accounted for one by one",
        ),
        ("@id", "the timestamp and station identify each observation"),
        ("@type", "always wx:ObservationStation"),
        ("elevation", "station metadata provides elevation"),
        ("station", "the command already names the station"),
        ("stationId", "the command already names the station"),
        ("stationName", "the command already names the station"),
        ("rawMessage", "decoded measurements are shown instead"),
        (
            "textDescription",
            "omitted to keep the history table scannable",
        ),
        ("icon", "not useful in a text history table"),
        (
            "presentWeather",
            "omitted to keep the history table scannable",
        ),
        (
            "maxTemperatureLast24Hours",
            "duplicated across adjacent history rows",
        ),
        (
            "minTemperatureLast24Hours",
            "duplicated across adjacent history rows",
        ),
        (
            "precipitationLastHour",
            "omitted to keep the history table scannable",
        ),
        (
            "precipitationLast3Hours",
            "omitted to keep the history table scannable",
        ),
        (
            "precipitationLast6Hours",
            "omitted to keep the history table scannable",
        ),
        ("cloudLayers", "omitted to keep the history table scannable"),
        ("title", "observation collections do not carry a title"),
        ("updated", "each observation carries its own timestamp"),
        ("pagination", "surfaced as the more-observations note"),
    ];
}

/// Zone observations retain their command-specific station column while using
/// the shared observation meaning and rendering seam.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ZoneObservations(pub FeatureCollection<Observation>);

impl Summarize for ZoneObservations {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let observations = &self.0;
        let mut summary = Summary::new("Zone observations").subtitle(count_noun(
            observations.len(),
            "observation",
            "observations",
        ));
        if observations.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("features"),
                message: "No observations available for this zone".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("Station", Some("stationId")).also(&["stationName"]),
                    Column::new("Time", Some("timestamp")),
                    Column::new("Weather", Some("textDescription")).also(&["presentWeather"]),
                    Column::new("Temperature", Some("temperature")),
                    Column::new("Dewpoint", Some("dewpoint")),
                    Column::new("Wind", Some("windSpeed")).also(&["windDirection", "windGust"]),
                    Column::new("Pressure", Some("seaLevelPressure")).also(&["barometricPressure"]),
                    Column::new("Visibility", Some("visibility")),
                    Column::new("Clouds", Some("cloudLayers")),
                ],
                rows: observations
                    .iter()
                    .map(|feature| zone_observation_row(&feature.properties, options))
                    .collect(),
            });
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        ("type", "always FeatureCollection or Feature"),
        ("features", "each observation is one table row"),
        ("id", "the timestamp and station identify each observation"),
        (
            "geometry",
            "station metadata provides the fixed coordinates",
        ),
        (
            "properties",
            "each observation's keys are accounted for one by one",
        ),
        ("@id", "the timestamp and station identify each observation"),
        ("@type", "always wx:ObservationStation"),
        ("elevation", "station metadata provides elevation"),
        ("station", "the station id is shown"),
        ("rawMessage", "decoded measurements are shown instead"),
        ("icon", "not useful in a text table"),
        (
            "maxTemperatureLast24Hours",
            "not part of current conditions",
        ),
        (
            "minTemperatureLast24Hours",
            "not part of current conditions",
        ),
        (
            "precipitationLastHour",
            "not part of this compact zone overview",
        ),
        (
            "precipitationLast3Hours",
            "not part of this compact zone overview",
        ),
        (
            "precipitationLast6Hours",
            "not part of this compact zone overview",
        ),
        ("relativeHumidity", "not part of this compact zone overview"),
        ("windChill", "not part of this compact zone overview"),
        ("heatIndex", "not part of this compact zone overview"),
        ("title", "zone observation collections do not carry a title"),
        ("updated", "each observation carries its own timestamp"),
        ("pagination", "this endpoint does not paginate"),
    ];
}

fn zone_observation_row(observation: &Observation, options: &SummaryOptions) -> Vec<Cell> {
    let weather = if observation.text_description.trim().is_empty() {
        present_weather(observation)
    } else {
        Value::text(Some(&observation.text_description))
    };
    let wind = Value::lines(vec![
        Value::quantity(&observation.wind_direction, QuantityKind::Angle, options),
        Value::quantity(&observation.wind_speed, QuantityKind::Speed, options),
        Value::quantity(&observation.wind_gust, QuantityKind::Speed, options),
    ]);
    let pressure = if observation.sea_level_pressure.value.is_some() {
        Value::quantity(
            &observation.sea_level_pressure,
            QuantityKind::Pressure,
            options,
        )
    } else {
        Value::quantity(
            &observation.barometric_pressure,
            QuantityKind::Pressure,
            options,
        )
    };
    let clouds = if observation.cloud_layers.is_empty() {
        Value::text(Some("Clear"))
    } else {
        Value::lines(
            observation
                .cloud_layers
                .iter()
                .flat_map(|layer| {
                    [
                        Value::text(Some(&layer.amount.to_string())),
                        Value::quantity(&layer.base, QuantityKind::Height, options),
                    ]
                })
                .collect(),
        )
    };
    vec![
        Value::lines(vec![
            Value::text(Some(&observation.station_name)),
            Value::identifier(observation.station_id.to_string()),
        ])
        .into(),
        Value::timestamp(observation.timestamp).into(),
        weather.into(),
        Value::quantity(&observation.temperature, QuantityKind::Temperature, options).into(),
        Value::quantity(&observation.dewpoint, QuantityKind::Temperature, options).into(),
        wind.into(),
        pressure.into(),
        Value::quantity(&observation.visibility, QuantityKind::Distance, options).into(),
        clouds.into(),
    ]
}

impl Summarize for TerminalAerodromeForecastsResponse {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut summary = Summary::new("Terminal Aerodrome Forecasts").subtitle(count_noun(
            self.forecasts.len(),
            "forecast",
            "forecasts",
        ));
        if self.forecasts.is_empty() {
            summary = summary.push(Section::Empty {
                key: Some("@graph"),
                message: "No current Terminal Aerodrome Forecasts".to_owned(),
            });
        } else {
            summary = summary.push(Section::Table {
                heading: None,
                columns: vec![
                    Column::new("ID", Some("id")),
                    Column::new("Issue Time", Some("issueTime")),
                    Column::new("Location", Some("location")),
                    Column::new("Start", Some("start")),
                    Column::new("End", Some("end")),
                    Column::new("Geometry", Some("geometry")),
                ],
                rows: self
                    .forecasts
                    .iter()
                    .map(|taf| {
                        vec![
                            Value::identifier_from_url(&taf.id).into(),
                            Value::timestamp(taf.issue_time).into(),
                            Value::identifier(taf.location.to_string()).into(),
                            Value::timestamp(taf.start).into(),
                            Value::timestamp(taf.end).into(),
                            Value::text(Some(&taf.geometry)).into(),
                        ]
                    })
                    .collect(),
            });
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] =
        &[("@graph", "each forecast is one table row")];
}

// TAF summary implementation and semantic formatting follow below.

impl Summarize for TerminalAerodromeForecast {
    fn summarize(&self, _options: &SummaryOptions) -> Summary {
        let mut facts = vec![
            Fact::new(
                "Airport",
                Some("aerodrome"),
                Value::identifier(self.aerodrome().icao_identifier()),
            ),
            Fact::new(
                "Issued",
                Some("issuedAt"),
                Value::timestamp(self.issued_at().into()),
            ),
            Fact::new(
                "Status",
                Some("reportMetadata"),
                Value::Text(format_status(self.report_metadata().status())),
            ),
            Fact::new(
                "Permissible use",
                None,
                Value::Text(format_usage(self.report_metadata().permissible_usage())),
            ),
            Fact::new(
                "Bulletin",
                Some("bulletinIdentifier"),
                Value::identifier(self.bulletin_identifier()),
            ),
        ];

        if self.aerodrome().designator() != self.aerodrome().icao_identifier() {
            facts.insert(
                1,
                Fact::new(
                    "Aerodrome designator",
                    None,
                    Value::identifier(self.aerodrome().designator()),
                ),
            );
        }
        if let Some(translation) = self.report_metadata().translation() {
            if let Some(identifier) = translation.source_bulletin_identifier() {
                facts.push(Fact::new(
                    "Translation source",
                    None,
                    Value::identifier(identifier),
                ));
            }
            if let Some(received) = translation.source_bulletin_received_at() {
                facts.push(Fact::new(
                    "Translation source received",
                    None,
                    Value::timestamp(received.into()),
                ));
            }
            let centre = match (translation.centre_designator(), translation.centre_name()) {
                (Some(designator), Some(name)) => Some(format!("{designator} ({name})")),
                (Some(designator), None) => Some(designator.to_owned()),
                (None, Some(name)) => Some(name.to_owned()),
                (None, None) => None,
            };
            if let Some(centre) = centre {
                facts.push(Fact::new("Translation centre", None, Value::Text(centre)));
            }
            if let Some(translated) = translation.translated_at() {
                facts.push(Fact::new(
                    "Translated",
                    None,
                    Value::timestamp(translated.into()),
                ));
            }
        }

        match self.report() {
            ForecastReport::Forecast {
                valid_period,
                groups,
            } => {
                facts.push(Fact::new(
                    "Report state",
                    Some("report"),
                    Value::text(Some("Forecast")),
                ));
                facts.push(Fact::new(
                    "Valid period",
                    None,
                    time_range_value(*valid_period),
                ));
                for group in groups {
                    add_forecast_group(&mut facts, group);
                }
            }
            ForecastReport::Cancellation { cancelled_period } => {
                facts.push(Fact::new(
                    "Report state",
                    Some("report"),
                    Value::text(Some("Cancelled")),
                ));
                facts.push(Fact::new(
                    "Cancelled period",
                    None,
                    time_range_value(*cancelled_period),
                ));
            }
            ForecastReport::Missing { reason } => {
                facts.push(Fact::new(
                    "Report state",
                    Some("report"),
                    Value::text(Some("Forecast unavailable")),
                ));
                facts.push(Fact::new(
                    "Reason",
                    None,
                    Value::Text(format_missing_forecast(reason)),
                ));
            }
            _ => {
                facts.push(Fact::new(
                    "Report state",
                    Some("report"),
                    Value::text(Some("Unknown report state")),
                ));
            }
        }

        Summary::new("Terminal Aerodrome Forecast")
            .subtitle(self.aerodrome().icao_identifier().to_owned())
            .push(Section::Facts {
                heading: None,
                facts,
            })
    }
}

fn time_range_value(period: TimeRange) -> Value {
    Value::interval(period.start().into(), Some(period.end().into()))
}

fn add_forecast_group(facts: &mut Vec<Fact>, group: &ForecastGroup) {
    let conditions = group.conditions();
    let is_base = matches!(group.kind(), ForecastGroupKind::Base);
    facts.push(Fact::new(
        format_group_kind(group.kind()),
        None,
        time_range_value(group.valid_period()),
    ));
    facts.push(Fact::new(
        "CAVOK",
        None,
        Value::yes_no(Some(conditions.is_cavok())),
    ));
    facts.push(Fact::new(
        "Wind",
        None,
        Value::Text(format_wind(conditions.wind(), is_base)),
    ));
    facts.push(Fact::new(
        "Visibility",
        None,
        Value::Text(format_visibility(conditions, is_base)),
    ));
    facts.push(Fact::new(
        "Weather",
        None,
        Value::Text(format_weather(
            conditions.weather(),
            conditions.is_cavok(),
            is_base,
        )),
    ));
    facts.push(Fact::new(
        "Clouds",
        None,
        Value::Text(format_clouds(
            conditions.clouds(),
            conditions.is_cavok(),
            is_base,
        )),
    ));
    if !conditions.temperatures().is_empty() {
        facts.push(Fact::new(
            "Temperatures",
            None,
            Value::Lines(
                conditions
                    .temperatures()
                    .iter()
                    .map(format_temperature)
                    .map(Value::Text)
                    .collect(),
            ),
        ));
    }
}

fn format_status(status: &ReportStatus) -> String {
    match status {
        ReportStatus::Normal => "Normal".to_owned(),
        ReportStatus::Amendment => "Amendment".to_owned(),
        ReportStatus::Correction => "Correction".to_owned(),
        ReportStatus::Other { code } => format!("Other ({code})"),
        _ => "Unknown".to_owned(),
    }
}

fn format_usage(usage: &PermissibleUsage) -> String {
    match usage {
        PermissibleUsage::Operational => "Operational".to_owned(),
        PermissibleUsage::NonOperational {
            reason,
            supplementary,
        } => format_restricted_usage("Non-operational", reason.as_ref(), supplementary.as_deref()),
        PermissibleUsage::Other {
            code,
            reason,
            supplementary,
        } => format_restricted_usage(
            &format!("Other ({code})"),
            reason.as_ref(),
            supplementary.as_deref(),
        ),
        _ => "Unknown".to_owned(),
    }
}

fn format_restricted_usage(
    prefix: &str,
    reason: Option<&PermissibleUsageReason>,
    supplementary: Option<&str>,
) -> String {
    match (reason.map(format_usage_reason), supplementary) {
        (Some(reason), Some(details)) => format!("{prefix} — {reason}: {details}"),
        (Some(reason), None) => format!("{prefix} — {reason}"),
        (None, Some(details)) => format!("{prefix} — {details}"),
        (None, None) => prefix.to_owned(),
    }
}

fn format_usage_reason(reason: &PermissibleUsageReason) -> String {
    match reason {
        PermissibleUsageReason::Test => "test".to_owned(),
        PermissibleUsageReason::Exercise => "exercise".to_owned(),
        PermissibleUsageReason::Other { code } => code.to_string(),
        _ => "unknown reason".to_owned(),
    }
}

fn format_missing_forecast(reason: &MissingForecastReason) -> String {
    match reason {
        MissingForecastReason::NotProvided => "Forecast content was not provided".to_owned(),
        MissingForecastReason::TranslationFailed { tac } => {
            format!("TAC-to-IWXXM translation failed\nSource TAC: {tac}")
        }
        _ => "Unknown reason".to_owned(),
    }
}

fn format_group_kind(kind: &ForecastGroupKind) -> String {
    match kind {
        ForecastGroupKind::Base => "INITIAL FORECAST".to_owned(),
        ForecastGroupKind::From => "CHANGE — FROM (FM)".to_owned(),
        ForecastGroupKind::Becoming => "CHANGE — BECOMING (BECMG)".to_owned(),
        ForecastGroupKind::Temporary => "CHANGE — TEMPORARY (TEMPO)".to_owned(),
        ForecastGroupKind::Probability { percent, temporary } => {
            let temporary = if *temporary { " — TEMPORARY" } else { "" };
            format!("CHANGE — PROBABILITY {percent}%{temporary}")
        }
        ForecastGroupKind::Other { code } => format!("CHANGE — {code}"),
        _ => "CHANGE — UNKNOWN".to_owned(),
    }
}

fn format_wind(wind: &ForecastWind, is_base: bool) -> String {
    match wind {
        ForecastElement::NotReported => unchanged_or_not_reported(is_base),
        ForecastElement::Value(wind) => format_surface_wind(wind),
        ForecastElement::Unavailable { reason } => format!("Unavailable ({})", format_nil(reason)),
        _ => "Unknown wind state".to_owned(),
    }
}

fn format_surface_wind(wind: &SurfaceWind) -> String {
    let direction = match wind.direction() {
        WindDirection::Variable => "Variable (VRB)".to_owned(),
        WindDirection::Degrees(degrees) => format!("{}°", format_number(degrees)),
        _ => "Unknown direction".to_owned(),
    };
    let mut value = format!("{direction} at {}", format_wind_speed(wind.speed()));
    if let Some(gust) = wind.gust() {
        value.push_str("; gusting ");
        value.push_str(&format_wind_speed(gust));
    }
    value
}

fn format_wind_speed(speed: &WindSpeed) -> String {
    format!(
        "{}{} kt",
        format_comparison(speed.comparison()),
        format_number(speed.knots())
    )
}

fn format_visibility(conditions: &ForecastConditions, is_base: bool) -> String {
    if conditions.is_cavok() {
        return "At least 10 km (CAVOK)".to_owned();
    }
    let visibility = match conditions.visibility() {
        ForecastElement::NotReported => return unchanged_or_not_reported(is_base),
        ForecastElement::Value(visibility) => visibility,
        ForecastElement::Unavailable { reason } => {
            return format!("Unavailable ({})", format_nil(reason));
        }
        _ => return "Unknown visibility state".to_owned(),
    };
    let meters = visibility.meters();
    let distance = if meters >= 1_000.0 {
        format!("{} km", format_number(meters / 1_000.0))
    } else {
        format!("{} m", format_number(meters))
    };
    format!(
        "{}{} ({:.1} mi)",
        format_comparison(visibility.comparison()),
        distance,
        meters / 1_609.344
    )
}

fn format_comparison(comparison: &Comparison) -> String {
    match comparison {
        Comparison::Exact => String::new(),
        Comparison::Above => "≥".to_owned(),
        Comparison::Below => "≤".to_owned(),
        Comparison::Other { code } => format!("{code} "),
        _ => "? ".to_owned(),
    }
}

fn format_weather(weather: &ForecastWeather, cavok: bool, is_base: bool) -> String {
    if cavok {
        return "No significant weather (CAVOK)".to_owned();
    }
    match weather {
        ForecastWeather::NotReported => unchanged_or_not_reported(is_base),
        ForecastWeather::NoSignificant => "No significant weather".to_owned(),
        ForecastWeather::Phenomena { items } => items
            .iter()
            .map(format_weather_item)
            .collect::<Vec<_>>()
            .join("\n"),
        ForecastWeather::Unavailable { reason } => format!("Unavailable ({})", format_nil(reason)),
        _ => "Unknown weather state".to_owned(),
    }
}

fn format_weather_item(weather: &Weather) -> String {
    let descriptor = weather.descriptor().map(format_descriptor);
    let phenomena = weather
        .phenomena()
        .iter()
        .map(format_phenomenon)
        .collect::<Vec<_>>()
        .join(" and ");
    let mut description = match (descriptor, phenomena.is_empty()) {
        (Some(descriptor), false) => format!("{descriptor} with {phenomena}"),
        (Some(descriptor), true) => descriptor,
        (None, false) => phenomena,
        (None, true) => "unclassified weather".to_owned(),
    };
    match weather.intensity() {
        WeatherIntensity::Light => description.insert_str(0, "light "),
        WeatherIntensity::Heavy => description.insert_str(0, "heavy "),
        WeatherIntensity::Moderate => {}
        _ => description.insert_str(0, "unknown-intensity "),
    }
    if weather.is_in_vicinity() {
        description.push_str(" in the vicinity");
    }
    format!("{} — {description}", weather.code())
}

fn format_descriptor(descriptor: &WeatherDescriptor) -> String {
    match descriptor {
        WeatherDescriptor::Shallow => "shallow".to_owned(),
        WeatherDescriptor::Partial => "partial".to_owned(),
        WeatherDescriptor::Patches => "patches".to_owned(),
        WeatherDescriptor::LowDrifting => "low drifting".to_owned(),
        WeatherDescriptor::Blowing => "blowing".to_owned(),
        WeatherDescriptor::Showers => "showers".to_owned(),
        WeatherDescriptor::Thunderstorm => "thunderstorm".to_owned(),
        WeatherDescriptor::Freezing => "freezing".to_owned(),
        WeatherDescriptor::Other { code } => format!("descriptor {code}"),
        _ => "unknown descriptor".to_owned(),
    }
}

fn format_phenomenon(phenomenon: &WeatherPhenomenon) -> String {
    match phenomenon {
        WeatherPhenomenon::Drizzle => "drizzle".to_owned(),
        WeatherPhenomenon::Rain => "rain".to_owned(),
        WeatherPhenomenon::Snow => "snow".to_owned(),
        WeatherPhenomenon::SnowGrains => "snow grains".to_owned(),
        WeatherPhenomenon::IceCrystals => "ice crystals".to_owned(),
        WeatherPhenomenon::IcePellets => "ice pellets".to_owned(),
        WeatherPhenomenon::Hail => "hail".to_owned(),
        WeatherPhenomenon::SmallHail => "small hail or snow pellets".to_owned(),
        WeatherPhenomenon::UnknownPrecipitation => "unknown precipitation".to_owned(),
        WeatherPhenomenon::Mist => "mist".to_owned(),
        WeatherPhenomenon::Fog => "fog".to_owned(),
        WeatherPhenomenon::Smoke => "smoke".to_owned(),
        WeatherPhenomenon::VolcanicAsh => "volcanic ash".to_owned(),
        WeatherPhenomenon::Dust => "widespread dust".to_owned(),
        WeatherPhenomenon::Sand => "sand".to_owned(),
        WeatherPhenomenon::Haze => "haze".to_owned(),
        WeatherPhenomenon::Spray => "spray".to_owned(),
        WeatherPhenomenon::DustWhirls => "dust or sand whirls".to_owned(),
        WeatherPhenomenon::Squalls => "squalls".to_owned(),
        WeatherPhenomenon::FunnelCloud => "funnel cloud or tornado/waterspout".to_owned(),
        WeatherPhenomenon::Sandstorm => "sandstorm".to_owned(),
        WeatherPhenomenon::Duststorm => "duststorm".to_owned(),
        WeatherPhenomenon::Other { code } => format!("unrecognized phenomenon {code}"),
        _ => "unknown phenomenon".to_owned(),
    }
}

fn format_clouds(clouds: &ForecastClouds, cavok: bool, is_base: bool) -> String {
    if cavok {
        return "No operationally significant cloud (CAVOK)".to_owned();
    }
    match clouds {
        ForecastClouds::NotReported => unchanged_or_not_reported(is_base),
        ForecastClouds::NoSignificant => "No significant cloud".to_owned(),
        ForecastClouds::VerticalVisibility { feet } => match feet {
            ForecastValue::Value(feet) => {
                format!("Vertical visibility {} ft", format_number(*feet))
            }
            ForecastValue::Unavailable { reason } => {
                format!("Vertical visibility unavailable ({})", format_nil(reason))
            }
            _ => "Unknown vertical visibility".to_owned(),
        },
        ForecastClouds::Layers { layers } => layers
            .iter()
            .map(format_cloud_layer)
            .collect::<Vec<_>>()
            .join("\n"),
        ForecastClouds::Unavailable { reason } => format!("Unavailable ({})", format_nil(reason)),
        _ => "Unknown cloud state".to_owned(),
    }
}

fn format_cloud_layer(layer: &CloudLayer) -> String {
    let amount = match layer.amount() {
        ForecastValue::Value(amount) => format_cloud_amount(amount),
        ForecastValue::Unavailable { reason } => {
            format!("Amount unavailable ({})", format_nil(reason))
        }
        _ => "Unknown amount".to_owned(),
    };
    let base = match layer.base_feet() {
        ForecastValue::Value(feet) => format!("at {} ft AGL", format_number(*feet)),
        ForecastValue::Unavailable { reason } => {
            format!("at unavailable base ({})", format_nil(reason))
        }
        _ => "at unknown base".to_owned(),
    };
    let cloud_type = layer
        .cloud_type()
        .map_or_else(String::new, |cloud_type| match cloud_type {
            ForecastValue::Value(cloud_type) => format!(" — {}", format_cloud_type(cloud_type)),
            ForecastValue::Unavailable { reason } => {
                format!(" — type unavailable ({})", format_nil(reason))
            }
            _ => " — unknown type".to_owned(),
        });
    format!("{amount} {base}{cloud_type}")
}

fn format_cloud_amount(amount: &CloudAmount) -> String {
    match amount {
        CloudAmount::Few => "Few (FEW)".to_owned(),
        CloudAmount::Scattered => "Scattered (SCT)".to_owned(),
        CloudAmount::Broken => "Broken (BKN)".to_owned(),
        CloudAmount::Overcast => "Overcast (OVC)".to_owned(),
        CloudAmount::NoSignificant => "No significant cloud (NSC)".to_owned(),
        CloudAmount::SkyClear => "Sky clear (SKC/CLR)".to_owned(),
        CloudAmount::Other { code } => format!("Other ({code})"),
        _ => "Unknown amount".to_owned(),
    }
}

fn format_cloud_type(cloud_type: &CloudType) -> String {
    match cloud_type {
        CloudType::Cumulonimbus => "Cumulonimbus (CB)".to_owned(),
        CloudType::ToweringCumulus => "Towering cumulus (TCU)".to_owned(),
        CloudType::Other { code } => format!("Other ({code})"),
        _ => "Unknown type".to_owned(),
    }
}

fn format_temperature(temperature: &TemperatureForecast) -> String {
    format!(
        "Maximum {} °C at {}; minimum {} °C at {}",
        format_number(temperature.maximum().celsius()),
        format_taf_timestamp(temperature.maximum().occurs_at()),
        format_number(temperature.minimum().celsius()),
        format_taf_timestamp(temperature.minimum().occurs_at()),
    )
}

fn format_taf_timestamp(timestamp: jiff::Timestamp) -> String {
    let day = timestamp.strftime("%d").to_string();
    format!(
        "{} {} UTC",
        day.trim_start_matches('0'),
        timestamp.strftime("%b %H:%M")
    )
}

fn unchanged_or_not_reported(is_base: bool) -> String {
    if is_base {
        "Not reported".to_owned()
    } else {
        "Unchanged from prevailing conditions".to_owned()
    }
}

fn format_nil(reason: &MissingReason) -> String {
    match reason {
        MissingReason::NoSignificant => "nothing operationally significant".to_owned(),
        MissingReason::NotObservable => "not observable".to_owned(),
        MissingReason::Missing => "missing".to_owned(),
        MissingReason::Withheld => "withheld".to_owned(),
        MissingReason::Other { code } => code.to_string(),
        _ => "unknown reason".to_owned(),
    }
}

fn format_number(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}
