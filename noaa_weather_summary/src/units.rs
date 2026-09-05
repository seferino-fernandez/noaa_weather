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
    /// The presentation kind a wire unit ordinarily represents.
    ///
    /// Metres default to height: a gridpoint key supplies the exceptional
    /// visibility context, because a unit alone cannot distinguish the two.
    pub(crate) fn of_unit(unit: &Unit) -> Self {
        match symbol(unit.code()) {
            "degC" | "Cel" | "degF" | "K" => Self::Temperature,
            "m_s-1" | "km_h-1" | "mi_h-1" | "kt" | "nmi_h-1" => Self::Speed,
            "mm" | "cm" | "in" => Self::Depth,
            "m" | "km" | "ft" | "mi" | "nmi" => Self::Height,
            "Pa" | "hPa" | "kPa" | "mbar" | "mb" | "inHg" => Self::Pressure,
            "percent" => Self::Percent,
            "degree_(angle)" => Self::Angle,
            _ => Self::Index,
        }
    }

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
pub(crate) fn label(unit: &Unit) -> &str {
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
pub(crate) fn symbol(code: &str) -> &str {
    code.rsplit([':', '/'])
        .find(|segment| !segment.is_empty())
        .unwrap_or(code)
}

/// The label a reading of `kind` measured in `source` ends up shown under.
///
/// A gridpoint layer states its unit once, in a column header, above a series
/// of numbers that [`Value::quantity`] labels one by one. Both answers come
/// from here, so the header cannot claim `°C` over a column of `°F`: the
/// question "does this unit convert to the kind's target" is asked exactly as
/// [`Value::quantity`] asks it, by trying the conversion on a placeholder.
///
/// `None` means there is nothing to say — a dimensionless index, or a layer
/// NOAA sent with no `uom` at all.
pub(crate) fn shown_unit(
    kind: QuantityKind,
    source: Option<&Unit>,
    options: &SummaryOptions,
) -> Option<String> {
    // A percentage is shown as `40%` whether or not NOAA bothered to say so,
    // because `Value::quantity` turns it into a `Value::Percent`.
    if kind == QuantityKind::Percent {
        return Some("%".to_owned());
    }
    let source = source?;
    let probe = Quantity::new(Some(0.0), source.clone());
    let converted = kind
        .target(options.units)
        .map(Unit::from)
        .and_then(|target| probe.in_unit(&target));
    Some(label(&converted.as_ref().unwrap_or(&probe).unit).to_owned())
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

    /// One reading from a gridpoint layer, which states its unit once for the
    /// whole series instead of on every number.
    ///
    /// The same policy as [`Value::quantity`], reached from the shape NOAA
    /// uses for layers. A layer with no `uom` — every dimensionless index,
    /// and [`probabilityOfThunder`] — has nothing to convert to, so the
    /// number is shown as sent, at the precision its kind asks for.
    ///
    /// [`probabilityOfThunder`]: noaa_weather_client::models::Gridpoint::probability_of_thunder
    pub fn reading(
        value: Option<f64>,
        unit: Option<&Unit>,
        kind: QuantityKind,
        options: &SummaryOptions,
    ) -> Self {
        match unit {
            Some(unit) => Self::quantity(&Quantity::new(value, unit.clone()), kind, options),
            None if kind == QuantityKind::Percent => Self::percent(value),
            None => Self::number(value, kind.precision(), None),
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

    #[test]
    fn unit_symbols_pick_their_presentation_kind() {
        let cases = [
            ("degC", QuantityKind::Temperature),
            ("Cel", QuantityKind::Temperature),
            ("degF", QuantityKind::Temperature),
            ("K", QuantityKind::Temperature),
            ("m_s-1", QuantityKind::Speed),
            ("km_h-1", QuantityKind::Speed),
            ("mi_h-1", QuantityKind::Speed),
            ("kt", QuantityKind::Speed),
            ("nmi_h-1", QuantityKind::Speed),
            ("mm", QuantityKind::Depth),
            ("cm", QuantityKind::Depth),
            ("in", QuantityKind::Depth),
            ("m", QuantityKind::Height),
            ("km", QuantityKind::Height),
            ("ft", QuantityKind::Height),
            ("mi", QuantityKind::Height),
            ("nmi", QuantityKind::Height),
            ("Pa", QuantityKind::Pressure),
            ("hPa", QuantityKind::Pressure),
            ("kPa", QuantityKind::Pressure),
            ("mbar", QuantityKind::Pressure),
            ("mb", QuantityKind::Pressure),
            ("inHg", QuantityKind::Pressure),
            ("percent", QuantityKind::Percent),
            ("degree_(angle)", QuantityKind::Angle),
            ("unknown", QuantityKind::Index),
        ];
        for (symbol, expected) in cases {
            let unit = Unit::from(symbol);
            assert_eq!(QuantityKind::of_unit(&unit), expected, "{symbol}");
        }
    }

    #[test]
    fn every_non_unitless_target_has_a_non_index_unit_policy() {
        for kind in [
            QuantityKind::Temperature,
            QuantityKind::Speed,
            QuantityKind::Height,
            QuantityKind::Distance,
            QuantityKind::Depth,
            QuantityKind::Pressure,
        ] {
            for system in [UnitSystem::Us, UnitSystem::Si] {
                let target = Unit::from(kind.target(system).expect("non-unitless kind"));
                let expected = if kind == QuantityKind::Distance {
                    // The unit policy has no gridpoint key, so distance targets
                    // conventionally metres as heights until that context arrives.
                    QuantityKind::Height
                } else {
                    kind
                };
                assert_eq!(
                    QuantityKind::of_unit(&target),
                    expected,
                    "{kind:?} {system:?}"
                );
                assert_ne!(QuantityKind::of_unit(&target), QuantityKind::Index);
            }
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

    /// A layer states its unit once; a reading of it lands in the same place
    /// the equivalent [`Quantity`] would.
    #[test]
    fn a_layer_reading_follows_the_same_policy_as_a_quantity() {
        let celsius = Unit::from("wmoUnit:degC");
        assert_eq!(
            Value::reading(
                Some(23.888_888_888_888_89),
                Some(&celsius),
                QuantityKind::Temperature,
                &options(UnitSystem::Us)
            ),
            Value::quantity(
                &quantity("wmoUnit:degC", 23.888_888_888_888_89),
                QuantityKind::Temperature,
                &options(UnitSystem::Us)
            )
        );
        assert_eq!(
            Value::reading(
                None,
                Some(&celsius),
                QuantityKind::Temperature,
                &options(UnitSystem::Us)
            ),
            Value::Missing
        );
    }

    /// `heatRisk` and `probabilityOfThunder` arrive with no `uom` at all. An
    /// index keeps its bare number; a probability is still a percentage.
    #[test]
    fn a_layer_without_a_unit_keeps_its_number() {
        let us = options(UnitSystem::Us);
        assert_eq!(
            Value::reading(Some(4.0), None, QuantityKind::Index, &us),
            Value::Quantity {
                value: 4.0,
                unit: None,
                precision: 0,
            }
        );
        assert_eq!(
            Value::reading(Some(44.0), None, QuantityKind::Percent, &us),
            Value::Percent(44.0)
        );
        assert_eq!(
            Value::reading(None, None, QuantityKind::Index, &us),
            Value::Missing
        );
    }

    /// The column header and the cells under it are one decision, so a header
    /// can never claim a unit the values are not shown in.
    #[test]
    fn the_shown_unit_agrees_with_every_value_under_it() {
        let cases = [
            (QuantityKind::Temperature, Some("wmoUnit:degC"), "\u{b0}F"),
            (QuantityKind::Speed, Some("wmoUnit:km_h-1"), "mph"),
            (QuantityKind::Height, Some("wmoUnit:m"), "ft"),
            (QuantityKind::Distance, Some("wmoUnit:m"), "mi"),
            (QuantityKind::Depth, Some("wmoUnit:mm"), "in"),
            (
                QuantityKind::Angle,
                Some("wmoUnit:degree_(angle)"),
                "\u{b0}",
            ),
            (QuantityKind::Percent, Some("wmoUnit:percent"), "%"),
            // No unit to convert through: the wire symbol stands.
            (QuantityKind::Depth, Some("bananaUnit:bunches"), "bunches"),
            // A kind whose unit measures something else keeps what it has.
            (QuantityKind::Temperature, Some("wmoUnit:m"), "m"),
        ];
        let us = options(UnitSystem::Us);
        for (kind, code, expected) in cases {
            let unit = code.map(Unit::from);
            assert_eq!(
                shown_unit(kind, unit.as_ref(), &us).as_deref(),
                Some(expected),
                "{kind:?} {code:?}"
            );
            let reading = Value::reading(Some(1.0), unit.as_ref(), kind, &us);
            let shown = match reading {
                Value::Quantity { unit, .. } => unit,
                Value::Percent(_) => Some("%".to_owned()),
                other => panic!("expected a labelled value, got {other:?}"),
            };
            assert_eq!(shown.as_deref(), Some(expected), "{kind:?} value label");
        }
    }

    /// A dimensionless index has no unit to name; a percentage names itself
    /// even when NOAA sent no `uom`.
    #[test]
    fn a_unitless_layer_has_a_unit_only_when_it_is_a_percentage() {
        let us = options(UnitSystem::Us);
        assert_eq!(shown_unit(QuantityKind::Index, None, &us), None);
        assert_eq!(shown_unit(QuantityKind::Height, None, &us), None);
        assert_eq!(
            shown_unit(QuantityKind::Percent, None, &us).as_deref(),
            Some("%")
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
