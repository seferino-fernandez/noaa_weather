use quick_xml::{
    XmlVersion,
    escape::unescape,
    events::{BytesStart, Event},
    name::ResolveResult,
    reader::NsReader,
};

use super::*;
use crate::models::terminal_aerodrome_forecast::{TafDecodeError, TafDecodeErrorKind};

type Result<T> = std::result::Result<T, TafDecodeError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Vocabulary {
    Collect,
    Iwxxm,
    Other,
}

struct Xml<'input> {
    reader: NsReader<&'input [u8]>,
    version: XmlVersion,
}

impl<'input> Xml<'input> {
    fn new(bytes: &'input [u8]) -> Self {
        let mut reader = NsReader::from_reader(bytes);
        reader.config_mut().trim_text(true);
        Self {
            reader,
            version: XmlVersion::Implicit1_0,
        }
    }

    fn resolved_event(&mut self) -> Result<(Vocabulary, Event<'input>)> {
        let (namespace, event) = self
            .reader
            .read_resolved_event()
            .map_err(TafDecodeError::xml)?;
        let vocabulary = match namespace {
            ResolveResult::Bound(namespace)
                if namespace.as_ref() == "http://def.wmo.int/collect/2014" =>
            {
                Vocabulary::Collect
            }
            ResolveResult::Bound(namespace)
                if namespace.as_ref() == "http://icao.int/iwxxm/2021-2" =>
            {
                Vocabulary::Iwxxm
            }
            ResolveResult::Unbound | ResolveResult::Bound(_) | ResolveResult::Unknown(_) => {
                Vocabulary::Other
            }
        };
        if let Event::Decl(declaration) = &event {
            self.version = declaration.xml_version().map_err(TafDecodeError::xml)?;
        }
        Ok((vocabulary, event))
    }

    fn event(&mut self) -> Result<Event<'input>> {
        self.resolved_event().map(|(_, event)| event)
    }

    fn attribute(&self, element: &BytesStart<'_>, name: &str) -> Result<Option<String>> {
        for attribute in element.attributes().with_checks(false) {
            let attribute = attribute.map_err(TafDecodeError::xml)?;
            if attribute.key.local_name().as_ref() == name {
                return attribute
                    .normalized_value(self.version)
                    .map(|value| Some(value.into_owned()))
                    .map_err(TafDecodeError::xml);
            }
        }
        Ok(None)
    }

    fn required_attribute(
        &self,
        element: &BytesStart<'_>,
        name: &str,
        path: &'static str,
    ) -> Result<String> {
        self.attribute(element, name)?
            .filter(|value| !value.is_empty())
            .ok_or_else(|| TafDecodeError::missing(path))
    }

    fn text(&mut self, element: &BytesStart<'_>) -> Result<String> {
        let text = self
            .reader
            .read_text(element.name())
            .map_err(TafDecodeError::xml)?;
        let decoded = text.xml_content(self.version);
        unescape(&decoded)
            .map(|value| value.into_owned())
            .map_err(TafDecodeError::xml)
    }

    fn skip(&mut self, element: &BytesStart<'_>) -> Result<()> {
        self.reader
            .read_to_end(element.to_end().name())
            .map(|_| ())
            .map_err(TafDecodeError::xml)
    }
}

pub(super) fn decode(bytes: &[u8]) -> Result<Bulletin> {
    let mut xml = Xml::new(bytes);
    let mut taf = None;
    let mut bulletin_identifier = None;

    loop {
        match xml.resolved_event()? {
            (Vocabulary::Iwxxm, Event::Start(element))
                if element.local_name().as_ref() == "TAF" =>
            {
                if taf.is_some() {
                    return Err(TafDecodeError::classified(
                        TafDecodeErrorKind::InvalidCombination,
                        "MeteorologicalBulletin.meteorologicalInformation",
                        "specific-TAF response contains more than one TAF",
                    ));
                }
                taf = Some(parse_taf(&mut xml, &element)?);
            }
            (Vocabulary::Collect, Event::Start(element))
                if element.local_name().as_ref() == "bulletinIdentifier" =>
            {
                bulletin_identifier = Some(required_text(
                    &mut xml,
                    &element,
                    "MeteorologicalBulletin.bulletinIdentifier",
                )?);
            }
            (_, Event::Start(element)) if element.local_name().as_ref() == "TAF" => {
                return Err(TafDecodeError::classified(
                    TafDecodeErrorKind::InvalidValue,
                    "MeteorologicalBulletin.meteorologicalInformation.TAF",
                    "TAF element uses an unsupported IWXXM namespace",
                ));
            }
            (_, Event::Eof) => break,
            _ => {}
        }
    }

    Ok(Bulletin {
        meteorological_information: MeteorologicalInformation {
            taf: taf.ok_or_else(|| {
                TafDecodeError::missing("MeteorologicalBulletin.meteorologicalInformation.TAF")
            })?,
        },
        bulletin_identifier: bulletin_identifier
            .ok_or_else(|| TafDecodeError::missing("MeteorologicalBulletin.bulletinIdentifier"))?,
    })
}

fn parse_taf(xml: &mut Xml<'_>, element: &BytesStart<'_>) -> Result<Taf> {
    let report_status = xml.required_attribute(element, "reportStatus", "TAF.reportStatus")?;
    let permissible_usage =
        xml.required_attribute(element, "permissibleUsage", "TAF.permissibleUsage")?;
    let permissible_usage_reason = xml.attribute(element, "permissibleUsageReason")?;
    let permissible_usage_supplementary =
        xml.attribute(element, "permissibleUsageSupplementary")?;
    let translated_bulletin_id = xml.attribute(element, "translatedBulletinID")?;
    let translated_bulletin_reception_time =
        xml.attribute(element, "translatedBulletinReceptionTime")?;
    let translation_centre_designator = xml.attribute(element, "translationCentreDesignator")?;
    let translation_centre_name = xml.attribute(element, "translationCentreName")?;
    let translation_time = xml.attribute(element, "translationTime")?;
    let translation_failed_tac = xml.attribute(element, "translationFailedTAC")?;
    let is_cancel_report = xml
        .attribute(element, "isCancelReport")?
        .map(|value| parse_boolean("TAF.isCancelReport", &value))
        .transpose()?
        .unwrap_or(false);

    let mut issue_time = None;
    let mut aerodrome = None;
    let mut valid_period = None;
    let mut cancelled_report_valid_period = None;
    let mut base_forecast = None;
    let mut change_forecasts = Vec::with_capacity(5);

    loop {
        match xml.event()? {
            Event::Start(child) => match child.local_name().as_ref() {
                "issueTime" => issue_time = Some(parse_time_instant(xml, "issueTime")?),
                "aerodrome" => aerodrome = Some(parse_aerodrome(xml)?),
                "validPeriod" => valid_period = Some(parse_time_period(xml, "validPeriod")?),
                "cancelledReportValidPeriod" => {
                    cancelled_report_valid_period =
                        Some(parse_time_period(xml, "cancelledReportValidPeriod")?);
                }
                "baseForecast" => {
                    base_forecast = Some(parse_forecast_property(xml, "baseForecast")?);
                }
                "changeForecast" => {
                    change_forecasts.push(parse_forecast_property(xml, "changeForecast")?);
                }
                _ => xml.skip(&child)?,
            },
            Event::End(end) if end.local_name().as_ref() == "TAF" => break,
            Event::Eof => return Err(unexpected_eof("TAF")),
            _ => {}
        }
    }

    Ok(Taf {
        report_status,
        permissible_usage,
        permissible_usage_reason,
        permissible_usage_supplementary,
        translated_bulletin_id,
        translated_bulletin_reception_time,
        translation_centre_designator,
        translation_centre_name,
        translation_time,
        translation_failed_tac,
        is_cancel_report,
        issue_time: issue_time.ok_or_else(|| TafDecodeError::missing("TAF.issueTime"))?,
        aerodrome: aerodrome.ok_or_else(|| TafDecodeError::missing("TAF.aerodrome"))?,
        valid_period,
        cancelled_report_valid_period,
        base_forecast,
        change_forecasts,
    })
}

fn parse_time_instant(xml: &mut Xml<'_>, wrapper: &str) -> Result<TimeInstantProperty> {
    let mut position = None;
    loop {
        match xml.event()? {
            Event::Start(element) if element.local_name().as_ref() == "timePosition" => {
                position = Some(required_text(xml, &element, "TAF.timePosition")?);
            }
            Event::End(end) if end.local_name().as_ref() == wrapper => break,
            Event::Eof => return Err(unexpected_eof("time instant")),
            _ => {}
        }
    }
    Ok(TimeInstantProperty {
        instant: TimeInstant {
            position: position.ok_or_else(|| TafDecodeError::missing("TAF.timePosition"))?,
        },
    })
}

fn parse_time_period(xml: &mut Xml<'_>, wrapper: &str) -> Result<TimePeriodProperty> {
    let mut begin = None;
    let mut end_position = None;
    loop {
        match xml.event()? {
            Event::Start(element) => match element.local_name().as_ref() {
                "beginPosition" => {
                    begin = Some(required_text(xml, &element, "TAF.timePeriod.begin")?);
                }
                "endPosition" => {
                    end_position = Some(required_text(xml, &element, "TAF.timePeriod.end")?);
                }
                _ => {}
            },
            Event::End(end) if end.local_name().as_ref() == wrapper => break,
            Event::Eof => return Err(unexpected_eof("time period")),
            _ => {}
        }
    }
    Ok(TimePeriodProperty {
        period: TimePeriod {
            begin: begin.ok_or_else(|| TafDecodeError::missing("TAF.timePeriod.begin"))?,
            end: end_position.ok_or_else(|| TafDecodeError::missing("TAF.timePeriod.end"))?,
        },
    })
}

fn parse_aerodrome(xml: &mut Xml<'_>) -> Result<AerodromeProperty> {
    let mut designator = None;
    let mut icao_identifier = None;
    let mut position = None;
    loop {
        match xml.event()? {
            Event::Start(element) => match element.local_name().as_ref() {
                "designator" => {
                    designator = Some(required_text(xml, &element, "TAF.aerodrome.designator")?);
                }
                "locationIndicatorICAO" => {
                    icao_identifier = Some(required_text(
                        xml,
                        &element,
                        "TAF.aerodrome.icaoIdentifier",
                    )?);
                }
                "pos" => {
                    position = Some(required_text(xml, &element, "TAF.aerodrome.position")?);
                }
                _ => {}
            },
            Event::End(end) if end.local_name().as_ref() == "aerodrome" => break,
            Event::Eof => return Err(unexpected_eof("aerodrome")),
            _ => {}
        }
    }

    Ok(AerodromeProperty {
        airport_heliport: AirportHeliport {
            time_slice: AirportHeliportTimeSliceProperty {
                value: AirportHeliportTimeSlice {
                    designator: designator
                        .ok_or_else(|| TafDecodeError::missing("TAF.aerodrome.designator"))?,
                    icao_identifier: icao_identifier
                        .ok_or_else(|| TafDecodeError::missing("TAF.aerodrome.icaoIdentifier"))?,
                    arp: position.map(|pos| ElevatedPointProperty {
                        point: ElevatedPoint { pos },
                    }),
                },
            },
        },
    })
}

fn parse_forecast_property(xml: &mut Xml<'_>, wrapper: &str) -> Result<ForecastProperty> {
    let mut forecast = None;
    loop {
        match xml.event()? {
            Event::Start(element)
                if element.local_name().as_ref() == "MeteorologicalAerodromeForecast" =>
            {
                forecast = Some(parse_forecast(xml, &element)?);
            }
            Event::End(end) if end.local_name().as_ref() == wrapper => break,
            Event::Eof => return Err(unexpected_eof("forecast property")),
            _ => {}
        }
    }
    Ok(ForecastProperty {
        forecast: forecast
            .ok_or_else(|| TafDecodeError::missing("TAF.forecastGroup.conditions"))?,
    })
}

fn parse_forecast(xml: &mut Xml<'_>, element: &BytesStart<'_>) -> Result<Forecast> {
    let cavok = parse_boolean(
        "TAF.forecastGroup.cloudAndVisibilityOK",
        &xml.required_attribute(
            element,
            "cloudAndVisibilityOK",
            "TAF.forecastGroup.cloudAndVisibilityOK",
        )?,
    )?;
    let change_indicator = xml.attribute(element, "changeIndicator")?;
    let mut phenomenon_time = None;
    let mut visibility = None;
    let mut visibility_operator = None;
    let mut surface_wind = None;
    let mut weather = Vec::with_capacity(3);
    let mut cloud = None;
    let mut temperature = Vec::with_capacity(2);

    loop {
        match xml.event()? {
            Event::Start(child) => match child.local_name().as_ref() {
                "phenomenonTime" => {
                    phenomenon_time = Some(parse_time_period(xml, "phenomenonTime")?);
                }
                "prevailingVisibility" => {
                    visibility = Some(parse_measure(xml, &child)?);
                }
                "prevailingVisibilityOperator" => {
                    visibility_operator = Some(required_text(
                        xml,
                        &child,
                        "TAF.forecastGroup.visibilityOperator",
                    )?);
                }
                "surfaceWind" => {
                    surface_wind = Some(parse_surface_wind_property(xml, &child)?);
                }
                "weather" => weather.push(parse_code_value(xml, &child)?),
                "cloud" => cloud = Some(parse_cloud_property(xml, &child)?),
                "temperature" => {
                    temperature.push(parse_temperature_property(xml)?);
                }
                _ => xml.skip(&child)?,
            },
            Event::Empty(child) => match child.local_name().as_ref() {
                "prevailingVisibility" => visibility = Some(empty_measure(xml, &child)?),
                "surfaceWind" => {
                    surface_wind = Some(empty_surface_wind_property(xml, &child)?);
                }
                "weather" => weather.push(empty_code_value(xml, &child)?),
                "cloud" => cloud = Some(empty_cloud_property(xml, &child)?),
                _ => {}
            },
            Event::End(end) if end.local_name().as_ref() == "MeteorologicalAerodromeForecast" => {
                break;
            }
            Event::Eof => return Err(unexpected_eof("forecast group")),
            _ => {}
        }
    }

    Ok(Forecast {
        cavok,
        change_indicator,
        phenomenon_time: phenomenon_time
            .ok_or_else(|| TafDecodeError::missing("TAF.forecastGroup.phenomenonTime"))?,
        visibility,
        visibility_operator,
        surface_wind,
        weather,
        cloud,
        temperature,
    })
}

fn parse_measure(xml: &mut Xml<'_>, element: &BytesStart<'_>) -> Result<Measure> {
    Ok(Measure {
        unit: xml.attribute(element, "uom")?,
        nil_reason: xml.attribute(element, "nilReason")?,
        value: nonempty(xml.text(element)?),
    })
}

fn empty_measure(xml: &Xml<'_>, element: &BytesStart<'_>) -> Result<Measure> {
    Ok(Measure {
        unit: xml.attribute(element, "uom")?,
        nil_reason: xml.attribute(element, "nilReason")?,
        value: None,
    })
}

fn parse_code_value(xml: &mut Xml<'_>, element: &BytesStart<'_>) -> Result<CodeValue> {
    let value = empty_code_value(xml, element)?;
    xml.skip(element)?;
    Ok(value)
}

fn empty_code_value(xml: &Xml<'_>, element: &BytesStart<'_>) -> Result<CodeValue> {
    Ok(CodeValue {
        href: xml.attribute(element, "href")?,
        nil_reason: xml.attribute(element, "nilReason")?,
    })
}

fn parse_surface_wind_property(
    xml: &mut Xml<'_>,
    element: &BytesStart<'_>,
) -> Result<SurfaceWindProperty> {
    let nil_reason = xml.attribute(element, "nilReason")?;
    let mut wind = None;
    loop {
        match xml.event()? {
            Event::Start(child)
                if child.local_name().as_ref() == "AerodromeSurfaceWindForecast" =>
            {
                wind = Some(parse_surface_wind(xml, &child)?);
            }
            Event::End(end) if end.local_name().as_ref() == "surfaceWind" => break,
            Event::Eof => return Err(unexpected_eof("surface wind")),
            _ => {}
        }
    }
    Ok(SurfaceWindProperty { nil_reason, wind })
}

fn empty_surface_wind_property(
    xml: &Xml<'_>,
    element: &BytesStart<'_>,
) -> Result<SurfaceWindProperty> {
    Ok(SurfaceWindProperty {
        nil_reason: xml.attribute(element, "nilReason")?,
        wind: None,
    })
}

fn parse_surface_wind(xml: &mut Xml<'_>, element: &BytesStart<'_>) -> Result<SurfaceWind> {
    let variable_direction = parse_boolean(
        "TAF.forecastGroup.wind.variableDirection",
        &xml.required_attribute(
            element,
            "variableWindDirection",
            "TAF.forecastGroup.wind.variableDirection",
        )?,
    )?;
    let mut mean_direction = None;
    let mut mean_speed = None;
    let mut mean_speed_operator = None;
    let mut gust_speed = None;
    let mut gust_speed_operator = None;
    loop {
        match xml.event()? {
            Event::Start(child) => match child.local_name().as_ref() {
                "meanWindDirection" => mean_direction = Some(parse_measure(xml, &child)?),
                "meanWindSpeed" => mean_speed = Some(parse_measure(xml, &child)?),
                "meanWindSpeedOperator" => {
                    mean_speed_operator = Some(required_text(
                        xml,
                        &child,
                        "TAF.forecastGroup.wind.speedOperator",
                    )?);
                }
                "windGustSpeed" => gust_speed = Some(parse_measure(xml, &child)?),
                "windGustSpeedOperator" => {
                    gust_speed_operator = Some(required_text(
                        xml,
                        &child,
                        "TAF.forecastGroup.wind.gustOperator",
                    )?);
                }
                _ => xml.skip(&child)?,
            },
            Event::Empty(child) => match child.local_name().as_ref() {
                "meanWindDirection" => mean_direction = Some(empty_measure(xml, &child)?),
                "meanWindSpeed" => mean_speed = Some(empty_measure(xml, &child)?),
                "windGustSpeed" => gust_speed = Some(empty_measure(xml, &child)?),
                _ => {}
            },
            Event::End(end) if end.local_name().as_ref() == "AerodromeSurfaceWindForecast" => {
                break;
            }
            Event::Eof => return Err(unexpected_eof("surface-wind conditions")),
            _ => {}
        }
    }
    Ok(SurfaceWind {
        variable_direction,
        mean_direction,
        mean_speed,
        mean_speed_operator,
        gust_speed,
        gust_speed_operator,
    })
}

fn parse_cloud_property(xml: &mut Xml<'_>, element: &BytesStart<'_>) -> Result<CloudProperty> {
    let nil_reason = xml.attribute(element, "nilReason")?;
    let mut forecast = None;
    loop {
        match xml.event()? {
            Event::Start(child) if child.local_name().as_ref() == "AerodromeCloudForecast" => {
                forecast = Some(parse_cloud_forecast(xml)?);
            }
            Event::End(end) if end.local_name().as_ref() == "cloud" => break,
            Event::Eof => return Err(unexpected_eof("cloud")),
            _ => {}
        }
    }
    Ok(CloudProperty {
        nil_reason,
        forecast,
    })
}

fn empty_cloud_property(xml: &Xml<'_>, element: &BytesStart<'_>) -> Result<CloudProperty> {
    Ok(CloudProperty {
        nil_reason: xml.attribute(element, "nilReason")?,
        forecast: None,
    })
}

fn parse_cloud_forecast(xml: &mut Xml<'_>) -> Result<CloudForecast> {
    let mut vertical_visibility = None;
    let mut layer = Vec::with_capacity(3);
    loop {
        match xml.event()? {
            Event::Start(child) => match child.local_name().as_ref() {
                "verticalVisibility" => {
                    vertical_visibility = Some(parse_measure(xml, &child)?);
                }
                "layer" => layer.push(parse_cloud_layer_property(xml)?),
                _ => xml.skip(&child)?,
            },
            Event::Empty(child) if child.local_name().as_ref() == "verticalVisibility" => {
                vertical_visibility = Some(empty_measure(xml, &child)?);
            }
            Event::End(end) if end.local_name().as_ref() == "AerodromeCloudForecast" => break,
            Event::Eof => return Err(unexpected_eof("cloud forecast")),
            _ => {}
        }
    }
    Ok(CloudForecast {
        vertical_visibility,
        layer,
    })
}

fn parse_cloud_layer_property(xml: &mut Xml<'_>) -> Result<CloudLayerProperty> {
    let mut layer = None;
    loop {
        match xml.event()? {
            Event::Start(child) if child.local_name().as_ref() == "CloudLayer" => {
                layer = Some(parse_cloud_layer(xml)?);
            }
            Event::End(end) if end.local_name().as_ref() == "layer" => break,
            Event::Eof => return Err(unexpected_eof("cloud layer property")),
            _ => {}
        }
    }
    Ok(CloudLayerProperty {
        layer: layer.ok_or_else(|| TafDecodeError::missing("TAF.forecastGroup.cloud.layer"))?,
    })
}

fn parse_cloud_layer(xml: &mut Xml<'_>) -> Result<CloudLayer> {
    let mut amount = None;
    let mut base = None;
    let mut cloud_type = None;
    loop {
        match xml.event()? {
            Event::Start(child) => match child.local_name().as_ref() {
                "amount" => amount = Some(parse_code_value(xml, &child)?),
                "base" => base = Some(parse_measure(xml, &child)?),
                "cloudType" => cloud_type = Some(parse_code_value(xml, &child)?),
                _ => xml.skip(&child)?,
            },
            Event::Empty(child) => match child.local_name().as_ref() {
                "amount" => amount = Some(empty_code_value(xml, &child)?),
                "base" => base = Some(empty_measure(xml, &child)?),
                "cloudType" => cloud_type = Some(empty_code_value(xml, &child)?),
                _ => {}
            },
            Event::End(end) if end.local_name().as_ref() == "CloudLayer" => break,
            Event::Eof => return Err(unexpected_eof("cloud layer")),
            _ => {}
        }
    }
    Ok(CloudLayer {
        amount: amount
            .ok_or_else(|| TafDecodeError::missing("TAF.forecastGroup.cloud.layer.amount"))?,
        base: base.ok_or_else(|| TafDecodeError::missing("TAF.forecastGroup.cloud.layer.base"))?,
        cloud_type,
    })
}

fn parse_temperature_property(xml: &mut Xml<'_>) -> Result<TemperatureProperty> {
    let mut forecast = None;
    loop {
        match xml.event()? {
            Event::Start(child)
                if child.local_name().as_ref() == "AerodromeAirTemperatureForecast" =>
            {
                forecast = Some(parse_air_temperature(xml)?);
            }
            Event::End(end) if end.local_name().as_ref() == "temperature" => break,
            Event::Eof => return Err(unexpected_eof("temperature property")),
            _ => {}
        }
    }
    Ok(TemperatureProperty {
        forecast: forecast
            .ok_or_else(|| TafDecodeError::missing("TAF.baseForecast.temperature"))?,
    })
}

fn parse_air_temperature(xml: &mut Xml<'_>) -> Result<AirTemperatureForecast> {
    let mut maximum = None;
    let mut maximum_time = None;
    let mut minimum = None;
    let mut minimum_time = None;
    loop {
        match xml.event()? {
            Event::Start(child) => match child.local_name().as_ref() {
                "maximumAirTemperature" => maximum = Some(parse_measure(xml, &child)?),
                "maximumAirTemperatureTime" => {
                    maximum_time = Some(parse_time_instant(xml, "maximumAirTemperatureTime")?);
                }
                "minimumAirTemperature" => minimum = Some(parse_measure(xml, &child)?),
                "minimumAirTemperatureTime" => {
                    minimum_time = Some(parse_time_instant(xml, "minimumAirTemperatureTime")?);
                }
                _ => xml.skip(&child)?,
            },
            Event::Empty(child) => match child.local_name().as_ref() {
                "maximumAirTemperature" => maximum = Some(empty_measure(xml, &child)?),
                "minimumAirTemperature" => minimum = Some(empty_measure(xml, &child)?),
                _ => {}
            },
            Event::End(end) if end.local_name().as_ref() == "AerodromeAirTemperatureForecast" => {
                break;
            }
            Event::Eof => return Err(unexpected_eof("air-temperature forecast")),
            _ => {}
        }
    }
    Ok(AirTemperatureForecast {
        maximum: maximum
            .ok_or_else(|| TafDecodeError::missing("TAF.baseForecast.temperature.maximum"))?,
        maximum_time: maximum_time
            .ok_or_else(|| TafDecodeError::missing("TAF.baseForecast.temperature.maximumTime"))?,
        minimum: minimum
            .ok_or_else(|| TafDecodeError::missing("TAF.baseForecast.temperature.minimum"))?,
        minimum_time: minimum_time
            .ok_or_else(|| TafDecodeError::missing("TAF.baseForecast.temperature.minimumTime"))?,
    })
}

fn required_text(
    xml: &mut Xml<'_>,
    element: &BytesStart<'_>,
    path: &'static str,
) -> Result<String> {
    nonempty(xml.text(element)?).ok_or_else(|| TafDecodeError::missing(path))
}

fn nonempty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}

fn parse_boolean(path: &'static str, value: &str) -> Result<bool> {
    match value {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(TafDecodeError::classified(
            TafDecodeErrorKind::InvalidValue,
            path,
            format!("invalid XML boolean {value:?}"),
        )),
    }
}

fn unexpected_eof(element: &'static str) -> TafDecodeError {
    TafDecodeError::classified(
        TafDecodeErrorKind::MalformedXml,
        element,
        "unexpected end of IWXXM document",
    )
}
