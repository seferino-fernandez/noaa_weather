use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::ids::{InvalidValue, ValueKind};

const SHAPE: &str = "must be latitude,longitude in decimal degrees (for example 39.7456,-97.0892)";
const LATITUDE: &str = "latitude must be a finite number between -90 and 90";
const LONGITUDE: &str = "longitude must be a finite number between -180 and 180";

/// A validated `latitude,longitude` pair in decimal degrees.
///
/// Both values are rounded to four decimals when constructed, so equality is
/// canonical: `39.74561` and `39.7456` are the same point. `Display` writes
/// the shortest decimal form NOAA accepts in `/points/{point}` paths.
///
/// ```
/// use noaa_weather_client::Coordinates;
///
/// let point = Coordinates::new(39.7456, -97.0892)?;
/// assert_eq!(point.latitude(), 39.7456);
/// assert_eq!(point.longitude(), -97.0892);
/// assert_eq!(point.to_string(), "39.7456,-97.0892");
/// assert_eq!(Coordinates::new(40.0, -105.0)?.to_string(), "40,-105");
/// assert!(Coordinates::new(91.0, 0.0).is_err());
/// # Ok::<(), noaa_weather_client::InvalidValue>(())
/// ```
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl Coordinates {
    /// Creates coordinates from decimal degrees, rounding both to four
    /// decimals.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidValue`] when either value is not finite, the latitude
    /// is outside `-90..=90`, or the longitude is outside `-180..=180`.
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, InvalidValue> {
        let reject = |reason| {
            InvalidValue::new(
                ValueKind::Coordinates,
                format!("{latitude},{longitude}"),
                reason,
            )
        };
        if !latitude.is_finite() || latitude.abs() > 90.0 {
            return Err(reject(LATITUDE));
        }
        if !longitude.is_finite() || longitude.abs() > 180.0 {
            return Err(reject(LONGITUDE));
        }
        Ok(Self {
            latitude: round4(latitude),
            longitude: round4(longitude),
        })
    }

    /// Returns the latitude in decimal degrees, rounded to four decimals.
    #[must_use]
    pub const fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Returns the longitude in decimal degrees, rounded to four decimals.
    #[must_use]
    pub const fn longitude(&self) -> f64 {
        self.longitude
    }
}

/// Rounds to four decimals and folds negative zero into zero.
fn round4(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0 + 0.0
}

impl fmt::Debug for Coordinates {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Coordinates({}, {})",
            self.latitude, self.longitude
        )
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{},{}", self.latitude, self.longitude)
    }
}

impl FromStr for Coordinates {
    type Err = InvalidValue;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let shape = || InvalidValue::new(ValueKind::Coordinates, input, SHAPE);
        let Some((latitude, longitude)) = input.split_once(',') else {
            return Err(shape());
        };
        let latitude = parse_degrees(latitude).ok_or_else(shape)?;
        let longitude = parse_degrees(longitude.trim_start()).ok_or_else(shape)?;
        Self::new(latitude, longitude)
            .map_err(|error| InvalidValue::new(ValueKind::Coordinates, input, error.reason()))
    }
}

/// Parses a decimal-degree value in canonical form: an optional `-`, an
/// integer part with no leading zero, and an optional fraction with at least
/// one digit. Exponents, a leading `+`, `inf`, and `nan` are rejected so the
/// text form matches the published schema pattern exactly.
fn parse_degrees(text: &str) -> Option<f64> {
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    let (integer, fraction) = match unsigned.split_once('.') {
        Some((integer, fraction)) => (integer, Some(fraction)),
        None => (unsigned, None),
    };
    let digits = |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
    if !digits(integer) || (integer.len() > 1 && integer.starts_with('0')) {
        return None;
    }
    if fraction.is_some_and(|fraction| !digits(fraction)) {
        return None;
    }
    text.parse().ok()
}

impl From<Coordinates> for String {
    fn from(value: Coordinates) -> Self {
        value.to_string()
    }
}

impl_try_from_str!(Coordinates);
impl_string_schema!(
    Coordinates,
    "Latitude and longitude in decimal degrees as lat,lon (for example 39.7456,-97.0892).",
    concat!(
        "^-?(90(\\.0+)?|[1-8]?[0-9](\\.[0-9]+)?)",
        ",\\s*",
        "-?(180(\\.0+)?|1[0-7][0-9](\\.[0-9]+)?|[1-9]?[0-9](\\.[0-9]+)?)$"
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_to_four_decimals_for_canonical_equality() {
        let rounded = Coordinates::new(39.74561, -97.08919).unwrap();
        assert_eq!(rounded.latitude(), 39.7456);
        assert_eq!(rounded.longitude(), -97.0892);
        assert_eq!(rounded, Coordinates::new(39.7456, -97.0892).unwrap());
        assert_eq!(rounded.to_string(), "39.7456,-97.0892");
        assert_eq!(Coordinates::new(39.74565, 0.0).unwrap().latitude(), 39.7457);
    }

    #[test]
    fn display_uses_minimal_decimals() {
        assert_eq!(
            Coordinates::new(40.0, -105.0).unwrap().to_string(),
            "40,-105"
        );
        assert_eq!(
            Coordinates::new(40.5, -105.25).unwrap().to_string(),
            "40.5,-105.25"
        );
        assert_eq!(Coordinates::new(-0.00001, 0.0).unwrap().to_string(), "0,0");
        assert_eq!(Coordinates::new(90.0, 180.0).unwrap().to_string(), "90,180");
        assert_eq!(
            Coordinates::new(-90.0, -180.0).unwrap().to_string(),
            "-90,-180"
        );
    }

    #[test]
    fn debug_shows_both_values() {
        let point = Coordinates::new(39.7456, -97.0892).unwrap();
        assert_eq!(format!("{point:?}"), "Coordinates(39.7456, -97.0892)");
    }

    #[test]
    fn rejects_non_finite_and_out_of_range_values() {
        let cases = [
            (f64::NAN, 0.0, LATITUDE),
            (f64::INFINITY, 0.0, LATITUDE),
            (90.0001, 0.0, LATITUDE),
            (-91.0, 0.0, LATITUDE),
            (0.0, f64::NAN, LONGITUDE),
            (0.0, f64::NEG_INFINITY, LONGITUDE),
            (0.0, 180.0001, LONGITUDE),
            (0.0, -181.0, LONGITUDE),
        ];
        for (latitude, longitude, reason) in cases {
            let error = Coordinates::new(latitude, longitude).unwrap_err();
            assert_eq!(error.kind(), ValueKind::Coordinates);
            assert_eq!(error.reason(), reason, "{latitude},{longitude}");
        }
        assert_eq!(
            Coordinates::new(91.0, 0.0).unwrap_err().to_string(),
            "invalid coordinates \"91,0\": latitude must be a finite number between -90 and 90"
        );
    }

    #[test]
    fn parses_with_optional_space_after_the_comma() {
        let expected = Coordinates::new(39.7456, -97.0892).unwrap();
        assert_eq!("39.7456,-97.0892".parse::<Coordinates>().unwrap(), expected);
        assert_eq!(
            "39.7456, -97.0892".parse::<Coordinates>().unwrap(),
            expected
        );
        assert_eq!(
            "39.7456,   -97.0892".parse::<Coordinates>().unwrap(),
            expected
        );
        assert_eq!(
            "39.74561,-97.08919".parse::<Coordinates>().unwrap(),
            expected
        );
        assert_eq!(
            Coordinates::try_from("40,-105").unwrap().to_string(),
            "40,-105"
        );
        assert_eq!(
            Coordinates::try_from(String::from("40,-105"))
                .unwrap()
                .to_string(),
            "40,-105"
        );
    }

    #[test]
    fn rejects_malformed_and_out_of_range_text() {
        let cases = [
            ("", SHAPE),
            ("39.7456", SHAPE),
            ("39.7456;-97.0892", SHAPE),
            ("39.7456,", SHAPE),
            (",-97.0892", SHAPE),
            ("north,west", SHAPE),
            ("39.7456 ,-97.0892", SHAPE),
            (" 39.7456,-97.0892", SHAPE),
            ("39.7456,-97.0892 ", SHAPE),
            ("39.7456,-97.0892,1", SHAPE),
            ("nan,0", SHAPE),
            ("0,inf", SHAPE),
            ("3e1,0", SHAPE),
            ("+40,-105", SHAPE),
            ("040,0", SHAPE),
            ("0,-007", SHAPE),
            (".5,0", SHAPE),
            ("5.,0", SHAPE),
            ("91,0", LATITUDE),
            ("90.00001,0", LATITUDE),
            ("0,181", LONGITUDE),
        ];
        for (input, reason) in cases {
            let error = input.parse::<Coordinates>().unwrap_err();
            assert_eq!(error.kind(), ValueKind::Coordinates, "{input:?}");
            assert_eq!(error.input(), input);
            assert_eq!(error.reason(), reason, "{input:?}");
        }
    }

    #[test]
    fn display_and_serde_round_trip() {
        let point = Coordinates::new(39.7456, -97.0892).unwrap();
        assert_eq!(point.to_string().parse::<Coordinates>().unwrap(), point);
        assert_eq!(
            serde_json::to_string(&point).unwrap(),
            "\"39.7456,-97.0892\""
        );
        assert_eq!(
            serde_json::from_str::<Coordinates>("\"39.74561, -97.08919\"").unwrap(),
            point
        );
        assert_eq!(String::from(point), "39.7456,-97.0892");
        let error = serde_json::from_str::<Coordinates>("\"91,0\"").unwrap_err();
        assert!(error.to_string().contains("invalid coordinates"), "{error}");
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_is_a_string() {
        let schema = schemars::schema_for!(Coordinates);
        let value = schema.as_value();
        assert_eq!(value["type"], "string");
        assert!(value.get("pattern").is_some());
    }

    #[cfg(feature = "schemars")]
    #[test]
    fn schema_pattern_agrees_with_the_parser() {
        crate::ids::schema_tests::assert_pattern_matches_parser::<Coordinates>(
            &[
                "39.7456,-97.0892",
                "40,-105",
                "39.7456, -97.0892",
                "39.7456,\t-97.0892",
                "90,180",
                "-90,-180",
                "0,0",
                "-0,0",
                "0.5,0",
                "89.99999,179.99999",
                "90.000,180.0",
                "9,99.5",
            ],
            &[
                "",
                "3e1,0",
                "+40,-105",
                "040,0",
                "0,-007",
                ".5,0",
                "5.,0",
                "91,0",
                "0,181",
                "90.00001,0",
                "-90.0001,0",
                "0,180.5",
                "nan,0",
                "0,inf",
                "40 ,0",
                " 40,0",
                "40,0 ",
                "39.7456;-97.0892",
                "40,0,1",
                "40",
            ],
        );
    }
}
