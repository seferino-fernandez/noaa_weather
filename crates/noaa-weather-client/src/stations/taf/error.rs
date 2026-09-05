use std::{error::Error, fmt};

/// Broad category of a TAF decoding failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TafDecodeErrorKind {
    /// The IWXXM document was not well-formed for the expected structure.
    MalformedXml,
    /// Required forecast meaning was absent.
    MissingRequiredField,
    /// A timestamp was not valid RFC 3339/ISO 8601 data.
    InvalidTimestamp,
    /// A time range ended before it began.
    InvalidPeriod,
    /// A numeric value could not be decoded.
    InvalidNumber,
    /// Aerodrome coordinates were malformed or outside valid bounds.
    InvalidCoordinate,
    /// A measurement used a unit this semantic model cannot safely convert.
    UnsupportedUnit,
    /// Individually valid values formed a contradictory forecast state.
    InvalidCombination,
    /// A value violated its semantic contract.
    InvalidValue,
}

/// Failure to turn an IWXXM document into semantic TAF meaning.
#[derive(Debug)]
pub struct TafDecodeError {
    kind: TafDecodeErrorKind,
    path: Box<str>,
    detail: Box<str>,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl TafDecodeError {
    pub(super) fn xml(source: impl Error + Send + Sync + 'static) -> Self {
        Self {
            kind: TafDecodeErrorKind::MalformedXml,
            path: "TAF".into(),
            detail: "could not decode IWXXM".into(),
            source: Some(Box::new(source)),
        }
    }

    pub(super) fn missing(path: impl Into<Box<str>>) -> Self {
        Self {
            kind: TafDecodeErrorKind::MissingRequiredField,
            path: path.into(),
            detail: "required forecast meaning is missing".into(),
            source: None,
        }
    }

    pub(super) fn invalid(path: impl Into<Box<str>>, detail: impl Into<Box<str>>) -> Self {
        Self {
            kind: TafDecodeErrorKind::InvalidValue,
            path: path.into(),
            detail: detail.into(),
            source: None,
        }
    }

    pub(super) fn classified(
        kind: TafDecodeErrorKind,
        path: impl Into<Box<str>>,
        detail: impl Into<Box<str>>,
    ) -> Self {
        Self {
            kind,
            path: path.into(),
            detail: detail.into(),
            source: None,
        }
    }

    pub(super) fn sourced(
        kind: TafDecodeErrorKind,
        path: impl Into<Box<str>>,
        detail: impl Into<Box<str>>,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            path: path.into(),
            detail: detail.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Category of this failure.
    #[must_use]
    pub const fn kind(&self) -> TafDecodeErrorKind {
        self.kind
    }

    /// Semantic path at which decoding failed.
    #[must_use]
    pub const fn path(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for TafDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at {}", self.detail, self.path)
    }
}

impl Error for TafDecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}
