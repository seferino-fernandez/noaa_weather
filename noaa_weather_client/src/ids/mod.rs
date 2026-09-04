//! Typed identifiers for NOAA Weather API resources.
//!
//! Every identifier NOAA puts in a URL path has a type here. Each type
//! validates its shape when constructed, so a bad station code or zone id is
//! reported as an [`InvalidValue`] before any request is made, and every
//! function that takes one can rely on it being well-formed.
//!
//! All identifiers share one surface:
//!
//! - `FromStr`, `TryFrom<&str>`, and `TryFrom<String>` validate and normalize.
//! - `Display`, `AsRef<str>`, and `as_str()` return the normalized text.
//! - serde reads and writes the plain string form.
//! - With the `schemars` feature, each type describes itself as a string
//!   schema with a pattern.
//!
//! Validation is structural: the input must be ASCII of the right length and
//! character set, and letters are uppercase-normalized where NOAA treats the
//! code as case-insensitive. Only [`ZoneId`] has a closed shape (state or
//! marine area prefix, `C` or `Z`, three digits). Server-issued opaque ids
//! such as headline or briefing ids stay plain strings; the one opaque
//! server-issued value with a type is the pagination [`Cursor`], because it
//! is fed back into a query and NOAA rejects malformed ones with HTTP 400.
//!
//! ```
//! use noaa_weather_client::{GridpointId, StationId, ZoneId};
//!
//! let station = "kslc".parse::<StationId>()?;
//! assert_eq!(station.as_str(), "KSLC");
//!
//! let zone = "coz040".parse::<ZoneId>()?;
//! assert_eq!(zone.to_string(), "COZ040");
//!
//! let grid = "TOP/31,80".parse::<GridpointId>()?;
//! assert_eq!(grid.x(), 31);
//!
//! let error = "kslc!".parse::<StationId>().unwrap_err();
//! assert_eq!(
//!     error.to_string(),
//!     "invalid station id \"kslc!\": must be 3 to 16 ASCII letters or digits"
//! );
//! # Ok::<(), noaa_weather_client::InvalidValue>(())
//! ```

mod codes;
mod gridpoint;
mod invalid;
mod office;
mod zone;

pub use codes::{
    AlertId, AtsuId, CallSign, Cursor, CwsuId, ProductId, ProductTypeCode, RadarStationId,
    StationId,
};
pub use gridpoint::GridpointId;
pub use invalid::{InvalidValue, ValueKind};
pub use office::OfficeId;
pub use zone::ZoneId;

#[cfg(all(test, feature = "schemars"))]
pub(crate) mod schema_tests {
    use std::str::FromStr;

    use super::*;

    fn pattern_of<T: schemars::JsonSchema>() -> regex::Regex {
        let schema = schemars::schema_for!(T);
        let pattern = schema.as_value()["pattern"]
            .as_str()
            .unwrap_or_else(|| panic!("{} publishes no pattern", T::schema_name()))
            .to_owned();
        regex::Regex::new(&pattern).unwrap_or_else(|error| {
            panic!("{} pattern is not a valid regex: {error}", T::schema_name())
        })
    }

    /// Asserts that the published schema pattern and the parser agree on
    /// every sample, and that the samples are split the way the caller says.
    pub(crate) fn assert_pattern_matches_parser<T>(accept: &[&str], reject: &[&str])
    where
        T: schemars::JsonSchema + FromStr,
    {
        let regex = pattern_of::<T>();
        let name = T::schema_name();
        for sample in accept {
            assert!(
                sample.parse::<T>().is_ok(),
                "{name}: parser rejected {sample:?}"
            );
        }
        for sample in reject {
            assert!(
                sample.parse::<T>().is_err(),
                "{name}: parser accepted {sample:?}"
            );
        }
        for sample in accept.iter().chain(reject) {
            assert_eq!(
                regex.is_match(sample),
                sample.parse::<T>().is_ok(),
                "{name}: pattern and parser disagree on {sample:?}"
            );
        }
    }

    #[test]
    fn identifier_patterns_agree_with_their_parsers() {
        let long_product = "a".repeat(64);
        let too_long_product = "a".repeat(65);
        let long_alert = "~".repeat(256);
        let too_long_alert = "!".repeat(257);
        let long_cursor = "A".repeat(512);
        let too_long_cursor = "A".repeat(513);

        assert_pattern_matches_parser::<StationId>(
            &["KSLC", "kslc", "ABC", "ABCDEFGHIJKLMNOP", "K1SL"],
            &[
                "",
                "KS",
                "ABCDEFGHIJKLMNOPQ",
                "kslc!",
                "K SLC",
                "KSLÇ",
                "KSLC\n",
                "KS-LC",
            ],
        );
        assert_pattern_matches_parser::<OfficeId>(
            &["TOP", "top", "KWNS", "T0P"],
            &["", "TO", "TOPEK", "T*P", "TO P", "TÖP"],
        );
        assert_pattern_matches_parser::<CwsuId>(
            &["ZAB", "zab", "KZAB"],
            &["", "ZA", "ZABCD", "Z-B"],
        );
        // `KKC` was in the reject list while the rule read "exactly 4".
        // NOAA's `ATSUIdentifier` is `^[A-Z]{3,4}$` and it issues SIGMETs
        // from ANC, FAI, HNL and JNU, so three characters have to be
        // accepted. `K1C2` stays accepted: the parser is deliberately one
        // notch wider than NOAA's character class, so a digit reaches NOAA
        // and is refused there rather than here.
        assert_pattern_matches_parser::<AtsuId>(
            &["KKCI", "kkci", "HNL", "hnl", "K1C2"],
            &["", "KK", "KKCII", "KK-I"],
        );
        assert_pattern_matches_parser::<CallSign>(
            &["WXK27", "wxk27", "WXK", "WXK27ABC"],
            &["", "WX", "WXK27ABCD", "WXK 27"],
        );
        assert_pattern_matches_parser::<ProductTypeCode>(
            &["AFD", "afd", "ZF", "RR3", "fa0"],
            &["", "A", "AFDX", "A-F"],
        );
        assert_pattern_matches_parser::<RadarStationId>(
            &["KABX", "kabx", "K1B2", "HWPA2", "tlka2"],
            &["", "ABX", "KABXXX", "KA-X", "HWPA 2"],
        );
        assert_pattern_matches_parser::<ProductId>(
            &[
                "0b5e9b3a-1c2d-4e5f-8a9b-0c1d2e3f4a5b",
                "a",
                &long_product,
                "ABC-123",
            ],
            &["", &too_long_product, "abc_def", "abc def", "abc.def"],
        );
        assert_pattern_matches_parser::<AlertId>(
            &["urn:oid:2.49.0.1.840.0.1234", "!", &long_alert, "a.b/c?d=e"],
            &[
                "",
                &too_long_alert,
                "urn:oid: 1",
                "urn:oid:1\n",
                "urn:oïd:1",
            ],
        );
        assert_pattern_matches_parser::<Cursor>(
            &[
                "eyJzIjo1MDB9",
                "eyJ0IjoxNzU2Nzc0NzAwfQ==",
                "a+b/c=d_e-f",
                "A",
                &long_cursor,
            ],
            &[
                "",
                &too_long_cursor,
                "has space",
                "percent%3D",
                "quest?",
                "amp&",
                "eyJzIjo1MDB9\n",
                "ünïcode",
            ],
        );
        assert_pattern_matches_parser::<ZoneId>(
            &[
                "COZ040", "azc013", "PZZ530", "HIZ001", "RIC005", "coz040", "Utz100",
            ],
            &[
                "", "XXZ040", "COZ04", "COA040", "COZ0400", "C0Z040", "HRZ001", "UZZ001", "CO Z040",
            ],
        );
        assert_pattern_matches_parser::<GridpointId>(
            &[
                "TOP/31,80",
                "top/0,65535",
                "TOP/00031,00080",
                "KWNS/1,1",
                "TOP/60000,65535",
            ],
            &[
                "",
                "TOP/31",
                "TOP/31,80/1",
                "TOP/-1,80",
                "TOP/31,65536",
                "TOP/+31,80",
                "TOP/31, 80",
                "TOP/000031,80",
                "TOP/31,100000",
                "TOP/99999,0",
                "T/31,80",
                "TOP31,80",
            ],
        );
    }

    fn assert_string_schema<T: schemars::JsonSchema>() {
        let schema = schemars::schema_for!(T);
        let value = schema.as_value();
        assert_eq!(
            value.get("type").and_then(serde_json::Value::as_str),
            Some("string"),
            "{}: {value}",
            T::schema_name()
        );
        assert!(value.get("description").is_some(), "{}", T::schema_name());
        assert!(value.get("pattern").is_some(), "{}", T::schema_name());
    }

    #[test]
    fn every_identifier_is_a_string_schema() {
        assert_string_schema::<StationId>();
        assert_string_schema::<OfficeId>();
        assert_string_schema::<CwsuId>();
        assert_string_schema::<AtsuId>();
        assert_string_schema::<CallSign>();
        assert_string_schema::<ProductId>();
        assert_string_schema::<ProductTypeCode>();
        assert_string_schema::<RadarStationId>();
        assert_string_schema::<AlertId>();
        assert_string_schema::<Cursor>();
        assert_string_schema::<ZoneId>();
        assert_string_schema::<GridpointId>();
    }

    #[test]
    fn identifiers_inline_into_containing_schemas() {
        #[derive(schemars::JsonSchema)]
        #[allow(dead_code)]
        struct Query {
            station: StationId,
            zone: Option<ZoneId>,
        }

        let schema = schemars::schema_for!(Query);
        let value = schema.as_value();
        assert!(value.get("$defs").is_none(), "{value}");
        assert_eq!(value["properties"]["station"]["type"], "string");
    }
}
