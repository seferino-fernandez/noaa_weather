//! Unit policy: the one place that decides what a measurement is called,
//! what it converts to, and how many decimals it shows.
//!
//! NOAA sends a [`Quantity`] in whatever unit its own pipeline used —
//! `degC` next to `km_h-1` next to `m` — and it sends the same unit whichever
//! `units` parameter the request asked for. Turning that into something a
//! person reads is one decision made three ways: which unit to convert to,
//! what to call it, and how precise to be about it. All three live here, and
//! [`Value::quantity`] is the only way in.
//!
//! Conversion arithmetic is not here: [`Quantity::in_unit`] already converts
//! the value and both bounds together and declines across dimensions.

use noaa_weather_client::models::{Quantity, Unit};

use crate::{SummaryOptions, UnitSystem, Value};

/// What a measurement measures, which is what decides the unit it is shown in.
///
/// NOAA's wire unit does not answer this on its own: `wmoUnit:m` is a
/// visibility, a ceiling height and a wave height, and those do not read well
/// in the same unit. The caller names the kind; this module names the unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuantityKind {
    /// Air, dewpoint, wet bulb, apparent temperature.
    Temperature,
    /// Wind speed and gusts.
    Speed,
    /// A height above ground or sea level: elevation, ceiling, wave.
    Height,
    /// A distance across the ground: visibility, how far a point is from town.
    Distance,
    /// An accumulation: rain, snowfall, ice.
    Depth,
    /// Barometric pressure.
    Pressure,
    /// A share of a whole, already in percent.
    Percent,
    /// A compass bearing in degrees.
    Angle,
    /// A dimensionless index such as a fire weather or hazard rating.
    Index,
}

impl QuantityKind {
    /// The wire unit code this kind is shown in under `system`, or `None` for
    /// the kinds that have nothing to convert to.
    const fn target(self, system: UnitSystem) -> Option<&'static str> {
        Some(match (self, system) {
            (Self::Temperature, UnitSystem::Us) => "wmoUnit:degF",
            (Self::Temperature, UnitSystem::Si) => "wmoUnit:degC",
            (Self::Speed, UnitSystem::Us) => "wmoUnit:mi_h-1",
            (Self::Speed, UnitSystem::Si) => "wmoUnit:km_h-1",
            (Self::Height, UnitSystem::Us) => "wmoUnit:ft",
            (Self::Height, UnitSystem::Si) => "wmoUnit:m",
            (Self::Distance, UnitSystem::Us) => "wmoUnit:mi",
            (Self::Distance, UnitSystem::Si) => "wmoUnit:km",
            (Self::Depth, UnitSystem::Us) => "wmoUnit:in",
            (Self::Depth, UnitSystem::Si) => "wmoUnit:mm",
            (Self::Pressure, UnitSystem::Us) => "wmoUnit:inHg",
            (Self::Pressure, UnitSystem::Si) => "wmoUnit:hPa",
            (Self::Percent | Self::Angle | Self::Index, _) => return None,
        })
    }

    /// How many decimal places this kind is worth reading to.
    ///
    /// A tenth of a degree is noise; a hundredth of an inch of rain is the
    /// difference between a wet road and a dry one.
    const fn precision(self) -> u8 {
        match self {
            Self::Temperature
            | Self::Speed
            | Self::Height
            | Self::Percent
            | Self::Angle
            | Self::Index => 0,
            Self::Distance => 1,
            Self::Depth | Self::Pressure => 2,
        }
    }
}

/// What a unit is called in output.
///
/// Curated rather than taken from [`Unit`]'s own registry label, which is WMO
/// catalog data and not display policy: it spells the degree sign as a ring
/// above, writes kilometres per hour as `km h^-1`, and gives the nautical mile
/// a bare space. An unlisted unit falls back to its wire symbol, which is
/// what the CLI printed before this table existed.
fn label(unit: &Unit) -> &str {
    let symbol = symbol(unit.code());
    match symbol {
        "degF" => "\u{b0}F",
        "degC" | "Cel" => "\u{b0}C",
        "mi_h-1" => "mph",
        "km_h-1" => "km/h",
        "kt" | "nmi_h-1" => "kt",
        "degree_(angle)" => "\u{b0}",
        "ft" => "ft",
        "m" => "m",
        "mi" => "mi",
        "km" => "km",
        "in" => "in",
        "mm" => "mm",
        "cm" => "cm",
        "inHg" => "inHg",
        "hPa" => "hPa",
        "percent" => "%",
        other => other,
    }
}

/// The unit symbol NOAA's `{namespace}:{unit}` wire code ends in, keyed the
/// same way [`Quantity::in_unit`] keys its conversions.
fn symbol(code: &str) -> &str {
    code.rsplit([':', '/'])
        .find(|segment| !segment.is_empty())
        .unwrap_or(code)
}

impl Value {
    /// A NOAA measurement, converted into the caller's unit system and
    /// labelled and rounded for the kind of thing it measures.
    ///
    /// A quantity carrying a single value becomes a [`Value::Quantity`]; one
    /// carrying only bounds, as the twelve-hour wind speed does, becomes a
    /// [`Value::Range`]; one carrying nothing becomes [`Value::Missing`].
    /// [`QuantityKind::Percent`] is the exception: a single value becomes a
    /// [`Value::Percent`], the variant that already exists for it, so a
    /// gridpoint humidity reads `40%` exactly as an alert's certainty does.
    ///
    /// When the conversion cannot be made — an unregistered unit, or a kind
    /// whose unit turns out to measure something else — the raw numbers are
    /// shown under the wire unit's own symbol. Degrading to the number NOAA
    /// sent beats making it disappear.
    pub fn quantity(quantity: &Quantity, kind: QuantityKind, options: &SummaryOptions) -> Self {
        let converted = kind
            .target(options.units)
            .map(Unit::from)
            .and_then(|target| quantity.in_unit(&target));
        let shown = converted.as_ref().unwrap_or(quantity);

        let precision = kind.precision();
        let unit = Some(label(&shown.unit));
        match (shown.value, shown.min_value, shown.max_value) {
            (Some(value), _, _) if kind == QuantityKind::Percent => Self::percent(Some(value)),
            (Some(_), _, _) => Self::number(shown.value, precision, unit),
            (None, min @ Some(_), max @ Some(_)) => Self::range(min, max, precision, unit),
            (None, _, _) => Self::Missing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{RangeStyle, RenderOptions, format_value};

    fn quantity(code: &str, value: f64) -> Quantity {
        serde_json::from_str(&format!(r#"{{"unitCode": "{code}", "value": {value}}}"#))
            .expect("test quantity decodes")
    }

    fn options(units: UnitSystem) -> SummaryOptions {
        SummaryOptions { units }
    }

    fn shown(quantity: &Quantity, kind: QuantityKind, units: UnitSystem) -> String {
        match Value::quantity(quantity, kind, &options(units)) {
            Value::Quantity {
                value,
                unit,
                precision,
            } => {
                let precision = usize::from(precision);
                format!("{value:.precision$} {}", unit.unwrap_or_default())
            }
            other => panic!("expected a quantity, got {other:?}"),
        }
    }

    #[test]
    fn every_kind_and_system_picks_its_unit_and_precision() {
        let cases = [
            (
                QuantityKind::Temperature,
                quantity("wmoUnit:degC", 23.888_888_888_888_89),
                "75 \u{b0}F",
                "24 \u{b0}C",
            ),
            (
                QuantityKind::Speed,
                quantity("wmoUnit:km_h-1", 16.093_44),
                "10 mph",
                "16 km/h",
            ),
            (
                QuantityKind::Height,
                quantity("wmoUnit:m", 304.8),
                "1000 ft",
                "305 m",
            ),
            (
                QuantityKind::Distance,
                quantity("wmoUnit:m", 16_093.44),
                "10.0 mi",
                "16.1 km",
            ),
            (
                QuantityKind::Depth,
                quantity("wmoUnit:mm", 25.4),
                "1.00 in",
                "25.40 mm",
            ),
            (
                QuantityKind::Pressure,
                quantity("wmoUnit:Pa", 101_325.0),
                "29.92 inHg",
                "1013.25 hPa",
            ),
            (
                QuantityKind::Angle,
                quantity("wmoUnit:degree_(angle)", 210.0),
                "210 \u{b0}",
                "210 \u{b0}",
            ),
            (
                QuantityKind::Index,
                quantity("nwsUnit:s", 4.0),
                "4 s",
                "4 s",
            ),
        ];
        for (kind, quantity, us, si) in cases {
            assert_eq!(shown(&quantity, kind, UnitSystem::Us), us, "{kind:?} US");
            assert_eq!(shown(&quantity, kind, UnitSystem::Si), si, "{kind:?} SI");
        }
    }

    /// A percentage has its own [`Value`] variant and its own rendering. The
    /// wire unit code is not a label a person should ever read: `40 percent`
    /// is `wmoUnit:percent` leaking through.
    #[test]
    fn a_percent_is_a_percent_in_either_system() {
        let humidity = quantity("wmoUnit:percent", 39.6);
        for units in [UnitSystem::Us, UnitSystem::Si] {
            assert_eq!(
                Value::quantity(&humidity, QuantityKind::Percent, &options(units)),
                Value::Percent(39.6),
                "{units:?}"
            );
        }
        assert_eq!(
            format_value(
                &Value::quantity(&humidity, QuantityKind::Percent, &options(UnitSystem::Us)),
                &RenderOptions::default(),
                RangeStyle::Words,
            ),
            "40%"
        );
    }

    /// A bounded probability keeps the range shape, labelled from the same
    /// table so the two percent paths cannot drift apart.
    #[test]
    fn a_percent_given_as_bounds_is_a_range_labelled_from_the_table() {
        let chance: Quantity = serde_json::from_str(
            r#"{"unitCode": "wmoUnit:percent", "minValue": 20, "maxValue": 60}"#,
        )
        .unwrap();
        let value = Value::quantity(&chance, QuantityKind::Percent, &options(UnitSystem::Us));
        assert_eq!(
            value,
            Value::Range {
                min: 20.0,
                max: 60.0,
                unit: Some("%".to_owned()),
                precision: 0,
            }
        );
        assert_eq!(
            format_value(&value, &RenderOptions::default(), RangeStyle::Words),
            "20 to 60%"
        );
    }

    /// The twelve-hour wind: `value` is null and the bounds carry the reading.
    #[test]
    fn bounds_without_a_value_become_a_range() {
        let wind: Quantity = serde_json::from_str(
            r#"{"unitCode": "wmoUnit:km_h-1", "minValue": 16.09344, "maxValue": 32.18688}"#,
        )
        .unwrap();
        let Value::Range {
            min,
            max,
            unit,
            precision,
        } = Value::quantity(&wind, QuantityKind::Speed, &options(UnitSystem::Us))
        else {
            panic!("bounds without a value must become a range");
        };
        assert!((min - 10.0).abs() < 1e-9, "{min}");
        assert!((max - 20.0).abs() < 1e-9, "{max}");
        assert_eq!(unit.as_deref(), Some("mph"));
        assert_eq!(precision, 0);
    }

    #[test]
    fn nothing_present_is_missing() {
        let empty: Quantity =
            serde_json::from_str(r#"{"unitCode": "wmoUnit:degC", "value": null}"#).unwrap();
        assert_eq!(
            Value::quantity(&empty, QuantityKind::Temperature, &options(UnitSystem::Us)),
            Value::Missing
        );
        let half: Quantity =
            serde_json::from_str(r#"{"unitCode": "wmoUnit:degC", "minValue": 1}"#).unwrap();
        assert_eq!(
            Value::quantity(&half, QuantityKind::Temperature, &options(UnitSystem::Us)),
            Value::Missing
        );
    }

    /// A unit outside both registries has no scale to convert through, so the
    /// number stays as sent and the wire symbol becomes the label.
    #[test]
    fn an_unknown_unit_falls_back_to_its_wire_symbol() {
        let odd = quantity("bananaUnit:bunches", 3.0);
        assert_eq!(
            Value::quantity(&odd, QuantityKind::Depth, &options(UnitSystem::Us)),
            Value::Quantity {
                value: 3.0,
                unit: Some("bunches".to_owned()),
                precision: 2,
            }
        );
        let bare = quantity("unheard-of", 3.0);
        assert_eq!(
            Value::quantity(&bare, QuantityKind::Height, &options(UnitSystem::Si)),
            Value::Quantity {
                value: 3.0,
                unit: Some("unheard-of".to_owned()),
                precision: 0,
            }
        );
    }

    /// Asking for a temperature and being handed a length: the number is
    /// still worth showing, under the unit it was actually measured in.
    #[test]
    fn an_incommensurable_target_keeps_the_raw_value() {
        let metres = quantity("wmoUnit:m", 456.895_2);
        assert_eq!(
            Value::quantity(&metres, QuantityKind::Temperature, &options(UnitSystem::Us)),
            Value::Quantity {
                value: 456.895_2,
                unit: Some("m".to_owned()),
                precision: 0,
            }
        );
    }

    #[test]
    fn labels_are_curated_and_never_the_registry_spelling() {
        let cases = [
            ("wmoUnit:degF", "\u{b0}F"),
            ("wmoUnit:degC", "\u{b0}C"),
            ("wmoUnit:Cel", "\u{b0}C"),
            ("wmoUnit:mi_h-1", "mph"),
            ("wmoUnit:km_h-1", "km/h"),
            ("wmoUnit:kt", "kt"),
            ("wmoUnit:nmi_h-1", "kt"),
            ("wmoUnit:ft", "ft"),
            ("wmoUnit:m", "m"),
            ("wmoUnit:mi", "mi"),
            ("wmoUnit:km", "km"),
            ("wmoUnit:in", "in"),
            ("wmoUnit:mm", "mm"),
            ("wmoUnit:cm", "cm"),
            ("wmoUnit:inHg", "inHg"),
            ("wmoUnit:hPa", "hPa"),
            ("wmoUnit:degree_(angle)", "\u{b0}"),
            ("wmoUnit:percent", "%"),
            ("nwsUnit:dBZ", "dBZ"),
        ];
        for (code, expected) in cases {
            assert_eq!(label(&Unit::from(code)), expected, "{code}");
        }
    }
}
