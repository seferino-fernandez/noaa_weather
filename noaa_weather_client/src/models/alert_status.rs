use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AlertStatus {
    #[serde(rename = "Actual")]
    Actual,
    #[serde(rename = "Exercise")]
    Exercise,
    #[serde(rename = "System")]
    System,
    #[serde(rename = "Test")]
    Test,
    #[serde(rename = "Draft")]
    Draft,
}

impl std::fmt::Display for AlertStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Actual => write!(f, "actual"),
            Self::Exercise => write!(f, "exercise"),
            Self::System => write!(f, "system"),
            Self::Test => write!(f, "test"),
            Self::Draft => write!(f, "draft"),
        }
    }
}

impl FromStr for AlertStatus {
    type Err = String;

    /// Parse a string into an [`AlertStatus`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use noaa_weather_client::models::AlertStatus;
    ///
    /// let status = AlertStatus::from_str("actual").unwrap();
    /// assert_eq!(status, AlertStatus::Actual);
    /// ```
    ///
    /// ```
    /// use std::str::FromStr;
    /// use noaa_weather_client::models::AlertStatus;
    ///
    /// let status = AlertStatus::from_str("ACTUAL").unwrap();
    /// assert_eq!(status, AlertStatus::Actual);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid alert status.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "actual" => Ok(Self::Actual),
            "exercise" => Ok(Self::Exercise),
            "system" => Ok(Self::System),
            "test" => Ok(Self::Test),
            "draft" => Ok(Self::Draft),
            _ => Err(format!("Invalid alert status: {s}")),
        }
    }
}
