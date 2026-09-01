use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, bail};
use clap::Args;
use comfy_table::Table;
use jiff::tz::TimeZone;
use noaa_weather_client::apis::BinaryPayload;
use serde::Serialize;
use serde_json::Value;

mod atomic_file;
mod presentation;
mod sink;

use sink::{DestinationAdapter, MediaKind, StdoutDestination};

use presentation::{DefaultPresentation, DefaultPresenter};

pub(crate) use presentation::zones::ZoneObservations;

/// Global command-line arguments that select successful-output behavior.
#[derive(Args, Debug)]
pub(crate) struct OutputArgs {
    /// Output successful structured responses as pretty JSON.
    #[arg(long, global = true)]
    json: bool,

    /// Write output to PATH; `-` means stdout for structured output.
    #[arg(short, long, global = true, value_name = "PATH")]
    output: Option<PathBuf>,
}

/// A user-facing description of the NOAA operation being performed.
#[derive(Clone, Debug)]
pub(crate) struct Operation(Cow<'static, str>);

impl fmt::Display for Operation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<&'static str> for Operation {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for Operation {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

/// A successful default presentation before destination-specific rendering.
pub(crate) enum PresentationDocument {
    Table(Box<Table>),
    #[cfg(any(feature = "radio", test))]
    Text(String),
}

impl PresentationDocument {
    fn table(table: Table) -> Self {
        Self::Table(Box::new(table))
    }
}

mod binary {
    pub trait Sealed {}
}

/// The binary response facts needed by output policy.
pub(crate) trait BinaryPresentation: binary::Sealed {
    fn bytes(&self) -> &[u8];
    fn content_type(&self) -> &str;
    fn source_url(&self) -> &str;
}

impl binary::Sealed for BinaryPayload {}

impl BinaryPresentation for BinaryPayload {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn content_type(&self) -> &str {
        self.content_type().essence_str()
    }

    fn source_url(&self) -> &str {
        self.final_url().as_str()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Format {
    Default,
    Json,
}

/// Executes NOAA operations and owns all successful-output policy.
pub(crate) struct Output {
    format: Format,
    destination: Box<dyn DestinationAdapter>,
    default_presenter: Option<DefaultPresenter>,
}

impl Output {
    pub(crate) fn configured(args: OutputArgs) -> Self {
        let format = if args.json {
            Format::Json
        } else {
            Format::Default
        };
        let destination: Box<dyn DestinationAdapter> = match args.output {
            None => Box::new(StdoutDestination::implicit()),
            Some(path) if path == Path::new("-") => Box::new(StdoutDestination::explicit()),
            Some(path) => Box::new(atomic_file::AtomicFileDestination::new(path)),
        };
        let default_presenter = (format == Format::Default)
            .then(|| DefaultPresenter::new(TimeZone::try_system().unwrap_or(TimeZone::UTC)));

        Self {
            format,
            destination,
            default_presenter,
        }
    }

    /// Runs a typed NOAA operation and selects its default or JSON presentation.
    pub(crate) async fn show<T, E>(
        &self,
        operation: impl Into<Operation>,
        request: impl Future<Output = std::result::Result<T, E>>,
    ) -> Result<()>
    where
        T: DefaultPresentation,
        E: StdError + Send + Sync + 'static,
    {
        let operation = operation.into();
        async {
            self.destination.validate(MediaKind::Structured)?;
            let value = request.await.map_err(anyhow::Error::new)?;
            match self.format {
                Format::Default => {
                    let presenter = self.default_presenter.as_ref().context(
                        "default presentation policy was not configured for default output",
                    )?;
                    self.write_presentation(presenter.present(&value)?)
                }
                Format::Json => self.write_json(&value),
            }
        }
        .await
        .with_context(|| operation.to_string())
    }

    /// Runs an untyped NOAA operation whose only stable presentation is JSON.
    pub(crate) async fn raw_json<E>(
        &self,
        operation: impl Into<Operation>,
        request: impl Future<Output = std::result::Result<Value, E>>,
    ) -> Result<()>
    where
        E: StdError + Send + Sync + 'static,
    {
        let operation = operation.into();
        async {
            self.destination.validate(MediaKind::Structured)?;
            let value = request.await.map_err(anyhow::Error::new)?;
            self.write_json(&value)
        }
        .await
        .with_context(|| operation.to_string())
    }

    /// Runs a NOAA binary operation after validating its file-only policy.
    pub(crate) async fn download<T, E>(
        &self,
        operation: impl Into<Operation>,
        request: impl Future<Output = std::result::Result<T, E>>,
    ) -> Result<()>
    where
        T: BinaryPresentation,
        E: StdError + Send + Sync + 'static,
    {
        let operation = operation.into();
        async {
            if self.format == Format::Json {
                bail!("--json cannot be used with binary output");
            }
            self.destination.validate(MediaKind::Binary)?;

            let payload = request.await.map_err(anyhow::Error::new)?;
            if payload.bytes().is_empty() {
                bail!(
                    "received empty binary payload with content type {} from {}",
                    payload.content_type(),
                    payload.source_url()
                );
            }

            let context = format!(
                "writing {} binary response from {} to {}",
                payload.content_type(),
                payload.source_url(),
                self.destination.label()
            );
            self.write_document(|writer| {
                writer
                    .write_all(payload.bytes())
                    .with_context(|| context.clone())
            })
        }
        .await
        .with_context(|| operation.to_string())
    }

    fn write_presentation(&self, document: PresentationDocument) -> Result<()> {
        match document {
            PresentationDocument::Table(mut table) => {
                if !self.destination.is_terminal() {
                    table.force_no_tty();
                }
                self.write_document(move |writer| write_table(writer, &table))
            }
            #[cfg(any(feature = "radio", test))]
            PresentationDocument::Text(text) => {
                self.write_document(move |writer| write_text(writer, &text))
            }
        }
    }

    fn write_json(&self, value: &impl Serialize) -> Result<()> {
        self.write_document(|writer| {
            serde_json::to_writer_pretty(&mut *writer, value)
                .context("serializing pretty JSON output")?;
            writer.write_all(b"\n").context("terminating JSON output")
        })
    }

    fn write_document(&self, write: impl FnOnce(&mut dyn io::Write) -> Result<()>) -> Result<()> {
        let label = self.destination.label();
        let mut transaction = self
            .destination
            .begin()
            .with_context(|| format!("opening output destination {label}"))?;
        let broken_pipe_is_success = transaction.broken_pipe_is_success();

        if let Err(error) = write(&mut *transaction) {
            if broken_pipe_is_success && is_broken_pipe(&error) {
                return Ok(());
            }
            return Err(error).with_context(|| format!("writing output to {label}"));
        }

        match transaction.commit() {
            Ok(()) => Ok(()),
            Err(error) if broken_pipe_is_success && is_broken_pipe(&error) => Ok(()),
            Err(error) => Err(error).with_context(|| format!("committing output to {label}")),
        }
    }

    #[cfg(test)]
    fn with_destination(format: Format, destination: Box<dyn DestinationAdapter>) -> Self {
        let default_presenter =
            (format == Format::Default).then(|| DefaultPresenter::new(TimeZone::UTC));
        Self {
            format,
            destination,
            default_presenter,
        }
    }
}

fn write_table(writer: &mut dyn io::Write, table: &Table) -> Result<()> {
    for line in table.lines() {
        writer
            .write_all(line.as_bytes())
            .context("writing table row")?;
        writer.write_all(b"\n").context("terminating table row")?;
    }
    Ok(())
}

#[cfg(any(feature = "radio", test))]
fn write_text(writer: &mut dyn io::Write, text: &str) -> Result<()> {
    let content = text.trim_end_matches(['\r', '\n']);
    writer
        .write_all(content.as_bytes())
        .context("writing text output")?;
    writer.write_all(b"\n").context("terminating text output")
}

fn is_broken_pipe(error: &anyhow::Error) -> bool {
    error.chain().any(|cause| {
        cause
            .downcast_ref::<io::Error>()
            .is_some_and(|error| error.kind() == io::ErrorKind::BrokenPipe)
            || cause
                .downcast_ref::<serde_json::Error>()
                .and_then(serde_json::Error::io_error_kind)
                == Some(io::ErrorKind::BrokenPipe)
    })
}

#[cfg(test)]
mod tests;
