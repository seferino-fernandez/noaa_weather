use std::{error, fmt};

/// The category of value that failed validation.
///
/// Rendered in human words by `Display`, for example `station id` or
/// `coordinates`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ValueKind {
    /// An observation station identifier such as `KSLC`.
    StationId,
    /// A forecast or county zone identifier such as `COZ040`.
    ZoneId,
    /// A forecast office or product location code such as `TOP`.
    OfficeId,
    /// A Center Weather Service Unit code such as `ZAB`.
    CwsuId,
    /// An Air Traffic Service Unit code such as `KKCI`.
    AtsuId,
    /// A NOAA Weather Radio call sign such as `WXK27`.
    CallSign,
    /// A text product identifier.
    ProductId,
    /// A text product type code such as `AFD`.
    ProductTypeCode,
    /// A radar station identifier such as `KABX`.
    RadarStationId,
    /// An alert identifier such as `urn:oid:2.49.0.1.840.0...`.
    AlertId,
    /// A gridpoint identifier such as `TOP/31,80`.
    GridpointId,
    /// A `latitude,longitude` pair.
    Coordinates,
    /// An ISO 8601 time interval.
    Interval,
}

impl ValueKind {
    /// Returns the human-readable name of this kind.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::StationId => "station id",
            Self::ZoneId => "zone id",
            Self::OfficeId => "office id",
            Self::CwsuId => "CWSU id",
            Self::AtsuId => "ATSU id",
            Self::CallSign => "call sign",
            Self::ProductId => "product id",
            Self::ProductTypeCode => "product type code",
            Self::RadarStationId => "radar station id",
            Self::AlertId => "alert id",
            Self::GridpointId => "gridpoint id",
            Self::Coordinates => "coordinates",
            Self::Interval => "interval",
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// An input that does not have the shape NOAA requires for a value.
///
/// Returned by every `FromStr`, `TryFrom`, and constructor in
/// [`crate::ids`], [`crate::geo`], and [`crate::time`]. The error keeps the
/// offending input and a fixed, human-readable reason so callers can report
/// it without matching on kinds.
///
/// ```
/// use noaa_weather_client::{StationId, ValueKind};
///
/// let error = "kslc!".parse::<StationId>().unwrap_err();
/// assert_eq!(error.kind(), ValueKind::StationId);
/// assert_eq!(error.input(), "kslc!");
/// assert_eq!(
///     error.to_string(),
///     "invalid station id \"kslc!\": must be 3 to 16 ASCII letters or digits"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidValue {
    kind: ValueKind,
    input: Box<str>,
    reason: &'static str,
}

impl InvalidValue {
    /// Creates an error for `input` failing the rule described by `reason`.
    #[must_use]
    pub fn new(kind: ValueKind, input: impl Into<Box<str>>, reason: &'static str) -> Self {
        Self {
            kind,
            input: input.into(),
            reason,
        }
    }

    /// Returns the kind of value that was expected.
    #[must_use]
    pub const fn kind(&self) -> ValueKind {
        self.kind
    }

    /// Returns the rejected input exactly as it was given.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Returns the rule the input broke, phrased as a requirement.
    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

impl fmt::Display for InvalidValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid {} {:?}: {}",
            self.kind, self.input, self.reason
        )
    }
}

impl error::Error for InvalidValue {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_names_kind_input_and_reason() {
        let error = InvalidValue::new(ValueKind::ZoneId, "XXZ040", "must look like COZ040");
        assert_eq!(
            error.to_string(),
            "invalid zone id \"XXZ040\": must look like COZ040"
        );
        assert_eq!(error.kind(), ValueKind::ZoneId);
        assert_eq!(error.input(), "XXZ040");
        assert_eq!(error.reason(), "must look like COZ040");
    }

    #[test]
    fn every_kind_has_a_lowercase_or_acronym_name() {
        for kind in [
            ValueKind::StationId,
            ValueKind::ZoneId,
            ValueKind::OfficeId,
            ValueKind::CwsuId,
            ValueKind::AtsuId,
            ValueKind::CallSign,
            ValueKind::ProductId,
            ValueKind::ProductTypeCode,
            ValueKind::RadarStationId,
            ValueKind::AlertId,
            ValueKind::GridpointId,
            ValueKind::Coordinates,
            ValueKind::Interval,
        ] {
            let name = kind.to_string();
            assert!(!name.is_empty());
            assert!(!name.contains('_'), "{name} should be human words");
        }
    }

    #[test]
    fn is_a_std_error() {
        fn assert_error<E: error::Error + Send + Sync + 'static>() {}
        assert_error::<InvalidValue>();
    }

    #[test]
    fn stays_compact() {
        assert!(std::mem::size_of::<InvalidValue>() <= 40);
    }
}
