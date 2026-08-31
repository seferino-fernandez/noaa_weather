use std::borrow::Cow;
use std::io::{self, IsTerminal as _, Write as _};

use anyhow::{Result, bail};

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
    fn validate(&self, media: MediaKind) -> Result<()> {
        if media == MediaKind::Binary {
            if self.explicit {
                bail!("binary output requires a filesystem path; --output - is not supported");
            }
            bail!("binary output requires --output <PATH>");
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
