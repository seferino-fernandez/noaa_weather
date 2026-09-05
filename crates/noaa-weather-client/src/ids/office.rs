use super::codes::{Case, Chars, Rule};
use super::{InvalidValue, ValueKind};
use crate::models::{NwsForecastOfficeId, NwsOfficeId};

const OFFICE: Rule = Rule {
    kind: ValueKind::OfficeId,
    min: 3,
    max: 4,
    chars: Chars::Alnum,
    case: Case::Upper,
    reason: "must be 3 to 4 ASCII letters or digits",
};

fn parse_office(input: &str) -> Result<Box<str>, InvalidValue> {
    OFFICE.parse(input)
}

str_id! {
    /// A forecast office or product location code such as `TOP` or `BOU`.
    ///
    /// One type serves `/offices/{officeId}`, the office segment of
    /// `/gridpoints/{wfo}/{x},{y}`, and product locations under
    /// `/products/locations/{locationId}`. Validation is structural (3 to 4
    /// ASCII letters or digits, uppercase-normalized) because product
    /// locations include national centers that are not forecast offices.
    /// [`OfficeId::KNOWN`] lists the forecast offices for completion hints.
    ///
    /// ```
    /// use noaa_weather_client::OfficeId;
    ///
    /// let office: OfficeId = "top".parse()?;
    /// assert_eq!(office.as_str(), "TOP");
    /// assert!(OfficeId::KNOWN.contains(&"TOP"));
    /// # Ok::<(), noaa_weather_client::InvalidValue>(())
    /// ```
    OfficeId, parse_office,
    "Forecast office or product location code, 3 to 4 ASCII letters or digits (for example TOP).",
    "^[A-Za-z0-9]{3,4}$"
}

impl OfficeId {
    /// The NWS forecast office codes, sorted, for completion hints.
    ///
    /// This is a hint and not a restriction: [`OfficeId`] accepts any
    /// structurally valid code so product locations outside this list work.
    pub const KNOWN: &'static [&'static str] = &[
        "ABQ", "ABR", "AER", "AFC", "AFG", "AJK", "AKQ", "ALU", "ALY", "AMA", "APX", "ARX", "BGM",
        "BIS", "BMX", "BOI", "BOU", "BOX", "BRO", "BTV", "BUF", "BYZ", "CAE", "CAR", "CHS", "CLE",
        "CRP", "CTP", "CYS", "DDC", "DLH", "DMX", "DTX", "DVN", "EAX", "EKA", "EPZ", "EWX", "FFC",
        "FGF", "FGZ", "FSD", "FWD", "GGW", "GID", "GJT", "GLD", "GRB", "GRR", "GSP", "GUM", "GYX",
        "HFO", "HGX", "HNX", "HPA", "HUN", "ICT", "ILM", "ILN", "ILX", "IND", "IWX", "JAN", "JAX",
        "JKL", "KEY", "LBF", "LCH", "LIX", "LKN", "LMK", "LOT", "LOX", "LSX", "LUB", "LWX", "LZK",
        "MAF", "MEG", "MFL", "MFR", "MHX", "MKX", "MLB", "MOB", "MPX", "MQT", "MRX", "MSO", "MTR",
        "NH1", "NH2", "OAX", "OHX", "OKX", "ONA", "ONP", "OTX", "OUN", "PAH", "PBZ", "PDT", "PHI",
        "PIH", "PPG", "PQE", "PQR", "PQW", "PSR", "PUB", "RAH", "REV", "RIW", "RLX", "RNK", "SEW",
        "SGF", "SGX", "SHV", "SJT", "SJU", "SLC", "STO", "STU", "TAE", "TBW", "TFX", "TOP", "TSA",
        "TWC", "UNR", "VEF",
    ];

    /// Returns whether this code is one of the NWS forecast offices in
    /// [`OfficeId::KNOWN`].
    #[must_use]
    pub fn is_known(&self) -> bool {
        Self::KNOWN.binary_search(&self.as_str()).is_ok()
    }
}

impl From<NwsForecastOfficeId> for OfficeId {
    fn from(office: NwsForecastOfficeId) -> Self {
        Self(office.to_string().into_boxed_str())
    }
}

impl From<&NwsForecastOfficeId> for OfficeId {
    fn from(office: &NwsForecastOfficeId) -> Self {
        Self::from(*office)
    }
}

impl From<NwsOfficeId> for OfficeId {
    fn from(office: NwsOfficeId) -> Self {
        Self(office.to_string().into_boxed_str())
    }
}

impl From<&NwsOfficeId> for OfficeId {
    fn from(office: &NwsOfficeId) -> Self {
        Self::from(*office)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_normalizes() {
        let office: OfficeId = "top".parse().unwrap();
        assert_eq!(office.as_str(), "TOP");
        assert_eq!(office.to_string(), "TOP");
        assert!(office.is_known());
        assert!("KWNS".parse::<OfficeId>().is_ok());
        assert!(!"KWNS".parse::<OfficeId>().unwrap().is_known());
    }

    #[test]
    fn rejects_out_of_shape_input() {
        for input in ["", "TO", "TOPEK", "T0P!", "TO P", "TÖP"] {
            let error = input.parse::<OfficeId>().unwrap_err();
            assert_eq!(error.kind(), ValueKind::OfficeId, "{input:?}");
            assert_eq!(error.reason(), "must be 3 to 4 ASCII letters or digits");
        }
    }

    #[test]
    fn known_is_sorted_unique_and_valid() {
        assert!(OfficeId::KNOWN.windows(2).all(|pair| pair[0] < pair[1]));
        for code in OfficeId::KNOWN {
            let office: OfficeId = code.parse().unwrap();
            assert_eq!(office.as_str(), *code);
            assert!(office.is_known());
        }
    }

    /// Every variant of the forecast office enum, read from its source so the
    /// list here cannot drift from the model without this test noticing.
    fn enum_variants() -> Vec<NwsForecastOfficeId> {
        let source = include_str!("../models/nws_forecast_office_id.rs");
        let codes: Vec<serde_json::Value> = source
            .lines()
            .filter_map(|line| line.trim().strip_prefix("#[serde(rename = \""))
            .filter_map(|rest| rest.strip_suffix("\")]"))
            .map(|code| serde_json::Value::String(code.to_owned()))
            .collect();
        assert!(
            codes.len() > 100,
            "enum source parse found {} codes",
            codes.len()
        );
        serde_json::from_value(serde_json::Value::Array(codes)).unwrap()
    }

    #[test]
    fn known_matches_the_forecast_office_enum_in_both_directions() {
        let variants = enum_variants();
        assert_eq!(OfficeId::KNOWN.len(), variants.len());
        for variant in &variants {
            let office = OfficeId::from(variant);
            assert!(office.is_known(), "{office} is a variant but not in KNOWN");
            assert!(OfficeId::KNOWN.contains(&office.as_str()));
        }
        for code in OfficeId::KNOWN {
            let variant: NwsForecastOfficeId =
                serde_json::from_value(serde_json::Value::String((*code).to_owned()))
                    .unwrap_or_else(|_| panic!("{code} is in KNOWN but not a variant"));
            assert_eq!(variant.to_string(), *code);
        }
    }

    #[test]
    fn converts_from_the_forecast_office_enum() {
        let office = OfficeId::from(NwsForecastOfficeId::Top);
        assert_eq!(office.as_str(), "TOP");
        assert_eq!(OfficeId::from(&NwsForecastOfficeId::Bou).as_str(), "BOU");
        assert!(office.is_known());
    }

    #[test]
    fn converts_from_any_office_enum() {
        let regional = NwsOfficeId::from(crate::models::NwsRegionalHqid::Wrh);
        assert_eq!(OfficeId::from(regional).as_str(), "WRH");
        assert_eq!(OfficeId::from(&regional).as_str(), "WRH");
        assert!(!OfficeId::from(regional).is_known());
    }

    #[test]
    fn serde_round_trip() {
        let office: OfficeId = "BOU".parse().unwrap();
        assert_eq!(serde_json::to_string(&office).unwrap(), "\"BOU\"");
        assert_eq!(serde_json::from_str::<OfficeId>("\"bou\"").unwrap(), office);
        assert!(serde_json::from_str::<OfficeId>("\"B\"").is_err());
    }
}
