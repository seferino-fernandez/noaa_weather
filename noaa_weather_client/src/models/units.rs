//! Measured values and the unit vocabulary NOAA writes them in.
//!
//! [`Quantity`] is the shape NOAA uses everywhere a number carries a unit:
//! `{"value": 23.9, "unitCode": "wmoUnit:degC"}` on a forecast temperature,
//! `{"minValue": 16.1, "maxValue": 24.1, "unitCode": "wmoUnit:km_h-1"}` on a
//! twelve-hour wind speed, and `{"value": null, "unitCode": "wmoUnit:km"}`
//! wherever a layer has no reading for a period.
//!
//! # Requiredness
//!
//! `value` is `Option` and always serialized: NOAA sends `"value": null` far
//! more often than it omits the key, and a range object carries `minValue`
//! and `maxValue` instead. `unit` is not `Option` — every quantity NOAA has
//! been observed to send names its unit, and a number without one is not a
//! measurement.
//!
//! # Units
//!
//! [`Unit`] closes the vocabulary over the two namespaces NOAA publishes,
//! [`WmoUnitCode`] and [`NwsUnitCode`], and keeps anything else verbatim in
//! [`Unit::Other`] rather than failing to decode. [`Unit::code`] returns the
//! exact wire string in every case.
//!
//! [`Quantity::in_unit`] converts between commensurable units. It covers
//! temperature, speed, length, and pressure — the four dimensions with more
//! than one unit in NOAA's responses — and returns `None` for everything
//! else, including percent and the dimensionless indices, which have nothing
//! to convert to.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::{NwsUnitCode, QualityControl, WmoUnitCode};

/// A measurement: a number, or a range of numbers, and its unit.
///
/// ```
/// use noaa_weather_client::models::{Quantity, Unit, WmoUnitCode};
///
/// let temperature: Quantity = serde_json::from_str(
///     r#"{"unitCode": "wmoUnit:degC", "value": 23.88888888888889}"#,
/// )?;
/// assert_eq!(temperature.unit, Unit::Wmo(WmoUnitCode::DegreesCelsius8));
/// assert_eq!(temperature.unit.code(), "wmoUnit:degC");
///
/// let fahrenheit = temperature.in_unit(&Unit::Other("wmoUnit:degF".into())).unwrap();
/// assert_eq!(fahrenheit.value.unwrap().round(), 75.0);
///
/// // Percent is not commensurable with anything.
/// let humidity: Quantity =
///     serde_json::from_str(r#"{"unitCode": "wmoUnit:percent", "value": 50}"#)?;
/// assert!(humidity.in_unit(&Unit::Wmo(WmoUnitCode::DegreesCelsius8)).is_none());
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Quantity {
    /// The measured value. `None` for both `"value": null` and a range
    /// object that carries only [`Quantity::min_value`] and
    /// [`Quantity::max_value`]; serialized as `null` either way.
    #[serde(default)]
    pub value: Option<f64>,
    /// The low end of a range of measured values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    /// The high end of a range of measured values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// The unit the numbers are expressed in.
    #[serde(rename = "unitCode")]
    pub unit: Unit,
    /// For values in observation records, the MADIS quality control flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_control: Option<QualityControl>,
}

impl Quantity {
    /// A measurement of `value` in `unit`, with no range and no quality
    /// control flag.
    ///
    /// [`Quantity`] is `#[non_exhaustive]`, so this is the only way to build
    /// one outside this crate. It exists for the places NOAA writes the
    /// number and its unit apart rather than together — a
    /// [`GridpointLayer`](super::GridpointLayer) carries one `uom` over a
    /// whole series of bare numbers — so a caller can hand a reading to
    /// [`Quantity::in_unit`] without reassembling JSON.
    ///
    /// ```
    /// use noaa_weather_client::models::{Quantity, Unit};
    ///
    /// let celsius = Quantity::new(Some(23.888_888_888_888_89), Unit::from("wmoUnit:degC"));
    /// let fahrenheit = celsius.in_unit(&Unit::from("wmoUnit:degF")).unwrap();
    /// assert_eq!(fahrenheit.value.unwrap().round(), 75.0);
    ///
    /// // A layer with no reading for a period is still a measurement.
    /// assert_eq!(Quantity::new(None, Unit::from("wmoUnit:m")).value, None);
    /// ```
    #[must_use]
    pub const fn new(value: Option<f64>, unit: Unit) -> Self {
        Self {
            value,
            min_value: None,
            max_value: None,
            unit,
            quality_control: None,
        }
    }

    /// Returns this measurement expressed in `unit`, or `None` when the two
    /// units measure different things.
    ///
    /// Every number present is converted: [`Quantity::value`],
    /// [`Quantity::min_value`], and [`Quantity::max_value`]. The quality
    /// control flag is carried through unchanged, since converting a number
    /// does not change how well it was measured.
    ///
    /// Conversions are defined for temperature, speed, length, and
    /// pressure. Converting a unit to itself always succeeds.
    #[must_use]
    pub fn in_unit(&self, unit: &Unit) -> Option<Self> {
        let from = Scale::of(&self.unit)?;
        let into = Scale::of(unit)?;
        if from.dimension != into.dimension {
            return None;
        }
        let convert = |value: Option<f64>| value.map(|value| into.into_unit(from.into_base(value)));
        Some(Self {
            value: convert(self.value),
            min_value: convert(self.min_value),
            max_value: convert(self.max_value),
            unit: unit.clone(),
            quality_control: self.quality_control,
        })
    }
}

/// A unit of measure, as NOAA names it.
///
/// NOAA writes units as `{namespace}:{unit}`. The `wmoUnit` namespace is the
/// World Meteorological Organization Codes Registry
/// (<http://codes.wmo.int/common/unit>) and `nwsUnit` is a small custom set.
/// Codes outside both, such as the `wmoUnit:degF` NOAA sends without the
/// quantitative-value feature flags, are kept verbatim in [`Unit::Other`].
///
/// ```
/// use noaa_weather_client::models::{NwsUnitCode, Unit, WmoUnitCode};
///
/// let wmo: Unit = serde_json::from_str(r#""wmoUnit:km_h-1""#)?;
/// assert_eq!(wmo, Unit::Wmo(WmoUnitCode::KilometresPerHour));
/// assert_eq!(wmo.code(), "wmoUnit:km_h-1");
///
/// let nws: Unit = serde_json::from_str(r#""nwsUnit:s""#)?;
/// assert_eq!(nws, Unit::Nws(NwsUnitCode::Second));
///
/// let unknown: Unit = serde_json::from_str(r#""wmoUnit:degF""#)?;
/// assert_eq!(unknown, Unit::Other("wmoUnit:degF".into()));
/// assert_eq!(unknown.code(), "wmoUnit:degF");
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Unit {
    /// A unit in the WMO Codes Registry.
    Wmo(WmoUnitCode),
    /// A unit in NOAA's custom `nwsUnit` namespace.
    Nws(NwsUnitCode),
    /// A unit outside both namespaces, kept exactly as NOAA wrote it.
    Other(Box<str>),
}

impl Unit {
    /// Returns the wire string, such as `wmoUnit:degC`.
    #[must_use]
    pub fn code(&self) -> &str {
        match self {
            Self::Wmo(unit) => unit.unit_code(),
            Self::Nws(unit) => unit.unit_code(),
            Self::Other(code) => code,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl From<&str> for Unit {
    fn from(code: &str) -> Self {
        match serde_json::from_value(serde_json::Value::String(code.to_owned())) {
            Ok(unit) => unit,
            Err(_) => Self::Other(code.into()),
        }
    }
}

impl_string_schema!(
    Unit,
    "A unit of measure as {namespace}:{unit}, for example wmoUnit:degC or nwsUnit:s."
);

/// The four dimensions [`Quantity::in_unit`] converts within.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dimension {
    Temperature,
    Speed,
    Length,
    Pressure,
}

/// An affine map onto the dimension's base unit: degrees Celsius, metres per
/// second, metres, and pascals.
#[derive(Clone, Copy, Debug)]
struct Scale {
    dimension: Dimension,
    factor: f64,
    offset: f64,
}

impl Scale {
    const fn new(dimension: Dimension, factor: f64, offset: f64) -> Self {
        Self {
            dimension,
            factor,
            offset,
        }
    }

    /// Returns the scale for `unit`, keyed on the symbol after the
    /// namespace so that a code NOAA sends outside its own registry
    /// (`wmoUnit:degF`) converts like the registered one.
    fn of(unit: &Unit) -> Option<Self> {
        let symbol = unit
            .code()
            .rsplit([':', '/'])
            .find(|segment| !segment.is_empty())?;
        Some(match symbol {
            "degC" | "Cel" => Self::new(Dimension::Temperature, 1.0, 0.0),
            "degF" => Self::new(Dimension::Temperature, 5.0 / 9.0, -160.0 / 9.0),
            "K" => Self::new(Dimension::Temperature, 1.0, -273.15),
            "m_s-1" => Self::new(Dimension::Speed, 1.0, 0.0),
            "km_h-1" => Self::new(Dimension::Speed, 1.0 / 3.6, 0.0),
            "mi_h-1" => Self::new(Dimension::Speed, 0.44704, 0.0),
            "kt" | "nmi_h-1" => Self::new(Dimension::Speed, 1852.0 / 3600.0, 0.0),
            "m" => Self::new(Dimension::Length, 1.0, 0.0),
            "km" => Self::new(Dimension::Length, 1000.0, 0.0),
            "cm" => Self::new(Dimension::Length, 0.01, 0.0),
            "mm" => Self::new(Dimension::Length, 0.001, 0.0),
            "ft" => Self::new(Dimension::Length, 0.3048, 0.0),
            "in" => Self::new(Dimension::Length, 0.0254, 0.0),
            "mi" => Self::new(Dimension::Length, 1609.344, 0.0),
            "nmi" => Self::new(Dimension::Length, 1852.0, 0.0),
            "Pa" => Self::new(Dimension::Pressure, 1.0, 0.0),
            "hPa" | "mbar" | "mb" => Self::new(Dimension::Pressure, 100.0, 0.0),
            "kPa" => Self::new(Dimension::Pressure, 1000.0, 0.0),
            "inHg" => Self::new(Dimension::Pressure, 3386.388640341, 0.0),
            _ => return None,
        })
    }

    fn into_base(self, value: f64) -> f64 {
        value * self.factor + self.offset
    }

    fn into_unit(self, value: f64) -> f64 {
        (value - self.offset) / self.factor
    }
}

/// A WMO or NWS unit code, without the open [`Unit::Other`] fallback.
///
/// Used by [`ValueUnit`], the older measurement shape retained by the
/// not-yet-curated radar wire models.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnitCodeType {
    /// Represents a World Meteorological Organization (WMO) unit code.
    Wmo(WmoUnitCode),
    /// Represents a National Weather Service (NWS) unit code.
    Nws(NwsUnitCode),
}

/// Legacy radar-wire value with an associated unit code.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValueUnit {
    /// The unit code, which can be either a WMO unit code or an NWS unit code.
    /// Examples: "wmoUnit:m", "nwsUnit:s".
    #[serde(rename = "unitCode", skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<UnitCodeType>,
    /// The numerical value. Using f64 to accommodate both integers and floating-point numbers.
    pub value: Option<f64>,
    /// The maximum value of a range of measured values
    #[serde(rename = "maxValue", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    /// The minimum value of a range of measured values
    #[serde(rename = "minValue", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<Option<f64>>,
    #[serde(rename = "qualityControl", skip_serializing_if = "Option::is_none")]
    pub quality_control: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    fn quantity(json: &str) -> Quantity {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn every_namespace_round_trips_through_its_wire_string() {
        for code in [
            "wmoUnit:degC",
            "wmoUnit:percent",
            "wmoUnit:m",
            "wmoUnit:mm",
            "wmoUnit:km",
            "wmoUnit:km_h-1",
            "wmoUnit:degree_(angle)",
            "wmoUnit:'",
            "wmoUnit:\"",
            "nwsUnit:s",
            "nwsUnit:dBZ",
            "wmoUnit:degF",
            "unheard-of",
        ] {
            let unit: Unit = serde_json::from_value(json!(code)).unwrap();
            assert_eq!(unit.code(), code, "{code}");
            assert_eq!(unit.to_string(), code, "{code}");
            assert_eq!(serde_json::to_value(&unit).unwrap(), json!(code), "{code}");
            assert_eq!(Unit::from(code), unit, "{code}");
        }
    }

    #[test]
    fn known_namespaces_are_typed_and_the_rest_are_kept_verbatim() {
        assert_eq!(
            Unit::from("wmoUnit:degC"),
            Unit::Wmo(WmoUnitCode::DegreesCelsius8)
        );
        assert_eq!(Unit::from("nwsUnit:s"), Unit::Nws(NwsUnitCode::Second));
        assert_eq!(
            Unit::from("wmoUnit:degF"),
            Unit::Other("wmoUnit:degF".into())
        );
    }

    #[test]
    fn a_null_value_survives_the_round_trip_as_null() {
        let raw = json!({"value": Value::Null, "unitCode": "wmoUnit:km"});
        let quantity: Quantity = serde_json::from_value(raw).unwrap();
        assert_eq!(quantity.value, None);
        assert_eq!(
            serde_json::to_value(&quantity).unwrap(),
            json!({"value": Value::Null, "unitCode": "wmoUnit:km"})
        );
    }

    #[test]
    fn a_range_keeps_both_bounds_and_reports_no_single_value() {
        let range = quantity(
            r#"{"unitCode": "wmoUnit:km_h-1", "maxValue": 24.14016, "minValue": 16.09344}"#,
        );
        assert_eq!(range.value, None);
        assert_eq!(range.min_value, Some(16.09344));
        assert_eq!(range.max_value, Some(24.14016));
    }

    #[test]
    fn absent_optional_keys_are_skipped_and_value_is_not() {
        let quantity = quantity(r#"{"unitCode": "wmoUnit:m", "value": 456.8952}"#);
        let value = serde_json::to_value(&quantity).unwrap();
        let object = value.as_object().unwrap();
        assert!(object.contains_key("value"));
        for key in ["minValue", "maxValue", "qualityControl"] {
            assert!(!object.contains_key(key), "{key} should be skipped");
        }
    }

    #[test]
    fn conversions_cover_temperature_speed_length_and_pressure() {
        let cases = [
            ("wmoUnit:degC", 100.0, "wmoUnit:degF", 212.0),
            ("wmoUnit:degF", 32.0, "wmoUnit:degC", 0.0),
            ("wmoUnit:degC", 0.0, "wmoUnit:K", 273.15),
            ("wmoUnit:km_h-1", 3.6, "wmoUnit:m_s-1", 1.0),
            ("wmoUnit:mi_h-1", 10.0, "wmoUnit:km_h-1", 16.09344),
            ("wmoUnit:km", 1.0, "wmoUnit:m", 1000.0),
            ("wmoUnit:m", 0.3048, "wmoUnit:ft", 1.0),
            ("wmoUnit:Pa", 100.0, "wmoUnit:hPa", 1.0),
        ];
        for (from, value, into, expected) in cases {
            let quantity = quantity(&format!(r#"{{"unitCode": "{from}", "value": {value}}}"#));
            let converted = quantity
                .in_unit(&Unit::from(into))
                .unwrap_or_else(|| panic!("{from} -> {into}"));
            let got = converted.value.unwrap();
            assert!(
                (got - expected).abs() < 1e-9,
                "{from} {value} -> {into}: {got} != {expected}"
            );
            assert_eq!(converted.unit, Unit::from(into));
        }
    }

    #[test]
    fn a_converted_range_moves_both_bounds() {
        let range =
            quantity(r#"{"unitCode": "wmoUnit:km_h-1", "maxValue": 36.0, "minValue": 18.0}"#);
        let converted = range.in_unit(&Unit::from("wmoUnit:m_s-1")).unwrap();
        assert_eq!(converted.value, None);
        assert_eq!(converted.min_value, Some(5.0));
        assert_eq!(converted.max_value, Some(10.0));
    }

    #[test]
    fn incommensurable_and_unconvertible_units_are_none() {
        let celsius = quantity(r#"{"unitCode": "wmoUnit:degC", "value": 20}"#);
        assert!(celsius.in_unit(&Unit::from("wmoUnit:m")).is_none());
        assert!(celsius.in_unit(&Unit::from("wmoUnit:percent")).is_none());

        let percent = quantity(r#"{"unitCode": "wmoUnit:percent", "value": 50}"#);
        assert!(percent.in_unit(&Unit::from("wmoUnit:percent")).is_none());

        let index = quantity(r#"{"unitCode": "nwsUnit:s", "value": 4}"#);
        assert!(index.in_unit(&Unit::from("wmoUnit:degC")).is_none());
    }

    #[test]
    fn converting_a_unit_to_itself_keeps_the_numbers() {
        let quantity =
            quantity(r#"{"unitCode": "wmoUnit:degC", "value": 23.5, "qualityControl": "V"}"#);
        let same = quantity.in_unit(&Unit::from("wmoUnit:degC")).unwrap();
        assert_eq!(same.value, Some(23.5));
        assert_eq!(same.quality_control, Some(QualityControl::V));
    }
}
