use std::borrow::Cow;
use std::fmt::Display;

use jiff::Timestamp;
use noaa_weather_client::OffsetDateTime;
use noaa_weather_client::models::radar::RadarMeasurement;
use noaa_weather_client::models::{QuantitativeValue, UnitCodeType, ValueUnit};

use super::{DefaultPresenter, PresentationError};

const MISSING: &str = "N/A";
const INVALID: &str = "Invalid";

impl DefaultPresenter {
    pub(super) fn missing(&self) -> String {
        MISSING.to_owned()
    }

    pub(super) fn text(&self, value: Option<&str>) -> String {
        value
            .filter(|value| !value.trim().is_empty())
            .unwrap_or(MISSING)
            .to_owned()
    }

    pub(super) fn integer(&self, value: Option<impl Display>) -> String {
        value.map_or_else(|| MISSING.to_owned(), |value| value.to_string())
    }

    pub(super) fn decimal(&self, value: Option<f64>) -> String {
        match value {
            None => MISSING.to_owned(),
            Some(value) if !value.is_finite() => INVALID.to_owned(),
            Some(value) if value.fract() == 0.0 => format!("{value:.0}"),
            Some(value) => format!("{value:.2}"),
        }
    }

    pub(super) fn precise_decimal(&self, value: Option<f64>) -> String {
        match value {
            None => MISSING.to_owned(),
            Some(value) if !value.is_finite() => INVALID.to_owned(),
            Some(value) => value.to_string(),
        }
    }

    pub(super) fn yes_no(&self, value: Option<bool>) -> String {
        match value {
            Some(true) => "Yes".to_owned(),
            Some(false) => "No".to_owned(),
            None => MISSING.to_owned(),
        }
    }

    pub(super) fn timestamp(
        &self,
        context: impl Into<Cow<'static, str>>,
        value: Option<&str>,
    ) -> Result<String, PresentationError> {
        let Some(value) = value else {
            return Ok(MISSING.to_owned());
        };
        let timestamp = value
            .parse::<Timestamp>()
            .map_err(|source| PresentationError::invalid_timestamp(context, value, source))?;
        Ok(self.parsed_timestamp(Some(timestamp)))
    }

    pub(super) fn parsed_timestamp(&self, value: Option<Timestamp>) -> String {
        value.map_or_else(
            || MISSING.to_owned(),
            |timestamp| {
                timestamp
                    .to_zoned(self.time_zone.clone())
                    .strftime("%D %r")
                    .to_string()
            },
        )
    }

    /// Renders a response timestamp in the presenter's time zone, exactly
    /// as [`DefaultPresenter::parsed_timestamp`] renders its instant; the
    /// offset NOAA sent is not shown.
    pub(super) fn offset_date_time(&self, value: &OffsetDateTime) -> String {
        self.parsed_timestamp(Some(value.timestamp()))
    }

    pub(super) fn optional_offset_date_time(&self, value: Option<&OffsetDateTime>) -> String {
        value.map_or_else(|| MISSING.to_owned(), |value| self.offset_date_time(value))
    }

    pub(super) fn resource_identifier(&self, value: Option<&str>) -> String {
        let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
            return MISSING.to_owned();
        };
        value
            .split('/')
            .rev()
            .find(|segment| !segment.is_empty())
            .unwrap_or(value)
            .to_owned()
    }

    pub(super) fn value_unit(&self, value: Option<&ValueUnit>) -> String {
        let Some(measurement) = value else {
            return MISSING.to_owned();
        };
        let Some(value) = measurement.value else {
            return MISSING.to_owned();
        };
        if !value.is_finite() {
            return INVALID.to_owned();
        }
        let number = format!("{value:.2}");
        match measurement.unit_code.as_ref().map(unit_code_label) {
            Some(unit) => format!("{number} {unit}"),
            None => number,
        }
    }

    pub(super) fn observation_pressure(
        &self,
        sea_level: Option<&ValueUnit>,
        barometric: Option<&ValueUnit>,
    ) -> String {
        let first = [sea_level, barometric]
            .into_iter()
            .flatten()
            .find(|value| value.value.is_some_and(f64::is_finite));
        self.value_unit(first)
    }

    pub(super) fn observation_weather(
        &self,
        description: Option<&str>,
        present_weather_summary: &str,
    ) -> String {
        match description.filter(|value| !value.trim().is_empty()) {
            Some(description) => description.trim().to_owned(),
            None => self.text(Some(present_weather_summary)),
        }
    }

    pub(super) fn quantitative_value(&self, value: Option<&QuantitativeValue>) -> String {
        let Some(value) = value else {
            return MISSING.to_owned();
        };
        let Some(number) = value.value.flatten() else {
            return MISSING.to_owned();
        };
        if !number.is_finite() {
            return INVALID.to_owned();
        }
        let number = if number.fract() == 0.0 {
            format!("{number:.0}")
        } else {
            number.to_string()
        };
        match value.unit_code.as_deref().and_then(unit_suffix) {
            Some(unit) => format!("{number} {unit}"),
            None => number,
        }
    }

    pub(super) fn rounded_temperature(&self, value: Option<&QuantitativeValue>) -> String {
        let Some(value) = value else {
            return MISSING.to_owned();
        };
        let Some(number) = value.value.flatten() else {
            return MISSING.to_owned();
        };
        if !number.is_finite() {
            return INVALID.to_owned();
        }
        let unit = value.unit_code.as_deref().and_then(temperature_label);
        match unit {
            Some(unit) => format!("{} {unit}", number.round()),
            None => number.round().to_string(),
        }
    }

    pub(super) fn percentage(&self, value: Option<&QuantitativeValue>) -> String {
        let Some(number) = value.and_then(|value| value.value.flatten()) else {
            return MISSING.to_owned();
        };
        if !number.is_finite() {
            return INVALID.to_owned();
        }
        format!("{number:.0}%")
    }

    pub(super) fn elevation(&self, value: Option<&QuantitativeValue>) -> String {
        let Some(measurement) = value else {
            return MISSING.to_owned();
        };
        let Some(number) = measurement.value.flatten() else {
            return MISSING.to_owned();
        };
        if !number.is_finite() {
            return INVALID.to_owned();
        }
        let number = format!("{number:.1}");
        match measurement.unit_code.as_deref().and_then(unit_suffix) {
            Some(unit) => format!("{number} {unit}"),
            None => number,
        }
    }

    pub(super) fn radar_measurement(&self, value: Option<&RadarMeasurement>) -> String {
        let Some(measurement) = value else {
            return MISSING.to_owned();
        };
        let Some(value) = measurement.value() else {
            return MISSING.to_owned();
        };
        if !value.is_finite() {
            return INVALID.to_owned();
        }
        let number = format!("{value:.2}");
        match measurement.unit().map(unit_code_label) {
            Some(unit) => format!("{number} {unit}"),
            None => number,
        }
    }

    pub(super) fn observation_wind(
        &self,
        speed: Option<&ValueUnit>,
        direction: Option<&ValueUnit>,
    ) -> String {
        let speed = self.value_unit(speed);
        if speed == MISSING || speed == INVALID {
            return speed;
        }
        let direction = self.value_unit(direction);
        if direction == MISSING || direction == INVALID {
            speed
        } else {
            format!("{speed} {direction}")
        }
    }

    pub(super) fn forecast_wind(
        &self,
        speed: Option<&QuantitativeValue>,
        gust: Option<&QuantitativeValue>,
        direction: Option<&str>,
    ) -> String {
        let mut parts = vec![self.quantitative_value(speed)];
        if let Some(direction) = direction.filter(|value| !value.trim().is_empty()) {
            parts.push(direction.to_owned());
        }
        if gust.is_some() {
            parts.push(format!("gust {}", self.quantitative_value(gust)));
        }
        parts.join(" ")
    }

    pub(super) fn bytes(&self, value: Option<i64>) -> String {
        let Some(bytes) = value else {
            return MISSING.to_owned();
        };
        if bytes < 0 {
            return "Invalid (negative)".to_owned();
        }
        if bytes < 1024 {
            return format!("{bytes} B");
        }
        let kib = bytes as f64 / 1024.0;
        if kib < 1024.0 {
            return format!("{kib:.2} KiB");
        }
        let mib = kib / 1024.0;
        if mib < 1024.0 {
            return format!("{mib:.2} MiB");
        }
        format!("{:.2} GiB", mib / 1024.0)
    }
}

fn unit_code_label(unit: &UnitCodeType) -> &str {
    match unit {
        UnitCodeType::Wmo(unit) => unit.alt_label(),
        UnitCodeType::Nws(unit) => unit.alt_label(),
    }
}

fn unit_suffix(unit: &str) -> Option<&str> {
    unit.rsplit([':', '/']).find(|segment| !segment.is_empty())
}

fn temperature_label(unit: &str) -> Option<&'static str> {
    match unit_suffix(unit)? {
        "degC" => Some("\u{b0}C"),
        "degF" => Some("\u{b0}F"),
        _ => None,
    }
}
