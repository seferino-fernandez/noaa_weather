//! Gridpoint-based forecasts and raw observation data.
//!
//! Covers the `/gridpoints/{wfo}/{x},{y}` endpoints. Use a point's metadata
//! (from [`super::points::get_point`]) to obtain the forecast office and grid
//! coordinates needed by these functions.

use super::{Error, configuration, http};
use crate::models;

/// Returns raw numerical forecast data for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}` endpoint.
/// This endpoint provides detailed forecast data layers like temperature, humidity, wind, etc.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office (e.g., TOP, LWX).
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
///
/// # Returns
///
/// A `Result` containing a [`models::GridpointGeoJson`] on success, which includes the detailed
/// forecast layers in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., invalid grid coordinates)
/// or the response cannot be parsed.
pub async fn get_gridpoint(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
) -> Result<models::GridpointGeoJson, Error> {
    let uri_str = format!(
        "/gridpoints/{forecast_office_id}/{x},{y}",
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}

/// Returns a textual forecast for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/forecast` endpoint.
/// This provides a human-readable, multi-day forecast summary.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office.
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
/// * `units`: Optional units for the forecast (us or si).
///
/// # Returns
///
/// A `Result` containing a [`models::Gridpoint12hForecastGeoJson`] on success, which includes
/// forecast periods with textual descriptions in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_gridpoint_forecast(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    units: Option<models::GridpointForecastUnits>,
) -> Result<models::Gridpoint12hForecastGeoJson, Error> {
    let uri_str = format!(
        "/gridpoints/{forecast_office_id}/{x},{y}/forecast",
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = units {
        req_builder = req_builder.query(&[("units", &param_value.to_string())]);
    }
    req_builder = req_builder.header(
        "Feature-Flags",
        "forecast_temperature_qv,forecast_wind_speed_qv",
    );

    req_builder.json().await
}

/// Returns a textual hourly forecast for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/forecast/hourly` endpoint.
/// This provides a human-readable, hour-by-hour forecast summary.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office.
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
/// * `units`: Optional units for the forecast (us or si).
///
/// # Returns
///
/// A `Result` containing a [`models::GridpointHourlyForecastGeoJson`] on success, which includes
/// hourly forecast periods with textual descriptions in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_gridpoint_forecast_hourly(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    units: Option<models::GridpointForecastUnits>,
) -> Result<models::GridpointHourlyForecastGeoJson, Error> {
    let uri_str = format!(
        "/gridpoints/{forecast_office_id}/{x},{y}/forecast/hourly",
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = units {
        req_builder = req_builder.query(&[("units", &param_value.to_string())]);
    }
    req_builder = req_builder.header(
        "Feature-Flags",
        "forecast_temperature_qv,forecast_wind_speed_qv",
    );

    req_builder.json().await
}

/// Returns a list of observation stations usable for a given 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/stations` endpoint.
/// This helps identify nearby stations for obtaining current observations.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
/// * `forecast_office_id`: The ID of the NWS forecast office.
/// * `x`: The grid X coordinate.
/// * `y`: The grid Y coordinate.
/// * `limit`: Optional limit on the number of stations returned.
///
/// # Returns
///
/// A `Result` containing an [`models::ObservationStationCollectionGeoJson`] on success.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails or the response
/// cannot be parsed.
pub async fn get_gridpoint_stations(
    configuration: &configuration::Configuration,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    limit: Option<i32>,
) -> Result<models::ObservationStationCollectionGeoJson, Error> {
    let uri_str = format!(
        "/gridpoints/{forecast_office_id}/{x},{y}/stations",
        forecast_office_id = forecast_office_id,
        x = x,
        y = y
    );
    let mut req_builder = http::get(configuration, &uri_str);

    if let Some(param_value) = limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    req_builder.json().await
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{get_gridpoint_forecast, get_gridpoint_forecast_hourly, get_gridpoint_stations};
    use crate::{
        apis::configuration::Configuration,
        models::{GridpointForecastUnits, NwsForecastOfficeId},
    };

    const FORECAST: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
    const STATIONS: &str = r#"{"type":"FeatureCollection","features":[]}"#;
    const REQUIRED_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

    fn configuration(server: &MockServer) -> Configuration {
        Configuration::new(None, Some(server.uri()), None, None)
    }

    async fn mount_json(server: &MockServer, body: &'static str) {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/geo+json"))
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn forecast_always_sends_quantitative_flags_and_units_query() {
        let server = MockServer::start().await;
        mount_json(&server, FORECAST).await;
        get_gridpoint_forecast(
            &configuration(&server),
            NwsForecastOfficeId::Psr,
            159,
            100,
            Some(GridpointForecastUnits::Si),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].headers["feature-flags"], REQUIRED_FLAGS);
        assert_eq!(requests[0].url.query(), Some("units=si"));
    }

    #[tokio::test]
    async fn hourly_forecast_always_sends_quantitative_flags() {
        let server = MockServer::start().await;
        mount_json(&server, FORECAST).await;
        get_gridpoint_forecast_hourly(
            &configuration(&server),
            NwsForecastOfficeId::Psr,
            159,
            100,
            None,
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].headers["feature-flags"], REQUIRED_FLAGS);
        assert_eq!(requests[0].url.query(), None);
    }

    #[tokio::test]
    async fn gridpoint_stations_preserves_limit_and_omits_feature_flags() {
        let server = MockServer::start().await;
        mount_json(&server, STATIONS).await;
        get_gridpoint_stations(
            &configuration(&server),
            NwsForecastOfficeId::Psr,
            159,
            100,
            Some(25),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), Some("limit=25"));
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }
}
