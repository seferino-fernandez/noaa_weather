//! Data models for NOAA Weather API responses.
//!
//! All types implement [`serde::Serialize`] and [`serde::Deserialize`] and
//! map directly to the JSON (or XML) payloads returned by `api.weather.gov`.
//!
//! Measurements are [`Quantity`] and its [`Unit`] vocabulary; response
//! timestamps are [`crate::time::OffsetDateTime`] and response periods are
//! [`crate::time::Interval`].

pub mod astronomical_data;
pub use self::astronomical_data::AstronomicalData;
pub mod alert;
pub use self::alert::{
    ActiveAlertCounts, Alert, AlertCategory, AlertCertainty, AlertEventTypes, AlertGeocode,
    AlertMessageType, AlertReference, AlertResponse, AlertScope, AlertSeverity, AlertStatus,
    AlertUrgency,
};
pub mod area_code;
pub use self::area_code::AreaCode;
pub mod center_weather_advisory;
pub use self::center_weather_advisory::CenterWeatherAdvisory;
pub mod cwsu_office;
pub use self::cwsu_office::{CwsuOffice, CwsuOfficeAddress, CwsuOfficeContext};
pub mod gridpoint;
pub use self::gridpoint::{
    Forecast, ForecastGenerator, ForecastPeriod, ForecastUnits, Gridpoint, GridpointLayer, Hazard,
    HazardPeriod, HazardsLayer, LayerValue, TemperatureTrend, WeatherAttribute, WeatherCondition,
    WeatherCoverage, WeatherIntensity, WeatherLayer, WeatherPeriod, WeatherPhenomenon,
    WindDirection,
};
pub mod glossary;
pub use self::glossary::{GlossaryResponse, GlossaryTerm};
pub mod json_ld_context;
pub use self::json_ld_context::{JsonLdContext, JsonLdContextElement};
pub mod land_region_code;
pub use self::land_region_code::LandRegionCode;
pub mod marine_area_code;
pub use self::marine_area_code::MarineAreaCode;
pub mod marine_region_code;
pub use self::marine_region_code::MarineRegionCode;
pub mod metar_phenomenon;
pub use self::metar_phenomenon::MetarPhenomenon;
pub mod metar_sky_coverage;
pub use self::metar_sky_coverage::MetarSkyCoverage;
pub mod nws_center_weather_service_unit_id;
pub use self::nws_center_weather_service_unit_id::NwsCenterWeatherServiceUnitId;
pub mod nws_forecast_office_id;
pub use self::nws_forecast_office_id::NwsForecastOfficeId;
pub mod nws_national_hqid;
pub use self::nws_national_hqid::NwsNationalHqid;
pub mod nws_office_id;
pub use self::nws_office_id::{NwsOfficeId, ParseNwsOfficeIdError};
pub mod nws_regional_hqid;
pub use self::nws_regional_hqid::NwsRegionalHqid;
pub mod nws_unit_code;
pub use self::nws_unit_code::NwsUnitCode;
pub mod noaa_weather_radio;
pub use self::noaa_weather_radio::NoaaWeatherRadio;
pub mod observation;
pub use self::observation::{Observation, ObservationCloudLayer};
pub mod observation_station;
pub use self::observation_station::{ObservationStation, ObservationType};
pub mod office;
pub use self::office::Office;
pub mod office_address;
pub use self::office_address::OfficeAddress;
pub mod office_headline;
pub use self::office_headline::OfficeHeadline;
pub mod office_headline_collection;
pub use self::office_headline_collection::OfficeHeadlineCollection;
pub mod office_documents;
pub use self::office_documents::{
    NwsConnectDocumentMetadata, OfficeBriefing, OfficeBriefingResponse, OfficeWeatherStory,
    OfficeWeatherStoryCollection,
};
pub mod point;
pub use self::point::{Point, PointType, RelativeLocation};
pub mod problem_detail;
pub use self::problem_detail::ProblemDetail;
pub mod quality_control;
pub use self::quality_control::QualityControl;
pub mod radar;
pub use self::radar::{RadarNormalizationError, RadarServerTelemetry, RadarStationTelemetry};
pub mod radar_queue;
pub use self::radar_queue::RadarQueue;
pub mod radar_queue_host;
pub use self::radar_queue_host::RadarQueueHost;
pub mod radar_queues_response;
pub use self::radar_queues_response::RadarQueuesResponse;
pub mod radar_server;
pub use self::radar_server::RadarServer;
pub mod radar_servers_response;
pub use self::radar_servers_response::RadarServersResponse;
pub mod radar_station_alarm;
pub use self::radar_station_alarm::RadarStationAlarm;
pub mod radar_station_alarms_response;
pub use self::radar_station_alarms_response::RadarStationAlarmsResponse;
pub mod radar_station;
pub use self::radar_station::{RadarStation, RadarStationFeature};
pub mod radar_stations_response;
pub use self::radar_stations_response::RadarStationsResponse;
pub mod radar_spgds;
pub use self::radar_spgds::{
    RadarSpgdsDiskStatus, RadarSpgdsEntry, RadarSpgdsGatewayStatus, RadarSpgdsLdmStatus,
    RadarSpgdsResponse, RadarSpgdsStatus, RadarSpgdsThroughput, RadarSpgdsUptime,
};
pub mod region_code;
pub use self::region_code::RegionCode;
pub mod sigmet;
pub use self::sigmet::Sigmet;
pub mod state_territory_code;
pub use self::state_territory_code::StateTerritoryCode;
pub mod terminal_aerodrome_forecast;
pub use self::terminal_aerodrome_forecast::TerminalAerodromeForecast;
pub mod terminal_aerodrome_forecasts_response;
pub use self::terminal_aerodrome_forecasts_response::TerminalAerodromeForecastsResponse;
pub mod product;
pub use self::product::{
    TextProduct, TextProductCollection, TextProductLocationCollection, TextProductType,
    TextProductTypeCollection,
};
pub mod units;
pub use self::units::{Quantity, Unit, UnitCodeType, ValueUnit};
pub mod wmo_unit_code;
pub use self::wmo_unit_code::WmoUnitCode;
pub mod zone;
pub use self::zone::{
    Zone, ZoneForecast, ZoneForecastPeriod, ZoneResourceType, ZoneState, ZoneType,
};
pub mod radio;
pub use self::radio::{
    BroadcastMark, Paragraph, RadioBroadcast, RadioTransmitter, RadioTransmitterCollection, SayAs,
    Sentence, SentenceContent,
};
