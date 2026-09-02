use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use noaa_weather_client::models::{
    ActiveAlertCounts, Alert, AlertEventTypes, CenterWeatherAdvisory, Gridpoint,
    Gridpoint12hForecast, GridpointHourlyForecast, Observation, ObservationStation, Point, Sigmet,
    Zone, ZoneForecast,
};
use noaa_weather_client::{Feature, FeatureCollection};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

const WHITELIST: &[(&str, &str)] = &[
    (
        "@context",
        "JSON-LD vocabulary, identical on every response, no weather data",
    ),
    ("observationStations", "duplicates features[].id"),
];

fn collect_key_paths(value: &Value, prefix: &str, paths: &mut BTreeSet<String>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                paths.insert(path.clone());
                if !WHITELIST.iter().any(|(whitelisted, _)| key == whitelisted) {
                    collect_key_paths(child, &path, paths);
                }
            }
        }
        Value::Array(array) => {
            let path = format!("{prefix}[]");
            paths.insert(path.clone());
            for child in array {
                collect_key_paths(child, &path, paths);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

fn key_paths(value: &Value) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    collect_key_paths(value, "", &mut paths);
    paths
}

fn is_whitelisted(path: &str) -> bool {
    let final_segment = path.rsplit('.').next().unwrap_or(path);
    WHITELIST.iter().any(|(key, _)| final_segment == *key)
}

fn is_envelope_level(path: &str) -> bool {
    if !path.contains('.') {
        return true;
    }

    path.strip_prefix("features[].")
        .is_some_and(|remainder| !remainder.contains('.') && !remainder.contains("[]"))
}

fn check_fixture<T>(relative_path: &str)
where
    T: DeserializeOwned + Serialize,
{
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(relative_path);
    let source = fs::read_to_string(&fixture)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture.display()));
    let raw: Value = serde_json::from_str(&source)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", fixture.display()));
    let typed: T = serde_json::from_value(raw.clone())
        .unwrap_or_else(|error| panic!("failed to deserialize {}: {error}", fixture.display()));
    let round_tripped = serde_json::to_value(typed)
        .unwrap_or_else(|error| panic!("failed to serialize {}: {error}", fixture.display()));

    let raw_paths = key_paths(&raw);
    let our_paths = key_paths(&round_tripped);

    let missing = raw_paths
        .difference(&our_paths)
        .filter(|path| !is_whitelisted(path))
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "{relative_path} silently dropped response paths:\n{}",
        missing.join("\n")
    );

    let extra = our_paths
        .difference(&raw_paths)
        .cloned()
        .collect::<Vec<_>>();
    let envelope_extra = extra
        .iter()
        .filter(|path| is_envelope_level(path))
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        envelope_extra.is_empty(),
        "{relative_path} added envelope-level paths:\n{}",
        envelope_extra.join("\n")
    );

    let model_extra = extra
        .iter()
        .filter(|path| !is_envelope_level(path))
        .collect::<Vec<_>>();
    if !model_extra.is_empty() {
        println!("{relative_path}: extra keys deeper in the model:");
        for path in model_extra {
            println!("  {path}");
        }
    }
}

#[test]
fn captured_responses_preserve_every_non_whitelisted_key_path() {
    macro_rules! fixtures {
        ($(($path:literal, $type:ty)),+ $(,)?) => {
            $(check_fixture::<$type>($path);)+
        };
    }

    fixtures!(
        ("alerts/list.json", FeatureCollection<Alert>),
        ("alerts/single.json", Feature<Alert>),
        ("alerts/count.json", ActiveAlertCounts),
        ("alerts/types.json", AlertEventTypes),
        ("stations/list.json", FeatureCollection<ObservationStation>),
        ("stations/single.json", Feature<ObservationStation>),
        ("stations/observations.json", FeatureCollection<Observation>),
        ("stations/latest.json", Feature<Observation>),
        ("points/point.json", Feature<Point>),
        ("gridpoints/gridpoint.json", Feature<Gridpoint>),
        ("gridpoints/forecast.json", Feature<Gridpoint12hForecast>),
        ("gridpoints/hourly.json", Feature<GridpointHourlyForecast>),
        (
            "gridpoints/stations.json",
            FeatureCollection<ObservationStation>
        ),
        ("zones/list.json", FeatureCollection<Zone>),
        ("zones/single.json", Feature<Zone>),
        ("zones/forecast.json", Feature<ZoneForecast>),
        ("zones/observations.json", FeatureCollection<Observation>),
        ("zones/stations.json", FeatureCollection<ObservationStation>),
        ("aviation/sigmets.json", FeatureCollection<Sigmet>),
        ("aviation/sigmet.json", Feature<Sigmet>),
        (
            "aviation/cwas.json",
            FeatureCollection<CenterWeatherAdvisory>
        ),
        ("aviation/cwa.json", Feature<CenterWeatherAdvisory>),
    );
}
