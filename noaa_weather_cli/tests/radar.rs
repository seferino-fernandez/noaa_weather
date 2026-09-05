//! The `radar` family, end to end.

mod common;

use common::noaa_weather;
use common::runner::{family, hermetic, live, run_against, stderr};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn every_radar_invocation_asks_for_the_path_and_query_the_table_records() {
    hermetic(family("radar")).await;
}

#[test]
fn test_radar_live_noaa_answers_every_tabled_invocation() {
    live(family("radar"));
}

#[test]
fn test_radar_data_queue_rejects_a_limit_outside_the_accepted_range() {
    for value in ["0", "50001"] {
        let output = noaa_weather()
            .args(["radar", "data-queue", "--host", "rds", "--limit", value])
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert_eq!(output.status.code(), Some(2), "{value}: {stderr}");
        assert!(
            stderr.contains(&format!("invalid value '{value}' for '--limit <LIMIT>'"))
                && stderr.contains(&format!("{value} is not in 1..=50000")),
            "{value}: {stderr}"
        );
    }
}

/// NOAA's real answer to `GET /radar/spgds?published=PT1H`, captured live:
/// `detail` is the bare string `"Bad Request"`, and `query.published` is
/// named only inside `parameterErrors`, which `ProblemDetail` does not model.
const BAD_REQUEST: &str = r#"{
  "correlationId": "1373f621",
  "parameterErrors": [
    {
      "parameter": "query.published",
      "message": "Does not match the regex pattern ^P(\\d+Y)?(\\d+M)?(\\d+D)?(T(\\d+H)?(\\d+M)?(\\d+S)?)?\\/(\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}(Z|[+-]\\d{2}:?\\d{2}?)|NOW)$"
    },
    {
      "parameter": "query.published",
      "message": "Failed to match exactly one schema"
    }
  ],
  "title": "Bad Request",
  "type": "https://api.weather.gov/problems/BadRequest",
  "status": 400,
  "detail": "Bad Request",
  "instance": "https://api.weather.gov/requests/1373f621"
}"#;

/// NOAA's own account of a refusal reaches the caller, including the parts
/// no model has a field for.
///
/// The body below is what `GET /radar/spgds?published=PT1H` really answers,
/// captured live. Two things about it decide this test. `detail` is the
/// bare string `"Bad Request"` — it does not name the parameter. And
/// `parameterErrors`, which is where the parameter *is* named, is not a
/// member of `ProblemDetail`; the client parses what it models and the CLI
/// prints the response body as it arrived.
///
/// So `query.published` on standard error can only have come through the
/// unmodelled half of the body. An earlier version of this test invented a
/// `detail` containing that string, which made the same assertion pass on
/// text this file had written itself.
#[tokio::test]
async fn test_radar_spgds_error_surfaces_unmodelled_problem_members() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/radar/spgds"))
        .respond_with(
            ResponseTemplate::new(400).set_body_raw(BAD_REQUEST, "application/problem+json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let output = run_against(&server, &["radar", "spgds", "--published", "PT1H"]).await;
    let stderr = stderr(&output);

    assert_eq!(output.status.code(), Some(3), "{stderr}");
    assert!(stderr.contains("getting radar SPGDS telemetry"), "{stderr}");
    assert!(stderr.contains("HTTP 400 Bad Request"), "{stderr}");

    // This string is in `parameterErrors` and nowhere else in the body, so
    // seeing it proves the unmodelled member survived to the caller.
    assert!(
        BAD_REQUEST.matches("query.published").count() > 0
            && !BAD_REQUEST.contains(r#""detail": "query.published"#),
        "the fixture must keep `query.published` out of `detail`, or this \
         test stops proving anything"
    );
    assert!(stderr.contains("parameterErrors"), "{stderr}");
    assert!(stderr.contains("query.published"), "{stderr}");
    server.verify().await;
}

/// The same body under `--json`, where the promise is structural rather
/// than textual.
///
/// The prose line embeds the response verbatim, so a substring check on it
/// passes whether or not anything understood the body. `error.problem` is
/// the documented machine-readable half, and it used to be re-serialized
/// through `ProblemDetail` — which has no `parameterErrors` field, so the
/// four entries naming the bad parameter were dropped from the one place a
/// program would look, leaving it to regex the human message.
#[tokio::test]
async fn test_radar_spgds_error_line_embeds_the_problem_body_whole() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/radar/spgds"))
        .respond_with(
            ResponseTemplate::new(400).set_body_raw(BAD_REQUEST, "application/problem+json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let output = run_against(
        &server,
        &["radar", "spgds", "--published", "PT1H", "--json"],
    )
    .await;
    assert_eq!(output.status.code(), Some(3), "{}", stderr(&output));

    let line = stderr(&output);
    let report: serde_json::Value = serde_json::from_str(line.trim_end())
        .unwrap_or_else(|error| panic!("standard error did not parse: {error}\n{line}"));
    let problem = &report["error"]["problem"];

    // Every key NOAA sent, and no more: the body is passed through rather
    // than rebuilt from the fields this workspace happens to model.
    let sent: serde_json::Value = serde_json::from_str(BAD_REQUEST).expect("the fixture parses");
    let mut wire: Vec<&String> = sent.as_object().expect("an object").keys().collect();
    let mut seen: Vec<&String> = problem.as_object().expect("an object").keys().collect();
    wire.sort();
    seen.sort();
    assert_eq!(seen, wire, "`problem` is not the body NOAA sent: {problem}");

    // The member `ProblemDetail` does not model, with its contents intact.
    let parameters = problem["parameterErrors"]
        .as_array()
        .unwrap_or_else(|| panic!("`problem` dropped `parameterErrors`: {problem}"));
    assert_eq!(parameters.len(), 2, "{problem}");
    assert_eq!(parameters[0]["parameter"], "query.published");
    assert!(
        parameters[0]["message"]
            .as_str()
            .is_some_and(|message| message.contains("regex pattern")),
        "{problem}"
    );

    // `ProblemDetail::status` is `f64`, so round-tripping turned NOAA's 400
    // into 400.0 while the sibling `status` stayed an integer. One line, one
    // key name, two JSON types.
    assert_eq!(problem["status"], serde_json::json!(400));
    assert!(
        problem["status"].is_i64() || problem["status"].is_u64(),
        "{problem}"
    );
    assert_eq!(report["error"]["status"], serde_json::json!(400));
    server.verify().await;
}

#[test]
fn test_radar_spgds_rejects_malformed_interval_as_usage_error() {
    let output = noaa_weather()
        .args(["radar", "spgds", "--published", "PT1H/NOW"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid interval"), "{stderr}");
}

#[test]
fn test_radar_station_rejects_malformed_station_id() {
    let output = noaa_weather()
        .args(["radar", "station", "--station-id", "KAB"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("invalid radar station id"), "{stderr}");
}

/// `radar spgds` summarizes the response for people and preserves the typed
/// wire shape for machines.
#[tokio::test]
async fn test_radar_spgds_supports_table_and_json() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/radar/spgds"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(common::fixtures::RADAR_SPGDS, common::fixtures::JSON_LD),
        )
        .mount(&server)
        .await;

    let table = run_against(&server, &["radar", "spgds"]).await;
    assert_eq!(table.status.code(), Some(0), "{}", stderr(&table));
    let table_text = String::from_utf8_lossy(&table.stdout);
    assert!(table_text.contains("Radar SPGDS telemetry"), "{table_text}");
    assert!(table_text.contains("spgds1"), "{table_text}");
    assert!(table_text.contains("7077517"), "{table_text}");

    let json = run_against(&server, &["radar", "spgds", "--json"]).await;
    assert_eq!(json.status.code(), Some(0), "{}", stderr(&json));
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert!(value["@graph"].is_array());
}
