use jiff::Timestamp;

use super::{
    Aerodrome, CloudAmount, CloudLayer, CloudType, Comparison, ForecastClouds, ForecastConditions,
    ForecastGroup, ForecastGroupKind, ForecastReport, ForecastValue, ForecastVisibility,
    ForecastWeather, ForecastWind, GeoPosition, MissingForecastReason, MissingReason,
    PermissibleUsage, PermissibleUsageReason, ReportMetadata, ReportStatus, SurfaceWind,
    TafDecodeError, TafDecodeErrorKind, TemperatureExtreme, TemperatureForecast,
    TerminalAerodromeForecast, TimeRange, TranslationMetadata, Visibility, Weather,
    WeatherDescriptor, WeatherIntensity, WeatherPhenomenon, WindDirection, WindSpeed, wire,
};

pub(super) fn decode_iwxxm(bytes: &[u8]) -> Result<TerminalAerodromeForecast, TafDecodeError> {
    let bulletin = wire::decode(bytes)?;
    let bulletin_identifier = bulletin.bulletin_identifier.into_boxed_str();
    let mut taf = bulletin.meteorological_information.taf;
    let (report_metadata, translation_failed_tac) = adapt_report_metadata(&mut taf)?;
    let wire::Taf {
        is_cancel_report,
        issue_time,
        aerodrome,
        valid_period,
        cancelled_report_valid_period,
        base_forecast,
        change_forecasts,
        ..
    } = taf;
    let issued_at = parse_timestamp("TAF.issueTime", issue_time.instant.position)?;
    let aerodrome = adapt_aerodrome(aerodrome)?;

    let report = if is_cancel_report {
        if valid_period.is_some()
            || base_forecast.is_some()
            || !change_forecasts.is_empty()
            || translation_failed_tac.is_some()
        {
            return Err(TafDecodeError::classified(
                TafDecodeErrorKind::InvalidCombination,
                "TAF.isCancelReport",
                "cancellation report also contains forecast or translation-failure content",
            ));
        }
        ForecastReport::Cancellation {
            cancelled_period: adapt_period(
                "TAF.cancelledReportValidPeriod",
                cancelled_report_valid_period
                    .ok_or_else(|| TafDecodeError::missing("TAF.cancelledReportValidPeriod"))?,
            )?,
        }
    } else if let Some(tac) = translation_failed_tac {
        if valid_period.is_some()
            || cancelled_report_valid_period.is_some()
            || base_forecast.is_some()
            || !change_forecasts.is_empty()
        {
            return Err(TafDecodeError::classified(
                TafDecodeErrorKind::InvalidCombination,
                "TAF.translationFailedTAC",
                "failed translation also contains partially translated forecast content",
            ));
        }
        ForecastReport::Missing {
            reason: MissingForecastReason::TranslationFailed {
                tac: tac.into_boxed_str(),
            },
        }
    } else if base_forecast.is_none() && change_forecasts.is_empty() {
        if valid_period.is_some() || cancelled_report_valid_period.is_some() {
            return Err(TafDecodeError::classified(
                TafDecodeErrorKind::InvalidCombination,
                "TAF",
                "missing report contains a forecast or cancellation period",
            ));
        }
        ForecastReport::Missing {
            reason: MissingForecastReason::NotProvided,
        }
    } else {
        if cancelled_report_valid_period.is_some() {
            return Err(TafDecodeError::classified(
                TafDecodeErrorKind::InvalidCombination,
                "TAF.cancelledReportValidPeriod",
                "ordinary forecast contains a cancelled-report period",
            ));
        }
        let base_forecast = base_forecast.ok_or_else(|| {
            TafDecodeError::classified(
                TafDecodeErrorKind::MissingRequiredField,
                "TAF.baseForecast",
                "change forecasts require a base forecast",
            )
        })?;
        let mut groups = Vec::with_capacity(1 + change_forecasts.len());
        groups.push(adapt_base(base_forecast)?);
        for change in change_forecasts {
            groups.push(adapt_change(change)?);
        }
        ForecastReport::Forecast {
            valid_period: adapt_period(
                "TAF.validPeriod",
                valid_period.ok_or_else(|| TafDecodeError::missing("TAF.validPeriod"))?,
            )?,
            groups: groups.into_boxed_slice(),
        }
    };

    Ok(TerminalAerodromeForecast {
        bulletin_identifier,
        report_metadata,
        issued_at,
        aerodrome,
        report,
    })
}

fn adapt_report_metadata(
    value: &mut wire::Taf,
) -> Result<(ReportMetadata, Option<String>), TafDecodeError> {
    let status_code = std::mem::take(&mut value.report_status);
    let usage_code = std::mem::take(&mut value.permissible_usage);
    let usage_reason = value.permissible_usage_reason.take();
    let usage_supplementary = value.permissible_usage_supplementary.take();
    let translated_bulletin_id = value.translated_bulletin_id.take();
    let translated_bulletin_reception_time = value.translated_bulletin_reception_time.take();
    let translation_centre_designator = value.translation_centre_designator.take();
    let translation_centre_name = value.translation_centre_name.take();
    let translation_time = value.translation_time.take();
    let translation_failed_tac = value.translation_failed_tac.take();
    let status = match status_code.as_str() {
        "NORMAL" => ReportStatus::Normal,
        "AMENDMENT" => ReportStatus::Amendment,
        "CORRECTION" => ReportStatus::Correction,
        _ => ReportStatus::Other {
            code: status_code.into_boxed_str(),
        },
    };
    let permissible_usage = match usage_code.as_str() {
        "OPERATIONAL" => {
            if usage_reason.is_some() || usage_supplementary.is_some() {
                let path = if usage_reason.is_some() {
                    "TAF.permissibleUsageReason"
                } else {
                    "TAF.permissibleUsageSupplementary"
                };
                return Err(TafDecodeError::classified(
                    TafDecodeErrorKind::InvalidCombination,
                    path,
                    "operational usage cannot carry a non-operational restriction",
                ));
            }
            PermissibleUsage::Operational
        }
        "NON-OPERATIONAL" => PermissibleUsage::NonOperational {
            reason: usage_reason.map(adapt_usage_reason),
            supplementary: usage_supplementary.map(String::into_boxed_str),
        },
        _ => PermissibleUsage::Other {
            code: usage_code.into_boxed_str(),
            reason: usage_reason.map(adapt_usage_reason),
            supplementary: usage_supplementary.map(String::into_boxed_str),
        },
    };

    let has_translation = translated_bulletin_id.is_some()
        || translated_bulletin_reception_time.is_some()
        || translation_centre_designator.is_some()
        || translation_centre_name.is_some()
        || translation_time.is_some();
    let translation = has_translation
        .then(|| {
            Ok(TranslationMetadata {
                source_bulletin_identifier: translated_bulletin_id.map(String::into_boxed_str),
                source_bulletin_received_at: translated_bulletin_reception_time
                    .map(|value| parse_timestamp("TAF.translatedBulletinReceptionTime", value))
                    .transpose()?,
                centre_designator: translation_centre_designator.map(String::into_boxed_str),
                centre_name: translation_centre_name.map(String::into_boxed_str),
                translated_at: translation_time
                    .map(|value| parse_timestamp("TAF.translationTime", value))
                    .transpose()?,
            })
        })
        .transpose()?;

    Ok((
        ReportMetadata {
            status,
            permissible_usage,
            translation,
        },
        translation_failed_tac,
    ))
}

fn adapt_usage_reason(reason: String) -> PermissibleUsageReason {
    match reason.as_str() {
        "TEST" => PermissibleUsageReason::Test,
        "EXERCISE" => PermissibleUsageReason::Exercise,
        _ => PermissibleUsageReason::Other {
            code: reason.into_boxed_str(),
        },
    }
}

fn adapt_aerodrome(value: wire::AerodromeProperty) -> Result<Aerodrome, TafDecodeError> {
    let value = value.airport_heliport.time_slice.value;
    let position = value
        .arp
        .map(|arp| parse_position(arp.point.pos))
        .transpose()?;
    Ok(Aerodrome {
        designator: value.designator.into_boxed_str(),
        icao_identifier: value.icao_identifier.into_boxed_str(),
        position,
    })
}

fn adapt_period(
    path: &'static str,
    value: wire::TimePeriodProperty,
) -> Result<TimeRange, TafDecodeError> {
    let start = parse_timestamp(path, value.period.begin)?;
    let end = parse_timestamp(path, value.period.end)?;
    if end < start {
        return Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidPeriod,
            path,
            "time range ends before it begins",
        ));
    }
    Ok(TimeRange { start, end })
}

fn parse_timestamp(path: &'static str, value: String) -> Result<Timestamp, TafDecodeError> {
    value.parse().map_err(|source| {
        TafDecodeError::sourced(
            TafDecodeErrorKind::InvalidTimestamp,
            path,
            format!("invalid timestamp {value:?}"),
            source,
        )
    })
}

fn parse_position(value: String) -> Result<GeoPosition, TafDecodeError> {
    const PATH: &str = "TAF.aerodrome.position";
    let mut coordinates = value.split_ascii_whitespace();
    let latitude = parse_coordinate(PATH, coordinates.next(), &value)?;
    let longitude = parse_coordinate(PATH, coordinates.next(), &value)?;
    if coordinates.next().is_some()
        || !(-90.0..=90.0).contains(&latitude)
        || !(-180.0..=180.0).contains(&longitude)
    {
        return Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidCoordinate,
            PATH,
            format!("invalid latitude/longitude pair {value:?}"),
        ));
    }
    Ok(GeoPosition {
        latitude,
        longitude,
    })
}

fn parse_coordinate(
    path: &'static str,
    coordinate: Option<&str>,
    pair: &str,
) -> Result<f64, TafDecodeError> {
    coordinate
        .ok_or_else(|| {
            TafDecodeError::classified(
                TafDecodeErrorKind::InvalidCoordinate,
                path,
                format!("invalid coordinate pair {pair:?}"),
            )
        })?
        .parse()
        .map_err(|source| {
            TafDecodeError::sourced(
                TafDecodeErrorKind::InvalidCoordinate,
                path,
                format!("invalid coordinate pair {pair:?}"),
                source,
            )
        })
}

fn adapt_base(value: wire::ForecastProperty) -> Result<ForecastGroup, TafDecodeError> {
    let forecast = value.forecast;
    if forecast.change_indicator.is_some() {
        return Err(TafDecodeError::invalid(
            "TAF.baseForecast.changeIndicator",
            "base forecast must not contain a change indicator",
        ));
    }
    adapt_group(ForecastGroupKind::Base, forecast)
}

fn adapt_change(value: wire::ForecastProperty) -> Result<ForecastGroup, TafDecodeError> {
    let mut forecast = value.forecast;
    let code = forecast
        .change_indicator
        .take()
        .ok_or_else(|| TafDecodeError::missing("TAF.changeForecast.changeIndicator"))?;
    let kind = match code.as_str() {
        "FROM" => ForecastGroupKind::From,
        "BECOMING" => ForecastGroupKind::Becoming,
        "TEMPORARY_FLUCTUATIONS" => ForecastGroupKind::Temporary,
        "PROBABILITY_30" => ForecastGroupKind::Probability {
            percent: 30,
            temporary: false,
        },
        "PROBABILITY_30_TEMPORARY_FLUCTUATIONS" => ForecastGroupKind::Probability {
            percent: 30,
            temporary: true,
        },
        "PROBABILITY_40" => ForecastGroupKind::Probability {
            percent: 40,
            temporary: false,
        },
        "PROBABILITY_40_TEMPORARY_FLUCTUATIONS" => ForecastGroupKind::Probability {
            percent: 40,
            temporary: true,
        },
        _ => ForecastGroupKind::Other {
            code: code.into_boxed_str(),
        },
    };

    adapt_group(kind, forecast)
}

fn adapt_group(
    kind: ForecastGroupKind,
    value: wire::Forecast,
) -> Result<ForecastGroup, TafDecodeError> {
    let valid_period = adapt_period("TAF.forecastGroup.phenomenonTime", value.phenomenon_time)?;
    let is_base = matches!(kind, ForecastGroupKind::Base);
    let visibility = match value.visibility {
        Some(visibility) => adapt_visibility(visibility, value.visibility_operator)?,
        None if value.visibility_operator.is_none() => ForecastVisibility::NotReported,
        None => {
            return Err(TafDecodeError::classified(
                TafDecodeErrorKind::InvalidCombination,
                "TAF.forecastGroup.visibilityOperator",
                "visibility operator is present without a visibility value",
            ));
        }
    };
    let weather = adapt_weather(value.weather)?;
    let clouds = adapt_clouds(value.cloud)?;
    if value.cavok
        && (!matches!(visibility, ForecastVisibility::NotReported)
            || !matches!(weather, ForecastWeather::NotReported)
            || !matches!(clouds, ForecastClouds::NotReported))
    {
        return Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidCombination,
            "TAF.forecastGroup.cloudAndVisibilityOK",
            "CAVOK forecast also reports visibility, weather, or cloud conditions",
        ));
    }
    let wind = value
        .surface_wind
        .map(adapt_wind_property)
        .transpose()?
        .unwrap_or(ForecastWind::NotReported);
    if !is_base && !value.temperature.is_empty() {
        return Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidCombination,
            "TAF.changeForecast.temperature",
            "temperature forecasts are permitted only on the base forecast",
        ));
    }
    let temperatures = value
        .temperature
        .into_iter()
        .map(|temperature| adapt_temperature(temperature.forecast, valid_period))
        .collect::<Result<Vec<_>, _>>()?
        .into_boxed_slice();

    Ok(ForecastGroup {
        kind,
        valid_period,
        conditions: ForecastConditions {
            cavok: value.cavok,
            visibility,
            wind,
            weather,
            clouds,
            temperatures,
        },
    })
}

fn adapt_temperature(
    value: wire::AirTemperatureForecast,
    valid_period: TimeRange,
) -> Result<TemperatureForecast, TafDecodeError> {
    let maximum = TemperatureExtreme {
        celsius: parse_measure(
            "TAF.baseForecast.temperature.maximum",
            value.maximum,
            |value, unit| (unit == "Cel").then_some(value),
        )?,
        occurs_at: parse_timestamp(
            "TAF.baseForecast.temperature.maximumTime",
            value.maximum_time.instant.position,
        )?,
    };
    let minimum = TemperatureExtreme {
        celsius: parse_measure(
            "TAF.baseForecast.temperature.minimum",
            value.minimum,
            |value, unit| (unit == "Cel").then_some(value),
        )?,
        occurs_at: parse_timestamp(
            "TAF.baseForecast.temperature.minimumTime",
            value.minimum_time.instant.position,
        )?,
    };
    if maximum.celsius < minimum.celsius {
        return Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidCombination,
            "TAF.baseForecast.temperature",
            "maximum temperature is below minimum temperature",
        ));
    }
    for (path, extreme) in [
        ("TAF.baseForecast.temperature.maximumTime", maximum),
        ("TAF.baseForecast.temperature.minimumTime", minimum),
    ] {
        if !(valid_period.start..=valid_period.end).contains(&extreme.occurs_at) {
            return Err(TafDecodeError::classified(
                TafDecodeErrorKind::InvalidPeriod,
                path,
                "temperature occurrence is outside the forecast-group period",
            ));
        }
    }

    Ok(TemperatureForecast { maximum, minimum })
}

fn adapt_weather(values: Vec<wire::CodeValue>) -> Result<ForecastWeather, TafDecodeError> {
    const PATH: &str = "TAF.forecastGroup.weather";

    if values.is_empty() {
        return Ok(ForecastWeather::NotReported);
    }

    let mut weather = Vec::with_capacity(values.len());
    let mut missing_reason = None;
    for value in values {
        match (value.href, value.nil_reason) {
            (Some(href), None) if missing_reason.is_none() => {
                weather.push(parse_weather_code(code_from_href(PATH, href)?)?);
            }
            (None, Some(reason)) if weather.is_empty() && missing_reason.is_none() => {
                missing_reason = Some(adapt_missing_reason(reason));
            }
            (Some(_), Some(_)) => {
                return Err(TafDecodeError::invalid(
                    PATH,
                    "weather cannot contain both a code and a nil reason",
                ));
            }
            (None, None) => return Err(TafDecodeError::missing(PATH)),
            _ => {
                return Err(TafDecodeError::invalid(
                    PATH,
                    "weather codes and unavailable weather cannot be combined",
                ));
            }
        }
    }

    if let Some(reason) = missing_reason {
        return Ok(if reason == MissingReason::NoSignificant {
            ForecastWeather::NoSignificant
        } else {
            ForecastWeather::Unavailable { reason }
        });
    }

    Ok(ForecastWeather::Phenomena {
        items: weather.into_boxed_slice(),
    })
}

fn parse_weather_code(code: String) -> Result<Weather, TafDecodeError> {
    const PATH: &str = "TAF.forecastGroup.weather";
    let mut remaining = code.as_str();
    let intensity = match remaining.as_bytes().first() {
        Some(b'-') => {
            remaining = &remaining[1..];
            WeatherIntensity::Light
        }
        Some(b'+') => {
            remaining = &remaining[1..];
            WeatherIntensity::Heavy
        }
        _ => WeatherIntensity::Moderate,
    };
    let in_vicinity = remaining.starts_with("VC");
    if in_vicinity {
        remaining = &remaining[2..];
    }
    let descriptor = (remaining.is_ascii() && remaining.len() >= 2)
        .then(|| &remaining[..2])
        .and_then(adapt_weather_descriptor);
    if descriptor.is_some() {
        remaining = &remaining[2..];
    }
    if remaining.is_empty() && descriptor.is_none() {
        return Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidValue,
            PATH,
            format!("weather code {code:?} contains no forecast meaning"),
        ));
    }

    let phenomena = if remaining.is_empty() {
        Vec::new()
    } else if !remaining.is_ascii() || !remaining.len().is_multiple_of(2) {
        vec![WeatherPhenomenon::Other {
            code: remaining.into(),
        }]
    } else {
        remaining
            .as_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|chunk| {
                let code = std::str::from_utf8(chunk).expect("ASCII was checked above");
                adapt_weather_phenomenon(code)
            })
            .collect()
    };

    Ok(Weather {
        code: code.into_boxed_str(),
        intensity,
        in_vicinity,
        descriptor,
        phenomena: phenomena.into_boxed_slice(),
    })
}

fn adapt_weather_descriptor(code: &str) -> Option<WeatherDescriptor> {
    Some(match code {
        "MI" => WeatherDescriptor::Shallow,
        "PR" => WeatherDescriptor::Partial,
        "BC" => WeatherDescriptor::Patches,
        "DR" => WeatherDescriptor::LowDrifting,
        "BL" => WeatherDescriptor::Blowing,
        "SH" => WeatherDescriptor::Showers,
        "TS" => WeatherDescriptor::Thunderstorm,
        "FZ" => WeatherDescriptor::Freezing,
        _ => return None,
    })
}

fn adapt_weather_phenomenon(code: &str) -> WeatherPhenomenon {
    match code {
        "DZ" => WeatherPhenomenon::Drizzle,
        "RA" => WeatherPhenomenon::Rain,
        "SN" => WeatherPhenomenon::Snow,
        "SG" => WeatherPhenomenon::SnowGrains,
        "IC" => WeatherPhenomenon::IceCrystals,
        "PL" => WeatherPhenomenon::IcePellets,
        "GR" => WeatherPhenomenon::Hail,
        "GS" => WeatherPhenomenon::SmallHail,
        "UP" => WeatherPhenomenon::UnknownPrecipitation,
        "BR" => WeatherPhenomenon::Mist,
        "FG" => WeatherPhenomenon::Fog,
        "FU" => WeatherPhenomenon::Smoke,
        "VA" => WeatherPhenomenon::VolcanicAsh,
        "DU" => WeatherPhenomenon::Dust,
        "SA" => WeatherPhenomenon::Sand,
        "HZ" => WeatherPhenomenon::Haze,
        "PY" => WeatherPhenomenon::Spray,
        "PO" => WeatherPhenomenon::DustWhirls,
        "SQ" => WeatherPhenomenon::Squalls,
        "FC" => WeatherPhenomenon::FunnelCloud,
        "SS" => WeatherPhenomenon::Sandstorm,
        "DS" => WeatherPhenomenon::Duststorm,
        _ => WeatherPhenomenon::Other { code: code.into() },
    }
}

fn adapt_clouds(value: Option<wire::CloudProperty>) -> Result<ForecastClouds, TafDecodeError> {
    const PATH: &str = "TAF.forecastGroup.cloud";
    let Some(value) = value else {
        return Ok(ForecastClouds::NotReported);
    };
    match (value.forecast, value.nil_reason) {
        (Some(_), Some(_)) => Err(TafDecodeError::invalid(
            PATH,
            "cloud forecast cannot contain both conditions and a nil reason",
        )),
        (None, None) => Err(TafDecodeError::missing(PATH)),
        (None, Some(reason)) => {
            let reason = adapt_missing_reason(reason);
            Ok(if reason == MissingReason::NoSignificant {
                ForecastClouds::NoSignificant
            } else {
                ForecastClouds::Unavailable { reason }
            })
        }
        (Some(forecast), None) => adapt_cloud_forecast(forecast),
    }
}

fn adapt_cloud_forecast(value: wire::CloudForecast) -> Result<ForecastClouds, TafDecodeError> {
    const PATH: &str = "TAF.forecastGroup.cloud";
    match (value.vertical_visibility, value.layer.is_empty()) {
        (Some(_), false) => Err(TafDecodeError::invalid(
            PATH,
            "vertical visibility and cloud layers cannot be reported together",
        )),
        (None, true) => Err(TafDecodeError::missing(PATH)),
        (Some(visibility), true) => Ok(ForecastClouds::VerticalVisibility {
            feet: adapt_nillable_measure(
                "TAF.forecastGroup.cloud.verticalVisibility",
                visibility,
                feet_from_unit,
            )?,
        }),
        (None, false) => {
            let layers = value
                .layer
                .into_iter()
                .map(|layer| adapt_cloud_layer(layer.layer))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ForecastClouds::Layers {
                layers: layers.into_boxed_slice(),
            })
        }
    }
}

fn adapt_cloud_layer(value: wire::CloudLayer) -> Result<CloudLayer, TafDecodeError> {
    Ok(CloudLayer {
        amount: adapt_nillable_code(
            "TAF.forecastGroup.cloud.layer.amount",
            value.amount,
            adapt_cloud_amount,
        )?,
        base_feet: adapt_nillable_measure(
            "TAF.forecastGroup.cloud.layer.base",
            value.base,
            feet_from_unit,
        )?,
        cloud_type: value
            .cloud_type
            .map(|cloud_type| {
                adapt_nillable_code(
                    "TAF.forecastGroup.cloud.layer.cloudType",
                    cloud_type,
                    adapt_cloud_type,
                )
            })
            .transpose()?,
    })
}

fn adapt_nillable_code<T>(
    path: &'static str,
    value: wire::CodeValue,
    adapt: impl FnOnce(String) -> T,
) -> Result<ForecastValue<T>, TafDecodeError> {
    match (value.href, value.nil_reason) {
        (Some(href), None) => Ok(ForecastValue::Value(adapt(code_from_href(path, href)?))),
        (None, Some(reason)) => Ok(ForecastValue::Unavailable {
            reason: adapt_missing_reason(reason),
        }),
        (Some(_), Some(_)) => Err(TafDecodeError::invalid(
            path,
            "value cannot contain both a code and a nil reason",
        )),
        (None, None) => Err(TafDecodeError::missing(path)),
    }
}

fn adapt_nillable_measure(
    path: &'static str,
    value: wire::Measure,
    convert: impl FnOnce(f64, &str) -> Option<f64>,
) -> Result<ForecastValue<f64>, TafDecodeError> {
    match (&value.value, &value.nil_reason) {
        (Some(_), None) => {
            parse_non_negative_measure(path, value, convert).map(ForecastValue::Value)
        }
        (None, Some(_)) => Ok(ForecastValue::Unavailable {
            reason: adapt_missing_reason(
                value
                    .nil_reason
                    .expect("matched a present nil reason above"),
            ),
        }),
        (Some(_), Some(_)) => Err(TafDecodeError::invalid(
            path,
            "measurement cannot contain both a value and a nil reason",
        )),
        (None, None) => Err(TafDecodeError::missing(path)),
    }
}

fn adapt_cloud_amount(code: String) -> CloudAmount {
    match code.as_str() {
        "FEW" => CloudAmount::Few,
        "SCT" => CloudAmount::Scattered,
        "BKN" => CloudAmount::Broken,
        "OVC" => CloudAmount::Overcast,
        "NSC" => CloudAmount::NoSignificant,
        "SKC" | "CLR" => CloudAmount::SkyClear,
        _ => CloudAmount::Other {
            code: code.into_boxed_str(),
        },
    }
}

fn adapt_cloud_type(code: String) -> CloudType {
    match code.as_str() {
        "CB" => CloudType::Cumulonimbus,
        "TCU" => CloudType::ToweringCumulus,
        _ => CloudType::Other {
            code: code.into_boxed_str(),
        },
    }
}

fn adapt_missing_reason(reason: String) -> MissingReason {
    match reason.rsplit(['/', '#']).next().unwrap_or(reason.as_str()) {
        "nothingOfOperationalSignificance" => MissingReason::NoSignificant,
        "notObservable" => MissingReason::NotObservable,
        "missing" => MissingReason::Missing,
        "withheld" => MissingReason::Withheld,
        _ => MissingReason::Other {
            code: reason.into_boxed_str(),
        },
    }
}

fn code_from_href(path: &'static str, href: String) -> Result<String, TafDecodeError> {
    let code = href
        .rsplit(['/', '#'])
        .find(|part| !part.is_empty())
        .ok_or_else(|| TafDecodeError::invalid(path, format!("code URI {href:?} has no code")))?;
    Ok(code.to_owned())
}

fn feet_from_unit(value: f64, unit: &str) -> Option<f64> {
    match unit {
        "[ft_i]" => Some(value),
        "m" => Some(value * 3.280_839_895_013_1),
        _ => None,
    }
}

fn adapt_visibility(
    value: wire::Measure,
    operator: Option<String>,
) -> Result<ForecastVisibility, TafDecodeError> {
    const PATH: &str = "TAF.forecastGroup.visibility";
    match (&value.value, &value.nil_reason) {
        (Some(_), None) => {
            let meters = parse_non_negative_measure(PATH, value, |value, unit| match unit {
                "m" => Some(value),
                "[ft_i]" => Some(value * 0.3048),
                _ => None,
            })?;
            Ok(ForecastVisibility::Value(Visibility {
                meters,
                comparison: adapt_comparison(operator),
            }))
        }
        (None, Some(_)) if operator.is_none() => Ok(ForecastVisibility::Unavailable {
            reason: adapt_missing_reason(
                value
                    .nil_reason
                    .expect("matched a present nil reason above"),
            ),
        }),
        (None, Some(_)) => Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidCombination,
            "TAF.forecastGroup.visibilityOperator",
            "unavailable visibility cannot carry a comparison operator",
        )),
        (Some(_), Some(_)) => Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidCombination,
            PATH,
            "visibility cannot contain both a value and a nil reason",
        )),
        (None, None) => Err(TafDecodeError::missing(PATH)),
    }
}

fn adapt_wind(value: wire::SurfaceWind) -> Result<SurfaceWind, TafDecodeError> {
    let direction = match (value.variable_direction, value.mean_direction) {
        (true, None) => WindDirection::Variable,
        (false, Some(direction)) => {
            let degrees = parse_measure(
                "TAF.forecastGroup.wind.direction",
                direction,
                |value, unit| (unit == "deg").then_some(value),
            )?;
            if !(0.0..=360.0).contains(&degrees) {
                return Err(TafDecodeError::invalid(
                    "TAF.forecastGroup.wind.direction",
                    format!("wind direction {degrees} is outside 0..=360 degrees"),
                ));
            }
            WindDirection::Degrees(degrees)
        }
        (true, Some(_)) => {
            return Err(TafDecodeError::invalid(
                "TAF.forecastGroup.wind.direction",
                "variable wind also reports a fixed direction",
            ));
        }
        (false, None) => {
            return Err(TafDecodeError::missing("TAF.forecastGroup.wind.direction"));
        }
    };
    let speed = adapt_wind_speed(
        "TAF.forecastGroup.wind.speed",
        value
            .mean_speed
            .ok_or_else(|| TafDecodeError::missing("TAF.forecastGroup.wind.speed"))?,
        value.mean_speed_operator,
    )?;
    let gust = value
        .gust_speed
        .map(|gust| {
            adapt_wind_speed(
                "TAF.forecastGroup.wind.gust",
                gust,
                value.gust_speed_operator,
            )
        })
        .transpose()?;

    Ok(SurfaceWind {
        direction,
        speed,
        gust,
    })
}

fn adapt_wind_property(value: wire::SurfaceWindProperty) -> Result<ForecastWind, TafDecodeError> {
    const PATH: &str = "TAF.forecastGroup.wind";
    match (value.wind, value.nil_reason) {
        (Some(wind), None) => adapt_wind(wind).map(ForecastWind::Value),
        (None, Some(reason)) => Ok(ForecastWind::Unavailable {
            reason: adapt_missing_reason(reason),
        }),
        (Some(_), Some(_)) => Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidCombination,
            PATH,
            "wind cannot contain both conditions and a nil reason",
        )),
        (None, None) => Err(TafDecodeError::missing(PATH)),
    }
}

fn adapt_wind_speed(
    path: &'static str,
    value: wire::Measure,
    operator: Option<String>,
) -> Result<WindSpeed, TafDecodeError> {
    let knots = parse_non_negative_measure(path, value, |value, unit| match unit {
        "[kn_i]" => Some(value),
        "m/s" | "m.s-1" => Some(value * 1.943_844_492_440_6),
        _ => None,
    })?;
    Ok(WindSpeed {
        knots,
        comparison: adapt_comparison(operator),
    })
}

fn adapt_comparison(value: Option<String>) -> Comparison {
    match value.as_deref() {
        None => Comparison::Exact,
        Some("ABOVE") => Comparison::Above,
        Some("BELOW") => Comparison::Below,
        Some(_) => Comparison::Other {
            code: value.expect("matched Some above").into_boxed_str(),
        },
    }
}

fn parse_measure(
    path: &'static str,
    value: wire::Measure,
    convert: impl FnOnce(f64, &str) -> Option<f64>,
) -> Result<f64, TafDecodeError> {
    if value.nil_reason.is_some() {
        return Err(TafDecodeError::invalid(
            path,
            "required measurement is unavailable",
        ));
    }
    let text = value.value.ok_or_else(|| TafDecodeError::missing(path))?;
    let parsed = text.parse::<f64>().map_err(|source| {
        TafDecodeError::sourced(
            TafDecodeErrorKind::InvalidNumber,
            path,
            format!("invalid measurement {text:?}"),
            source,
        )
    })?;
    if !parsed.is_finite() {
        return Err(TafDecodeError::invalid(
            path,
            format!("measurement {text:?} must be finite"),
        ));
    }
    let unit = value.unit.ok_or_else(|| TafDecodeError::missing(path))?;
    convert(parsed, &unit).ok_or_else(|| {
        TafDecodeError::classified(
            TafDecodeErrorKind::UnsupportedUnit,
            path,
            format!("unsupported unit {unit:?}"),
        )
    })
}

fn parse_non_negative_measure(
    path: &'static str,
    value: wire::Measure,
    convert: impl FnOnce(f64, &str) -> Option<f64>,
) -> Result<f64, TafDecodeError> {
    let parsed = parse_measure(path, value, convert)?;
    if parsed.is_sign_negative() {
        return Err(TafDecodeError::invalid(
            path,
            "measurement must be non-negative",
        ));
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use std::{hint::black_box, time::Instant};

    use super::decode_iwxxm;
    use crate::client::measure_allocations;

    const FIXTURE: &[u8] = include_bytes!("../../../tests/fixtures/taf/kflg_normal.xml");
    const MAX_ALLOCATION_OPERATIONS: usize = 200;
    const MAX_ALLOCATED_BYTES: usize = 16_000;
    const MAX_RETAINED_BYTES: isize = 1_200;

    #[test]
    fn semantic_decode_stays_below_the_legacy_allocation_ceiling() {
        for _ in 0..10 {
            black_box(decode_iwxxm(FIXTURE).unwrap());
        }

        let (forecast, stats) = measure_allocations(|| decode_iwxxm(FIXTURE).unwrap());
        let retained = stats.bytes_allocated as isize - stats.bytes_deallocated as isize
            + stats.bytes_reallocated;
        let operations = stats.allocations + stats.reallocations;

        assert!(
            operations <= MAX_ALLOCATION_OPERATIONS,
            "semantic decode exceeded {MAX_ALLOCATION_OPERATIONS} allocation operations: {stats:?}"
        );
        assert!(
            stats.bytes_allocated <= MAX_ALLOCATED_BYTES,
            "semantic decode exceeded the {MAX_ALLOCATED_BYTES}-byte allocation ceiling: {stats:?}"
        );
        assert!(
            retained <= MAX_RETAINED_BYTES,
            "semantic result exceeded the {MAX_RETAINED_BYTES}-byte retained-memory ceiling: retained={retained}, stats={stats:?}"
        );

        black_box(forecast);
    }

    // Run with:
    // cargo test -p noaa_weather_client --no-default-features --features xml --release \
    //   models::terminal_aerodrome_forecast::decode::tests::semantic_decode_performance_receipt \
    //   -- --ignored --exact --nocapture --test-threads=1
    #[test]
    #[ignore = "manual release-mode allocation and timing receipt"]
    fn semantic_decode_performance_receipt() {
        const ITERATIONS: u128 = 2_000;
        const LEGACY_NANOSECONDS_PER_DECODE: u128 = 283_136;

        for _ in 0..100 {
            black_box(decode_iwxxm(FIXTURE).unwrap());
        }
        let (_, stats) = measure_allocations(|| decode_iwxxm(FIXTURE).unwrap());
        let retained = stats.bytes_allocated as isize - stats.bytes_deallocated as isize
            + stats.bytes_reallocated;

        let started = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(decode_iwxxm(FIXTURE).unwrap());
        }
        let nanoseconds_per_decode = started.elapsed().as_nanos() / ITERATIONS;

        eprintln!("allocations={}", stats.allocations);
        eprintln!("reallocations={}", stats.reallocations);
        eprintln!("allocated_bytes={}", stats.bytes_allocated);
        eprintln!("retained_bytes={retained}");
        eprintln!("nanoseconds_per_decode={nanoseconds_per_decode}");
        assert!(
            nanoseconds_per_decode <= LEGACY_NANOSECONDS_PER_DECODE * 6 / 5,
            "semantic decode exceeded the 20% timing tolerance"
        );
    }
}
