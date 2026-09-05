//! Default presentation policy and implementations for typed NOAA responses.

use std::error::Error as StdError;
use std::fmt;

use noaa_weather_client::models::{
    ActiveAlertCounts, Alert, AlertEventTypes, CenterWeatherAdvisory, CwsuOffice, Forecast,
    GlossaryResponse, Gridpoint, Observation, ObservationStation, Office, OfficeBriefingResponse,
    OfficeHeadline, OfficeHeadlineCollection, OfficeWeatherStoryCollection, Point,
    RadarQueuesResponse, RadarServerTelemetry, RadarServersResponse, RadarSpgdsResponse,
    RadarStationAlarmsResponse, RadarStationTelemetry, RadarStationsResponse, RadioBroadcast,
    RadioTransmitter, RadioTransmitterCollection, Sigmet, TerminalAerodromeForecast,
    TerminalAerodromeForecastsResponse, TextProduct, TextProductCollection,
    TextProductLocationCollection, TextProductTypeCollection, Zone, ZoneForecast,
};
use noaa_weather_client::{Feature, FeatureCollection};
use noaa_weather_summary::SummaryOptions;
use serde::Serialize;

use super::PresentationDocument;

/// Owns the policy used to turn typed NOAA responses into default output.
pub(crate) struct DefaultPresenter {
    summary: SummaryOptions,
}

impl DefaultPresenter {
    pub(super) fn new(summary: SummaryOptions) -> Self {
        Self { summary }
    }

    /// The meaning choices every response family is summarized under.
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
/// A blanket impl is impossible because [`Summarize`] is foreign while
/// [`DefaultPresentation`] is local. This list is therefore the explicit
/// bridge between the two crates.
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
    GlossaryResponse,
    TextProduct,
    TextProductCollection,
    TextProductLocationCollection,
    TextProductTypeCollection,
    RadioBroadcast,
    RadioTransmitter,
    RadioTransmitterCollection,
    Office,
    OfficeHeadlineCollection,
    OfficeHeadline,
    OfficeBriefingResponse,
    OfficeWeatherStoryCollection,
    RadarQueuesResponse,
    RadarServerTelemetry,
    RadarServersResponse,
    RadarSpgdsResponse,
    RadarStationAlarmsResponse,
    RadarStationTelemetry,
    RadarStationsResponse,
    Feature<Point>,
    Feature<Gridpoint>,
    Feature<Forecast>,
    CwsuOffice,
    Feature<CenterWeatherAdvisory>,
    FeatureCollection<CenterWeatherAdvisory>,
    Feature<Sigmet>,
    FeatureCollection<Sigmet>,
    Feature<ObservationStation>,
    FeatureCollection<ObservationStation>,
    Feature<Observation>,
    FeatureCollection<Observation>,
    TerminalAerodromeForecastsResponse,
    TerminalAerodromeForecast,
    noaa_weather_summary::stations::ZoneObservations,
    Feature<Zone>,
    FeatureCollection<Zone>,
    Feature<ZoneForecast>,
);

/// The default presentation is infallible now that every family summarizes
/// typed models rather than parsing source values in the CLI.
#[derive(Debug)]
pub(crate) enum PresentationError {}

impl fmt::Display for PresentationError {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl StdError for PresentationError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {}
    }
}
