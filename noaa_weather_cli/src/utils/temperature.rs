/// Converts a temperature in Celsius (represented as a `f64`) to Fahrenheit.
///
/// Apply the conversion formula: F = C * 9/5 + 32
///
/// # Arguments
///
/// * `celsius` - A `f64` containing the Celsius temperature.
///
/// # Returns
///
/// A `f64` representing the temperature in Fahrenheit.
///
/// # Examples
///
/// ```
/// let fahrenheit = celsius_to_fahrenheit(7.777777777777778);
/// assert_eq!(fahrenheit, 46.0);
/// ```
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Converts a temperature in Fahrenheit (represented as a `f64`) to Celsius.
///
/// Apply the conversion formula: C = (F - 32) * 5/9
///
/// # Arguments
/// * `fahrenheit` - A `f64` containing the Fahrenheit temperature.
///
/// # Returns
///
/// A `f64` representing the temperature in Celsius.
///
/// # Examples
///
/// ```
/// let celsius = fahrenheit_to_celsius(46.0);
/// assert_eq!(celsius, 7.777777777777778);
/// ```
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let celsius = 7.777777777777778;
        let expected_fahrenheit = 46.0;
        assert_eq!(celsius_to_fahrenheit(celsius), expected_fahrenheit);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let fahrenheit = 46.0;
        let expected_celsius = 7.777777777777778;
        assert_eq!(fahrenheit_to_celsius(fahrenheit), expected_celsius);
    }
}
