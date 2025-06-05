use anyhow::{Result, anyhow};
use noaa_weather_client::models::{
    AreaCode, LandRegionCode, MarineAreaCode, MarineRegionCode, RegionCode, StateTerritoryCode,
};
use std::fmt::Display;
use std::str::FromStr;

/// Generic arg parser that converts a Vec<String> into a Vec of T: FromStr
pub fn parse_string_args_into_vec<T: FromStr + Send + Sync + 'static>(
    items: Option<Vec<String>>,
) -> anyhow::Result<Option<Vec<T>>>
where
    <T as FromStr>::Err: Display + Send + Sync + 'static,
{
    items
        .map(|items| {
            items
                .into_iter()
                .map(|item| {
                    T::from_str(&item).map_err(|e| anyhow!("Invalid input '{}': {}", item, e))
                })
                .collect::<anyhow::Result<Vec<T>>>()
        })
        .transpose()
}

// Parses RegionCode which can be LandRegionCode or MarineRegionCode
pub fn parse_region_codes(codes: Option<Vec<String>>) -> Result<Option<Vec<RegionCode>>> {
    codes
        .map(|strs| {
            strs.into_iter()
                .map(|c| {
                    LandRegionCode::from_str(&c)
                        .map(RegionCode::LandRegionCode)
                        .or_else(|_| {
                            MarineRegionCode::from_str(&c).map(RegionCode::MarineRegionCode)
                        })
                        .map_err(|_| anyhow!("Invalid region code: {}", c))
                })
                .collect::<Result<Vec<_>>>()
        })
        .transpose()
}

// Parses AreaCode which can be StateTerritoryCode or MarineAreaCode
pub fn parse_area_codes(codes: Option<Vec<String>>) -> Result<Option<Vec<AreaCode>>> {
    codes
        .map(|strs| {
            strs.into_iter()
                .map(|c| {
                    StateTerritoryCode::from_str(&c)
                        .map(AreaCode::StateTerritoryCode)
                        .or_else(|_| MarineAreaCode::from_str(&c).map(AreaCode::MarineAreaCode))
                        .map_err(|_| anyhow!("Invalid area code: {}", c))
                })
                .collect::<Result<Vec<_>>>()
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use noaa_weather_client::models::{
        MarineAreaCode, MarineRegionCode, RegionCode, StateTerritoryCode,
    };
    use std::num::ParseIntError;

    // Helper struct for testing parse_string_args_into_vec
    #[derive(Debug, PartialEq)]
    struct MyInt(i32);

    impl FromStr for MyInt {
        type Err = ParseIntError;
        fn from_str(input_string: &str) -> Result<Self, Self::Err> {
            input_string.parse::<i32>().map(MyInt)
        }
    }

    #[test]
    fn test_parse_string_args_into_vec_none() {
        let result = parse_string_args_into_vec::<MyInt>(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_string_args_into_vec_empty() {
        let input: Option<Vec<String>> = Some(vec![]);
        let result = parse_string_args_into_vec::<MyInt>(input);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert!(unwrapped.is_some());
        assert!(unwrapped.unwrap().is_empty());
    }

    #[test]
    fn test_parse_string_args_into_vec_valid() {
        let input = Some(vec!["1".to_string(), "-10".to_string(), "0".to_string()]);
        let expected = Some(vec![MyInt(1), MyInt(-10), MyInt(0)]);
        let result = parse_string_args_into_vec::<MyInt>(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_string_args_into_vec_invalid() {
        let input = Some(vec!["1".to_string(), "abc".to_string(), "0".to_string()]);
        let result = parse_string_args_into_vec::<MyInt>(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid input 'abc'")
        );
    }

    #[test]
    fn test_parse_region_codes_none() {
        let result = parse_region_codes(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_region_codes_empty() {
        let input: Option<Vec<String>> = Some(vec![]);
        let result = parse_region_codes(input);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert!(unwrapped.is_some());
        assert!(unwrapped.unwrap().is_empty());
    }

    #[test]
    fn test_parse_region_codes_valid() {
        let input = Some(vec!["AL".to_string(), "GM".to_string(), "PA".to_string()]);
        let expected = Some(vec![
            RegionCode::MarineRegionCode(MarineRegionCode::Al),
            RegionCode::MarineRegionCode(MarineRegionCode::Gm),
            RegionCode::MarineRegionCode(MarineRegionCode::Pa),
        ]);
        let result = parse_region_codes(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_region_codes_invalid() {
        let input = Some(vec!["AL".to_string(), "XX".to_string(), "PA".to_string()]);
        let result = parse_region_codes(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid region code: XX")
        );
    }

    #[test]
    fn test_parse_area_codes_none() {
        let result = parse_area_codes(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_parse_area_codes_empty() {
        let input: Option<Vec<String>> = Some(vec![]);
        let result = parse_area_codes(input);
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert!(unwrapped.is_some());
        assert!(unwrapped.unwrap().is_empty());
    }

    #[test]
    fn test_parse_area_codes_valid() {
        let input = Some(vec!["CA".to_string(), "GM".to_string(), "NY".to_string()]);
        let expected = Some(vec![
            AreaCode::StateTerritoryCode(StateTerritoryCode::Ca),
            AreaCode::MarineAreaCode(MarineAreaCode::Gm),
            AreaCode::StateTerritoryCode(StateTerritoryCode::Ny),
        ]);
        let result = parse_area_codes(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_area_codes_invalid() {
        let input = Some(vec![
            "CA".to_string(),
            "INVALID".to_string(),
            "NY".to_string(),
        ]);
        let result = parse_area_codes(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid area code: INVALID")
        );
    }
}
