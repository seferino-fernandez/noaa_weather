//! Summaries for the `/points` family.
//!
//! A point answers "where am I, in NOAA's terms". The one thing a person
//! recognizes in that answer is the nearest town, so it is the title —
//! `4.2 mi N of Linn, KS` — and everything a *next command* needs is a fact:
//! the grid cell, the office, the zones, the radar station.
//!
//! Nothing here is alarming, so nothing carries emphasis.

use noaa_weather_client::Feature;
use noaa_weather_client::geo::Geometry;
use noaa_weather_client::models::{Point, RelativeLocation};

use crate::render::{RangeStyle, RenderOptions, format_value};
use crate::{Fact, QuantityKind, Section, Summarize, Summary, SummaryOptions, Value};

/// The 16-point compass, in bearing order from due north.
const COMPASS: [&str; 16] = [
    "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW",
    "NNW",
];

/// The compass point a bearing in degrees falls in.
///
/// [`RelativeLocation::bearing`] is documented as the bearing *from the place
/// to the point*, so the label reads off it directly with no reversal. The
/// captured fixture agrees: it bears 358 from Linn at 39.6793 N and sits at
/// 39.7456 N, which is north of the town, and the title reads `N of Linn`.
/// Reversing the bearing would put it south.
fn compass_point(degrees: f64) -> Option<&'static str> {
    if !degrees.is_finite() {
        return None;
    }
    // Each of the sixteen points owns 22.5 degrees centered on its bearing,
    // so rounding rather than truncating is what picks the nearest one.
    let sector = (degrees / 22.5).round().rem_euclid(16.0);
    COMPASS.get(sector as usize).copied()
}

/// The coordinates of a geometry, when the geometry is a single point.
///
/// A point's own geometry should be one, and is in every fixture. Anything
/// else has no single pair of numbers to show, and a subtitle is the wrong
/// place to start listing polygon vertices, so it yields no subtitle rather
/// than a guess.
fn coordinates(geometry: &Geometry) -> Option<Value> {
    match geometry {
        Geometry::Point(position) => Some(Value::coordinates(position.lat(), position.lon())),
        _ => None,
    }
}

/// The relative location as a sentence: `4.2 mi N of Linn, KS`.
///
/// The distance honors the caller's unit system through the same path every
/// other measurement takes. A distance or bearing NOAA left out drops its
/// clause rather than the whole sentence, so the town is never lost.
fn relative_location(location: &RelativeLocation, options: &SummaryOptions) -> String {
    let place = format!("{}, {}", location.city.trim(), location.state.trim());
    let distance = match Value::quantity(&location.distance, QuantityKind::Distance, options) {
        value @ Value::Quantity { .. } => Some(format_value(
            &value,
            &RenderOptions::default(),
            RangeStyle::Words,
        )),
        _ => None,
    };
    let bearing = location.bearing.value.and_then(compass_point);
    match (distance, bearing) {
        (Some(distance), Some(bearing)) => format!("{distance} {bearing} of {place}"),
        (Some(distance), None) => format!("{distance} from {place}"),
        (None, Some(bearing)) => format!("{bearing} of {place}"),
        (None, None) => place,
    }
}

impl Summarize for Feature<Point> {
    fn summarize(&self, options: &SummaryOptions) -> Summary {
        let point = &self.properties;
        let mut summary = Summary::new(relative_location(&point.relative_location, options));
        if let Some(here) = self.geometry.as_ref().and_then(coordinates) {
            summary = summary.subtitle(format_value(
                &here,
                &RenderOptions::default(),
                RangeStyle::Words,
            ));
        }

        summary.push(Section::Facts {
            heading: None,
            facts: vec![
                Fact::new(
                    "Grid cell",
                    Some("gridId"),
                    Value::identifier(format!(
                        "{}/{},{}",
                        point.grid_id, point.grid_x, point.grid_y
                    )),
                )
                .also(&["gridX", "gridY"]),
                // Not the same question as the grid cell: `cwa` is the office
                // responsible for the point, `gridId` the office whose grid
                // it falls in. Anchorage answers `AFC` and `AER`.
                Fact::new(
                    "Responsible office",
                    Some("cwa"),
                    Value::identifier(point.cwa.to_string()),
                ),
                Fact::new(
                    "Time zone",
                    Some("timeZone"),
                    Value::text(point.time_zone.iana_name()),
                ),
                Fact::new(
                    "Radar station",
                    Some("radarStation"),
                    Value::identifier(point.radar_station.trim()),
                ),
                Fact::new(
                    "Forecast zone",
                    Some("forecastZone"),
                    Value::identifier_from_url(&point.forecast_zone),
                ),
                Fact::new(
                    "County zone",
                    Some("county"),
                    Value::identifier_from_url(&point.county),
                ),
                Fact::new(
                    "Fire weather zone",
                    Some("fireWeatherZone"),
                    Value::identifier_from_url(&point.fire_weather_zone),
                ),
                // The GeoJSON envelope also has a `type`, always "Feature".
                // Both keys are spelled the same, so this one fact accounts
                // for both; the envelope's value is never worth printing.
                Fact::new(
                    "Land or sea",
                    Some("type"),
                    Value::text(Some(&point.point_type.to_string())),
                ),
            ],
        })
    }

    const OMITTED: &'static [(&'static str, &'static str)] = &[
        // Envelope.
        (
            "id",
            "the point's own URL; the coordinates address it again",
        ),
        ("geometry", "shown as the subtitle"),
        (
            "properties",
            "the point itself; its keys are accounted for one by one",
        ),
        // Shown outside the facts.
        ("relativeLocation", "shown as the title"),
        // Provenance.
        ("@id", "the point's own URL, as the envelope id already is"),
        ("@type", "always wx:Point"),
        // The five URLs. Each names an endpoint the CLI has its own command
        // for, addressed by the grid cell this summary already shows.
        (
            "forecast",
            "the URL of `gridpoints forecast` for this grid cell",
        ),
        (
            "forecastHourly",
            "the URL of `gridpoints forecast-hourly` for this grid cell",
        ),
        (
            "forecastGridData",
            "the URL of `gridpoints gridpoint` for this grid cell",
        ),
        (
            "observationStations",
            "the URL of `gridpoints stations` for this grid cell",
        ),
        (
            "forecastOffice",
            "the URL of the office `cwa` names; `offices office` fetches it",
        ),
        (
            "nwr",
            "NOAA Weather Radio transmitter and SAME code, for a receiver rather than a reader",
        ),
        (
            "astronomicalData",
            "sunrise, sunset and the twilights; deferred to a later slice, not judged unimportant",
        ),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sector_of_the_compass_has_a_name() {
        let cases = [
            (0.0, "N"),
            (11.24, "N"),
            (11.26, "NNE"),
            (22.5, "NNE"),
            (90.0, "E"),
            (180.0, "S"),
            (202.5, "SSW"),
            (270.0, "W"),
            (340.0, "NNW"),
            // The boundary between NNW and N; a tie rounds up, as everywhere.
            (348.75, "N"),
            (358.0, "N"),
            (360.0, "N"),
        ];
        for (degrees, expected) in cases {
            assert_eq!(compass_point(degrees), Some(expected), "{degrees}");
        }
    }

    /// No fixture carries one and nothing here assumes NOAA would, but a
    /// bearing outside 0..360 or a non-finite one must not index past the end
    /// of the compass.
    #[test]
    fn a_bearing_outside_the_circle_still_lands_on_a_point() {
        assert_eq!(compass_point(-90.0), Some("W"));
        assert_eq!(compass_point(720.0), Some("N"));
        assert_eq!(compass_point(f64::NAN), None);
        assert_eq!(compass_point(f64::INFINITY), None);
    }
}
