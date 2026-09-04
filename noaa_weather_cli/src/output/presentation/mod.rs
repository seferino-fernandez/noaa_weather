//! Default presentation policy and implementations for typed NOAA responses.

use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;

use jiff::tz::TimeZone;
use noaa_weather_client::models::{
    ActiveAlertCounts, Alert, AlertEventTypes, CenterWeatherAdvisory, CwsuOffice, Forecast,
    Gridpoint, Point, Sigmet,
};
use noaa_weather_client::{Feature, FeatureCollection};
use noaa_weather_summary::SummaryOptions;
use serde::Serialize;

use super::PresentationDocument;

mod values;

pub mod glossary;
pub mod offices;
pub mod products;
pub mod radar;
pub mod radio;
pub mod stations;
pub mod zones;

/// Owns the policy used to turn typed NOAA responses into default output.
pub(crate) struct DefaultPresenter {
    time_zone: TimeZone,
    summary: SummaryOptions,
}

impl DefaultPresenter {
    pub(super) fn new(time_zone: TimeZone, summary: SummaryOptions) -> Self {
        Self { time_zone, summary }
    }

    /// The meaning choices the ported families are summarized under.
    ///
    /// Only [`SummaryOptions`] belongs here: the un-ported presenters make
    /// every decision themselves, and appearance is
    /// [`RenderOptions`](crate::output::render::RenderOptions)'s job either
    /// way.
    fn summary_options(&self) -> &SummaryOptions {
        &self.summary
    }

    pub(super) fn present<T>(&self, value: &T) -> Result<PresentationDocument, PresentationError>
    where
        T: DefaultPresentation,
    {
        value.present_default(self)
    }
}

/// Associates a typed NOAA response with its default presentation implementation.
pub(crate) trait DefaultPresentation: Serialize {
    fn present_default(
        &self,
        presenter: &DefaultPresenter,
    ) -> Result<PresentationDocument, PresentationError>;
}

/// Declares that these types present themselves through their [`Summarize`]
/// impls, so the summary crate decides meaning and `output::render` decides
/// appearance.
///
/// A blanket impl is impossible: [`Summarize`] is foreign, [`DefaultPresentation`]
/// is local, and the un-ported families still have their own impls. This list
/// is therefore the answer to "which families are ported", and it grows one
/// line per family.
///
/// [`Summarize`]: noaa_weather_summary::Summarize
macro_rules! summarized {
    ($($response:ty),+ $(,)?) => {
        $(
            impl DefaultPresentation for $response {
                fn present_default(
                    &self,
                    presenter: &DefaultPresenter,
                ) -> Result<PresentationDocument, PresentationError> {
                    Ok(PresentationDocument::Summary(Box::new(
                        noaa_weather_summary::Summarize::summarize(
                            self,
                            presenter.summary_options(),
                        ),
                    )))
                }
            }
        )+
    };
}

summarized!(
    Feature<Alert>,
    FeatureCollection<Alert>,
    ActiveAlertCounts,
    AlertEventTypes,
    Feature<Point>,
    Feature<Gridpoint>,
    Feature<Forecast>,
    CwsuOffice,
    Feature<CenterWeatherAdvisory>,
    FeatureCollection<CenterWeatherAdvisory>,
    Feature<Sigmet>,
    FeatureCollection<Sigmet>,
);

/// A failure to construct a complete default presentation document.
#[derive(Debug)]
pub(crate) enum PresentationError {
    InvalidTimestamp {
        context: Cow<'static, str>,
        value: String,
        source: jiff::Error,
    },
    SourceData {
        source: Box<dyn StdError + Send + Sync>,
    },
}

impl PresentationError {
    pub(super) fn invalid_timestamp(
        context: impl Into<Cow<'static, str>>,
        value: &str,
        source: jiff::Error,
    ) -> Self {
        Self::InvalidTimestamp {
            context: context.into(),
            value: value.to_owned(),
            source,
        }
    }

    pub(super) fn source_data(error: impl StdError + Send + Sync + 'static) -> Self {
        Self::SourceData {
            source: Box::new(error),
        }
    }
}

impl fmt::Display for PresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTimestamp { context, value, .. } => {
                write!(formatter, "invalid timestamp in {context} ({value:?})")
            }
            Self::SourceData { source } => write!(formatter, "invalid source data: {source}"),
        }
    }
}

impl StdError for PresentationError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidTimestamp { source, .. } => Some(source),
            Self::SourceData { source } => Some(source.as_ref()),
        }
    }
}

#[cfg(test)]
mod tests {
    use noaa_weather_client::models::ValueUnit;

    use super::*;

    #[derive(Serialize)]
    struct PolicyExample;

    impl DefaultPresentation for PolicyExample {
        fn present_default(
            &self,
            presenter: &DefaultPresenter,
        ) -> Result<PresentationDocument, PresentationError> {
            let incomplete_pressure = ValueUnit::default();
            let unitless_pressure = ValueUnit {
                value: Some(101_325.0),
                ..ValueUnit::default()
            };
            let invalid_measurement = ValueUnit {
                value: Some(f64::NAN),
                ..ValueUnit::default()
            };
            let values = [
                presenter.timestamp("policy timestamp", Some("2026-08-31T15:59:00Z"))?,
                presenter.text(Some("  ")),
                presenter
                    .resource_identifier(Some("https://api.weather.gov/zones/forecast/AZZ551/")),
                presenter.value_unit(Some(&unitless_pressure)),
                presenter
                    .observation_pressure(Some(&incomplete_pressure), Some(&unitless_pressure)),
                presenter.value_unit(Some(&invalid_measurement)),
                presenter.bytes(Some(-1)),
            ];
            Ok(PresentationDocument::Text(values.join(" | ")))
        }
    }

    #[derive(Serialize)]
    struct InvalidTimestampExample;

    impl DefaultPresentation for InvalidTimestampExample {
        fn present_default(
            &self,
            presenter: &DefaultPresenter,
        ) -> Result<PresentationDocument, PresentationError> {
            Ok(PresentationDocument::Text(
                presenter.timestamp("example.timestamp", Some("not-a-timestamp"))?,
            ))
        }
    }

    #[test]
    fn owns_weather_value_policy() {
        let presenter = DefaultPresenter::new(
            TimeZone::get("America/Phoenix").expect("test time zone must exist"),
            SummaryOptions::default(),
        );

        let document = presenter.present(&PolicyExample).unwrap();
        let PresentationDocument::Text(rendered) = document else {
            panic!("policy example should render text");
        };

        assert_eq!(
            rendered,
            "08/31/26 8:59:00 AM | N/A | AZZ551 | 101325.00 | 101325.00 | Invalid | Invalid (negative)"
        );
    }

    #[test]
    fn malformed_timestamp_is_typed_and_contextual() {
        let presenter = DefaultPresenter::new(TimeZone::UTC, SummaryOptions::default());

        let error = match presenter.present(&InvalidTimestampExample) {
            Ok(_) => panic!("malformed timestamp should fail presentation"),
            Err(error) => error,
        };

        assert!(matches!(error, PresentationError::InvalidTimestamp { .. }));
        assert!(error.to_string().contains("example.timestamp"));
        assert!(error.to_string().contains("not-a-timestamp"));
        assert!(StdError::source(&error).is_some());
    }
}
