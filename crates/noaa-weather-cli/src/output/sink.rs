use std::borrow::Cow;
use std::io::{self, IsTerminal as _, Write as _};

use anyhow::{Result, anyhow};

use super::UsageFailure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum MediaKind {
    Structured,
    Binary,
}

pub(super) trait DestinationAdapter {
    fn validate(&self, media: MediaKind) -> Result<()>;
    fn label(&self) -> Cow<'_, str>;
    fn is_terminal(&self) -> bool;
    fn begin(&self) -> Result<Box<dyn SinkTransaction>>;
}

pub(super) trait SinkTransaction: io::Write {
    fn broken_pipe_is_success(&self) -> bool {
        false
    }

    fn commit(self: Box<Self>) -> Result<()>;
}

pub(super) struct StdoutDestination {
    explicit: bool,
}

impl StdoutDestination {
    pub(super) const fn implicit() -> Self {
        Self { explicit: false }
    }

    pub(super) const fn explicit() -> Self {
        Self { explicit: true }
    }
}

impl DestinationAdapter for StdoutDestination {
    /// Refuses binary bytes, whether or not standard output is a terminal.
    ///
    /// Nothing here consults [`Self::is_terminal`]: piping a PDF is refused
    /// as firmly as printing one, so the refusal depends on argv alone and
    /// is a [`UsageFailure`] rather than an output failure. A caller who
    /// sees exit 5 should look at their disk; these two want `--output
    /// <PATH>` instead.
    fn validate(&self, media: MediaKind) -> Result<()> {
        if media == MediaKind::Binary {
            if self.explicit {
                return Err(UsageFailure::wrap(anyhow!(
                    "binary output requires a filesystem path; --output - is not supported"
                )));
            }
            return Err(UsageFailure::wrap(anyhow!(
                "binary output requires --output <PATH>"
            )));
        }
        Ok(())
    }

    fn label(&self) -> Cow<'_, str> {
        Cow::Borrowed("stdout")
    }

    fn is_terminal(&self) -> bool {
        io::stdout().is_terminal()
    }

    fn begin(&self) -> Result<Box<dyn SinkTransaction>> {
        Ok(Box::new(StdoutTransaction {
            writer: io::BufWriter::new(io::stdout()),
        }))
    }
}

struct StdoutTransaction {
    writer: io::BufWriter<io::Stdout>,
}

impl io::Write for StdoutTransaction {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.writer.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl SinkTransaction for StdoutTransaction {
    fn broken_pipe_is_success(&self) -> bool {
        true
    }

    fn commit(mut self: Box<Self>) -> Result<()> {
        self.flush().map_err(Into::into)
    }
}
