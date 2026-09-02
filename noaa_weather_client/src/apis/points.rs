//! Geographic point metadata: the `/points` family.
//!
//! Obtain the handle with [`Client::points`]. A point lookup returns the
//! forecast office, grid cell, and zones for a latitude/longitude pair, which
//! is where most forecast workflows begin. [`Points::forecast_for`] performs
//! that lookup and fetches the textual forecast in one call.
//!
//! ```no_run
//! use noaa_weather_client::{Client, Coordinates};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let here = Coordinates::new(39.7456, -97.0892)?;
//! let forecast = client.points().forecast_for(here).await?;
//! for period in forecast.properties.periods.iter().flatten().take(2) {
//!     println!("{:?}: {:?}", period.name, period.short_forecast);
//! }
//! # Ok(())
//! # }
//! ```

use super::Error;
use super::gridpoints::ForecastQuery;
use crate::client::{Client, http};
use crate::geo::Coordinates;
use crate::ids::GridpointId;
use crate::models;

/// The `/points` endpoints, obtained from [`Client::points`].
#[derive(Clone, Copy, Debug)]
pub struct Points<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/points` endpoints.
    #[must_use]
    pub fn points(&self) -> Points<'_> {
        Points { client: self }
    }
}

impl Points<'_> {
    /// Returns metadata for one point: its forecast office, grid cell,
    /// zones, and links to the forecast endpoints that cover it.
    ///
    /// `GET /points/{latitude},{longitude}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, Coordinates};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let point = client
    ///     .points()
    ///     .get(Coordinates::new(39.7456, -97.0892)?)
    ///     .await?;
    /// println!("{:?}", point.properties.forecast_office);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails (for example a point outside
    /// NOAA's coverage) or the response cannot be decoded.
    pub async fn get(&self, point: Coordinates) -> Result<models::PointGeoJson, Error> {
        http::request(self.client, "/points")
            .path_segment(point)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the multi-day textual forecast for the grid cell covering
    /// `point`, in US customary units.
    ///
    /// Composes `GET /points/{latitude},{longitude}` with
    /// `GET /gridpoints/{wfo}/{x},{y}/forecast`.
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, Coordinates};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let forecast = client
    ///     .points()
    ///     .forecast_for(Coordinates::new(39.7456, -97.0892)?)
    ///     .await?;
    /// # let _ = forecast;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Invalid`] when the point response carries no usable
    /// grid coordinates, and any other [`Error`] from either request.
    pub async fn forecast_for(
        &self,
        point: Coordinates,
    ) -> Result<models::Gridpoint12hForecastGeoJson, Error> {
        let point = self.get(point).await?;
        let grid = GridpointId::try_from(point.properties.as_ref())?;
        self.client
            .gridpoints()
            .forecast(&grid, &ForecastQuery::default())
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use crate::client::test_support::client_for;
    use crate::{Coordinates, Error};

    const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;

    #[tokio::test]
    async fn coordinates_are_one_geo_json_path_segment() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/points/39.7456,-97.0892"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(FEATURE, "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        client_for(&server)
            .points()
            .get(Coordinates::new(39.74561, -97.08919).unwrap())
            .await
            .unwrap();
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
    }

    #[tokio::test]
    async fn forecast_for_resolves_the_grid_then_requests_the_forecast() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/points/39.7456,-97.0892"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"Feature","geometry":null,"properties":{"gridId":"TOP","gridX":31,"gridY":80}}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/gridpoints/TOP/31,80/forecast"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(FEATURE, "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        client_for(&server)
            .points()
            .forecast_for(Coordinates::new(39.7456, -97.0892).unwrap())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[1].url.query(), None);
        assert_eq!(
            requests[1].headers["feature-flags"],
            "forecast_temperature_qv,forecast_wind_speed_qv"
        );
    }

    #[tokio::test]
    async fn forecast_for_reports_a_point_without_grid_as_invalid() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/points/39.7456,-97.0892"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(FEATURE, "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;

        let error = client_for(&server)
            .points()
            .forecast_for(Coordinates::new(39.7456, -97.0892).unwrap())
            .await
            .unwrap_err();
        assert!(matches!(error, Error::Invalid(_)), "{error}");
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }
}
