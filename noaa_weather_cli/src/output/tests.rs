use std::cell::{Cell, RefCell};
use std::fmt;
use std::io;
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use comfy_table::Table;
use serde::ser::Error as _;
use serde::{Serialize, Serializer};

use super::binary::Sealed;
use super::sink::{DestinationAdapter, MediaKind, SinkTransaction};
use super::{BinaryPresentation, Format, HumanDocument, HumanPresentation, Output, OutputArgs};

#[derive(Serialize)]
struct Example {
    value: &'static str,
}

impl HumanPresentation for Example {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::Text(format!("value: {}\n\n", self.value))
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

impl HumanPresentation for TableExample {
    fn human_presentation(&self) -> HumanDocument {
        let mut table = Table::new();
        table.set_header(["Column"]);
        table.add_row(["Value"]);
        HumanDocument::table(table)
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

impl HumanPresentation for InvalidJson {
    fn human_presentation(&self) -> HumanDocument {
        HumanDocument::Text("unused".to_owned())
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
async fn human_text_has_one_trailing_newline() {
    let (output, bytes) = memory_output(Format::Human);

    output
        .show("showing example", async {
            Ok::<_, FetchError>(Example { value: "forecast" })
        })
        .await
        .unwrap();

    assert_eq!(&*bytes.borrow(), b"value: forecast\n");
}

#[tokio::test]
async fn human_table_is_written_by_lines_with_one_final_newline() {
    let (output, bytes) = memory_output(Format::Human);

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
async fn raw_json_ignores_the_human_default() {
    let (output, bytes) = memory_output(Format::Human);

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
async fn binary_policy_is_validated_before_polling() {
    let polled = Cell::new(false);
    let output = Output::configured(OutputArgs {
        json: false,
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
        json: true,
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
    let (output, bytes) = memory_output(Format::Human);

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
    let (output, bytes) = memory_output(Format::Human);

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
    let (output, _) = memory_output(Format::Human);

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
        Format::Human,
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
        Format::Human,
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
        json: true,
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
        json: false,
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
        json: false,
        output: Some(Path::new("-").to_path_buf()),
    });
    let error = output.destination.validate(MediaKind::Binary).unwrap_err();
    assert!(format!("{error:#}").contains("filesystem path"));
}
