use noaa_weather_client::models::TerminalAerodromeForecast;
use noaa_weather_client::models::terminal_aerodrome_forecast::{
    ForecastGroupKind, ForecastReport, MissingForecastReason, MissingReason,
};

fn decode(bytes: &[u8]) -> TerminalAerodromeForecast {
    TerminalAerodromeForecast::from_iwxxm(bytes).expect("fixture should decode")
}

fn assert_ordinary_forecast(forecast: &TerminalAerodromeForecast, expected_icao: &str) {
    assert_eq!(forecast.aerodrome().icao_identifier(), expected_icao);
    match forecast.report() {
        ForecastReport::Forecast { .. } => {}
        _ => panic!("expected ordinary forecast"),
    }
    assert_eq!(forecast.groups().len(), 6);
}

#[test]
fn kflg_normal_decodes_as_an_ordered_ordinary_forecast() {
    let forecast = decode(include_bytes!("fixtures/taf/kflg_normal.xml"));

    assert_ordinary_forecast(&forecast, "KFLG");
    assert_eq!(
        forecast
            .groups()
            .iter()
            .map(|group| group.kind())
            .collect::<Vec<_>>(),
        [
            &ForecastGroupKind::Base,
            &ForecastGroupKind::Temporary,
            &ForecastGroupKind::From,
            &ForecastGroupKind::Probability {
                percent: 30,
                temporary: false,
            },
            &ForecastGroupKind::From,
            &ForecastGroupKind::Temporary,
        ],
    );
}

#[test]
fn semantic_edges_preserves_unavailable_forecast_meaning() {
    let forecast = decode(include_bytes!("fixtures/taf/semantic_edges.xml"));

    assert_ordinary_forecast(&forecast, "KXYZ");
    assert_eq!(
        forecast.change_forecasts()[1]
            .conditions()
            .wind()
            .unavailable_reason(),
        Some(&MissingReason::NotObservable),
    );
}

#[test]
fn cancellation_decodes_as_a_cancellation_report() {
    let forecast = decode(include_bytes!("fixtures/taf/cancellation.xml"));

    assert_eq!(forecast.aerodrome().icao_identifier(), "KCXL");
    match forecast.report() {
        ForecastReport::Cancellation { .. } => {}
        _ => panic!("expected cancellation report"),
    }
}

#[test]
fn translation_failure_preserves_the_failed_tac() {
    let forecast = decode(include_bytes!("fixtures/taf/translation_failed.xml"));

    assert_eq!(forecast.aerodrome().icao_identifier(), "KERR");
    match forecast.report() {
        ForecastReport::Missing {
            reason: MissingForecastReason::TranslationFailed { tac },
        } => assert_eq!(tac.as_ref(), "TAF KERR malformed source"),
        _ => panic!("expected translation failure"),
    }
}
