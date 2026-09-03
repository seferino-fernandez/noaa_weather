//! Summaries for the `/gridpoints` family: the raw layers and the two
//! textual forecasts generated from them.
//!
//! A raw gridpoint is 59 layers deep and most of them are empty for any given
//! office, so the summary is a census: one row per layer that actually has
//! values, saying what it measures in, how many readings there are, what
//! stretch of time they cover, and what the first one says. The hazards and
//! the expected weather each get their own table, because both are English
//! rather than numbers.
//!
//! # Why the *first* value
//!
//! The layers table shows a layer's first reading, never the one valid right
//! now. Reading a clock inside `summarize` would make the same response
//! render differently every minute and every snapshot non-deterministic;
//! `--json` carries the whole series for a caller that wants to index into
//! it.
//!
//! # Emphasis
//!
//! Nothing in a forecast is alarming enough to color except a hazard: a
//! hazard period reads as [`Emphasis::Warning`], and significance `W` — a
//! warning, the office's firmest word — escalates to [`Emphasis::Danger`].
//! Coloring a temperature would invent a threshold NOAA does not state.

use noaa_weather_client::models::{
    Forecast, ForecastGenerator, ForecastPeriod, Gridpoint, GridpointLayer, HazardPeriod, Quantity,
    Unit, WeatherCondition, WeatherCoverage, WeatherIntensity, WeatherPeriod, WeatherPhenomenon,
};
use noaa_weather_client::{Feature, Interval, OffsetDateTime};

use crate::render::{RangeStyle, RenderOptions, format_value};
use crate::units::shown_unit;
use crate::{
    Align, Cell, Column, Emphasis, Fact, QuantityKind, Section, Summarize, Summary, SummaryOptions,
    Value, vtec,
};

/// Every layer key a gridpoint carries, in the order NOAA writes them.
///
/// The layers table renders these one row at a time, so its Layer column
/// carries the whole list as its `also` keys and [`crate::coverage_gaps`]
/// counts every one of them as accounted for — including the layers that
/// turned out empty, which have no row, and `weather` and `hazards`, which
/// have their own tables and would go uncovered on a grid that has neither.
///
/// A layer NOAA adds beyond this list arrives in [`Gridpoint::other`], gets a
/// row, and is *not* in this slice, so it surfaces as a coverage gap. That is
/// the alarm working.
const LAYER_KEYS: [&str; 59] = [
    "temperature",
    "dewpoint",
    "maxTemperature",
    "minTemperature",
    "relativeHumidity",
    "apparentTemperature",
    "wetBulbGlobeTemperature",
    "heatIndex",
    "windChill",
    "skyCover",
    "windDirection",
    "windSpeed",
    "windGust",
    "weather",
    "hazards",
    "heatRisk",
    "probabilityOfPrecipitation",
    "quantitativePrecipitation",
    "iceAccumulation",
    "snowfallAmount",
    "snowLevel",
    "ceilingHeight",
    "visibility",
    "transportWindSpeed",
    "transportWindDirection",
    "mixingHeight",
    "hainesIndex",
    "lightningActivityLevel",
    "twentyFootWindSpeed",
    "twentyFootWindDirection",
    "waveHeight",
    "wavePeriod",
    "waveDirection",
    "primarySwellHeight",
    "primarySwellDirection",
    "secondarySwellHeight",
    "secondarySwellDirection",
    "wavePeriod2",
    "windWaveHeight",
    "dispersionIndex",
    "pressure",
    "probabilityOfTropicalStormWinds",
    "probabilityOfHurricaneWinds",
    "potentialOf15mphWinds",
    "potentialOf25mphWinds",
    "potentialOf35mphWinds",
    "potentialOf45mphWinds",
    "potentialOf20mphWindGusts",
    "potentialOf30mphWindGusts",
    "potentialOf40mphWindGusts",
    "potentialOf50mphWindGusts",
    "potentialOf60mphWindGusts",
    "grasslandFireDangerIndex",
    "probabilityOfThunder",
    "davisStabilityIndex",
    "atmosphericDispersionIndex",
    "lowVisibilityOccurrenceRiskIndex",
    "stability",
    "redFlagThreatIndex",
];

// `Gridpoint` names 57 numerical layers plus `weather` and `hazards`. This
// fails to compile if the list above is edited without the count following,
// which is the only thing standing between a mistyped slice and a silently
// uncovered key.
const _: () = assert!(LAYER_KEYS.len() == 59);

/// The numerical layers, paired with the key each is named by on the wire.
///
/// Ordered as NOAA writes them, which is also the order the table reads in.
fn numerical_layers(gridpoint: &Gridpoint) -> Vec<(&str, &GridpointLayer)> {
    let named: [(&str, &GridpointLayer); 57] = [
        ("temperature", &gridpoint.temperature),
        ("dewpoint", &gridpoint.dewpoint),
        ("maxTemperature", &gridpoint.max_temperature),
        ("minTemperature", &gridpoint.min_temperature),
        ("relativeHumidity", &gridpoint.relative_humidity),
        ("apparentTemperature", &gridpoint.apparent_temperature),
        (
            "wetBulbGlobeTemperature",
            &gridpoint.wet_bulb_globe_temperature,
        ),
        ("heatIndex", &gridpoint.heat_index),
        ("windChill", &gridpoint.wind_chill),
        ("skyCover", &gridpoint.sky_cover),
        ("windDirection", &gridpoint.wind_direction),
        ("windSpeed", &gridpoint.wind_speed),
        ("windGust", &gridpoint.wind_gust),
        ("heatRisk", &gridpoint.heat_risk),
        (
            "probabilityOfPrecipitation",
            &gridpoint.probability_of_precipitation,
        ),
        (
            "quantitativePrecipitation",
            &gridpoint.quantitative_precipitation,
        ),
        ("iceAccumulation", &gridpoint.ice_accumulation),
        ("snowfallAmount", &gridpoint.snowfall_amount),
        ("snowLevel", &gridpoint.snow_level),
        ("ceilingHeight", &gridpoint.ceiling_height),
        ("visibility", &gridpoint.visibility),
        ("transportWindSpeed", &gridpoint.transport_wind_speed),
        (
            "transportWindDirection",
            &gridpoint.transport_wind_direction,
        ),
        ("mixingHeight", &gridpoint.mixing_height),
        ("hainesIndex", &gridpoint.haines_index),
        (
            "lightningActivityLevel",
            &gridpoint.lightning_activity_level,
        ),
        ("twentyFootWindSpeed", &gridpoint.twenty_foot_wind_speed),
        (
            "twentyFootWindDirection",
            &gridpoint.twenty_foot_wind_direction,
        ),
        ("waveHeight", &gridpoint.wave_height),
        ("wavePeriod", &gridpoint.wave_period),
        ("waveDirection", &gridpoint.wave_direction),
        ("primarySwellHeight", &gridpoint.primary_swell_height),
        ("primarySwellDirection", &gridpoint.primary_swell_direction),
        ("secondarySwellHeight", &gridpoint.secondary_swell_height),
        (
            "secondarySwellDirection",
            &gridpoint.secondary_swell_direction,
        ),
        ("wavePeriod2", &gridpoint.wave_period_2),
        ("windWaveHeight", &gridpoint.wind_wave_height),
        ("dispersionIndex", &gridpoint.dispersion_index),
        ("pressure", &gridpoint.pressure),
        (
            "probabilityOfTropicalStormWinds",
            &gridpoint.probability_of_tropical_storm_winds,
        ),
        (
            "probabilityOfHurricaneWinds",
            &gridpoint.probability_of_hurricane_winds,
        ),
        ("potentialOf15mphWinds", &gridpoint.potential_of_15mph_winds),
        ("potentialOf25mphWinds", &gridpoint.potential_of_25mph_winds),
        ("potentialOf35mphWinds", &gridpoint.potential_of_35mph_winds),
        ("potentialOf45mphWinds", &gridpoint.potential_of_45mph_winds),
        (
            "potentialOf20mphWindGusts",
            &gridpoint.potential_of_20mph_wind_gusts,
        ),
        (
            "potentialOf30mphWindGusts",
            &gridpoint.potential_of_30mph_wind_gusts,
        ),
        (
            "potentialOf40mphWindGusts",
            &gridpoint.potential_of_40mph_wind_gusts,
        ),
        (
            "potentialOf50mphWindGusts",
            &gridpoint.potential_of_50mph_wind_gusts,
        ),
        (
            "potentialOf60mphWindGusts",
            &gridpoint.potential_of_60mph_wind_gusts,
        ),
        (
            "grasslandFireDangerIndex",
            &gridpoint.grassland_fire_danger_index,
        ),
        ("probabilityOfThunder", &gridpoint.probability_of_thunder),
        ("davisStabilityIndex", &gridpoint.davis_stability_index),
        (
            "atmosphericDispersionIndex",
            &gridpoint.atmospheric_dispersion_index,
        ),
        (
            "lowVisibilityOccurrenceRiskIndex",
            &gridpoint.low_visibility_occurrence_risk_index,
        ),
        ("stability", &gridpoint.stability),
        ("redFlagThreatIndex", &gridpoint.red_flag_threat_index),
    ];
    named
        .into_iter()
        .chain(
            gridpoint
                .other
                .iter()
                .map(|(key, layer)| (key.as_str(), layer)),
        )
        .collect()
}

/// What a layer measures, and so which unit it is shown in.
///
/// The unit answers this almost everywhere: `degC` is a temperature, `mm` an
/// accumulation, `percent` a share. `wmoUnit:m` is the one code that does not,
/// because it carries both a visibility and a ceiling height — the gridpoint
/// fixture has both — and those do not read well in the same unit. So
/// `visibility` is named here as a [`QuantityKind::Distance`], and everything
/// else on metres stays a height. `probabilityOfThunder` is named for the
/// opposite reason: the fixture sends its 44 values with no `uom` at all, so
/// without the exception it would read as a bare index rather than a
/// percentage.
fn layer_kind(key: &str, unit: Option<&Unit>) -> QuantityKind {
    match key {
        "visibility" => return QuantityKind::Distance,
        "probabilityOfThunder" => return QuantityKind::Percent,
        _ => {}
    }
    let Some(unit) = unit else {
        return QuantityKind::Index;
    };
    match crate::units::symbol(unit.code()) {
        "degC" | "Cel" | "degF" | "K" => QuantityKind::Temperature,
        "m_s-1" | "km_h-1" | "mi_h-1" | "kt" | "nmi_h-1" => QuantityKind::Speed,
        "mm" | "cm" | "in" => QuantityKind::Depth,
        "m" | "km" | "ft" | "mi" | "nmi" => QuantityKind::Height,
        "Pa" | "hPa" | "kPa" | "mbar" | "mb" | "inHg" => QuantityKind::Pressure,
        "percent" => QuantityKind::Percent,
        "degree_(angle)" => QuantityKind::Angle,
        _ => QuantityKind::Index,
    }
}

/// An interval as a value, with its end resolved.
///
/// Every `validTimes` and `validTime` in the captured fixtures is the
/// `start/duration` form — `P8DT1H` on the grid, `PT1H` and `P4DT12H` on the
/// layers — which [`Interval::end`] has no answer for, hence `resolved_end`.
/// The other three forms are handled all the same, and a form with no start
/// at all yields [`Value::Missing`] rather than half an interval.
fn interval(interval: Interval) -> Value {
    match interval.start() {
        None => Value::Missing,
        Some(start) => Value::interval(
            OffsetDateTime::from(start),
            interval.resolved_end().map(OffsetDateTime::from),
        ),
    }
}

/// The stretch of time a layer's readings cover, first start to last end.
fn covers(layer: &GridpointLayer) -> Value {
    let (Some(first), Some(last)) = (layer.values.first(), layer.values.last()) else {
        return Value::Missing;
    };
    match (first.valid_time.start(), last.valid_time.resolved_end()) {
        (Some(start), end) => {
            Value::interval(OffsetDateTime::from(start), end.map(OffsetDateTime::from))
        }
        (None, _) => Value::Missing,
    }
}

fn count_noun(count: usize, singular: &str, plural: &str) -> String {
    if count == 1 {
        format!("{count} {singular}")
    } else {
        format!("{count} {plural}")
    }
}

/// The census of numerical layers, or an explanation of why there is none.
fn layers_section(gridpoint: &Gridpoint, options: &SummaryOptions) -> Section {
    let layers = numerical_layers(gridpoint);
    let (populated, empty): (Vec<_>, Vec<_>) = layers
        .iter()
        .partition(|(_, layer)| !layer.values.is_empty());

    if populated.is_empty() {
        return Section::Empty {
            key: None,
            message: format!(
                "No layer has any values ({})",
                count_noun(empty.len(), "layer is empty", "layers are empty")
            ),
        };
    }

    let rows = populated
        .iter()
        .map(|(key, layer)| {
            let kind = layer_kind(key, layer.unit.as_ref());
            let first = layer.values.first().map_or(Value::Missing, |reading| {
                Value::reading(reading.value, layer.unit.as_ref(), kind, options)
            });
            vec![
                Value::text(Some(*key)).into(),
                Value::text(shown_unit(kind, layer.unit.as_ref(), options).as_deref()).into(),
                Value::count(layer.values.len() as u64).into(),
                covers(layer).into(),
                first.into(),
            ]
        })
        .collect();

    Section::Table {
        heading: Some("Layers".to_owned()),
        columns: vec![
            // Every layer key rides here, so the layers this grid left empty
            // are accounted for even though they have no row.
            Column::new("Layer", None).also(&LAYER_KEYS),
            Column::new("Unit", None),
            Column::new("Values", None).align(Align::Right),
            Column::new("Covers", None),
            Column::new("First value", None).align(Align::Right),
        ],
        rows,
    }
}

/// One hazard as English: `Heat Advisory`, or the raw codes when NWS has
/// added one since the tables in [`crate::vtec`] were written.
fn hazard_name(phenomenon: &str, significance: &str) -> String {
    let phenomenon = vtec::phenomenon(phenomenon).unwrap_or(phenomenon);
    let significance = vtec::significance(significance).unwrap_or(significance);
    format!("{phenomenon} {significance}")
}

/// A warning is the office's firmest word; everything else in the layer is a
/// watch, an advisory or a statement.
fn hazard_emphasis(hazards: &HazardPeriod) -> Emphasis {
    if hazards
        .value
        .iter()
        .any(|hazard| hazard.significance == "W")
    {
        Emphasis::Danger
    } else {
        Emphasis::Warning
    }
}

fn hazards_section(periods: &[HazardPeriod]) -> Option<Section> {
    let rows: Vec<Vec<Cell>> = periods
        .iter()
        .filter(|period| !period.value.is_empty())
        .map(|period| {
            let emphasis = hazard_emphasis(period);
            let names = Value::lines(
                period
                    .value
                    .iter()
                    .map(|hazard| {
                        Value::text(Some(&hazard_name(&hazard.phenomenon, &hazard.significance)))
                    })
                    .collect(),
            );
            vec![
                Cell::new(interval(period.valid_time), emphasis),
                Cell::new(names, emphasis),
            ]
        })
        .collect();

    (!rows.is_empty()).then(|| Section::Table {
        heading: Some("Hazards".to_owned()),
        columns: vec![Column::new("When", None), Column::new("Hazard", None)],
        rows,
    })
}

/// How much of the area or the period a phenomenon covers, as the clause it
/// opens a forecast sentence with.
fn coverage_label(coverage: WeatherCoverage) -> &'static str {
    match coverage {
        WeatherCoverage::Areas => "areas of",
        WeatherCoverage::Brief => "brief",
        WeatherCoverage::Chance => "chance of",
        WeatherCoverage::Definite => "",
        WeatherCoverage::Few => "a few",
        WeatherCoverage::Frequent => "frequent",
        WeatherCoverage::Intermittent => "intermittent",
        WeatherCoverage::Isolated => "isolated",
        WeatherCoverage::Likely => "likely",
        WeatherCoverage::Numerous => "numerous",
        WeatherCoverage::Occasional => "occasional",
        WeatherCoverage::Patchy => "patchy",
        WeatherCoverage::Periods => "periods of",
        WeatherCoverage::Scattered => "scattered",
        WeatherCoverage::SlightChance => "slight chance of",
        WeatherCoverage::Widespread => "widespread",
    }
}

fn intensity_label(intensity: WeatherIntensity) -> &'static str {
    match intensity {
        WeatherIntensity::VeryLight => "very light",
        WeatherIntensity::Light => "light",
        WeatherIntensity::Moderate => "moderate",
        WeatherIntensity::Heavy => "heavy",
    }
}

fn phenomenon_label(weather: WeatherPhenomenon) -> &'static str {
    match weather {
        WeatherPhenomenon::BlowingDust => "blowing dust",
        WeatherPhenomenon::BlowingSand => "blowing sand",
        WeatherPhenomenon::BlowingSnow => "blowing snow",
        WeatherPhenomenon::Drizzle => "drizzle",
        WeatherPhenomenon::Fog => "fog",
        WeatherPhenomenon::FreezingFog => "freezing fog",
        WeatherPhenomenon::FreezingDrizzle => "freezing drizzle",
        WeatherPhenomenon::FreezingRain => "freezing rain",
        WeatherPhenomenon::FreezingSpray => "freezing spray",
        WeatherPhenomenon::Frost => "frost",
        WeatherPhenomenon::Hail => "hail",
        WeatherPhenomenon::Haze => "haze",
        WeatherPhenomenon::IceCrystals => "ice crystals",
        WeatherPhenomenon::IceFog => "ice fog",
        WeatherPhenomenon::Rain => "rain",
        WeatherPhenomenon::RainShowers => "rain showers",
        WeatherPhenomenon::Sleet => "sleet",
        WeatherPhenomenon::Smoke => "smoke",
        WeatherPhenomenon::Snow => "snow",
        WeatherPhenomenon::SnowShowers => "snow showers",
        WeatherPhenomenon::Thunderstorms => "thunderstorms",
        WeatherPhenomenon::VolcanicAsh => "volcanic ash",
        WeatherPhenomenon::WaterSpouts => "water spouts",
    }
}

/// One expected phenomenon as a phrase: `Slight chance of light rain
/// showers`.
///
/// Returns `None` for the entry NOAA writes with every field null, which is
/// how it says "nothing expected" rather than a phenomenon in its own right.
fn weather_phrase(condition: &WeatherCondition) -> Option<String> {
    let weather = condition.weather?;
    let parts = [
        condition.coverage.map_or("", coverage_label),
        condition.intensity.map_or("", intensity_label),
        phenomenon_label(weather),
    ];
    let phrase = parts
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let mut characters = phrase.chars();
    let first = characters.next()?;
    Some(first.to_uppercase().collect::<String>() + characters.as_str())
}

fn weather_section(periods: &[WeatherPeriod]) -> Option<Section> {
    let rows: Vec<Vec<Cell>> = periods
        .iter()
        .filter_map(|period| {
            let phrases: Vec<Value> = period
                .value
                .iter()
                .filter_map(weather_phrase)
                .map(|phrase| Value::text(Some(&phrase)))
                .collect();
            (!phrases.is_empty()).then(|| {
                vec![
                    interval(period.valid_time).into(),
                    Value::lines(phrases).into(),
                ]
            })
        })
        .collect();

    (!rows.is_empty()).then(|| Section::Table {
        heading: Some("Weather".to_owned()),
        columns: vec![Column::new("When", None), Column::new("Expected", None)],
        rows,
    })
}

impl Summarize for Feature<Gridpoint> {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let grid = &self.properties;
        let mut summary = Summary::new(format!(
            "Gridpoint {}/{},{}",
            grid.grid_id, grid.grid_x, grid.grid_y
        ));
        if let Value::Identifier(office) = Value::identifier_from_url(&grid.forecast_office) {
            summary = summary.subtitle(format!("Issued by {office}"));
        }

        summary = summary.push(Section::Facts {
            heading: None,
            facts: vec![
                Fact::new(
                    "Updated",
                    Some("updateTime"),
                    Value::timestamp(grid.update_time),
                ),
                Fact::new("Covers", Some("validTimes"), interval(grid.valid_times)),
                Fact::new(
                    "Elevation",
                    Some("elevation"),
                    Value::quantity(&grid.elevation, QuantityKind::Height, options),
                ),
                Fact::new(
                    "Grid cell",
                    Some("gridId"),
                    Value::identifier(format!("{}/{},{}", grid.grid_id, grid.grid_x, grid.grid_y)),
                )
                .also(&["gridX", "gridY"]),
            ],
        });

        summary = summary.push(layers_section(grid, options));
        if let Some(section) = hazards_section(&grid.hazards.values) {
            summary = summary.push(section);
        }
        if let Some(section) = weather_section(&grid.weather.values) {
            summary = summary.push(section);
        }

        let empty = numerical_layers(grid)
            .iter()
            .filter(|(_, layer)| layer.values.is_empty())
            .count();
        if empty > 0 {
            summary = summary.note(format!(
                "{} for this grid",
                count_noun(empty, "layer has no values", "layers have no values")
            ));
        }
        summary
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        // Envelope.
        ("type", "always Feature"),
        (
            "id",
            "the grid cell's own URL; the grid cell fact addresses it again",
        ),
        (
            "geometry",
            "the polygon of the cell's four corners; the grid cell names it in six characters",
        ),
        (
            "properties",
            "the gridpoint itself; its keys are accounted for one by one",
        ),
        // Provenance.
        (
            "@id",
            "the grid cell's own URL, as the envelope id already is",
        ),
        ("@type", "always wx:Gridpoint"),
        // Shown outside the facts.
        ("forecastOffice", "shown as the subtitle"),
    ];
}

/// The wind of one forecast period: speed, then the direction it blows from,
/// then the gust when the office forecasts one.
///
/// One phrase rather than three values because it reads as one: `10 to 15 mph
/// S gust 25 mph`. The twelve-hour speed is the reason [`Value::Range`]
/// exists — NOAA sends it as `minValue`/`maxValue` with a null `value`.
fn wind(period: &ForecastPeriod, options: &SummaryOptions) -> Value {
    let show = |value: &Value| format_value(value, &RenderOptions::default(), RangeStyle::Words);
    let speed = Value::quantity(&period.wind_speed, QuantityKind::Speed, options);
    let mut parts = vec![show(&speed)];
    if let Some(direction) = period.wind_direction {
        parts.push(direction.to_string());
    }
    if let Some(gust) = period.wind_gust.as_ref() {
        let gust = Value::quantity(gust, QuantityKind::Speed, options);
        parts.push(format!("gust {}", show(&gust)));
    }
    Value::text(Some(&parts.join(" ")))
}

/// The twelve-hour forecast: one row per named period.
fn twelve_hour_section(periods: &[ForecastPeriod], options: &SummaryOptions) -> Section {
    let rows = periods
        .iter()
        .map(|period| {
            vec![
                Value::text(period.name.as_deref()).into(),
                Value::interval(period.start_time, Some(period.end_time)).into(),
                Value::quantity(&period.temperature, QuantityKind::Temperature, options).into(),
                wind(period, options).into(),
                Value::text(Some(&period.short_forecast)).into(),
            ]
        })
        .collect();
    Section::Table {
        heading: None,
        columns: vec![
            Column::new("Period", Some("periods")),
            Column::new("Time", None),
            Column::new("Temp", None).align(Align::Right),
            Column::new("Wind", None),
            Column::new("Forecast", None),
        ],
        rows,
    }
}

/// The hourly forecast, which carries the dewpoint and humidity NOAA fills in
/// only there.
fn hourly_section(periods: &[ForecastPeriod], options: &SummaryOptions) -> Section {
    let percent = |value: Option<&Quantity>| {
        value.map_or(Value::Missing, |value| {
            Value::quantity(value, QuantityKind::Percent, options)
        })
    };
    let rows = periods
        .iter()
        .map(|period| {
            vec![
                Value::timestamp(period.start_time).into(),
                Value::quantity(&period.temperature, QuantityKind::Temperature, options).into(),
                period
                    .dewpoint
                    .as_ref()
                    .map_or(Value::Missing, |dewpoint| {
                        Value::quantity(dewpoint, QuantityKind::Temperature, options)
                    })
                    .into(),
                percent(Some(&period.probability_of_precipitation)).into(),
                percent(period.relative_humidity.as_ref()).into(),
                wind(period, options).into(),
                Value::text(Some(&period.short_forecast)).into(),
            ]
        })
        .collect();
    Section::Table {
        heading: None,
        columns: vec![
            Column::new("Hour", Some("periods")),
            Column::new("Temp", None).align(Align::Right),
            Column::new("Dewpoint", None).align(Align::Right),
            Column::new("Precip", None).align(Align::Right),
            Column::new("Humidity", None).align(Align::Right),
            Column::new("Wind", None),
            Column::new("Forecast", None),
        ],
        rows,
    }
}

impl Summarize for Feature<Forecast> {
    /// Both forecast endpoints return this type, and
    /// [`Forecast::forecast_generator`] is what tells them apart. It picks the
    /// title and the table shape, which is how it is shown.
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let forecast = &self.properties;
        let hourly = forecast.forecast_generator == ForecastGenerator::Hourly;
        // NOAA's forecast response names neither the office nor the grid
        // cell: the grid is in the request URL and nowhere in the body, and
        // the feature has no `id` either. So the title says which forecast
        // this is and leaves the grid cell to the command the caller typed.
        //
        // There is no subtitle. The updated time is the only thing that could
        // fill one, and a subtitle is a `String` — an instant written into it
        // would be frozen at the offset NOAA sent and would sit unmoved next
        // to a `Covers` fact that follows `--time-zone`. Both are facts here,
        // as they already are on a gridpoint.
        let summary = Summary::new(if hourly {
            "Hourly forecast"
        } else {
            "Forecast"
        })
        .push(Section::Facts {
            heading: None,
            facts: vec![
                Fact::new(
                    "Updated",
                    Some("updateTime"),
                    Value::timestamp(forecast.update_time),
                ),
                Fact::new("Covers", Some("validTimes"), interval(forecast.valid_times)),
            ],
        });

        if forecast.periods.is_empty() {
            return summary.push(Section::Empty {
                key: Some("periods"),
                message: "No forecast periods".to_owned(),
            });
        }
        summary.push(if hourly {
            hourly_section(&forecast.periods, options)
        } else {
            twelve_hour_section(&forecast.periods, options)
        })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        // Envelope.
        ("type", "always Feature"),
        (
            "geometry",
            "the polygon of the grid cell's four corners, the same cell the request named",
        ),
        (
            "properties",
            "the forecast itself; its keys are accounted for one by one",
        ),
        // Shown outside the facts.
        (
            "forecastGenerator",
            "it chooses the title and the table shape, which is how it is shown",
        ),
        // Left out.
        (
            "detailedForecast",
            "a paragraph per period; the short forecast is already a column and twelve paragraphs are not a summary",
        ),
        (
            "generatedAt",
            "when NOAA rendered the text; updateTime is when the data behind it changed",
        ),
        (
            "elevation",
            "the grid cell's elevation, which `gridpoints gridpoint` shows and a forecast does not turn on",
        ),
        (
            "units",
            "the echoed request parameter; the feature flags this crate always sends make it inert, and --units decides what is shown",
        ),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two keys that name layers this table does not render, and so
    /// would go uncovered on a grid that publishes neither.
    #[test]
    fn the_layer_keys_include_the_two_that_have_their_own_tables() {
        assert!(LAYER_KEYS.contains(&"weather"));
        assert!(LAYER_KEYS.contains(&"hazards"));
    }

    #[test]
    fn a_layer_kind_comes_from_its_unit_with_two_named_exceptions() {
        let cases = [
            (
                "temperature",
                Some("wmoUnit:degC"),
                QuantityKind::Temperature,
            ),
            ("windSpeed", Some("wmoUnit:km_h-1"), QuantityKind::Speed),
            ("snowfallAmount", Some("wmoUnit:mm"), QuantityKind::Depth),
            ("pressure", Some("wmoUnit:Pa"), QuantityKind::Pressure),
            ("skyCover", Some("wmoUnit:percent"), QuantityKind::Percent),
            (
                "windDirection",
                Some("wmoUnit:degree_(angle)"),
                QuantityKind::Angle,
            ),
            ("heatRisk", None, QuantityKind::Index),
            // Everything on the ambiguous metre is a height...
            ("ceilingHeight", Some("wmoUnit:m"), QuantityKind::Height),
            ("mixingHeight", Some("wmoUnit:m"), QuantityKind::Height),
            ("waveHeight", Some("wmoUnit:m"), QuantityKind::Height),
            // ...except the two layers named for it.
            ("visibility", Some("wmoUnit:m"), QuantityKind::Distance),
            ("probabilityOfThunder", None, QuantityKind::Percent),
        ];
        for (key, code, expected) in cases {
            let unit = code.map(Unit::from);
            assert_eq!(layer_kind(key, unit.as_ref()), expected, "{key}");
        }
    }

    /// The exceptions are named layers, not named units: whatever unit NOAA
    /// decides to send them in, a visibility is a distance and a probability
    /// of thunder is a percentage.
    #[test]
    fn the_two_exceptions_hold_whatever_unit_arrives() {
        assert_eq!(
            layer_kind("visibility", Some(&Unit::from("wmoUnit:km"))),
            QuantityKind::Distance
        );
        assert_eq!(
            layer_kind("probabilityOfThunder", Some(&Unit::from("wmoUnit:percent"))),
            QuantityKind::Percent
        );
    }

    #[test]
    fn a_hazard_reads_as_english_and_falls_back_to_its_codes() {
        assert_eq!(hazard_name("HT", "Y"), "Heat Advisory");
        assert_eq!(hazard_name("TO", "W"), "Tornado Warning");
        assert_eq!(hazard_name("QQ", "Z"), "QQ Z");
    }

    #[test]
    fn a_warning_is_the_only_hazard_that_escalates() {
        let period = |significance: &str| -> HazardPeriod {
            serde_json::from_value(serde_json::json!({
                "validTime": "2026-09-02T18:00:00+00:00/P2DT7H",
                "value": [{"phenomenon": "HT", "significance": significance}],
            }))
            .expect("hazard period decodes")
        };
        assert_eq!(hazard_emphasis(&period("W")), Emphasis::Danger);
        for significance in ["A", "Y", "S", "F", "O", "N"] {
            assert_eq!(
                hazard_emphasis(&period(significance)),
                Emphasis::Warning,
                "{significance}"
            );
        }
    }

    fn condition(json: serde_json::Value) -> WeatherCondition {
        serde_json::from_value(json).expect("weather condition decodes")
    }

    #[test]
    fn a_phenomenon_composes_into_one_sentence() {
        assert_eq!(
            weather_phrase(&condition(serde_json::json!({
                "coverage": "slight_chance",
                "weather": "rain_showers",
                "intensity": "light",
                "visibility": {"unitCode": "wmoUnit:km", "value": null},
                "attributes": [],
            }))),
            Some("Slight chance of light rain showers".to_owned())
        );
        assert_eq!(
            weather_phrase(&condition(serde_json::json!({
                "coverage": "definite",
                "weather": "fog",
                "intensity": null,
                "visibility": {"unitCode": "wmoUnit:km", "value": null},
                "attributes": [],
            }))),
            Some("Fog".to_owned())
        );
        assert_eq!(
            weather_phrase(&condition(serde_json::json!({
                "coverage": null,
                "weather": "thunderstorms",
                "intensity": null,
                "visibility": {"unitCode": "wmoUnit:km", "value": null},
                "attributes": [],
            }))),
            Some("Thunderstorms".to_owned())
        );
    }

    /// NOAA fills every field with null to say "nothing expected". That is
    /// not a phenomenon and does not earn a row.
    #[test]
    fn nothing_expected_is_not_a_phrase() {
        assert_eq!(
            weather_phrase(&condition(serde_json::json!({
                "coverage": null,
                "weather": null,
                "intensity": null,
                "visibility": {"unitCode": "wmoUnit:km", "value": null},
                "attributes": [],
            }))),
            None
        );
    }
}
