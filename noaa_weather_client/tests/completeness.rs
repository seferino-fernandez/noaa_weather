//! Whether the curated models keep every key NOAA sends.
//!
//! Two tests, one differ. The fixture test replays what
//! `tests/fixtures/capture.sh` last captured, so it is fast, hermetic, and
//! exactly as current as the last person to run `just fixtures`. The live
//! test asks NOAA the same questions now, which is the only way to see a
//! field NOAA added, removed, or renamed since then; it is `#[ignore]`d
//! because it needs the network.
//!
//! The differ has caught real losses: `icon`, `cwa`, `forecastOffices`, and
//! the `@id`/`@type` pair.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use noaa_weather_client::models::radar_station::{CommandChannel, CommandChannelMode};
use noaa_weather_client::models::{
    ActiveAlertCounts, Alert, AlertEventTypes, CenterWeatherAdvisory, CwsuOffice, Forecast,
    GlossaryResponse, Gridpoint, Observation, ObservationStation, Point, RadarStationFeature,
    Sigmet, TerminalAerodromeForecastsResponse, TextProduct, TextProductCollection,
    TextProductLocationCollection, TextProductTypeCollection, Zone, ZoneForecast,
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
    (
        "shortPulseVerticaldBZ0",
        "present on both WSR-88D fixtures KFSX and KLNX captured 2026-09-03; radar curation is deferred",
    ),
    (
        "longPulseVerticaldBZ0",
        "present on both WSR-88D fixtures KFSX and KLNX captured 2026-09-03; radar curation is deferred",
    ),
];

const PATH_WHITELIST: &[(&str, &str)] = &[
    (
        "properties.performance.properties",
        "all 45 TDWR stations sent [] here on 2026-09-03; [] is intentionally normalized to None",
    ),
    (
        "properties.performance.properties[]",
        "all 45 TDWR stations sent [] here on 2026-09-03; [] is intentionally normalized to None",
    ),
    (
        "properties.adaptation.properties",
        "all 45 TDWR stations sent [] here on 2026-09-03; [] is intentionally normalized to None",
    ),
    (
        "properties.adaptation.properties[]",
        "all 45 TDWR stations sent [] here on 2026-09-03; [] is intentionally normalized to None",
    ),
];

const TSLC_NULL_PATH_WHITELIST: &[(&str, &str)] = &[
    (
        "properties.rda.properties.resolutionVersion",
        "null-only exemption on radar/TSLC.json, captured 2026-09-03",
    ),
    (
        "properties.performance.timestamp",
        "null-only exemption on radar/TSLC.json, captured 2026-09-03",
    ),
    (
        "properties.adaptation.timestamp",
        "null-only exemption on radar/TSLC.json, captured 2026-09-03",
    ),
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

fn value_at_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(value, |value, key| value.get(key))
}

fn is_whitelisted(label: &str, path: &str, raw: &Value) -> bool {
    if PATH_WHITELIST
        .iter()
        .any(|(whitelisted, _)| path == *whitelisted)
    {
        return true;
    }
    let is_tscl = matches!(label, "radar/TSLC.json" | "/radar/stations/TSLC");
    if is_tscl
        && TSLC_NULL_PATH_WHITELIST
            .iter()
            .any(|(whitelisted, _)| path == *whitelisted)
        && value_at_path(raw, path).is_some_and(Value::is_null)
    {
        return true;
    }
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
    round_trips::<T>(relative_path, &raw);
}

/// Deserializes `raw` into `T`, serializes it back, and fails on any key
/// path the round trip lost.
///
/// `label` names the response in every message, because this runs over both
/// a file on disk and a URL.
fn round_trips<T>(label: &str, raw: &Value)
where
    T: DeserializeOwned + Serialize,
{
    let typed: T = serde_json::from_value(raw.clone())
        .unwrap_or_else(|error| panic!("failed to deserialize {label}: {error}"));
    let round_tripped = serde_json::to_value(typed)
        .unwrap_or_else(|error| panic!("failed to serialize {label}: {error}"));

    let raw_paths = key_paths(raw);
    let our_paths = key_paths(&round_tripped);

    let missing = raw_paths
        .difference(&our_paths)
        .filter(|path| !is_whitelisted(label, path, raw))
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "{label} silently dropped response paths:\n{}",
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
        "{label} added envelope-level paths:\n{}",
        envelope_extra.join("\n")
    );

    let model_extra = extra
        .iter()
        .filter(|path| !is_envelope_level(path))
        .collect::<Vec<_>>();
    if !model_extra.is_empty() {
        println!("{label}: extra keys deeper in the model:");
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
        ("stations/tafs.json", TerminalAerodromeForecastsResponse),
        ("points/point.json", Feature<Point>),
        ("gridpoints/gridpoint.json", Feature<Gridpoint>),
        ("gridpoints/forecast.json", Feature<Forecast>),
        ("gridpoints/hourly.json", Feature<Forecast>),
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
        ("aviation/cwsu.json", CwsuOffice),
        ("glossary/terms.json", GlossaryResponse),
        ("products/list.json", TextProductCollection),
        ("products/product.json", TextProduct),
        ("products/locations.json", TextProductLocationCollection),
        ("products/types.json", TextProductTypeCollection),
        ("products/type.json", TextProductCollection),
        ("products/type_locations.json", TextProductLocationCollection),
        ("products/type_location.json", TextProductCollection),
        ("products/location_types.json", TextProductTypeCollection),
        ("products/latest.json", TextProduct),
        ("radar/KFSX.json", RadarStationFeature),
        ("radar/KLNX.json", RadarStationFeature),
        ("radar/TSLC.json", RadarStationFeature),
    );
}

#[test]
fn radar_command_channel_round_trips_as_the_original_json_scalar() {
    for (path, expected) in [
        ("radar/KLNX.json", r#""commandChannel":"Single""#),
        ("radar/KFSX.json", r#""commandChannel":2"#),
    ] {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(path);
        let source = fs::read_to_string(&fixture)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture.display()));
        let station: RadarStationFeature = serde_json::from_str(&source)
            .unwrap_or_else(|error| panic!("failed to deserialize {path}: {error}"));
        let serialized = serde_json::to_string(&station)
            .unwrap_or_else(|error| panic!("failed to serialize {path}: {error}"));
        assert!(
            serialized.contains(expected),
            "{path} serialized commandChannel with the wrong JSON scalar: {serialized}"
        );
    }
}

#[test]
fn radar_single_command_channel_deserializes_as_a_mode() {
    let channel: CommandChannel = serde_json::from_str(r#""Single""#).unwrap();

    assert!(matches!(
        channel,
        CommandChannel::Mode(CommandChannelMode::Single)
    ));
}

#[test]
fn radar_empty_properties_arrays_deserialize_as_absent() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/radar/TSLC.json");
    let source = fs::read_to_string(&fixture)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", fixture.display()));
    let station: RadarStationFeature = serde_json::from_str(&source)
        .unwrap_or_else(|error| panic!("failed to deserialize {}: {error}", fixture.display()));
    let station = station.radar_station.as_ref().expect("TSLC properties");

    assert!(
        station
            .performance
            .as_ref()
            .expect("TSLC performance")
            .properties
            .is_none()
    );
    assert!(
        station
            .adaptation
            .as_ref()
            .expect("TSLC adaptation")
            .properties
            .is_none()
    );
}

/// The API root the live test asks.
const BASE_URL: &str = "https://api.weather.gov";

/// The `User-Agent` NOAA asks callers to identify themselves with.
const USER_AGENT: &str = concat!(
    "noaa-weather-completeness/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/seferino-fernandez/noaa_weather)"
);

const GEO_JSON: &str = "application/geo+json";
const JSON_LD: &str = "application/ld+json";

/// The feature flags the client always sends for a forecast, which change
/// what NOAA puts in the response.
const FORECAST_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

/// One live response, fetched as NOAA's own JSON rather than as a model.
///
/// Going through `reqwest` rather than through this crate's own handles is
/// the point: the handles deserialize, and what this test needs is the
/// document before anything typed has touched it.
async fn fetch(client: &reqwest::Client, path: &str, accept: &str, flags: Option<&str>) -> Value {
    let url = format!("{BASE_URL}{path}");
    let mut request = client
        .get(&url)
        .header(reqwest::header::ACCEPT, accept)
        .header(reqwest::header::USER_AGENT, USER_AGENT);
    if let Some(flags) = flags {
        request = request.header("Feature-Flags", flags);
    }

    let response = request
        .send()
        .await
        .unwrap_or_else(|error| panic!("GET {url} failed: {error}"));
    let status = response.status();
    let body = response
        .text()
        .await
        .unwrap_or_else(|error| panic!("GET {url} returned an unreadable body: {error}"));
    assert!(
        status.is_success(),
        "GET {url} answered {status}: {}",
        body.chars().take(400).collect::<String>()
    );
    serde_json::from_str(&body)
        .unwrap_or_else(|error| panic!("GET {url} did not return JSON: {error}\n{body}"))
}

/// A collection's first member, when it has one.
///
/// NOAA legitimately has no SIGMETs and no advisories over a quiet centre,
/// so the caller reports the emptiness rather than failing on it. The
/// collection itself has already been checked by then.
fn first_member<'a>(document: &'a Value, collection: &str) -> Option<&'a Value> {
    document[collection].as_array()?.first()
}

/// A feature's identifier, wherever NOAA put it.
///
/// An alert carries it on the feature and a SIGMET carries it under
/// `properties`, and both are `api.weather.gov` URLs.
fn member_id(member: &Value) -> &str {
    member["id"]
        .as_str()
        .or_else(|| member["properties"]["id"].as_str())
        .unwrap_or_else(|| panic!("no `id` on the feature or its properties: {member}"))
}

/// The request path for a member whose `id` is an `api.weather.gov` URL.
///
/// The colons in an alert's `urn:oid:` identifier are sent as they arrived.
/// NOAA answers the raw form and the percent-escaped form alike, and the
/// client does not escape them either: `client/http.rs` special-cases only
/// `.` and `..`, and `path_segments_mut().push()` leaves `:` alone.
fn path_of(id: &str) -> &str {
    id.strip_prefix(BASE_URL)
        .unwrap_or_else(|| panic!("{id} is not a {BASE_URL} URL"))
}

/// The same check as the fixture test, against NOAA as it is right now.
///
/// The fixture test can only be as current as the last `just fixtures`, so
/// it says nothing about a field NOAA renamed this morning. This does, and
/// is the reason to run it before believing the models are complete.
///
/// `#[ignore]`d because it needs the network and NOAA's cooperation.
#[tokio::test]
#[ignore = "asks live NOAA; run with --run-ignored all"]
async fn live_responses_preserve_every_non_whitelisted_key_path() {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("an HTTP client");

    // The three-argument form comes first: matched the other way round, an
    // `$flags:expr` would swallow the type argument of a call that has no
    // flags, and the error would be about comparison operators.
    macro_rules! live {
        ($path:expr, $accept:expr, $type:ty) => {
            live!($path, $accept, None, $type)
        };
        ($path:expr, $accept:expr, $flags:expr, $type:ty) => {{
            let raw = fetch(&client, $path, $accept, $flags).await;
            round_trips::<$type>($path, &raw);
            raw
        }};
    }

    let alerts = live!("/alerts?limit=5", GEO_JSON, FeatureCollection<Alert>);
    live!("/alerts/active/count", JSON_LD, ActiveAlertCounts);
    live!("/alerts/types", JSON_LD, AlertEventTypes);
    match first_member(&alerts, "features") {
        Some(first) => {
            live!(&path_of(member_id(first)), GEO_JSON, Feature<Alert>);
        }
        None => eprintln!(
            "/alerts?limit=5 returned a well-formed empty `features` array, \
             so there was no id to fetch; the collection was checked, the \
             single-alert response was not"
        ),
    }

    live!(
        "/stations?limit=5",
        GEO_JSON,
        FeatureCollection<ObservationStation>
    );
    live!("/stations/KSLC", GEO_JSON, Feature<ObservationStation>);
    live!(
        "/stations/KSLC/observations?limit=5",
        GEO_JSON,
        FeatureCollection<Observation>
    );
    live!(
        "/stations/KSLC/observations/latest",
        GEO_JSON,
        Feature<Observation>
    );

    live!("/radar/stations/KFSX", GEO_JSON, RadarStationFeature);
    live!("/radar/stations/KLNX", GEO_JSON, RadarStationFeature);
    live!("/radar/stations/TSLC", GEO_JSON, RadarStationFeature);

    live!("/points/39.7456,-97.0892", GEO_JSON, Feature<Point>);

    live!("/glossary", JSON_LD, GlossaryResponse);

    let products = live!("/products?limit=500", JSON_LD, TextProductCollection);
    match first_member(&products, "@graph") {
        Some(first) => {
            let url = first["@id"]
                .as_str()
                .expect("a text-product catalog entry carries @id");
            live!(&path_of(url), JSON_LD, TextProduct);
        }
        None => eprintln!(
            "/products?limit=500 returned a well-formed empty `@graph`, so \
             there was no product id to fetch; the collection was checked, \
             the single-product response was not"
        ),
    }
    live!(
        "/products/locations",
        JSON_LD,
        TextProductLocationCollection
    );
    live!("/products/types", JSON_LD, TextProductTypeCollection);
    live!("/products/types/AFD", JSON_LD, TextProductCollection);
    live!(
        "/products/types/AFD/locations",
        JSON_LD,
        TextProductLocationCollection
    );
    live!(
        "/products/types/AFD/locations/LWX",
        JSON_LD,
        TextProductCollection
    );
    live!(
        "/products/locations/PSR/types",
        JSON_LD,
        TextProductTypeCollection
    );
    live!(
        "/products/types/AFD/locations/PSR/latest",
        JSON_LD,
        TextProduct
    );

    live!("/gridpoints/TOP/31,80", GEO_JSON, Feature<Gridpoint>);
    live!(
        "/gridpoints/TOP/31,80/forecast",
        GEO_JSON,
        Some(FORECAST_FLAGS),
        Feature<Forecast>
    );
    live!(
        "/gridpoints/TOP/31,80/forecast/hourly",
        GEO_JSON,
        Some(FORECAST_FLAGS),
        Feature<Forecast>
    );
    live!(
        "/gridpoints/TOP/31,80/stations?limit=5",
        GEO_JSON,
        FeatureCollection<ObservationStation>
    );

    live!("/zones?limit=5", GEO_JSON, FeatureCollection<Zone>);
    live!("/zones/forecast/UTZ101", GEO_JSON, Feature<Zone>);
    live!(
        "/zones/forecast/UTZ101/forecast",
        GEO_JSON,
        Feature<ZoneForecast>
    );
    live!(
        "/zones/forecast/UTZ101/observations?limit=5",
        GEO_JSON,
        FeatureCollection<Observation>
    );
    live!(
        "/zones/forecast/UTZ101/stations?limit=5",
        GEO_JSON,
        FeatureCollection<ObservationStation>
    );

    let sigmets = live!("/aviation/sigmets", GEO_JSON, FeatureCollection<Sigmet>);
    match first_member(&sigmets, "features") {
        Some(first) => {
            live!(&path_of(member_id(first)), GEO_JSON, Feature<Sigmet>);
        }
        None => eprintln!(
            "/aviation/sigmets returned a well-formed empty `features` \
             array, so there were no current SIGMETs; the collection was \
             checked, the single-SIGMET response was not"
        ),
    }

    let advisories = live!(
        "/aviation/cwsus/ZAB/cwas",
        GEO_JSON,
        FeatureCollection<CenterWeatherAdvisory>
    );
    live!("/aviation/cwsus/ZAB", JSON_LD, CwsuOffice);
    match first_member(&advisories, "features") {
        Some(first) => {
            let properties = &first["properties"];
            let cwsu = properties["cwsu"].as_str().expect("a CWA carries its CWSU");
            let issued = properties["issueTime"]
                .as_str()
                .expect("a CWA carries its issue time");
            let sequence = properties["sequence"]
                .as_u64()
                .expect("a CWA carries its sequence");
            let date = issued.split('T').next().expect("an issue date");
            live!(
                &format!("/aviation/cwsus/{cwsu}/cwas/{date}/{sequence}"),
                GEO_JSON,
                Feature<CenterWeatherAdvisory>
            );
        }
        None => eprintln!(
            "/aviation/cwsus/ZAB/cwas returned a well-formed empty \
             `features` array, so ZAB has no current advisory; the \
             collection was checked, the single-CWA response was not"
        ),
    }
}
