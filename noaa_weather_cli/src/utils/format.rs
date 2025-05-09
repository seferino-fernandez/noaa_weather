use anyhow::Result;
use jiff::Timestamp;
use jiff::tz::TimeZone;
use std::fs::File;
use std::io::Write;

use crate::utils::temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius};

/// Write output to either stdout or a file
pub fn write_output(output_path: Option<&str>, content: &str) -> Result<()> {
    if let Some(path) = output_path {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
    } else {
        println!("{}", content);
    }
    Ok(())
}

pub fn get_zone_from_url(url: Option<String>) -> Option<String> {
    if let Some(url) = url {
        let parts: Vec<&str> = url.split('/').collect();
        let last_part = parts.last().map_or("N/A", |v| v);
        Some(last_part.to_string())
    } else {
        None
    }
}

pub fn format_datetime_human_readable(datetime_str_opt: Option<&str>) -> String {
    if datetime_str_opt.is_none() {
        return "N/A".to_string();
    }
    let timestamp = datetime_str_opt.unwrap().parse::<Timestamp>().unwrap();
    let user_timezone = TimeZone::try_system().unwrap_or(TimeZone::UTC);
    let zoned_timestamp = timestamp.to_zoned(user_timezone);
    return zoned_timestamp.strftime("%D %r").to_string();
}

// Unit strings used in the API.
const WMO_UNIT_DEGC: &str = "wmoUnit:degC";
const WMO_UNIT_DEGF: &str = "wmoUnit:degF";

// User-requested target units.
pub const FAHRENHEIT: &str = "fahrenheit";
pub const CELSIUS: &str = "celsius";

// Display strings for the temperature units.
const DEG_C_DISPLAY: &str = "°C";
const DEG_F_DISPLAY: &str = "°F";

/// Formats a dewpoint temperature value based on its original unit and a user's target unit preference.
///
/// This function handles parsing the dewpoint value, identifying its initial unit,
/// performing a temperature scale conversion if a target unit is specified and different,
/// rounding the final value to the nearest integer, and formatting the output string
/// with the rounded value and the appropriate unit symbol.
///
/// # Arguments
///
/// * `dewpoint_str_value` - A `String` containing the numerical value of the dewpoint temperature.
/// * `api_unit_str_opt` - An `Option<&str>` indicating the unit of the `dewpoint_str_value`
///   as provided by an API (e.g., "wmoUnit:degC"). If `None`, an error string is returned.
/// * `user_target_unit_opt` - An `Option<&str>` indicating the user's desired output unit
///   (e.g., "fahrenheit", "celsius"). If `None`, the original unit is used, and no
///   conversion takes place, but the value is still rounded.
///
/// # Returns
///
/// A `String` ready for display, containing the formatted dewpoint temperature
/// (e.g., "46 °F" or "8 °C"), or an error message if the input is invalid or incomplete.
pub fn format_dewpoint(
    dewpoint_str_value: String,
    api_unit_str_opt: Option<&str>,
    user_target_unit_opt: Option<&str>,
) -> String {
    // Ensure the API unit string is provided.
    let api_unit_actual_str = match api_unit_str_opt {
        Some(unit_str) => unit_str,
        None => return "N/A".to_string(),
    };

    // Attempt to parse the dewpoint value string into a floating-point number.
    let initial_dewpoint_val_f64 = match dewpoint_str_value.parse::<f64>() {
        Ok(val) => val,
        Err(_) => return "N/A".to_string(),
    };

    // Determine the initial unit type.
    let initial_unit_is_celsius = api_unit_actual_str == WMO_UNIT_DEGC;
    let initial_unit_is_fahrenheit = api_unit_actual_str == WMO_UNIT_DEGF;

    // Return an error if the provided API unit is not supported.
    if !initial_unit_is_celsius && !initial_unit_is_fahrenheit {
        return "N/A".to_string();
    }

    let mut final_dewpoint_val_f64 = initial_dewpoint_val_f64;
    let mut display_unit_str = "";

    // Determine the target unit and perform conversion if necessary.
    match user_target_unit_opt {
        Some(target_unit_str) => {
            if initial_unit_is_celsius && target_unit_str == FAHRENHEIT {
                // Convert Celsius to Fahrenheit
                final_dewpoint_val_f64 = celsius_to_fahrenheit(initial_dewpoint_val_f64);
                display_unit_str = DEG_F_DISPLAY;
            } else if initial_unit_is_fahrenheit && target_unit_str == CELSIUS {
                // Convert Fahrenheit to Celsius
                final_dewpoint_val_f64 = fahrenheit_to_celsius(initial_dewpoint_val_f64);
                display_unit_str = DEG_C_DISPLAY;
            } else {
                // If the target unit is the same as the initial, or an unsupported target
                // was provided, use the initial value and determine the display unit
                // based on the initial unit.
                if initial_unit_is_celsius {
                    display_unit_str = DEG_C_DISPLAY;
                } else if initial_unit_is_fahrenheit {
                    display_unit_str = DEG_F_DISPLAY;
                }
            }
        }
        None => {
            // If no target unit is specified, use the initial value and unit.
            final_dewpoint_val_f64 = initial_dewpoint_val_f64; // No conversion needed
            if initial_unit_is_celsius {
                display_unit_str = DEG_C_DISPLAY;
            } else if initial_unit_is_fahrenheit {
                display_unit_str = DEG_F_DISPLAY;
            }
        }
    }

    // Round the final temperature value to the nearest integer as required by examples.
    let rounded_final_value = final_dewpoint_val_f64.round();

    // Format the rounded value with the determined unit symbol.
    format!("{} {}", rounded_final_value, display_unit_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_zone_from_url_valid() {
        let url = Some("https://api.weather.gov/zones/forecast/NYZ072".to_string());
        assert_eq!(get_zone_from_url(url), Some("NYZ072".to_string()));

        let url2 = Some("https://api.weather.gov/zones/county/NYC061".to_string());
        assert_eq!(get_zone_from_url(url2), Some("NYC061".to_string()));
    }

    #[test]
    fn test_get_zone_from_url_none() {
        assert_eq!(get_zone_from_url(None), None);
    }

    #[test]
    fn test_get_zone_from_url_empty_string() {
        // An empty string URL technically doesn't split, last() gives None -> "N/A"
        let url = Some("".to_string());
        assert_eq!(get_zone_from_url(url), Some("".to_string()));
    }

    #[test]
    fn test_get_zone_from_url_no_slashes() {
        let url = Some("justafile".to_string());
        assert_eq!(get_zone_from_url(url), Some("justafile".to_string()));
    }

    #[test]
    fn test_get_zone_from_url_trailing_slash() {
        let url = Some("https://api.weather.gov/zones/forecast/XYZ123/".to_string());
        assert_eq!(get_zone_from_url(url), Some("".to_string()));
    }
}
