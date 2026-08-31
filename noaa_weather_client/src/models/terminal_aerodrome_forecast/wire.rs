mod decode;

pub(super) fn decode(bytes: &[u8]) -> Result<Bulletin, super::TafDecodeError> {
    decode::decode(bytes)
}

pub(super) struct Bulletin {
    pub(super) meteorological_information: MeteorologicalInformation,
    pub(super) bulletin_identifier: String,
}

pub(super) struct MeteorologicalInformation {
    pub(super) taf: Taf,
}

pub(super) struct Taf {
    pub(super) report_status: String,
    pub(super) permissible_usage: String,
    pub(super) permissible_usage_reason: Option<String>,
    pub(super) permissible_usage_supplementary: Option<String>,
    pub(super) translated_bulletin_id: Option<String>,
    pub(super) translated_bulletin_reception_time: Option<String>,
    pub(super) translation_centre_designator: Option<String>,
    pub(super) translation_centre_name: Option<String>,
    pub(super) translation_time: Option<String>,
    pub(super) translation_failed_tac: Option<String>,
    pub(super) is_cancel_report: bool,
    pub(super) issue_time: TimeInstantProperty,
    pub(super) aerodrome: AerodromeProperty,
    pub(super) valid_period: Option<TimePeriodProperty>,
    pub(super) cancelled_report_valid_period: Option<TimePeriodProperty>,
    pub(super) base_forecast: Option<ForecastProperty>,
    pub(super) change_forecasts: Vec<ForecastProperty>,
}

pub(super) struct AerodromeProperty {
    pub(super) airport_heliport: AirportHeliport,
}

pub(super) struct AirportHeliport {
    pub(super) time_slice: AirportHeliportTimeSliceProperty,
}

pub(super) struct AirportHeliportTimeSliceProperty {
    pub(super) value: AirportHeliportTimeSlice,
}

pub(super) struct AirportHeliportTimeSlice {
    pub(super) designator: String,
    pub(super) icao_identifier: String,
    pub(super) arp: Option<ElevatedPointProperty>,
}

pub(super) struct ElevatedPointProperty {
    pub(super) point: ElevatedPoint,
}

pub(super) struct ElevatedPoint {
    pub(super) pos: String,
}

pub(super) struct TimeInstantProperty {
    pub(super) instant: TimeInstant,
}

pub(super) struct TimeInstant {
    pub(super) position: String,
}

pub(super) struct TimePeriodProperty {
    pub(super) period: TimePeriod,
}

pub(super) struct TimePeriod {
    pub(super) begin: String,
    pub(super) end: String,
}

pub(super) struct ForecastProperty {
    pub(super) forecast: Forecast,
}

pub(super) struct Forecast {
    pub(super) cavok: bool,
    pub(super) change_indicator: Option<String>,
    pub(super) phenomenon_time: TimePeriodProperty,
    pub(super) visibility: Option<Measure>,
    pub(super) visibility_operator: Option<String>,
    pub(super) surface_wind: Option<SurfaceWindProperty>,
    pub(super) weather: Vec<CodeValue>,
    pub(super) cloud: Option<CloudProperty>,
    pub(super) temperature: Vec<TemperatureProperty>,
}

pub(super) struct Measure {
    pub(super) unit: Option<String>,
    pub(super) nil_reason: Option<String>,
    pub(super) value: Option<String>,
}

pub(super) struct CodeValue {
    pub(super) href: Option<String>,
    pub(super) nil_reason: Option<String>,
}

pub(super) struct SurfaceWindProperty {
    pub(super) nil_reason: Option<String>,
    pub(super) wind: Option<SurfaceWind>,
}

pub(super) struct SurfaceWind {
    pub(super) variable_direction: bool,
    pub(super) mean_direction: Option<Measure>,
    pub(super) mean_speed: Option<Measure>,
    pub(super) mean_speed_operator: Option<String>,
    pub(super) gust_speed: Option<Measure>,
    pub(super) gust_speed_operator: Option<String>,
}

pub(super) struct CloudProperty {
    pub(super) nil_reason: Option<String>,
    pub(super) forecast: Option<CloudForecast>,
}

pub(super) struct CloudForecast {
    pub(super) vertical_visibility: Option<Measure>,
    pub(super) layer: Vec<CloudLayerProperty>,
}

pub(super) struct CloudLayerProperty {
    pub(super) layer: CloudLayer,
}

pub(super) struct CloudLayer {
    pub(super) amount: CodeValue,
    pub(super) base: Measure,
    pub(super) cloud_type: Option<CodeValue>,
}

pub(super) struct TemperatureProperty {
    pub(super) forecast: AirTemperatureForecast,
}

pub(super) struct AirTemperatureForecast {
    pub(super) maximum: Measure,
    pub(super) maximum_time: TimeInstantProperty,
    pub(super) minimum: Measure,
    pub(super) minimum_time: TimeInstantProperty,
}
