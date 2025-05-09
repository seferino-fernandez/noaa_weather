use serde::{Deserialize, Serialize};
use std::option::Option;

#[derive(Serialize, Deserialize)]
pub struct TerminalAerodromeForecast {
    #[serde(rename = "@xmlns:ns0")]
    pub xmlns_ns0: String,
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "@xmlns:ns1")]
    pub xmlns_ns1: String,
    #[serde(rename = "@schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "meteorologicalInformation")]
    pub ns0_meteorological_information: Ns0MeteorologicalInformation,
    #[serde(rename = "bulletinIdentifier")]
    pub ns0_bulletin_identifier: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ns0MeteorologicalInformation {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TAF")]
    pub taf: Taf,
}

#[derive(Serialize, Deserialize)]
pub struct Taf {
    #[serde(rename = "@xmlns:aixm")]
    pub xmlns_aixm: String,
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:xlink")]
    pub xmlns_xlink: String,
    #[serde(rename = "@schemaLocation")]
    pub xsi_schema_location: String,
    #[serde(rename = "@reportStatus")]
    pub report_status: String,
    #[serde(rename = "@permissibleUsage")]
    pub permissible_usage: String,
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "issueTime")]
    pub issue_time: IssueTime,
    pub aerodrome: Aerodrome,
    #[serde(rename = "validPeriod")]
    pub valid_period: ValidPeriod,
    #[serde(rename = "baseForecast")]
    pub base_forecast: BaseForecast,
    #[serde(rename = "changeForecast")]
    pub change_forecast: Vec<ChangeForecast>,
}

#[derive(Serialize, Deserialize)]
pub struct IssueTime {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TimeInstant")]
    pub ns1_time_instant: Ns1TimeInstant,
}

#[derive(Serialize, Deserialize)]
pub struct Ns1TimeInstant {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "timePosition")]
    pub ns1_time_position: String,
}

#[derive(Serialize, Deserialize)]
pub struct Aerodrome {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AirportHeliport")]
    pub aixm_airport_heliport: AixmAirportHeliport,
}

#[derive(Serialize, Deserialize)]
pub struct AixmAirportHeliport {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "timeSlice")]
    pub aixm_time_slice: AixmTimeSlice,
}

#[derive(Serialize, Deserialize)]
pub struct AixmTimeSlice {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AirportHeliportTimeSlice")]
    pub aixm_airport_heliport_time_slice: AixmAirportHeliportTimeSlice,
}

#[derive(Serialize, Deserialize)]
pub struct AixmAirportHeliportTimeSlice {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "validTime")]
    pub ns1_valid_time: Ns1ValidTime,
    #[serde(rename = "interpretation")]
    pub aixm_interpretation: String,
    #[serde(rename = "designator")]
    pub aixm_designator: String,
    #[serde(rename = "locationIndicatorICAO")]
    pub aixm_location_indicator_icao: String,
    #[serde(rename = "ARP")]
    pub aixm_arp: AixmArp,
}

#[derive(Serialize, Deserialize)]
pub struct Ns1ValidTime {}

#[derive(Serialize, Deserialize)]
pub struct AixmArp {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElevatedPoint")]
    pub aixm_elevated_point: AixmElevatedPoint,
}

#[derive(Serialize, Deserialize)]
pub struct AixmElevatedPoint {
    #[serde(rename = "@srsDimension")]
    pub srs_dimension: String,
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "@axisLabels")]
    pub axis_labels: String,
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "pos")]
    pub ns1_pos: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidPeriod {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TimePeriod")]
    pub ns1_time_period: Ns0MeteorologicalInformationTafValidPeriodNs1TimePeriod,
}

#[derive(Serialize, Deserialize)]
pub struct Ns0MeteorologicalInformationTafValidPeriodNs1TimePeriod {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "beginPosition")]
    pub ns1_begin_position: String,
    #[serde(rename = "endPosition")]
    pub ns1_end_position: String,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecast {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MeteorologicalAerodromeForecast")]
    pub meteorological_aerodrome_forecast: BaseForecastMeteorologicalAerodromeForecast,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecast {
    #[serde(rename = "@cloudAndVisibilityOK")]
    pub cloud_and_visibility_ok: String,
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "phenomenonTime")]
    pub phenomenon_time: Option<BaseForecastMeteorologicalAerodromeForecastPhenomenonTime>,
    #[serde(rename = "prevailingVisibility")]
    pub prevailing_visibility:
        Option<BaseForecastMeteorologicalAerodromeForecastPrevailingVisibility>,
    #[serde(rename = "prevailingVisibilityOperator")]
    pub prevailing_visibility_operator: Option<String>,
    #[serde(rename = "surfaceWind")]
    pub surface_wind: Option<BaseForecastMeteorologicalAerodromeForecastSurfaceWind>,
    pub cloud: Option<BaseForecastMeteorologicalAerodromeForecastCloud>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastPhenomenonTime {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TimePeriod")]
    pub ns1_time_period: BaseForecastMeteorologicalAerodromeForecastPhenomenonTimeNs1TimePeriod,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastPhenomenonTimeNs1TimePeriod {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "beginPosition")]
    pub ns1_begin_position: String,
    #[serde(rename = "endPosition")]
    pub ns1_end_position: String,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastPrevailingVisibility {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastSurfaceWind {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AerodromeSurfaceWindForecast")]
    pub aerodrome_surface_wind_forecast:
        BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast {
    #[serde(rename = "@variableWindDirection")]
    pub variable_wind_direction: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "meanWindDirection")]
    pub mean_wind_direction: Option<BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindDirection>,
    #[serde(rename = "meanWindSpeed")]
    pub mean_wind_speed: Option<BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindSpeed>,
    #[serde(rename = "windGustSpeed")]
    pub wind_gust_speed: Option<BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastWindGustSpeed>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindDirection
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindSpeed
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastWindGustSpeed
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastCloud {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AerodromeCloudForecast")]
    pub aerodrome_cloud_forecast:
        BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecast,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecast {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub layer: Vec<BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CloudLayer")]
    pub cloud_layer:
        BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayer,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayer {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub amount:
        BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerAmount,
    pub base:
        BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerBase,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerAmount
{
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Serialize, Deserialize)]
pub struct BaseForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerBase
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecast {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MeteorologicalAerodromeForecast")]
    pub meteorological_aerodrome_forecast: ChangeForecastMeteorologicalAerodromeForecast,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecast {
    #[serde(rename = "@cloudAndVisibilityOK")]
    pub cloud_and_visibility_ok: String,
    #[serde(rename = "@changeIndicator")]
    pub change_indicator: String,
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub weather: Option<Vec<Weather>>,
    #[serde(rename = "phenomenonTime")]
    pub phenomenon_time: ChangeForecastMeteorologicalAerodromeForecastPhenomenonTime,
    #[serde(rename = "prevailingVisibility")]
    pub prevailing_visibility:
        Option<ChangeForecastMeteorologicalAerodromeForecastPrevailingVisibility>,
    pub cloud: ChangeForecastMeteorologicalAerodromeForecastCloud,
    #[serde(rename = "surfaceWind")]
    pub surface_wind: Option<ChangeForecastMeteorologicalAerodromeForecastSurfaceWind>,
    #[serde(rename = "prevailingVisibilityOperator")]
    pub prevailing_visibility_operator: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastPhenomenonTime {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TimePeriod")]
    pub ns1_time_period: ChangeForecastMeteorologicalAerodromeForecastPhenomenonTimeNs1TimePeriod,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastPhenomenonTimeNs1TimePeriod {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "beginPosition")]
    pub ns1_begin_position: String,
    #[serde(rename = "endPosition")]
    pub ns1_end_position: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastPrevailingVisibility {
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastCloud {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AerodromeCloudForecast")]
    pub aerodrome_cloud_forecast:
        ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecast,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecast {
    #[serde(rename = "@id")]
    pub ns1_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub layer: Vec<ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayer {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "CloudLayer")]
    pub cloud_layer:
        ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayer,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayer {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub amount: ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerAmount,
    pub base: ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerBase,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerAmount
{
    #[serde(rename = "@href")]
    pub xlink_href: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastCloudAerodromeCloudForecastLayerCloudLayerBase
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastSurfaceWind {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AerodromeSurfaceWindForecast")]
    pub aerodrome_surface_wind_forecast:
        ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecast {
    #[serde(rename = "@variableWindDirection")]
    pub variable_wind_direction: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "windGustSpeed")]
    pub wind_gust_speed: Option<ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastWindGustSpeed>,
    #[serde(rename = "meanWindDirection")]
    pub mean_wind_direction: Option<ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindDirection>,
    #[serde(rename = "meanWindSpeed")]
    pub mean_wind_speed: Option<ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindSpeed>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastWindGustSpeed
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindDirection
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ChangeForecastMeteorologicalAerodromeForecastSurfaceWindAerodromeSurfaceWindForecastMeanWindSpeed
{
    #[serde(rename = "@uom")]
    pub uom: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}
