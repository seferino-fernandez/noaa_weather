use super::{InvalidValue, ValueKind};

/// Two-letter prefixes NOAA uses for zone identifiers: US states,
/// territories, and marine areas. Sorted for binary search.
const PREFIXES: &[&[u8; 2]] = &[
    b"AK", b"AL", b"AM", b"AN", b"AR", b"AS", b"AZ", b"CA", b"CO", b"CT", b"DC", b"DE", b"FL",
    b"FM", b"GA", b"GM", b"GU", b"HI", b"IA", b"ID", b"IL", b"IN", b"KS", b"KY", b"LA", b"LC",
    b"LE", b"LH", b"LM", b"LO", b"LS", b"MA", b"MD", b"ME", b"MH", b"MI", b"MN", b"MO", b"MP",
    b"MS", b"MT", b"NC", b"ND", b"NE", b"NH", b"NJ", b"NM", b"NV", b"NY", b"OH", b"OK", b"OR",
    b"PA", b"PH", b"PK", b"PM", b"PR", b"PS", b"PW", b"PZ", b"RI", b"SC", b"SD", b"SL", b"TN",
    b"TX", b"UT", b"VA", b"VI", b"VT", b"WA", b"WI", b"WV", b"WY",
];

const REASON: &str =
    "must be a state or marine area code, C or Z, and three digits (for example COZ040)";

fn parse_zone(input: &str) -> Result<Box<str>, InvalidValue> {
    let reject = || InvalidValue::new(ValueKind::ZoneId, input, REASON);
    let bytes = input.as_bytes();
    let [state @ .., kind, d1, d2, d3] = bytes else {
        return Err(reject());
    };
    let [s1, s2] = state else {
        return Err(reject());
    };
    let prefix = [s1.to_ascii_uppercase(), s2.to_ascii_uppercase()];
    let kind = kind.to_ascii_uppercase();
    let shape_ok = PREFIXES.binary_search(&&prefix).is_ok()
        && matches!(kind, b'C' | b'Z')
        && [d1, d2, d3].iter().all(|digit| digit.is_ascii_digit());
    if !shape_ok {
        return Err(reject());
    }
    Ok(input.to_ascii_uppercase().into_boxed_str())
}

str_id! {
    /// A forecast zone or county identifier such as `COZ040` or `AZC013`.
    ///
    /// Used by `/zones/{type}/{zoneId}`, `/alerts/active/zone/{zoneId}`,
    /// and the county segment of `/radio`. This is the one identifier with a
    /// closed shape: a two-letter state, territory, or marine area code, then
    /// `C` (county) or `Z` (forecast zone), then three digits. Letters are
    /// uppercase-normalized.
    ///
    /// ```
    /// use noaa_weather_client::ZoneId;
    ///
    /// let zone: ZoneId = "azc013".parse()?;
    /// assert_eq!(zone.as_str(), "AZC013");
    /// assert_eq!(zone.state(), "AZ");
    /// assert!(zone.is_county());
    /// assert!("XXZ040".parse::<ZoneId>().is_err());
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    ZoneId, parse_zone,
    "Forecast zone or county identifier: a two-letter state or marine area code, C or Z, and three digits (for example COZ040).",
    concat!(
        "^([Aa][KkLlMmNnRrSsZz]|[Cc][AaOoTt]|[Dd][CcEe]|[Ff][LlMm]|[Gg][AaMmUu]|[Ii][AaDdLlNn]|",
        "[Kk][SsYy]|[Ll][AaCcEeHhMmOoSs]|[Mm][AaDdEeHhIiNnOoPpSsTt]|[Nn][CcDdEeHhJjMmVvYy]|",
        "[Oo][HhKkRr]|[Pp][AaHhKkMmRrSsWwZz]|[Ss][CcDdLl]|[Tt][NnXx]|[Uu][Tt]|[Vv][AaIiTt]|",
        "[Ww][AaIiVvYy]|[HhRr][Ii])[CcZz][0-9]{3}$"
    )
}

impl ZoneId {
    /// Returns the two-letter state, territory, or marine area code.
    #[must_use]
    pub fn state(&self) -> &str {
        &self.0[..2]
    }

    /// Returns whether this identifies a county (`C`).
    #[must_use]
    pub fn is_county(&self) -> bool {
        self.0.as_bytes()[2] == b'C'
    }

    /// Returns whether this identifies a forecast zone (`Z`).
    #[must_use]
    pub fn is_zone(&self) -> bool {
        self.0.as_bytes()[2] == b'Z'
    }

    /// Returns the three-digit zone or county number.
    #[must_use]
    pub fn number(&self) -> u16 {
        self.0[3..].parse().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes_are_sorted_for_binary_search() {
        assert!(PREFIXES.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(PREFIXES.len(), 74);
    }

    #[test]
    fn accepts_zones_and_counties_with_normalization() {
        let zone: ZoneId = "COZ040".parse().unwrap();
        assert_eq!(zone.as_str(), "COZ040");
        assert_eq!(zone.state(), "CO");
        assert!(zone.is_zone());
        assert!(!zone.is_county());
        assert_eq!(zone.number(), 40);

        let county: ZoneId = "azc013".parse().unwrap();
        assert_eq!(county.as_str(), "AZC013");
        assert_eq!(county.state(), "AZ");
        assert!(county.is_county());
        assert_eq!(county.number(), 13);

        let marine: ZoneId = "PZZ530".parse().unwrap();
        assert_eq!(marine.state(), "PZ");
        assert_eq!(marine.number(), 530);

        for input in ["HIZ001", "RIC005", "UTZ100", "amz300", "gmz001", "LSZ140"] {
            assert!(input.parse::<ZoneId>().is_ok(), "{input}");
        }
    }

    #[test]
    fn rejects_unknown_prefixes_and_bad_shapes() {
        for input in [
            "XXZ040", "COZ04", "COA040", "COZ0400", "", "COZ", "C0Z040", "COZ04A", "CO Z040",
            "COZ040 ", "ÇOZ040", "HRZ001", "UZZ001",
        ] {
            let error = input.parse::<ZoneId>().unwrap_err();
            assert_eq!(error.kind(), ValueKind::ZoneId, "{input:?}");
            assert_eq!(error.input(), input);
            assert_eq!(error.reason(), REASON);
        }
    }

    #[test]
    fn display_and_serde_round_trip() {
        let zone: ZoneId = "coz040".parse().unwrap();
        assert_eq!(zone.to_string(), "COZ040");
        assert_eq!(format!("{zone:?}"), "\"COZ040\"");
        assert_eq!(serde_json::to_string(&zone).unwrap(), "\"COZ040\"");
        assert_eq!(serde_json::from_str::<ZoneId>("\"coz040\"").unwrap(), zone);
        assert!(serde_json::from_str::<ZoneId>("\"XXZ040\"").is_err());
    }
}
