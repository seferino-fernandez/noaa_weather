//! Aviation weather products: SIGMETs and Center Weather Advisories (CWAs).
//!
//! Covers the `/aviation` endpoints for in-flight weather hazard reports
//! issued by Air Traffic Service Units and Center Weather Service Units.

use super::{Error, configuration, http};
use crate::models;

/// Returns a specific Center Weather Advisory (CWA) identified by CWSU, date, and sequence number.
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}/cwas/{date}/{sequence}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the issuing Center Weather Service Unit (CWSU).
/// * `date`: The date of the advisory in `YYYY-MM-DD` format.
/// * `sequence`: The sequence number of the advisory (must be >= 100).
///
/// # Returns
///
/// A `Result` containing a [`models::CenterWeatherAdvisoryGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_advisories_by_date_and_sequence(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
    date: String,
    sequence: i32,
) -> Result<models::CenterWeatherAdvisoryGeoJson, Error> {
    let uri_str = format!(
        "/aviation/cwsus/{center_weather_service_unit_id}/cwas/{date}/{sequence}",
        center_weather_service_unit_id = center_weather_service_unit_id,
        date = date,
        sequence = sequence
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a collection of current Center Weather Advisories (CWAs) for a specific Center Weather Service Unit (CWSU).
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}/cwas` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the Center Weather Service Unit (CWSU).
///
/// # Returns
///
/// A `Result` containing a [`models::CenterWeatherAdvisoryCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_advisories(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<models::CenterWeatherAdvisoryCollectionGeoJson, Error> {
    let uri_str = format!(
        "/aviation/cwsus/{center_weather_service_unit_id}/cwas",
        center_weather_service_unit_id = center_weather_service_unit_id
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns metadata about a specific Center Weather Service Unit (CWSU).
///
/// Corresponds to the `/aviation/cwsus/{center_weather_service_unit_id}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `center_weather_service_unit_id`: The ID of the Center Weather Service Unit (CWSU).
///
/// # Returns
///
/// A `Result` containing [`models::Office`] metadata on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_center_weather_service_unit(
    configuration: &configuration::Configuration,
    center_weather_service_unit_id: models::NwsCenterWeatherServiceUnitId,
) -> Result<models::CwsuOffice, Error> {
    let uri_str = format!(
        "/aviation/cwsus/{center_weather_service_unit_id}",
        center_weather_service_unit_id = center_weather_service_unit_id
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a specific SIGMET or AIRMET product.
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}/{date}/{time}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the issuing Air Traffic Service Unit (ATSU).
/// * `date`: The date of issuance in `YYYY-MM-DD` format.
/// * `time`: The time of issuance in `HHMM` format (UTC).
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmet(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
    date: String,
    time: &str,
) -> Result<models::SigmetGeoJson, Error> {
    let uri_str = format!(
        "/aviation/sigmets/{air_traffic_service_unit}/{date}/{time}",
        air_traffic_service_unit = crate::apis::urlencode(air_traffic_service_unit),
        date = date,
        time = crate::apis::urlencode(time)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a collection of SIGMET/AIRMET products based on query parameters.
///
/// Corresponds to the `/aviation/sigmets` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `start`: Optional start time for the query period (ISO 8601 format).
/// * `end`: Optional end time for the query period (ISO 8601 format).
/// * `date`: Optional date filter (`YYYY-MM-DD` format).
/// * `air_traffic_service_unit`: Optional Air Traffic Service Unit (ATSU) identifier filter.
/// * `sequence`: Optional sequence number filter.
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets(
    configuration: &configuration::Configuration,
    start: Option<String>,
    end: Option<String>,
    date: Option<String>,
    air_traffic_service_unit: Option<&str>,
    sequence: Option<&str>,
) -> Result<models::SigmetCollectionGeoJson, Error> {
    let uri_str = "/aviation/sigmets".to_owned();
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = start {
        req_builder = req_builder.query(&[("start", &param_value)]);
    }
    if let Some(param_value) = end {
        req_builder = req_builder.query(&[("end", &param_value)]);
    }
    if let Some(param_value) = date {
        req_builder = req_builder.query(&[("date", &param_value)]);
    }
    if let Some(param_value) = air_traffic_service_unit {
        req_builder = req_builder.query(&[("atsu", &param_value)]);
    }
    if let Some(param_value) = sequence {
        req_builder = req_builder.query(&[("sequence", &param_value)]);
    }

    req_builder.json().await
}

/// Returns a collection of SIGMET/AIRMET products for a specific Air Traffic Service Unit (ATSU).
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the Air Traffic Service Unit (ATSU).
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets_by_air_traffic_service_unit(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
) -> Result<models::SigmetCollectionGeoJson, Error> {
    let uri_str = format!(
        "/aviation/sigmets/{air_traffic_service_unit}",
        air_traffic_service_unit = crate::apis::urlencode(air_traffic_service_unit)
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a collection of SIGMET/AIRMET products for a specific Air Traffic Service Unit (ATSU) on a specific date.
///
/// Corresponds to the `/aviation/sigmets/{air_traffic_service_unit}/{date}` endpoint.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `air_traffic_service_unit`: The identifier of the Air Traffic Service Unit (ATSU).
/// * `date`: The date filter in `YYYY-MM-DD` format.
///
/// # Returns
///
/// A `Result` containing a [`models::SigmetCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response cannot be parsed.
pub async fn get_sigmets_by_air_traffic_service_unit_and_date(
    configuration: &configuration::Configuration,
    air_traffic_service_unit: &str,
    date: String,
) -> Result<models::SigmetCollectionGeoJson, Error> {
    let uri_str = format!(
        "/aviation/sigmets/{air_traffic_service_unit}/{date}",
        air_traffic_service_unit = crate::apis::urlencode(air_traffic_service_unit),
        date = date
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}
