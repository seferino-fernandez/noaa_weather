use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;
use std::future::Future;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result, anyhow, bail};
use clap::{Args, ValueEnum};
use comfy_table::Table;
use noaa_weather_client::apis::BinaryPayload;
use noaa_weather_summary::{SummaryOptions, UnitSystem};
use serde::Serialize;
use serde_json::Value;

mod atomic_file;
mod presentation;
mod render;
mod sink;

use render::{ColorMode, RenderOptions, TimeZoneChoice};
use sink::{DestinationAdapter, MediaKind, StdoutDestination};

use presentation::{DefaultPresentation, DefaultPresenter};

pub(crate) use noaa_weather_summary::stations::ZoneObservations;

/// Global command-line arguments that select successful-output behavior.
#[derive(Args, Debug)]
pub(crate) struct OutputArgs {
    /// Render successful structured responses in this format.
    #[arg(
        short,
        long,
        global = true,
        value_name = "FORMAT",
        default_value = "table"
    )]
    format: Format,

    /// Output successful structured responses as pretty JSON; an alias for
    /// `--format json`.
    #[arg(long, global = true, conflicts_with = "format")]
    json: bool,

    /// When to write color and bold escapes.
    #[arg(long, global = true, value_name = "WHEN", default_value = "auto")]
    color: ColorMode,

    /// Wrap output to N columns; 0 means never wrap.
    #[arg(long, global = true, value_name = "N")]
    width: Option<u16>,

    /// Show measurements in this unit system.
    #[arg(long, global = true, value_name = "SYSTEM", default_value = "us")]
    units: Units,

    /// Show timestamps in this zone: `auto`, `source`, or an IANA zone name.
    #[arg(
        long,
        global = true,
        value_name = "ZONE",
        default_value = "auto",
        value_parser = render::parse_time_zone
    )]
    time_zone: TimeZoneChoice,

    /// Write output to PATH; `-` means stdout for structured output.
    #[arg(short, long, global = true, value_name = "PATH")]
    output: Option<PathBuf>,
}

/// A failure to deliver output that the filesystem, or the terminal, caused.
///
/// This and [`UsageFailure`] both reach `main` as one `anyhow::Error` and exit
/// with different codes, so each is tagged where it happens rather than
/// guessed at from its message later.
///
/// What earns this tag is a destination that could not take the bytes *on
/// this machine, right now*: a parent directory that does not exist, a file
/// that cannot be opened for writing, a rename that failed. Anything that
/// would fail identically on every machine is a [`UsageFailure`] instead.
/// Presenting a value earns neither, because a presenter that cannot describe
/// a response is a bug in this program.
///
/// The wrapped `anyhow::Error` is not a `std::error::Error`, so it cannot be
/// returned from [`std::error::Error::source`]; [`fmt::Display`] writes the
/// whole chain instead, which is what `{:#}` on the outer error would have
/// printed anyway.
#[derive(Debug)]
pub struct OutputFailure(anyhow::Error);

impl OutputFailure {
    /// Tags `error` as an output failure, leaving an already-classified one
    /// alone.
    ///
    /// The [`UsageFailure`] arm matters: `show` and `download` wrap whatever
    /// `validate` returns, and a stdout destination refusing binary bytes has
    /// already classified itself by then.
    pub(crate) fn wrap(error: anyhow::Error) -> anyhow::Error {
        if error.is::<Self>() || error.is::<UsageFailure>() {
            error
        } else {
            anyhow::Error::new(Self(error))
        }
    }
}

impl fmt::Display for OutputFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#}", self.0)
    }
}

impl StdError for OutputFailure {}

/// An output request that argv alone makes impossible.
///
/// `--json` on a command whose response is a PDF, and a binary download aimed
/// at a terminal, depend on nothing but the arguments: no request is made, no
/// file is touched, and they would fail the same way on every machine. That
/// is a value the caller typed, so it exits 2 with the rest of the usage
/// errors rather than telling a script its disk is bad.
///
/// clap cannot express either — `--format` is global and the conflict is with
/// the subcommand — but "clap did not catch it" was never the test. The
/// library's own [`noaa_weather_client::Error::Invalid`] is exit 2 on the
/// same reasoning.
#[derive(Debug)]
pub struct UsageFailure(anyhow::Error);

impl UsageFailure {
    /// Tags `error` as a usage failure.
    ///
    /// Named to match [`OutputFailure::wrap`]; both take an error and hand
    /// back a classified one rather than constructing `Self`.
    pub(crate) fn wrap(error: anyhow::Error) -> anyhow::Error {
        anyhow::Error::new(Self(error))
    }
}

impl fmt::Display for UsageFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#}", self.0)
    }
}

impl StdError for UsageFailure {}

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
    /// A ported family: meaning only, rendered by [`render`].
    Summary(Box<noaa_weather_summary::Summary>),
    /// An un-ported family that still draws its own table.
    Table(Box<Table>),
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

/// Which unit system measurements are converted to before they are shown.
///
/// The command-line spelling of [`UnitSystem`]. NOAA's own `units` request
/// parameter is a different question and an inert one — the feature flags
/// this crate always sends make every response metric — so this flag is
/// presentation policy, resolved here alongside `--color` and `--width`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
enum Units {
    /// Fahrenheit, miles per hour, feet, miles, inches, inches of mercury.
    #[default]
    Us,
    /// Celsius, kilometres per hour, metres, kilometres, millimetres, hectopascals.
    Si,
}

impl From<Units> for UnitSystem {
    fn from(units: Units) -> Self {
        match units {
            Units::Us => Self::Us,
            Units::Si => Self::Si,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
enum Format {
    /// Box-drawing tables, wrapped and colored for a terminal.
    #[default]
    #[value(name = "table")]
    Default,
    /// Pretty JSON.
    Json,
}

/// Executes NOAA operations and owns all successful-output policy.
pub(crate) struct Output {
    format: Format,
    destination: Box<dyn DestinationAdapter>,
    default_presenter: Option<DefaultPresenter>,
    render: RenderOptions,
}

impl Output {
    pub(crate) fn configured(args: OutputArgs) -> Self {
        let OutputArgs {
            format,
            json,
            color,
            width,
            units,
            time_zone,
            output,
        } = args;
        // `--json` is the documented alias, and clap already refused the
        // combination of both flags.
        let format = if json { Format::Json } else { format };
        let destination: Box<dyn DestinationAdapter> = match output {
            None => Box::new(StdoutDestination::implicit()),
            Some(path) if path == Path::new("-") => Box::new(StdoutDestination::explicit()),
            Some(path) => Box::new(atomic_file::AtomicFileDestination::new(path)),
        };
        let render = RenderOptions::new(color, width, &time_zone, destination.is_terminal());
        let summary = SummaryOptions {
            units: units.into(),
        };
        let default_presenter = (format == Format::Default)
            .then(|| DefaultPresenter::new(render.presenter_time_zone(), summary));

        Self {
            format,
            destination,
            default_presenter,
            render,
        }
    }

    /// Whether the caller asked for machine-readable output.
    ///
    /// Failures follow the same choice as successes: a caller parsing JSON
    /// out of this program gets its errors as JSON too.
    pub(crate) fn is_machine_readable(&self) -> bool {
        self.format == Format::Json
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
            self.destination
                .validate(MediaKind::Structured)
                .map_err(OutputFailure::wrap)?;
            let value = request.await.map_err(anyhow::Error::new)?;
            match self.format {
                Format::Default => {
                    let presenter = self.default_presenter.as_ref().context(
                        "default presentation policy was not configured for default output",
                    )?;
                    let document = presenter.present(&value)?;
                    self.write_presentation(document)
                        .map_err(OutputFailure::wrap)
                }
                Format::Json => self.write_json(&value).map_err(OutputFailure::wrap),
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
            self.destination
                .validate(MediaKind::Structured)
                .map_err(OutputFailure::wrap)?;
            let value = request.await.map_err(anyhow::Error::new)?;
            self.write_json(&value).map_err(OutputFailure::wrap)
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
            // Argv alone decides this one, so it is a usage error: no
            // request is made and no file is touched, and it would fail
            // identically on any machine. `StdoutDestination::validate`
            // classifies the other half the same way.
            if self.format == Format::Json {
                return Err(UsageFailure::wrap(anyhow!(
                    "--json cannot be used with binary output"
                )));
            }
            self.destination
                .validate(MediaKind::Binary)
                .map_err(OutputFailure::wrap)?;

            let payload = request.await.map_err(anyhow::Error::new)?;
            // An empty body is NOAA's answer rather than a problem with the
            // destination, so this one is not tagged and exits 1.
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
            .map_err(OutputFailure::wrap)
        }
        .await
        .with_context(|| operation.to_string())
    }

    fn write_presentation(&self, document: PresentationDocument) -> Result<()> {
        match document {
            PresentationDocument::Summary(summary) => {
                let text = self.render.render(&summary);
                self.write_document(move |writer| write_text(writer, &text))
            }
            PresentationDocument::Table(mut table) => {
                self.render.apply(&mut table);
                self.write_document(move |writer| write_table(writer, &table))
            }
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
        let default_presenter = (format == Format::Default)
            .then(|| DefaultPresenter::new(jiff::tz::TimeZone::UTC, SummaryOptions::default()));
        Self {
            format,
            destination,
            default_presenter,
            render: RenderOptions::new(
                ColorMode::Never,
                Some(render::FALLBACK_WIDTH),
                &TimeZoneChoice::Named(jiff::tz::TimeZone::UTC),
                false,
            ),
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
