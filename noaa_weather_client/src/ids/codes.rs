//! Structurally validated ASCII identifiers.
//!
//! NOAA does not publish a closed list for most of these identifiers, so
//! validation is structural: the input must be ASCII of a known shape and
//! length. Letters are uppercase-normalized where NOAA treats codes as
//! case-insensitive and preserved where the server issues mixed-case ids.

use super::{InvalidValue, ValueKind};

/// Which ASCII characters an identifier admits.
#[derive(Clone, Copy)]
pub(super) enum Chars {
    /// ASCII letters and digits.
    Alnum,
    /// ASCII letters, digits, and `-`.
    AlnumHyphen,
    /// Printable ASCII other than space (`!` through `~`).
    Printable,
    /// ASCII letters, digits, and the base64 and URL-safe base64 symbols
    /// `+`, `/`, `=`, `_`, and `-`.
    Base64,
}

impl Chars {
    const fn admits(self, byte: u8) -> bool {
        match self {
            Self::Alnum => byte.is_ascii_alphanumeric(),
            Self::AlnumHyphen => byte.is_ascii_alphanumeric() || byte == b'-',
            Self::Printable => byte.is_ascii_graphic(),
            Self::Base64 => {
                byte.is_ascii_alphanumeric() || matches!(byte, b'+' | b'/' | b'=' | b'_' | b'-')
            }
        }
    }
}

/// How letter case is treated on input.
#[derive(Clone, Copy)]
pub(super) enum Case {
    /// ASCII letters are converted to uppercase.
    Upper,
    /// Input is kept exactly as given.
    Preserve,
}

/// The shape of one identifier family.
pub(super) struct Rule {
    pub(super) kind: ValueKind,
    pub(super) min: usize,
    pub(super) max: usize,
    pub(super) chars: Chars,
    pub(super) case: Case,
    pub(super) reason: &'static str,
}

impl Rule {
    /// Validates `input` against this rule and returns the normalized form.
    pub(super) fn parse(&self, input: &str) -> Result<Box<str>, InvalidValue> {
        let length = input.len();
        let shape_ok = (self.min..=self.max).contains(&length)
            && input.bytes().all(|byte| self.chars.admits(byte));
        if !shape_ok {
            return Err(InvalidValue::new(self.kind, input, self.reason));
        }
        Ok(match self.case {
            Case::Upper => input.to_ascii_uppercase().into_boxed_str(),
            Case::Preserve => Box::from(input),
        })
    }
}

const STATION: Rule = Rule {
    kind: ValueKind::StationId,
    min: 3,
    max: 16,
    chars: Chars::Alnum,
    case: Case::Upper,
    reason: "must be 3 to 16 ASCII letters or digits",
};

const CWSU: Rule = Rule {
    kind: ValueKind::CwsuId,
    min: 3,
    max: 4,
    chars: Chars::Alnum,
    case: Case::Upper,
    reason: "must be 3 to 4 ASCII letters or digits",
};

// NOAA's `ATSUIdentifier` schema is `^[A-Z]{3,4}$`. The length half of that
// is a floor this rule has to meet: `/aviation/sigmets` returns 3-character
// units such as `HNL` in roughly one feature in ten, and
// `GET /aviation/sigmets/HNL` answers 200, so a 4-only rule rejected
// identifiers NOAA had just handed us.
//
// The character-class half is deliberately *not* mirrored. `[A-Z]` says NOAA
// only issues letters, not that a client must refuse anything else, and the
// two failure modes are not symmetric: accepting a digit costs one round trip
// and NOAA's own refusal, while rejecting one costs a document the caller
// cannot fetch at all until this crate ships again. Too narrow is the bug
// this rule was just fixed for.
const ATSU: Rule = Rule {
    kind: ValueKind::AtsuId,
    min: 3,
    max: 4,
    chars: Chars::Alnum,
    case: Case::Upper,
    reason: "must be 3 to 4 ASCII letters or digits",
};

const CALL_SIGN: Rule = Rule {
    kind: ValueKind::CallSign,
    min: 3,
    max: 8,
    chars: Chars::Alnum,
    case: Case::Upper,
    reason: "must be 3 to 8 ASCII letters or digits",
};

const PRODUCT_TYPE: Rule = Rule {
    kind: ValueKind::ProductTypeCode,
    min: 2,
    max: 3,
    chars: Chars::Alnum,
    case: Case::Upper,
    reason: "must be 2 to 3 ASCII letters or digits",
};

const RADAR_STATION: Rule = Rule {
    kind: ValueKind::RadarStationId,
    min: 4,
    max: 5,
    chars: Chars::Alnum,
    case: Case::Upper,
    reason: "must be 4 to 5 ASCII letters or digits",
};

const PRODUCT: Rule = Rule {
    kind: ValueKind::ProductId,
    min: 1,
    max: 64,
    chars: Chars::AlnumHyphen,
    case: Case::Preserve,
    reason: "must be 1 to 64 ASCII letters, digits, or hyphens",
};

const ALERT: Rule = Rule {
    kind: ValueKind::AlertId,
    min: 1,
    max: 256,
    chars: Chars::Printable,
    case: Case::Preserve,
    reason: "must be 1 to 256 printable ASCII characters with no whitespace",
};

const CURSOR: Rule = Rule {
    kind: ValueKind::Cursor,
    min: 1,
    max: 512,
    chars: Chars::Base64,
    case: Case::Preserve,
    reason: "must be 1 to 512 ASCII letters, digits, or the symbols + / = _ -",
};

fn parse_station(input: &str) -> Result<Box<str>, InvalidValue> {
    STATION.parse(input)
}

fn parse_cursor(input: &str) -> Result<Box<str>, InvalidValue> {
    CURSOR.parse(input)
}

fn parse_cwsu(input: &str) -> Result<Box<str>, InvalidValue> {
    CWSU.parse(input)
}

fn parse_atsu(input: &str) -> Result<Box<str>, InvalidValue> {
    ATSU.parse(input)
}

fn parse_call_sign(input: &str) -> Result<Box<str>, InvalidValue> {
    CALL_SIGN.parse(input)
}

fn parse_product_type(input: &str) -> Result<Box<str>, InvalidValue> {
    PRODUCT_TYPE.parse(input)
}

fn parse_radar_station(input: &str) -> Result<Box<str>, InvalidValue> {
    RADAR_STATION.parse(input)
}

fn parse_product(input: &str) -> Result<Box<str>, InvalidValue> {
    PRODUCT.parse(input)
}

fn parse_alert(input: &str) -> Result<Box<str>, InvalidValue> {
    ALERT.parse(input)
}

str_id! {
    /// An observation station identifier such as `KSLC` or `KDEN`.
    ///
    /// Used by `/stations/{stationId}` and its observation and TAF
    /// sub-resources. Accepts 3 to 16 ASCII letters or digits and
    /// uppercase-normalizes them.
    ///
    /// ```
    /// use noaa_weather_client::StationId;
    ///
    /// let station: StationId = "kslc".parse()?;
    /// assert_eq!(station.as_str(), "KSLC");
    /// assert!("K SLC".parse::<StationId>().is_err());
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    StationId, parse_station,
    "Observation station identifier, 3 to 16 ASCII letters or digits (for example KSLC).",
    "^[A-Za-z0-9]{3,16}$"
}

str_id! {
    /// A Center Weather Service Unit identifier such as `ZAB`.
    ///
    /// Used by `/aviation/cwsus/{cwsuId}`. Accepts 3 to 4 ASCII letters or
    /// digits and uppercase-normalizes them.
    ///
    /// ```
    /// use noaa_weather_client::CwsuId;
    ///
    /// assert_eq!("zab".parse::<CwsuId>()?.as_str(), "ZAB");
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    CwsuId, parse_cwsu,
    "Center Weather Service Unit identifier, 3 to 4 ASCII letters or digits (for example ZAB).",
    "^[A-Za-z0-9]{3,4}$"
}

str_id! {
    /// An Air Traffic Service Unit identifier such as `KKCI`.
    ///
    /// Used by `/aviation/sigmets/{atsu}`. Accepts 3 to 4 ASCII letters or
    /// digits and uppercase-normalizes them.
    ///
    /// NOAA's own identifiers are letters only — its published schema is
    /// `^[A-Z]{3,4}$` — but that describes what NOAA issues rather than what
    /// a client must refuse, so a digit is forwarded and left for NOAA to
    /// reject.
    ///
    /// ```
    /// use noaa_weather_client::AtsuId;
    ///
    /// assert_eq!("kkci".parse::<AtsuId>()?.as_str(), "KKCI");
    /// // Three characters too: NOAA issues SIGMETs from `HNL` and `ANC`.
    /// assert_eq!("hnl".parse::<AtsuId>()?.as_str(), "HNL");
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    AtsuId, parse_atsu,
    "Air Traffic Service Unit identifier, 3 to 4 ASCII letters or digits (for example KKCI or HNL).",
    "^[A-Za-z0-9]{3,4}$"
}

str_id! {
    /// A NOAA Weather Radio transmitter call sign such as `WXK27`.
    ///
    /// Used by `/radio/{callSign}`. Accepts 3 to 8 ASCII letters or digits
    /// and uppercase-normalizes them.
    ///
    /// ```
    /// use noaa_weather_client::CallSign;
    ///
    /// assert_eq!("wxk27".parse::<CallSign>()?.as_str(), "WXK27");
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    CallSign, parse_call_sign,
    "NOAA Weather Radio call sign, 3 to 8 ASCII letters or digits (for example WXK27).",
    "^[A-Za-z0-9]{3,8}$"
}

str_id! {
    /// A text product type code such as `AFD`, `ZFP`, or `RR3`.
    ///
    /// Used by `/products/types/{typeId}`. Accepts 2 to 3 ASCII letters or
    /// digits and uppercase-normalizes them. NOAA's live type catalog includes
    /// digit-bearing codes such as `FA0`, `RR3`, and `WS9`.
    ///
    /// ```
    /// use noaa_weather_client::ProductTypeCode;
    ///
    /// assert_eq!("afd".parse::<ProductTypeCode>()?.as_str(), "AFD");
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    ProductTypeCode, parse_product_type,
    "Text product type code, 2 to 3 ASCII letters or digits (for example AFD or RR3).",
    "^[A-Za-z0-9]{2,3}$"
}

str_id! {
    /// A radar station identifier such as `KABX` or the wind profiler
    /// `HWPA2`.
    ///
    /// Used by `/radar/stations/{stationId}`. NOAA's station list mixes
    /// 4-character NEXRAD and TDWR sites with 5-character profilers, so 4 to
    /// 5 ASCII letters or digits are accepted and uppercase-normalized.
    ///
    /// ```
    /// use noaa_weather_client::RadarStationId;
    ///
    /// assert_eq!("kabx".parse::<RadarStationId>()?.as_str(), "KABX");
    /// assert_eq!("hwpa2".parse::<RadarStationId>()?.as_str(), "HWPA2");
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    RadarStationId, parse_radar_station,
    "Radar station identifier, 4 to 5 ASCII letters or digits (for example KABX or HWPA2).",
    "^[A-Za-z0-9]{4,5}$"
}

str_id! {
    /// A server-issued text product identifier.
    ///
    /// Used by `/products/{productId}`. NOAA issues UUID-like ids such as
    /// `0b5e9b3a-1c2d-4e5f-8a9b-0c1d2e3f4a5b`; this accepts 1 to 64 ASCII
    /// letters, digits, or hyphens and preserves case.
    ///
    /// ```
    /// use noaa_weather_client::ProductId;
    ///
    /// let id: ProductId = "0b5e9b3a-1c2d-4e5f-8a9b-0c1d2e3f4a5b".parse()?;
    /// assert_eq!(id.as_str(), "0b5e9b3a-1c2d-4e5f-8a9b-0c1d2e3f4a5b");
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    ProductId, parse_product,
    "Text product identifier, 1 to 64 ASCII letters, digits, or hyphens.",
    "^[A-Za-z0-9-]{1,64}$"
}

str_id! {
    /// A server-issued alert identifier.
    ///
    /// Used by `/alerts/{id}`. NOAA issues URN-like ids such as
    /// `urn:oid:2.49.0.1.840.0.1234`; this accepts 1 to 256 printable ASCII
    /// characters with no whitespace and preserves case.
    ///
    /// ```
    /// use noaa_weather_client::AlertId;
    ///
    /// let id: AlertId = "urn:oid:2.49.0.1.840.0.1234".parse()?;
    /// assert_eq!(id.to_string(), "urn:oid:2.49.0.1.840.0.1234");
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    AlertId, parse_alert,
    "Alert identifier, 1 to 256 printable ASCII characters with no whitespace (URN-like).",
    "^[!-~]{1,256}$"
}

str_id! {
    /// An opaque pagination cursor issued by NOAA.
    ///
    /// Used by the `cursor` query parameter of `/alerts`, `/stations`,
    /// `/stations/{id}/observations`, `/zones/forecast/{id}/stations`, and
    /// `/radio`. NOAA issues base64 tokens such as `eyJzIjo1MDB9`, sometimes
    /// with `=` padding; this accepts 1 to 512 ASCII letters, digits, or the
    /// symbols `+ / = _ -` and preserves the text exactly. Obtain one from
    /// [`FeatureCollection::next_cursor`](crate::geo::FeatureCollection::next_cursor)
    /// rather than constructing it by hand.
    ///
    /// ```
    /// use noaa_weather_client::Cursor;
    ///
    /// let cursor: Cursor = "eyJzIjo1MDB9".parse()?;
    /// assert_eq!(cursor.as_str(), "eyJzIjo1MDB9");
    /// assert!("has space".parse::<Cursor>().is_err());
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    Cursor, parse_cursor,
    "Opaque NOAA pagination cursor, 1 to 512 ASCII letters, digits, or the symbols + / = _ - (base64).",
    "^[A-Za-z0-9+/=_-]{1,512}$"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rejected<T: std::str::FromStr<Err = InvalidValue>>(input: &str) -> InvalidValue {
        match input.parse::<T>() {
            Ok(_) => panic!("{input:?} should be rejected"),
            Err(error) => error,
        }
    }

    #[test]
    fn station_id_normalizes_and_bounds() {
        let station: StationId = "kslc".parse().unwrap();
        assert_eq!(station.as_str(), "KSLC");
        assert_eq!(station, "KSLC".parse().unwrap());
        assert_eq!(format!("{station:?}"), "\"KSLC\"");
        assert_eq!(station.to_string(), "KSLC");
        assert_eq!(StationId::try_from("KSLC").unwrap(), station);
        assert_eq!(StationId::try_from(String::from("KSLC")).unwrap(), station);
        assert_eq!(String::from(station.clone()), "KSLC");
        assert_eq!(station.as_ref(), "KSLC");
        assert!("ABCDEFGHIJKLMNOP".parse::<StationId>().is_ok());
    }

    #[test]
    fn station_id_rejections_name_the_rule() {
        for input in [
            "",
            "KS",
            "ABCDEFGHIJKLMNOPQ",
            "kslc!",
            "K SLC",
            " KSLC",
            "KSLÇ",
            "KS-LC",
        ] {
            let error = rejected::<StationId>(input);
            assert_eq!(error.kind(), ValueKind::StationId, "{input:?}");
            assert_eq!(error.input(), input);
            assert_eq!(error.reason(), "must be 3 to 16 ASCII letters or digits");
        }
        assert_eq!(
            rejected::<StationId>("kslc!").to_string(),
            "invalid station id \"kslc!\": must be 3 to 16 ASCII letters or digits"
        );
    }

    #[test]
    fn cwsu_id_accepts_three_or_four_characters() {
        assert_eq!("zab".parse::<CwsuId>().unwrap().as_str(), "ZAB");
        assert_eq!("kzab".parse::<CwsuId>().unwrap().as_str(), "KZAB");
        assert_eq!(rejected::<CwsuId>("ZA").kind(), ValueKind::CwsuId);
        assert_eq!(rejected::<CwsuId>("ZABCD").kind(), ValueKind::CwsuId);
        assert_eq!(rejected::<CwsuId>("Z-B").kind(), ValueKind::CwsuId);
    }

    /// Both bounds, because moving the floor from 4 to 3 is only safe if
    /// something still holds it: `KK` has to stay rejected, or the rule
    /// stops being a rule.
    #[test]
    fn atsu_id_accepts_three_and_four_characters_and_nothing_else() {
        assert_eq!("kkci".parse::<AtsuId>().unwrap().as_str(), "KKCI");
        // NOAA issues SIGMETs from three-character units; `GET
        // /aviation/sigmets/HNL` answers 200.
        assert_eq!("hnl".parse::<AtsuId>().unwrap().as_str(), "HNL");
        // A digit is forwarded rather than refused here: NOAA's `^[A-Z]{3,4}$`
        // says what it issues, not what a client must reject, and refusing
        // costs more than one wasted round trip would.
        assert_eq!("k1c2".parse::<AtsuId>().unwrap().as_str(), "K1C2");
        assert_eq!(rejected::<AtsuId>("KK").kind(), ValueKind::AtsuId);
        assert_eq!(rejected::<AtsuId>("KKCII").kind(), ValueKind::AtsuId);
        assert_eq!(rejected::<AtsuId>("").kind(), ValueKind::AtsuId);
        assert_eq!(rejected::<AtsuId>("KK-I").kind(), ValueKind::AtsuId);
    }

    #[test]
    fn call_sign_bounds() {
        assert_eq!("wxk27".parse::<CallSign>().unwrap().as_str(), "WXK27");
        assert!("WXK".parse::<CallSign>().is_ok());
        assert!("WXK27ABC".parse::<CallSign>().is_ok());
        assert_eq!(rejected::<CallSign>("WX").kind(), ValueKind::CallSign);
        assert_eq!(
            rejected::<CallSign>("WXK27ABCD").kind(),
            ValueKind::CallSign
        );
        assert_eq!(rejected::<CallSign>("WXK 27").kind(), ValueKind::CallSign);
    }

    #[test]
    fn product_type_code_accepts_live_digit_bearing_codes() {
        assert_eq!("afd".parse::<ProductTypeCode>().unwrap().as_str(), "AFD");
        assert!("ZF".parse::<ProductTypeCode>().is_ok());
        assert_eq!("rr3".parse::<ProductTypeCode>().unwrap().as_str(), "RR3");
        assert_eq!(
            rejected::<ProductTypeCode>("A").kind(),
            ValueKind::ProductTypeCode
        );
        assert_eq!(
            rejected::<ProductTypeCode>("AFDX").kind(),
            ValueKind::ProductTypeCode
        );
        assert_eq!(
            rejected::<ProductTypeCode>("A-F").reason(),
            "must be 2 to 3 ASCII letters or digits"
        );
    }

    #[test]
    fn radar_station_id_accepts_four_or_five() {
        assert_eq!("kabx".parse::<RadarStationId>().unwrap().as_str(), "KABX");
        assert_eq!("hwpa2".parse::<RadarStationId>().unwrap().as_str(), "HWPA2");
        assert_eq!(
            rejected::<RadarStationId>("ABX").kind(),
            ValueKind::RadarStationId
        );
        assert_eq!(
            rejected::<RadarStationId>("KABXXX").kind(),
            ValueKind::RadarStationId
        );
    }

    #[test]
    fn product_id_preserves_case_and_allows_hyphens() {
        let id: ProductId = "0b5E9b3a-1c2d-4e5f-8a9b-0c1d2e3f4a5b".parse().unwrap();
        assert_eq!(id.as_str(), "0b5E9b3a-1c2d-4e5f-8a9b-0c1d2e3f4a5b");
        assert!("a".parse::<ProductId>().is_ok());
        assert!("a".repeat(64).parse::<ProductId>().is_ok());
        assert_eq!(rejected::<ProductId>("").kind(), ValueKind::ProductId);
        assert_eq!(
            rejected::<ProductId>(&"a".repeat(65)).kind(),
            ValueKind::ProductId
        );
        assert_eq!(
            rejected::<ProductId>("abc_def").kind(),
            ValueKind::ProductId
        );
        assert_eq!(
            rejected::<ProductId>("abc def").kind(),
            ValueKind::ProductId
        );
    }

    #[test]
    fn alert_id_accepts_urns_and_rejects_whitespace() {
        let id: AlertId = "urn:oid:2.49.0.1.840.0.1234".parse().unwrap();
        assert_eq!(id.as_str(), "urn:oid:2.49.0.1.840.0.1234");
        assert!("!".repeat(256).parse::<AlertId>().is_ok());
        assert_eq!(rejected::<AlertId>("").kind(), ValueKind::AlertId);
        assert_eq!(
            rejected::<AlertId>(&"!".repeat(257)).kind(),
            ValueKind::AlertId
        );
        assert_eq!(rejected::<AlertId>("urn:oid: 1").kind(), ValueKind::AlertId);
        assert_eq!(
            rejected::<AlertId>("urn:oid:1\n").kind(),
            ValueKind::AlertId
        );
        assert_eq!(rejected::<AlertId>("urn:oïd:1").kind(), ValueKind::AlertId);
    }

    #[test]
    fn cursor_preserves_base64_text_and_rejects_other_symbols() {
        let cursor: Cursor = "eyJ0IjoxNzU2Nzc0NzAwfQ==".parse().unwrap();
        assert_eq!(cursor.as_str(), "eyJ0IjoxNzU2Nzc0NzAwfQ==");
        assert_eq!(cursor.to_string(), "eyJ0IjoxNzU2Nzc0NzAwfQ==");
        assert!("a+b/c=d_e-f".parse::<Cursor>().is_ok());
        assert!("A".repeat(512).parse::<Cursor>().is_ok());
        assert_eq!(rejected::<Cursor>("").kind(), ValueKind::Cursor);
        assert_eq!(
            rejected::<Cursor>(&"A".repeat(513)).kind(),
            ValueKind::Cursor
        );
        for input in [
            "has space",
            "percent%3D",
            "quest?",
            "amp&",
            "tab\t",
            "ünïcode",
        ] {
            let error = rejected::<Cursor>(input);
            assert_eq!(error.kind(), ValueKind::Cursor, "{input:?}");
            assert_eq!(error.input(), input);
        }
        assert_eq!(
            rejected::<Cursor>("not valid!").to_string(),
            "invalid cursor \"not valid!\": must be 1 to 512 ASCII letters, digits, or the symbols + / = _ -"
        );
    }

    #[test]
    fn serde_round_trips_through_strings() {
        let station: StationId = "KSLC".parse().unwrap();
        let json = serde_json::to_string(&station).unwrap();
        assert_eq!(json, "\"KSLC\"");
        let parsed: StationId = serde_json::from_str("\"kslc\"").unwrap();
        assert_eq!(parsed, station);
        let error = serde_json::from_str::<StationId>("\"k!\"").unwrap_err();
        assert!(error.to_string().contains("invalid station id"), "{error}");

        let alert: AlertId = "urn:oid:2.49.0.1.840.0.1234".parse().unwrap();
        let json = serde_json::to_string(&alert).unwrap();
        assert_eq!(serde_json::from_str::<AlertId>(&json).unwrap(), alert);
    }

    #[test]
    fn ordering_and_hashing_follow_the_normalized_string() {
        use std::collections::HashSet;
        let a: StationId = "kden".parse().unwrap();
        let b: StationId = "KSLC".parse().unwrap();
        assert!(a < b);
        let set: HashSet<StationId> = ["KDEN".parse().unwrap(), a.clone()].into_iter().collect();
        assert_eq!(set.len(), 1);
    }
}
