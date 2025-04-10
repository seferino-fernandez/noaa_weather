use std::fmt;
use std::str::FromStr;

/// Represents US states
#[derive(Debug, Clone, Copy)]
pub enum USState {
    Arizona,
}

impl USState {
    pub fn code(&self) -> &'static str {
        match self {
            USState::Arizona => "AZ",
        }
    }
}

impl FromStr for USState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AZ" | "ARIZONA" => Ok(USState::Arizona),
            _ => Err(format!("Unknown state: {}", s)),
        }
    }
}

impl fmt::Display for USState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Represents major US cities and their coordinates
#[derive(Debug, Clone, Copy)]
pub enum USCity {
    Phoenix(USState),
}

impl USCity {
    /// Returns the latitude and longitude for the city
    pub fn coordinates(&self) -> (f64, f64) {
        match self {
            USCity::Phoenix(_) => (33.4484, -112.0740),
        }
    }

    /// Returns the formatted coordinate string needed for the API
    pub fn coordinate_string(&self) -> String {
        let (lat, lon) = self.coordinates();
        format!("{},{}", lat, lon)
    }

    /// Returns the state of the city
    pub fn state(&self) -> USState {
        match self {
            USCity::Phoenix(state) => *state,
        }
    }

    /// Returns the name of the city without state
    pub fn name(&self) -> &'static str {
        match self {
            USCity::Phoenix(_) => "Phoenix",
        }
    }

    /// Try to create a city from name and state
    pub fn from_str(city: &str, state: USState) -> Result<Self, String> {
        match city.to_lowercase().as_str() {
            "phoenix" => {
                if matches!(state, USState::Arizona) {
                    Ok(USCity::Phoenix(state))
                } else {
                    Err(format!("{} is not in state {}", city, state))
                }
            }
            _ => Err(format!("Unknown city: {}", city)),
        }
    }
}

impl fmt::Display for USCity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name(), self.state())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_from_str() {
        assert!(matches!(USState::from_str("AZ"), Ok(USState::Arizona)));
        assert!(matches!(USState::from_str("ARIZONA"), Ok(USState::Arizona)));
        assert!(USState::from_str("XX").is_err());
    }

    #[test]
    fn test_city_from_str() {
        let state = USState::Arizona;
        assert!(matches!(
            USCity::from_str("Phoenix", state),
            Ok(USCity::Phoenix(_))
        ));
        assert!(USCity::from_str("Unknown", state).is_err());
    }

    #[test]
    fn test_city_display() {
        assert_eq!(USCity::Phoenix(USState::Arizona).to_string(), "Phoenix, AZ");
    }

    #[test]
    fn test_coordinates() {
        let (lat, lon) = USCity::Phoenix(USState::Arizona).coordinates();
        assert_eq!(lat, 33.4484);
        assert_eq!(lon, -112.0740);
    }

    #[test]
    fn test_coordinate_string() {
        assert_eq!(
            USCity::Phoenix(USState::Arizona).coordinate_string(),
            "33.4484,-112.074"
        );
    }
}
