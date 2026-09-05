//! Keeps the command guides' property-accounting tables in sync with `Summarize`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use noaa_weather_client::models::{
    ActiveAlertCounts, Alert, AlertEventTypes, CenterWeatherAdvisory, CwsuOffice, Forecast,
    GlossaryResponse, Gridpoint, Observation, ObservationStation, Office, OfficeBriefingResponse,
    OfficeHeadline, OfficeHeadlineCollection, OfficeWeatherStoryCollection, Point,
    RadarQueuesResponse, RadarServerTelemetry, RadarServersResponse, RadarSpgdsResponse,
    RadarStationAlarmsResponse, RadarStationTelemetry, RadarStationsResponse, RadioBroadcast,
    RadioTransmitter, RadioTransmitterCollection, Sigmet, TerminalAerodromeForecast,
    TerminalAerodromeForecastsResponse, TextProduct, TextProductCollection,
    TextProductLocationCollection, TextProductTypeCollection, Zone, ZoneForecast,
};
use noaa_weather_client::{Feature, FeatureCollection};
use noaa_weather_summary::stations::ZoneObservations;
use noaa_weather_summary::{Summarize, SummaryOptions};
use serde::de::DeserializeOwned;
use serde_json::Value as Json;

const BEGIN: &str = "<!-- BEGIN GENERATED SHOWN/OMITTED -->";
const END: &str = "<!-- END GENERATED SHOWN/OMITTED -->";
const UPDATE_ENV: &str = "UPDATE_SHOWN_OMITTED_DOCS";

#[derive(Clone, Copy)]
enum PropertyShape {
    Api,
    SemanticTopLevel,
}

struct RegisteredResponse {
    guide: &'static str,
    name: &'static str,
    rust_impl: &'static str,
    rows: Vec<PropertyRow>,
}

struct PropertyRow {
    property: String,
    treatment: &'static str,
    reason: Option<&'static str>,
}

fn fixture<T: DeserializeOwned>(source: &str) -> T {
    serde_json::from_str(source).expect("checked-in JSON fixture decodes")
}

fn register<T: Summarize>(
    guide: &'static str,
    name: &'static str,
    rust_impl: &'static str,
    value: T,
    shape: PropertyShape,
) -> RegisteredResponse {
    let json = serde_json::to_value(&value).expect("response serializes to JSON");
    let shown = value
        .summarize(&SummaryOptions::default())
        .keys()
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    let omitted = T::OMITTED.iter().copied().collect::<BTreeMap<_, _>>();
    let mut properties = match shape {
        PropertyShape::Api => api_property_keys(&json),
        PropertyShape::SemanticTopLevel => json
            .as_object()
            .into_iter()
            .flat_map(|object| object.keys().cloned())
            .collect(),
    };
    properties.extend(shown.iter().cloned());
    properties.extend(omitted.keys().map(|key| (*key).to_owned()));

    let rows = properties
        .into_iter()
        .map(|property| {
            if shown.contains(&property) {
                PropertyRow {
                    property,
                    treatment: "Shown",
                    reason: None,
                }
            } else if let Some(reason) = omitted.get(property.as_str()) {
                PropertyRow {
                    property,
                    treatment: "Otherwise accounted for",
                    reason: Some(reason),
                }
            } else {
                panic!(
                    "{name}: property {property:?} is neither shown nor otherwise accounted for"
                );
            }
        })
        .collect();

    RegisteredResponse {
        guide,
        name,
        rust_impl,
        rows,
    }
}

fn register_json<T: DeserializeOwned + Summarize>(
    guide: &'static str,
    name: &'static str,
    rust_impl: &'static str,
    source: &str,
) -> RegisteredResponse {
    register(
        guide,
        name,
        rust_impl,
        fixture::<T>(source),
        PropertyShape::Api,
    )
}

fn registry() -> Vec<RegisteredResponse> {
    vec![
        register_json::<FeatureCollection<Alert>>(
            "docs/cli/alerts.md",
            "Active alert list",
            "FeatureCollection<Alert>",
            include_str!("../../noaa_weather_client/tests/fixtures/alerts/list.json"),
        ),
        register_json::<Feature<Alert>>(
            "docs/cli/alerts.md",
            "Alert",
            "Feature<Alert>",
            include_str!("../../noaa_weather_client/tests/fixtures/alerts/single.json"),
        ),
        register_json::<ActiveAlertCounts>(
            "docs/cli/alerts.md",
            "Active alert count",
            "ActiveAlertCounts",
            include_str!("../../noaa_weather_client/tests/fixtures/alerts/count.json"),
        ),
        register_json::<AlertEventTypes>(
            "docs/cli/alerts.md",
            "Alert event types",
            "AlertEventTypes",
            include_str!("../../noaa_weather_client/tests/fixtures/alerts/types.json"),
        ),
        register_json::<CwsuOffice>(
            "docs/cli/aviation.md",
            "Center Weather Service Unit",
            "CwsuOffice",
            include_str!("../../noaa_weather_client/tests/fixtures/aviation/cwsu.json"),
        ),
        register_json::<Feature<CenterWeatherAdvisory>>(
            "docs/cli/aviation.md",
            "Center Weather Advisory",
            "Feature<CenterWeatherAdvisory>",
            include_str!("../../noaa_weather_client/tests/fixtures/aviation/cwa.json"),
        ),
        register_json::<FeatureCollection<CenterWeatherAdvisory>>(
            "docs/cli/aviation.md",
            "Center Weather Advisory list",
            "FeatureCollection<CenterWeatherAdvisory>",
            include_str!("../../noaa_weather_client/tests/fixtures/aviation/cwas.json"),
        ),
        register_json::<Feature<Sigmet>>(
            "docs/cli/aviation.md",
            "SIGMET or AIRMET",
            "Feature<Sigmet>",
            include_str!("../../noaa_weather_client/tests/fixtures/aviation/sigmet.json"),
        ),
        register_json::<FeatureCollection<Sigmet>>(
            "docs/cli/aviation.md",
            "SIGMET or AIRMET list",
            "FeatureCollection<Sigmet>",
            include_str!("../../noaa_weather_client/tests/fixtures/aviation/sigmets.json"),
        ),
        register_json::<GlossaryResponse>(
            "docs/cli/glossary.md",
            "Glossary",
            "GlossaryResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/glossary/terms.json"),
        ),
        register_json::<Feature<Gridpoint>>(
            "docs/cli/gridpoints.md",
            "Gridpoint",
            "Feature<Gridpoint>",
            include_str!("../../noaa_weather_client/tests/fixtures/gridpoints/gridpoint.json"),
        ),
        register_json::<Feature<Forecast>>(
            "docs/cli/gridpoints.md",
            "Forecast and hourly forecast",
            "Feature<Forecast>",
            include_str!("../../noaa_weather_client/tests/fixtures/gridpoints/forecast.json"),
        ),
        register_json::<Office>(
            "docs/cli/offices.md",
            "Office metadata",
            "Office",
            include_str!("../../noaa_weather_client/tests/fixtures/offices/office.json"),
        ),
        register_json::<OfficeHeadline>(
            "docs/cli/offices.md",
            "Office headline",
            "OfficeHeadline",
            include_str!("../../noaa_weather_client/tests/fixtures/offices/headline.json"),
        ),
        register_json::<OfficeHeadlineCollection>(
            "docs/cli/offices.md",
            "Office headline list",
            "OfficeHeadlineCollection",
            include_str!("../../noaa_weather_client/tests/fixtures/offices/headlines.json"),
        ),
        register_json::<OfficeBriefingResponse>(
            "docs/cli/offices.md",
            "Active briefing metadata",
            "OfficeBriefingResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/offices/briefing.json"),
        ),
        register_json::<OfficeWeatherStoryCollection>(
            "docs/cli/offices.md",
            "Active weather-story metadata",
            "OfficeWeatherStoryCollection",
            include_str!("../../noaa_weather_client/tests/fixtures/offices/weather_stories.json"),
        ),
        register_json::<Feature<Point>>(
            "docs/cli/points.md",
            "Point metadata",
            "Feature<Point>",
            include_str!("../../noaa_weather_client/tests/fixtures/points/point.json"),
        ),
        register_json::<TextProduct>(
            "docs/cli/products.md",
            "Text product",
            "TextProduct",
            include_str!("../../noaa_weather_client/tests/fixtures/products/product.json"),
        ),
        register_json::<TextProductCollection>(
            "docs/cli/products.md",
            "Text product list",
            "TextProductCollection",
            include_str!("../../noaa_weather_client/tests/fixtures/products/list.json"),
        ),
        register_json::<TextProductLocationCollection>(
            "docs/cli/products.md",
            "Product location list",
            "TextProductLocationCollection",
            include_str!("../../noaa_weather_client/tests/fixtures/products/locations.json"),
        ),
        register_json::<TextProductTypeCollection>(
            "docs/cli/products.md",
            "Product type list",
            "TextProductTypeCollection",
            include_str!("../../noaa_weather_client/tests/fixtures/products/types.json"),
        ),
        register_json::<RadarStationTelemetry>(
            "docs/cli/radar.md",
            "Radar station telemetry",
            "RadarStationTelemetry",
            include_str!("../../noaa_weather_client/tests/fixtures/radar/station.json"),
        ),
        register_json::<RadarStationsResponse>(
            "docs/cli/radar.md",
            "Radar station list",
            "RadarStationsResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/radar/stations.json"),
        ),
        register_json::<RadarStationAlarmsResponse>(
            "docs/cli/radar.md",
            "Radar station alarms",
            "RadarStationAlarmsResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/radar/alarms.json"),
        ),
        register_json::<RadarQueuesResponse>(
            "docs/cli/radar.md",
            "Radar data queue",
            "RadarQueuesResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/radar/queue.json"),
        ),
        register_json::<RadarServerTelemetry>(
            "docs/cli/radar.md",
            "Radar server telemetry",
            "RadarServerTelemetry",
            include_str!("../../noaa_weather_client/tests/fixtures/radar/server.json"),
        ),
        register_json::<RadarServersResponse>(
            "docs/cli/radar.md",
            "Radar server list",
            "RadarServersResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/radar/servers.json"),
        ),
        register_json::<RadarSpgdsResponse>(
            "docs/cli/radar.md",
            "Radar SPGDS telemetry",
            "RadarSpgdsResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/radar/spgds.json"),
        ),
        register_json::<RadioTransmitter>(
            "docs/cli/radio.md",
            "Radio transmitter",
            "RadioTransmitter",
            include_str!("../../noaa_weather_client/tests/fixtures/radio/transmitter.json"),
        ),
        register_json::<RadioTransmitterCollection>(
            "docs/cli/radio.md",
            "Radio transmitter list",
            "RadioTransmitterCollection",
            include_str!("../../noaa_weather_client/tests/fixtures/radio/transmitters.json"),
        ),
        register(
            "docs/cli/radio.md",
            "Radio broadcast",
            "RadioBroadcast",
            RadioBroadcast::from_ssml(include_str!(
                "../../noaa_weather_client/tests/fixtures/radio/broadcast.xml"
            ))
            .expect("checked-in radio fixture decodes"),
            PropertyShape::SemanticTopLevel,
        ),
        register_json::<Feature<ObservationStation>>(
            "docs/cli/stations.md",
            "Observation station",
            "Feature<ObservationStation>",
            include_str!("../../noaa_weather_client/tests/fixtures/stations/single.json"),
        ),
        register_json::<FeatureCollection<ObservationStation>>(
            "docs/cli/stations.md",
            "Observation station list",
            "FeatureCollection<ObservationStation>",
            include_str!("../../noaa_weather_client/tests/fixtures/stations/list.json"),
        ),
        register_json::<Feature<Observation>>(
            "docs/cli/stations.md",
            "Latest or specific observation",
            "Feature<Observation>",
            include_str!("../../noaa_weather_client/tests/fixtures/stations/latest.json"),
        ),
        register_json::<FeatureCollection<Observation>>(
            "docs/cli/stations.md",
            "Station observation history",
            "FeatureCollection<Observation>",
            include_str!("../../noaa_weather_client/tests/fixtures/stations/observations.json"),
        ),
        register(
            "docs/cli/stations.md",
            "Zone observation list",
            "ZoneObservations",
            ZoneObservations(fixture::<FeatureCollection<Observation>>(include_str!(
                "../../noaa_weather_client/tests/fixtures/zones/observations.json"
            ))),
            PropertyShape::Api,
        ),
        register_json::<TerminalAerodromeForecastsResponse>(
            "docs/cli/stations.md",
            "Terminal Aerodrome Forecast list",
            "TerminalAerodromeForecastsResponse",
            include_str!("../../noaa_weather_client/tests/fixtures/stations/tafs.json"),
        ),
        register(
            "docs/cli/stations.md",
            "Decoded Terminal Aerodrome Forecast",
            "TerminalAerodromeForecast",
            TerminalAerodromeForecast::from_iwxxm(include_bytes!(
                "../../noaa_weather_client/tests/fixtures/stations/taf.xml"
            ))
            .expect("checked-in TAF fixture decodes"),
            PropertyShape::SemanticTopLevel,
        ),
        register_json::<Feature<Zone>>(
            "docs/cli/zones.md",
            "Zone metadata",
            "Feature<Zone>",
            include_str!("../../noaa_weather_client/tests/fixtures/zones/single.json"),
        ),
        register_json::<FeatureCollection<Zone>>(
            "docs/cli/zones.md",
            "Zone list",
            "FeatureCollection<Zone>",
            include_str!("../../noaa_weather_client/tests/fixtures/zones/list.json"),
        ),
        register_json::<Feature<ZoneForecast>>(
            "docs/cli/zones.md",
            "Zone forecast",
            "Feature<ZoneForecast>",
            include_str!("../../noaa_weather_client/tests/fixtures/zones/forecast.json"),
        ),
    ]
}

fn api_property_keys(json: &Json) -> BTreeSet<String> {
    let mut keys = BTreeSet::new();
    let Some(object) = json.as_object() else {
        return keys;
    };
    keys.extend(object.keys().cloned());
    if let Some(properties) = object.get("properties").and_then(Json::as_object) {
        keys.extend(properties.keys().cloned());
    }
    if let Some(features) = object.get("features").and_then(Json::as_array) {
        for feature in features.iter().filter_map(Json::as_object) {
            keys.extend(feature.keys().filter(|key| *key != "properties").cloned());
            if let Some(properties) = feature.get("properties").and_then(Json::as_object) {
                keys.extend(properties.keys().cloned());
            }
        }
    }
    if let Some(graph) = object.get("@graph").and_then(Json::as_array) {
        for item in graph.iter().filter_map(Json::as_object) {
            keys.extend(item.keys().cloned());
        }
    }
    if let Some(address) = object.get("address").and_then(Json::as_object) {
        keys.extend(address.keys().cloned());
    }
    if let Some(briefing) = object.get("briefing").and_then(Json::as_object) {
        keys.extend(briefing.keys().cloned());
    }
    if let Some(stories) = object.get("stories").and_then(Json::as_array) {
        for story in stories.iter().filter_map(Json::as_object) {
            keys.extend(story.keys().cloned());
        }
    }
    if is_radar_response(object) {
        collect_radar_keys(json, &mut keys);
    }
    keys
}

fn is_radar_response(object: &serde_json::Map<String, Json>) -> bool {
    object
        .get("properties")
        .and_then(Json::as_object)
        .is_some_and(|properties| properties.contains_key("stationType"))
        || object.contains_key("ping")
        || object
            .get("@graph")
            .and_then(Json::as_array)
            .and_then(|items| items.first())
            .and_then(Json::as_object)
            .is_some_and(|item| item.contains_key("dataflow"))
}

fn collect_radar_keys(json: &Json, keys: &mut BTreeSet<String>) {
    match json {
        Json::Object(object) => {
            for (key, value) in object {
                keys.insert(key.clone());
                if key == "geometry" || is_measurement(value) {
                    continue;
                }
                if key == "targets" {
                    if let Some(targets) = value.as_object() {
                        keys.extend(targets.keys().cloned());
                    }
                    continue;
                }
                if key == "spg" {
                    if let Some(gateways) = value.as_object() {
                        for gateway in gateways.values() {
                            collect_radar_keys(gateway, keys);
                        }
                    }
                    continue;
                }
                collect_radar_keys(value, keys);
            }
        }
        Json::Array(array) => {
            for value in array {
                collect_radar_keys(value, keys);
            }
        }
        Json::Null | Json::Bool(_) | Json::Number(_) | Json::String(_) => {}
    }
}

fn is_measurement(value: &Json) -> bool {
    value.as_object().is_some_and(|object| {
        !object.is_empty()
            && object.keys().all(|key| {
                matches!(
                    key.as_str(),
                    "value" | "minValue" | "maxValue" | "unitCode" | "qualityControl"
                )
            })
    })
}

fn rendered_guides(registry: &[RegisteredResponse]) -> BTreeMap<&'static str, String> {
    let mut by_guide = BTreeMap::<_, Vec<_>>::new();
    for response in registry {
        by_guide.entry(response.guide).or_default().push(response);
    }

    by_guide
        .into_iter()
        .map(|(guide, mut responses)| {
            responses.sort_by_key(|response| response.name);
            let mut block = format!(
                "{BEGIN}\n\n## Human-summary property coverage\n\nThe table is generated from the summary contracts. `Shown` properties appear in keyed human-summary content; `Otherwise accounted for` properties are deliberately handled without a keyed table or fact.\n\n| Response | Property | Treatment | Reason |\n| :--- | :--- | :--- | :--- |\n"
            );
            for response in responses {
                for row in &response.rows {
                    let reason = row.reason.map_or_else(|| "—".to_owned(), escape_markdown);
                    writeln!(
                        block,
                        "| {} | `{}` | {} | {} |",
                        escape_markdown(response.name),
                        escape_code(&row.property),
                        row.treatment,
                        reason
                    )
                    .expect("writing to a String cannot fail");
                }
            }
            write!(block, "\n{END}").expect("writing to a String cannot fail");
            (guide, block)
        })
        .collect()
}

fn escape_markdown(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace(['\r', '\n'], "<br>")
}

fn escape_code(value: &str) -> String {
    escape_markdown(value).replace('`', "\\`")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("summary crate is inside the workspace")
        .to_owned()
}

fn current_block<'a>(source: &'a str, path: &Path) -> &'a str {
    let begin = source
        .find(BEGIN)
        .unwrap_or_else(|| panic!("{} has no {BEGIN} marker", path.display()));
    let end_offset = source[begin..]
        .find(END)
        .unwrap_or_else(|| panic!("{} has no {END} marker", path.display()));
    let end = begin + end_offset + END.len();
    &source[begin..end]
}

fn replace_block(source: &str, replacement: &str, path: &Path) -> String {
    let current = current_block(source, path);
    source.replacen(current, replacement, 1)
}

#[test]
fn generated_shown_and_omitted_docs_are_current() {
    let registry = registry();
    let root = workspace_root();
    let update = std::env::var_os(UPDATE_ENV).is_some_and(|value| value == "1");
    let mut stale = Vec::new();

    for (relative, expected) in rendered_guides(&registry) {
        let path = root.join(relative);
        let source = fs::read_to_string(&path).expect("guide is readable UTF-8");
        if current_block(&source, &path) == expected {
            continue;
        }
        if update {
            fs::write(&path, replace_block(&source, &expected, &path))
                .expect("updated guide can be written");
        } else {
            stale.push(relative);
        }
    }

    assert!(
        stale.is_empty(),
        "stale generated human-summary property tables: {}\nrun `just shown-omitted-docs`",
        stale.join(", ")
    );
}

#[test]
fn registry_covers_every_production_summarize_impl() {
    let registered = registry()
        .into_iter()
        .map(|response| normalize_type(response.rust_impl))
        .collect::<BTreeSet<_>>();
    let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut implemented = BTreeSet::new();

    for entry in fs::read_dir(source_dir).expect("summary source directory is readable") {
        let path = entry.expect("source entry is readable").path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs")
            || path.file_name().and_then(|name| name.to_str()) == Some("audit.rs")
        {
            continue;
        }
        let source = fs::read_to_string(path).expect("summary source is readable UTF-8");
        for suffix in source.split("impl Summarize for ").skip(1) {
            let declaration = suffix
                .split_once('{')
                .map(|(declaration, _)| declaration)
                .expect("Summarize impl has a body");
            implemented.insert(normalize_type(declaration));
        }
    }

    assert_eq!(
        registered, implemented,
        "the docs registry and production Summarize implementations differ"
    );
}

fn normalize_type(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

#[test]
fn markdown_table_content_is_escaped() {
    assert_eq!(
        escape_markdown("first | second\nthird\\fourth"),
        "first \\| second<br>third\\\\fourth"
    );
    assert_eq!(escape_code("property`name"), "property\\`name");
}

#[test]
fn block_replacement_preserves_surrounding_prose() {
    let path = Path::new("guide.md");
    let source = format!("before\n{BEGIN}\nold\n{END}\nafter\n");
    let replacement = format!("{BEGIN}\nnew\n{END}");

    assert_eq!(
        replace_block(&source, &replacement, path),
        format!("before\n{BEGIN}\nnew\n{END}\nafter\n")
    );
}
