use crate::models;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Any office identifier accepted by the weather.gov `/offices` endpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NwsOfficeId {
    NwsForecastOfficeId(models::NwsForecastOfficeId),
    NwsRegionalHqid(models::NwsRegionalHqid),
    NwsNationalHqid(models::NwsNationalHqid),
}

impl fmt::Display for NwsOfficeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NwsForecastOfficeId(id) => id.fmt(formatter),
            Self::NwsRegionalHqid(id) => id.fmt(formatter),
            Self::NwsNationalHqid(id) => id.fmt(formatter),
        }
    }
}

impl From<models::NwsForecastOfficeId> for NwsOfficeId {
    fn from(id: models::NwsForecastOfficeId) -> Self {
        Self::NwsForecastOfficeId(id)
    }
}

impl From<models::NwsRegionalHqid> for NwsOfficeId {
    fn from(id: models::NwsRegionalHqid) -> Self {
        Self::NwsRegionalHqid(id)
    }
}

impl From<models::NwsNationalHqid> for NwsOfficeId {
    fn from(id: models::NwsNationalHqid) -> Self {
        Self::NwsNationalHqid(id)
    }
}

/// Error returned when a string is not a recognized NWS office identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseNwsOfficeIdError {
    invalid_value: String,
}

impl ParseNwsOfficeIdError {
    fn new(invalid_value: String) -> Self {
        Self { invalid_value }
    }

    /// Returns the value that could not be parsed.
    pub fn invalid_value(&self) -> &str {
        &self.invalid_value
    }
}

impl fmt::Display for ParseNwsOfficeIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "invalid NWS office ID `{}`; expected a forecast office, regional HQ, or national HQ identifier",
            self.invalid_value
        )
    }
}

impl std::error::Error for ParseNwsOfficeIdError {}

impl FromStr for NwsOfficeId {
    type Err = ParseNwsOfficeIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(id) = value.parse::<models::NwsForecastOfficeId>() {
            return Ok(id.into());
        }

        match value.to_ascii_uppercase().as_str() {
            "ARH" => Ok(models::NwsRegionalHqid::Arh.into()),
            "CRH" => Ok(models::NwsRegionalHqid::Crh.into()),
            "ERH" => Ok(models::NwsRegionalHqid::Erh.into()),
            "PRH" => Ok(models::NwsRegionalHqid::Prh.into()),
            "SRH" => Ok(models::NwsRegionalHqid::Srh.into()),
            "WRH" => Ok(models::NwsRegionalHqid::Wrh.into()),
            "NWS" => Ok(models::NwsNationalHqid::Nws.into()),
            _ => Err(ParseNwsOfficeIdError::new(value.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NwsOfficeId, ParseNwsOfficeIdError};
    use crate::models::{NwsForecastOfficeId, NwsNationalHqid, NwsRegionalHqid};

    #[test]
    fn parses_every_office_id_family_case_insensitively() {
        assert_eq!(
            "psr".parse::<NwsOfficeId>(),
            Ok(NwsOfficeId::NwsForecastOfficeId(NwsForecastOfficeId::Psr))
        );
        assert_eq!(
            "WrH".parse::<NwsOfficeId>(),
            Ok(NwsOfficeId::NwsRegionalHqid(NwsRegionalHqid::Wrh))
        );
        assert_eq!(
            "nws".parse::<NwsOfficeId>(),
            Ok(NwsOfficeId::NwsNationalHqid(NwsNationalHqid::Nws))
        );
    }

    #[test]
    fn displays_and_converts_every_office_id_family() {
        let forecast = NwsOfficeId::from(NwsForecastOfficeId::Psr);
        let regional = NwsOfficeId::from(NwsRegionalHqid::Wrh);
        let national = NwsOfficeId::from(NwsNationalHqid::Nws);

        assert_eq!(forecast.to_string(), "PSR");
        assert_eq!(regional.to_string(), "WRH");
        assert_eq!(national.to_string(), "NWS");
    }

    #[test]
    fn invalid_office_id_has_a_typed_useful_error() {
        let error = "not-an-office".parse::<NwsOfficeId>().unwrap_err();

        assert_eq!(error.invalid_value(), "not-an-office");
        assert_eq!(
            error.to_string(),
            "invalid NWS office ID `not-an-office`; expected a forecast office, regional HQ, or national HQ identifier"
        );
        assert_eq!(
            error,
            ParseNwsOfficeIdError::new("not-an-office".to_owned())
        );
    }

    #[test]
    fn preserves_untagged_serde_wire_values() {
        let office = NwsOfficeId::from(NwsRegionalHqid::Erh);
        assert_eq!(serde_json::to_string(&office).unwrap(), r#""ERH""#);
        assert_eq!(
            serde_json::from_str::<NwsOfficeId>(r#""NWS""#).unwrap(),
            NwsOfficeId::from(NwsNationalHqid::Nws)
        );
    }
}
