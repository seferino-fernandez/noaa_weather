use std::cell::{Cell, RefCell};
use std::fmt;
use std::io;
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use comfy_table::Table;
use serde::ser::Error as _;
use serde::{Serialize, Serializer};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

use super::binary::Sealed;
use super::presentation::{DefaultPresentation, DefaultPresenter, PresentationError};
use super::render::{ColorMode, TimeZoneChoice};
use super::sink::{DestinationAdapter, MediaKind, SinkTransaction};
use super::{BinaryPresentation, Format, Output, OutputArgs, PresentationDocument};

#[derive(Serialize)]
struct Example {
    value: &'static str,
}

impl DefaultPresentation for Example {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::Text(format!(
            "value: {}\n\n",
            self.value
        )))
    }
}

struct TableExample;

impl Serialize for TableExample {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit()
    }
}

impl DefaultPresentation for TableExample {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        let mut table = Table::new();
        table.set_header(["Column"]);
        table.add_row(["Value"]);
        Ok(PresentationDocument::table(table))
    }
}

struct InvalidJson;

impl Serialize for InvalidJson {
    fn serialize<S>(&self, _serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(S::Error::custom("intentional serialization failure"))
    }
}

impl DefaultPresentation for InvalidJson {
    fn present_default(
        &self,
        _presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError> {
        Ok(PresentationDocument::Text("unused".to_owned()))
    }
}

struct FakeBinary {
    bytes: Vec<u8>,
}

impl Sealed for FakeBinary {}

impl BinaryPresentation for FakeBinary {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn content_type(&self) -> &str {
        "application/pdf"
    }

    fn source_url(&self) -> &str {
        "https://api.weather.gov/example.pdf"
    }
}

#[derive(Debug)]
struct FetchError;

impl fmt::Display for FetchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("root fetch failure")
    }
}

impl std::error::Error for FetchError {}

#[tokio::test]
async fn default_text_has_one_trailing_newline() {
    let (output, bytes) = memory_output(Format::Default);

    output
        .show("showing example", async {
            Ok::<_, FetchError>(Example { value: "forecast" })
        })
        .await
        .unwrap();

    assert_eq!(&*bytes.borrow(), b"value: forecast\n");
}

#[tokio::test]
async fn default_table_is_written_by_lines_with_one_final_newline() {
    let (output, bytes) = memory_output(Format::Default);

    output
        .show("showing table", async { Ok::<_, FetchError>(TableExample) })
        .await
        .unwrap();

    let output = String::from_utf8(bytes.borrow().clone()).unwrap();
    assert!(output.contains("Column"));
    assert!(output.contains("Value"));
    assert!(output.ends_with('\n'));
    assert!(!output.ends_with("\n\n"));
}

#[test]
fn json_configuration_does_not_construct_a_default_presenter() {
    let output = Output::configured(OutputArgs {
        format: Format::Default,
        json: true,
        color: ColorMode::Never,
        width: None,
        time_zone: TimeZoneChoice::Source,
        output: None,
    });

    assert!(output.default_presenter.is_none());
}

#[tokio::test]
async fn json_is_pretty_and_has_one_trailing_newline() {
    let (output, bytes) = memory_output(Format::Json);

    output
        .show("showing JSON", async {
            Ok::<_, FetchError>(Example { value: "forecast" })
        })
        .await
        .unwrap();

    assert_eq!(
        String::from_utf8(bytes.borrow().clone()).unwrap(),
        "{\n  \"value\": \"forecast\"\n}\n"
    );
}

#[tokio::test]
async fn normalized_taf_meaning_flows_through_the_default_output_seam() {
    use noaa_weather_client::{Client, StationId};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stations/KXYZ/tafs/2026-08-30/1200"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("../../../noaa_weather_client/tests/fixtures/taf/semantic_edges.xml"),
            "application/vnd.wmo.iwxxm+xml",
        ))
        .mount(&server)
        .await;
    let client = Client::builder("noaa-weather-tests/1.0")
        .base_url(server.uri())
        .build()
        .unwrap();
    let (output, bytes) = memory_output(Format::Default);

    output
        .show(
            "showing semantic TAF",
            client.stations().taf(
                &"KXYZ".parse::<StationId>().unwrap(),
                "2026-08-30T12:00:00Z".parse().unwrap(),
            ),
        )
        .await
        .unwrap();

    let rendered = String::from_utf8(bytes.borrow().clone()).unwrap();
    for expected in [
        "KXYZ",
        "INITIAL FORECAST",
        "Vertical visibility 300 ft",
        "Maximum 7 °C",
        "minimum -5 °C",
        "Unavailable (not observable)",
        "No significant weather",
        "No significant cloud",
        "Unchanged from prevailing conditions",
        "CHANGE — PROBABILITY 40% — TEMPORARY",
    ] {
        assert!(
            rendered.contains(expected),
            "missing {expected:?} in:\n{rendered}"
        );
    }
    assert!(rendered.ends_with('\n'));
    assert!(!rendered.ends_with("\n\n"));
}

#[tokio::test]
async fn normalized_taf_json_flows_through_the_output_seam() {
    use noaa_weather_client::{Client, StationId};

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/stations/KCXL/tafs/2026-08-30/1500"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("../../../noaa_weather_client/tests/fixtures/taf/cancellation.xml"),
            "application/vnd.wmo.iwxxm+xml",
        ))
        .mount(&server)
        .await;
    let client = Client::builder("noaa-weather-tests/1.0")
        .base_url(server.uri())
        .build()
        .unwrap();
    let (output, bytes) = memory_output(Format::Json);

    output
        .show(
            "showing semantic TAF JSON",
            client.stations().taf(
                &"KCXL".parse::<StationId>().unwrap(),
                "2026-08-30T15:00:00Z".parse().unwrap(),
            ),
        )
        .await
        .unwrap();

    let rendered = String::from_utf8(bytes.borrow().clone()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    assert_eq!(json["aerodrome"]["icaoIdentifier"], "KCXL");
    assert_eq!(json["report"]["kind"], "cancellation");
    assert_eq!(
        json["report"]["cancelledPeriod"]["start"],
        "2026-08-30T12:00:00Z"
    );
    for wire_artifact in ["ns0", "ns1", "xlink", "xmlns", "meteorologicalInformation"] {
        assert!(!rendered.contains(wire_artifact));
    }
    assert!(rendered.ends_with('\n'));
}

#[tokio::test]
async fn raw_json_ignores_the_default_presentation() {
    let (output, bytes) = memory_output(Format::Default);

    output
        .raw_json("showing raw JSON", async {
            Ok::<_, FetchError>(serde_json::json!({"raw": true}))
        })
        .await
        .unwrap();

    assert_eq!(
        String::from_utf8(bytes.borrow().clone()).unwrap(),
        "{\n  \"raw\": true\n}\n"
    );
}

#[tokio::test]
async fn radar_station_default_presentation_uses_normalized_meaning() {
    let station: noaa_weather_client::models::RadarStationFeature = serde_json::from_str(
        include_str!("../../../noaa_weather_client/tests/fixtures/radar/station.json"),
    )
    .unwrap();
    let (output, bytes) = memory_output(Format::Default);

    output
        .show("showing radar station", async {
            Ok::<_, FetchError>(station)
        })
        .await
        .unwrap();

    let rendered = String::from_utf8(bytes.borrow().clone()).unwrap();
    for expected in [
        "KXYZ",
        "Example Radar",
        "Lon: -112.14690, Lat: 33.29030",
        "1.25 s",
        "9.75 s",
        "ldm.example",
        "America/Phoenix",
    ] {
        assert!(
            rendered.contains(expected),
            "missing {expected:?} in:\n{rendered}"
        );
    }
    let maximum_time = "2026-08-31T15:59:00Z"
        .parse::<jiff::Timestamp>()
        .unwrap()
        .to_zoned(jiff::tz::TimeZone::UTC)
        .strftime("%D %r")
        .to_string();
    assert_eq!(rendered.matches(&maximum_time).count(), 1, "{rendered}");
}

#[tokio::test]
async fn radar_server_default_presentation_uses_normalized_meaning() {
    let server: noaa_weather_client::models::RadarServer = serde_json::from_str(include_str!(
        "../../../noaa_weather_client/tests/fixtures/radar/server.json"
    ))
    .unwrap();
    let (output, bytes) = memory_output(Format::Default);

    output
        .show("showing radar server", async {
            Ok::<_, FetchError>(server)
        })
        .await
        .unwrap();

    let rendered = String::from_utf8(bytes.borrow().clone()).unwrap();
    for expected in [
        "Radar Server Status: ldm1",
        "2 / 3 up",
        "0 targets",
        "eno1",
        "100/2/3",
        "eth1 Interface",
        "2.00 KiB",
    ] {
        assert!(
            rendered.contains(expected),
            "missing {expected:?} in:\n{rendered}"
        );
    }
}

#[tokio::test]
async fn malformed_radar_timestamp_fails_only_default_presentation() {
    let malformed = noaa_weather_client::models::RadarServer {
        id: Some("broken".to_owned()),
        collection_time: Some("not-a-timestamp".to_owned()),
        ..noaa_weather_client::models::RadarServer::default()
    };
    let (default_output, default_bytes) = memory_output(Format::Default);

    let error = default_output
        .show("showing malformed radar server", async {
            Ok::<_, FetchError>(malformed.clone())
        })
        .await
        .unwrap_err();
    let chain = format!("{error:#}");
    assert!(chain.contains("showing malformed radar server"), "{chain}");
    assert!(chain.contains("collection_time"), "{chain}");
    assert!(default_bytes.borrow().is_empty());

    let (json_output, json_bytes) = memory_output(Format::Json);
    json_output
        .show("showing malformed radar JSON", async {
            Ok::<_, FetchError>(malformed)
        })
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&json_bytes.borrow()).unwrap();
    assert_eq!(json["collectionTime"], "not-a-timestamp");
}

#[tokio::test]
async fn binary_policy_is_validated_before_polling() {
    let polled = Cell::new(false);
    let output = Output::configured(OutputArgs {
        format: Format::Default,
        json: false,
        color: ColorMode::Never,
        width: None,
        time_zone: TimeZoneChoice::Source,
        output: None,
    });

    let error = output
        .download("downloading example", async {
            polled.set(true);
            Ok::<_, FetchError>(FakeBinary { bytes: vec![1] })
        })
        .await
        .unwrap_err();

    assert!(!polled.get());
    assert!(format!("{error:#}").contains("requires --output <PATH>"));
}

#[tokio::test]
async fn json_rejection_precedes_binary_destination_validation() {
    let polled = Cell::new(false);
    let output = Output::configured(OutputArgs {
        format: Format::Default,
        json: true,
        color: ColorMode::Never,
        width: None,
        time_zone: TimeZoneChoice::Source,
        output: None,
    });

    let error = output
        .download("downloading example", async {
            polled.set(true);
            Ok::<_, FetchError>(FakeBinary { bytes: vec![1] })
        })
        .await
        .unwrap_err();

    assert!(!polled.get());
    assert!(format!("{error:#}").contains("--json cannot be used"));
}

#[tokio::test]
async fn binary_bytes_are_not_text_framed() {
    let (output, bytes) = memory_output(Format::Default);

    output
        .download("downloading example", async {
            Ok::<_, FetchError>(FakeBinary {
                bytes: vec![0, 1, 2, 255],
            })
        })
        .await
        .unwrap();

    assert_eq!(&*bytes.borrow(), &[0, 1, 2, 255]);
}

#[tokio::test]
async fn empty_binary_payload_is_rejected_without_committing() {
    let (output, bytes) = memory_output(Format::Default);

    let error = output
        .download("downloading example", async {
            Ok::<_, FetchError>(FakeBinary { bytes: vec![] })
        })
        .await
        .unwrap_err();

    assert!(format!("{error:#}").contains("empty binary payload"));
    assert!(bytes.borrow().is_empty());
}

#[tokio::test]
async fn operation_context_preserves_the_fetch_source() {
    let (output, _) = memory_output(Format::Default);

    let error = output
        .show("fetching contextual example", async {
            Err::<Example, _>(FetchError)
        })
        .await
        .unwrap_err();
    let chain = format!("{error:#}");

    assert!(chain.contains("fetching contextual example"));
    assert!(chain.contains("root fetch failure"));
    assert!(error.chain().any(|cause| cause.is::<FetchError>()));
}

#[tokio::test]
async fn broken_pipe_is_success_for_stdout_like_destinations() {
    let output = Output::with_destination(
        Format::Default,
        Box::new(FailingDestination {
            kind: io::ErrorKind::BrokenPipe,
            broken_pipe_is_success: true,
        }),
    );

    output
        .show("writing a pipe", async {
            Ok::<_, FetchError>(Example { value: "forecast" })
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn other_write_failures_retain_operation_and_sink_context() {
    let output = Output::with_destination(
        Format::Default,
        Box::new(FailingDestination {
            kind: io::ErrorKind::WriteZero,
            broken_pipe_is_success: false,
        }),
    );

    let error = output
        .show("writing an example", async {
            Ok::<_, FetchError>(Example { value: "forecast" })
        })
        .await
        .unwrap_err();
    let chain = format!("{error:#}");

    assert!(chain.contains("writing an example"));
    assert!(chain.contains("writing output to failing adapter"));
}

#[tokio::test]
async fn serialization_failure_leaves_existing_file_unchanged() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("output.json");
    std::fs::write(&path, "existing\n").unwrap();
    let output = Output::configured(OutputArgs {
        format: Format::Default,
        json: true,
        color: ColorMode::Never,
        width: None,
        time_zone: TimeZoneChoice::Source,
        output: Some(path.clone()),
    });

    output
        .show("serializing example", async {
            Ok::<_, FetchError>(InvalidJson)
        })
        .await
        .unwrap_err();

    assert_eq!(std::fs::read_to_string(path).unwrap(), "existing\n");
}

#[tokio::test]
async fn missing_parent_is_rejected_before_polling() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("missing").join("output.txt");
    let polled = Cell::new(false);
    let output = Output::configured(OutputArgs {
        format: Format::Default,
        json: false,
        color: ColorMode::Never,
        width: None,
        time_zone: TimeZoneChoice::Source,
        output: Some(path),
    });

    output
        .show("showing example", async {
            polled.set(true);
            Ok::<_, FetchError>(Example { value: "forecast" })
        })
        .await
        .unwrap_err();

    assert!(!polled.get());
}

fn memory_output(format: Format) -> (Output, Rc<RefCell<Vec<u8>>>) {
    let bytes = Rc::new(RefCell::new(Vec::new()));
    let destination = MemoryDestination {
        committed: Rc::clone(&bytes),
    };
    (
        Output::with_destination(format, Box::new(destination)),
        bytes,
    )
}

struct MemoryDestination {
    committed: Rc<RefCell<Vec<u8>>>,
}

impl DestinationAdapter for MemoryDestination {
    fn validate(&self, _media: MediaKind) -> Result<()> {
        Ok(())
    }

    fn label(&self) -> std::borrow::Cow<'_, str> {
        "memory".into()
    }

    fn is_terminal(&self) -> bool {
        false
    }

    fn begin(&self) -> Result<Box<dyn SinkTransaction>> {
        Ok(Box::new(MemoryTransaction {
            pending: Vec::new(),
            committed: Rc::clone(&self.committed),
        }))
    }
}

struct MemoryTransaction {
    pending: Vec<u8>,
    committed: Rc<RefCell<Vec<u8>>>,
}

impl io::Write for MemoryTransaction {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.pending.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl SinkTransaction for MemoryTransaction {
    fn commit(self: Box<Self>) -> Result<()> {
        *self.committed.borrow_mut() = self.pending;
        Ok(())
    }
}

struct FailingDestination {
    kind: io::ErrorKind,
    broken_pipe_is_success: bool,
}

impl DestinationAdapter for FailingDestination {
    fn validate(&self, _media: MediaKind) -> Result<()> {
        Ok(())
    }

    fn label(&self) -> std::borrow::Cow<'_, str> {
        "failing adapter".into()
    }

    fn is_terminal(&self) -> bool {
        false
    }

    fn begin(&self) -> Result<Box<dyn SinkTransaction>> {
        Ok(Box::new(FailingTransaction {
            kind: self.kind,
            broken_pipe_is_success: self.broken_pipe_is_success,
        }))
    }
}

struct FailingTransaction {
    kind: io::ErrorKind,
    broken_pipe_is_success: bool,
}

impl io::Write for FailingTransaction {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(self.kind))
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(io::Error::from(self.kind))
    }
}

impl SinkTransaction for FailingTransaction {
    fn broken_pipe_is_success(&self) -> bool {
        self.broken_pipe_is_success
    }

    fn commit(self: Box<Self>) -> Result<()> {
        Err(io::Error::from(self.kind).into())
    }
}

#[test]
fn dash_selects_explicit_stdout_but_binary_still_requires_a_file() {
    let output = Output::configured(OutputArgs {
        format: Format::Default,
        json: false,
        color: ColorMode::Never,
        width: None,
        time_zone: TimeZoneChoice::Source,
        output: Some(Path::new("-").to_path_buf()),
    });
    let error = output.destination.validate(MediaKind::Binary).unwrap_err();
    assert!(format!("{error:#}").contains("filesystem path"));
}
