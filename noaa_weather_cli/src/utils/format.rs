use anyhow::Result;
use jiff::Timestamp;
use jiff::tz::TimeZone;
use noaa_weather_client::models::{UnitCodeType, ValueUnit};
use std::fs::File;
use std::io::Write;

use crate::utils::temperature::{celsius_to_fahrenheit, fahrenheit_to_celsius};

/// Write output to either stdout or a file
pub fn write_output(output_path: Option<&str>, content: &str) -> Result<()> {
    if let Some(path) = output_path {
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
    } else {
        println!("{content}");
    }
    Ok(())
}

/// Extracts the part of a URL-like string after the last `/`.
///
/// This function will:
/// - Return `None` if the input `url_opt` is `None`.
/// - If `url_opt` is `Some(url_str)`:
///   - If `url_str` ends with `/` (e.g., "https://api.weather.gov/zones/forecast/AZZ551"), it returns `Some("".to_string())`.
///   - If `url_str` is empty (e.g., ""), it returns `Some("".to_string())`.
///   - If `url_str` has no `/` (e.g., "AZZ551"), it returns `Some("AZZ551".to_string())`.
///   - Otherwise, it returns the segment after the last `/` (e.g., for "https://api.weather.gov/zones/forecast/AZZ551", it returns `Some("AZZ551".to_string())`).
///
/// # Arguments
///
/// * `url_opt`: An `Option` containing a value that can be referenced as a string slice (`&str`). This allows for flexibility with input types like `String`, `&String`, or `&str`.
///
/// # Returns
///
/// An `Option<String>` containing the extracted segment, or `None` if the input `url_opt` was `None`.
/// The extracted segment can be an empty string if the original URL path ended with a `/` or was empty.
///
pub fn get_zone_from_url<S: AsRef<str>>(url_opt: Option<S>) -> Option<String> {
    url_opt.map(|s_ref| {
        let url_str = s_ref.as_ref();
        // `rsplit` splits the string by the delimiter starting from the right.
        // `next()` then gets the first item from this reversed iterator. This item is
        // the segment of the string after the last occurrence of `/`.
        // If `/` is not present, it returns the entire string.
        // If the string is empty, it returns an empty string.
        // If the string ends with `/`, it returns an empty string for the part after the last `/`.
        // `unwrap()` is safe here because `rsplit` on any `&str` (even an empty one)
        // always yields an iterator that produces at least one item (e.g., `""` for an empty input string).
        url_str.rsplit('/').next().unwrap().to_string()
    })
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

pub fn format_optional_number(number_opt: Option<i32>) -> String {
    match number_opt {
        Some(number) => number.to_string(),
        None => "N/A".to_string(),
    }
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
            final_dewpoint_val_f64 = initial_dewpoint_val_f64;
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
    format!("{rounded_final_value} {display_unit_str}")
}

/// Formats an `Option<String>` for display, using "N/A" if None.
pub fn format_optional_string(optional_string: &Option<String>) -> String {
    optional_string.as_deref().unwrap_or("N/A").to_string()
}

/// Formats an `Option<f64>` intended for build numbers or similar integers.
/// Displays as an integer if it has no fractional part, otherwise as float with 2 decimal places.
/// Uses "N/A" if None.
pub fn format_optional_f64_display(optional_float: &Option<f64>) -> String {
    optional_float.map_or("N/A".to_string(), |value| {
        if value.fract() == 0.0 {
            format!("{}", value.trunc())
        } else {
            format!("{value:.2}")
        }
    })
}

/// Formats an `Option<f64>` for precise display, typically converting to string directly.
/// Uses "N/A" if None.
pub fn format_optional_f64_precise(optional_float: &Option<f64>) -> String {
    optional_float.map_or("N/A".to_string(), |value| value.to_string())
}

/// Formats an `Option<i32>` for display, using "N/A" if None.
pub fn format_optional_i32(optional_integer: &Option<i32>) -> String {
    optional_integer.map_or("N/A".to_string(), |value| value.to_string())
}

/// Formats an `Option<i64>` for display, using "N/A" if None.
pub fn format_optional_i64(optional_integer: &Option<i64>) -> String {
    optional_integer.map_or("N/A".to_string(), |value| value.to_string())
}

/// Formats an `Option<ValueUnit>` for display.
/// Shows value with 2 decimal places and unit code. Uses "N/A" if None or parts are missing.
pub fn format_optional_value_unit(opt_value_unit: &Option<ValueUnit>) -> String {
    if opt_value_unit.is_none() {
        return "N/A".to_string();
    }
    let vu = opt_value_unit.as_ref().unwrap();
    if vu.value.is_none() {
        return "N/A".to_string();
    }
    let val_str = format!("{:.2}", vu.value.unwrap());
    let unit_str = opt_unit_code_val(&vu.unit_code);
    format!("{val_str} {unit_str}").trim().to_string()
}

/// Formats an `Option<UnitCodeType>` to its alternative label or "N/A".
pub fn opt_unit_code_val(unit_code_opt: &Option<UnitCodeType>) -> String {
    if unit_code_opt.is_none() {
        return "N/A".to_string();
    }
    let unit_code = unit_code_opt.as_ref().unwrap();
    match unit_code {
        UnitCodeType::Wmo(wmo_unit_code) => wmo_unit_code.alt_label().to_string(),
        UnitCodeType::Nws(nws_unit_code) => nws_unit_code.alt_label().to_string(),
    }
}

/// Formats an `Option<bool>` to "Yes", "No", or "N/A".
pub fn format_optional_bool_as_yes_no(optional_bool: &Option<bool>) -> String {
    match optional_bool {
        Some(true) => "Yes".to_string(),
        Some(false) => "No".to_string(),
        None => "N/A".to_string(),
    }
}

/// Formats byte sizes into human-readable strings (B, KiB, MiB, GiB).
pub fn format_bytes_to_human_readable(bytes_opt: Option<i64>) -> String {
    bytes_opt.map_or_else(
        || "N/A".to_string(),
        |bytes| {
            if bytes < 0 {
                return "Invalid (negative)".to_string();
            }
            if bytes < 1024 {
                format!("{bytes} B")
            } else {
                let kib = bytes as f64 / 1024.0;
                if kib < 1024.0 {
                    format!("{kib:.2} KiB")
                } else {
                    let mib = kib / 1024.0;
                    if mib < 1024.0 {
                        format!("{mib:.2} MiB")
                    } else {
                        let gib = mib / 1024.0;
                        format!("{gib:.2} GiB")
                    }
                }
            }
        },
    )
}

/// Formats wind observation data.
pub fn format_observation_wind(
    speed_vu_opt: Option<ValueUnit>,
    direction_vu_opt: Option<ValueUnit>,
) -> String {
    if speed_vu_opt.is_none() {
        return "N/A".to_string();
    }
    let speed_vu = speed_vu_opt.unwrap();
    if speed_vu.value.is_none() {
        return "N/A".to_string();
    }
    let speed_str = format_optional_value_unit(&Some(speed_vu));
    if direction_vu_opt.is_none() {
        return speed_str;
    }
    let direction_vu = direction_vu_opt.unwrap();
    if direction_vu.value.is_none() {
        return speed_str;
    }
    let direction_str = format_optional_value_unit(&Some(direction_vu));
    format!("{speed_str} {direction_str}")
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
