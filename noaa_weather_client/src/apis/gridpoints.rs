//! Gridpoint-based forecasts and raw observation data.
//!
//! Covers the `/gridpoints/{wfo}/{x},{y}` endpoints. Use a point's metadata
//! (from [`super::points::get_point`]) to obtain the forecast office and grid
//! coordinates needed by these functions.

use std::fmt;

use super::Error;
use crate::client::{Client, http};
use crate::models;

struct GridCoordinates {
    x: i32,
    y: i32,
}

impl fmt::Display for GridCoordinates {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{},{}", self.x, self.y)
    }
}

/// Returns raw numerical forecast data for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}` endpoint.
/// This endpoint provides detailed forecast data layers like temperature, humidity, wind, etc.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
) -> Result<models::GridpointGeoJson, Error> {
    http::request(client, "/gridpoints")
        .path_segment(forecast_office_id)
        .path_segment(GridCoordinates { x, y })
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a textual forecast for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/forecast` endpoint.
/// This provides a human-readable, multi-day forecast summary.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    units: Option<models::GridpointForecastUnits>,
) -> Result<models::Gridpoint12hForecastGeoJson, Error> {
    http::request(client, "/gridpoints")
        .path_segment(forecast_office_id)
        .path_segment(GridCoordinates { x, y })
        .literal_path("forecast")
        .query_scalar("units", units)
        .feature_flags([
            http::FeatureFlag::ForecastTemperatureQuantitativeValue,
            http::FeatureFlag::ForecastWindSpeedQuantitativeValue,
        ])
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a textual hourly forecast for a 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/forecast/hourly` endpoint.
/// This provides a human-readable, hour-by-hour forecast summary.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    units: Option<models::GridpointForecastUnits>,
) -> Result<models::GridpointHourlyForecastGeoJson, Error> {
    http::request(client, "/gridpoints")
        .path_segment(forecast_office_id)
        .path_segment(GridCoordinates { x, y })
        .literal_path("forecast/hourly")
        .query_scalar("units", units)
        .feature_flags([
            http::FeatureFlag::ForecastTemperatureQuantitativeValue,
            http::FeatureFlag::ForecastWindSpeedQuantitativeValue,
        ])
        .json(http::JsonMedia::GeoJson)
        .await
}

/// Returns a list of observation stations usable for a given 2.5km grid area.
///
/// Corresponds to the `/gridpoints/{forecast_office_id}/{x},{y}/stations` endpoint.
/// This helps identify nearby stations for obtaining current observations.
///
/// # Parameters
///
/// * `client`: The API client.
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
    client: &Client,
    forecast_office_id: models::NwsForecastOfficeId,
    x: i32,
    y: i32,
    limit: Option<i32>,
) -> Result<models::ObservationStationCollectionGeoJson, Error> {
    http::request(client, "/gridpoints")
        .path_segment(forecast_office_id)
        .path_segment(GridCoordinates { x, y })
        .literal_path("stations")
        .query_scalar("limit", limit)
        .json(http::JsonMedia::GeoJson)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{
        get_gridpoint, get_gridpoint_forecast, get_gridpoint_forecast_hourly,
        get_gridpoint_stations,
    };
    use crate::{
        client::test_support::client_for,
        models::{GridpointForecastUnits, NwsForecastOfficeId},
    };

    const FORECAST: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
    const STATIONS: &str = r#"{"type":"FeatureCollection","features":[]}"#;
    const REQUIRED_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

    async fn mount_json(server: &MockServer, body: &'static str) {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/geo+json"))
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn gridpoint_path_preserves_typed_office_and_coordinate_pair_meaning() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&server)
            .await;

        let result = get_gridpoint(&client_for(&server), NwsForecastOfficeId::Psr, -159, 100).await;

        assert!(result.is_err());
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/gridpoints/PSR/-159,100");
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/geo+json"
        );
    }

    #[tokio::test]
    async fn forecast_always_sends_quantitative_flags_and_units_query() {
        let server = MockServer::start().await;
        mount_json(&server, FORECAST).await;
        get_gridpoint_forecast(
            &client_for(&server),
            NwsForecastOfficeId::Psr,
            159,
            100,
            Some(GridpointForecastUnits::Si),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/gridpoints/PSR/159,100/forecast");
        assert_eq!(requests[0].headers["feature-flags"], REQUIRED_FLAGS);
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/geo+json"
        );
        assert_eq!(requests[0].url.query(), Some("units=si"));
    }

    #[tokio::test]
    async fn hourly_forecast_always_sends_quantitative_flags() {
        let server = MockServer::start().await;
        mount_json(&server, FORECAST).await;
        get_gridpoint_forecast_hourly(
            &client_for(&server),
            NwsForecastOfficeId::Psr,
            159,
            100,
            None,
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/gridpoints/PSR/159,100/forecast/hourly"
        );
        assert_eq!(requests[0].headers["feature-flags"], REQUIRED_FLAGS);
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/geo+json"
        );
        assert_eq!(requests[0].url.query(), None);
    }

    #[tokio::test]
    async fn gridpoint_stations_preserves_limit_and_omits_feature_flags() {
        let server = MockServer::start().await;
        mount_json(&server, STATIONS).await;
        get_gridpoint_stations(
            &client_for(&server),
            NwsForecastOfficeId::Psr,
            159,
            100,
            Some(25),
        )
        .await
        .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/gridpoints/PSR/159,100/stations");
        assert_eq!(requests[0].url.query(), Some("limit=25"));
        assert_eq!(
            requests[0].headers["accept"].to_str().unwrap(),
            "application/geo+json"
        );
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }
}
