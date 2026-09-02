//! Grid-cell forecasts and raw forecast layers: the `/gridpoints` family.
//!
//! Obtain the handle with [`Client::gridpoints`]. Every operation names a
//! grid cell with a [`GridpointId`] (`OFFICE/x,y`), which a point lookup
//! ([`crate::apis::points::Points::get`]) provides via
//! `GridpointId::try_from(&point.properties)`.
//!
//! ```no_run
//! use noaa_weather_client::{Client, GridpointId, apis::gridpoints::{ForecastQuery, ForecastUnits}};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let grid: GridpointId = "TOP/31,80".parse()?;
//! let forecast = client
//!     .gridpoints()
//!     .forecast(&grid, &ForecastQuery { units: Some(ForecastUnits::Si) })
//!     .await?;
//! # let _ = forecast;
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::ids::GridpointId;
use crate::models;

/// Units for the textual forecast: `us` (default) or `si`.
///
/// This is the response model's units enumeration under its request-side
/// name, since forecast responses echo the units they were rendered in.
pub use crate::models::GridpointForecastUnits as ForecastUnits;

/// Options for [`Gridpoints::forecast`] and [`Gridpoints::forecast_hourly`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ForecastQuery {
    /// Units for textual values; NOAA defaults to US customary.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub units: Option<ForecastUnits>,
}

impl http::QueryParams for ForecastQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("units", self.units.as_ref());
    }
}

/// Options for [`Gridpoints::stations`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct GridpointStationsQuery {
    /// Maximum number of stations to return (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
}

impl http::QueryParams for GridpointStationsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("limit", self.limit.as_ref());
    }
}

/// The `/gridpoints` endpoints, obtained from [`Client::gridpoints`].
#[derive(Clone, Copy, Debug)]
pub struct Gridpoints<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/gridpoints` endpoints.
    #[must_use]
    pub fn gridpoints(&self) -> Gridpoints<'_> {
        Gridpoints { client: self }
    }
}

const FORECAST_FLAGS: [http::FeatureFlag; 2] = [
    http::FeatureFlag::ForecastTemperatureQuantitativeValue,
    http::FeatureFlag::ForecastWindSpeedQuantitativeValue,
];

impl Gridpoints<'_> {
    fn grid(&self, grid: &GridpointId) -> http::ContractRequest<'_> {
        http::request(self.client, "/gridpoints")
            .path_segment(grid.office())
            .path_segment(format_args!("{},{}", grid.x(), grid.y()))
    }

    /// Returns the raw numerical forecast layers for one 2.5 km grid cell.
    ///
    /// `GET /gridpoints/{wfo}/{x},{y}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, GridpointId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let grid: GridpointId = "TOP/31,80".parse()?;
    /// let layers = client.gridpoints().get(&grid).await?;
    /// # let _ = layers;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn get(&self, grid: &GridpointId) -> Result<models::GridpointGeoJson, Error> {
        self.grid(grid).json(http::JsonMedia::GeoJson).await
    }

    /// Returns the multi-day textual forecast for one grid cell. NOAA's
    /// quantitative temperature and wind formats are always requested.
    ///
    /// `GET /gridpoints/{wfo}/{x},{y}/forecast`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, GridpointId, apis::gridpoints::ForecastQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let grid: GridpointId = "TOP/31,80".parse()?;
    /// let forecast = client
    ///     .gridpoints()
    ///     .forecast(&grid, &ForecastQuery::default())
    ///     .await?;
    /// # let _ = forecast;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn forecast(
        &self,
        grid: &GridpointId,
        query: &ForecastQuery,
    ) -> Result<models::Gridpoint12hForecastGeoJson, Error> {
        self.grid(grid)
            .literal_path("forecast")
            .query(query)
            .feature_flags(FORECAST_FLAGS)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the hour-by-hour textual forecast for one grid cell. NOAA's
    /// quantitative temperature and wind formats are always requested.
    ///
    /// `GET /gridpoints/{wfo}/{x},{y}/forecast/hourly`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, GridpointId, apis::gridpoints::ForecastQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let grid: GridpointId = "TOP/31,80".parse()?;
    /// let hourly = client
    ///     .gridpoints()
    ///     .forecast_hourly(&grid, &ForecastQuery::default())
    ///     .await?;
    /// # let _ = hourly;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn forecast_hourly(
        &self,
        grid: &GridpointId,
        query: &ForecastQuery,
    ) -> Result<models::GridpointHourlyForecastGeoJson, Error> {
        self.grid(grid)
            .literal_path("forecast/hourly")
            .query(query)
            .feature_flags(FORECAST_FLAGS)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the observation stations usable for one grid cell.
    ///
    /// `GET /gridpoints/{wfo}/{x},{y}/stations`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, GridpointId, apis::gridpoints::GridpointStationsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let grid: GridpointId = "TOP/31,80".parse()?;
    /// let stations = client
    ///     .gridpoints()
    ///     .stations(&grid, &GridpointStationsQuery { limit: Some(5) })
    ///     .await?;
    /// # let _ = stations;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn stations(
        &self,
        grid: &GridpointId,
        query: &GridpointStationsQuery,
    ) -> Result<models::ObservationStationCollectionGeoJson, Error> {
        self.grid(grid)
            .literal_path("stations")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{ForecastQuery, ForecastUnits, GridpointStationsQuery};
    use crate::{GridpointId, client::test_support::client_for};

    const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
    const COLLECTION: &str = r#"{"type":"FeatureCollection","features":[]}"#;
    const REQUIRED_FLAGS: &str = "forecast_temperature_qv,forecast_wind_speed_qv";

    async fn mount_geo_json(server: &MockServer, body: &'static str) {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/geo+json"))
            .mount(server)
            .await;
    }

    fn psr() -> GridpointId {
        "psr/159,100".parse().unwrap()
    }

    #[tokio::test]
    async fn gridpoint_path_is_office_then_comma_joined_coordinates() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        client_for(&server).gridpoints().get(&psr()).await.unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/gridpoints/PSR/159,100");
        assert_eq!(requests[0].url.query(), None);
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }

    #[tokio::test]
    async fn forecast_sends_quantitative_flags_and_units_query() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        client_for(&server)
            .gridpoints()
            .forecast(
                &psr(),
                &ForecastQuery {
                    units: Some(ForecastUnits::Si),
                },
            )
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/gridpoints/PSR/159,100/forecast");
        assert_eq!(requests[0].url.query(), Some("units=si"));
        assert_eq!(requests[0].headers["feature-flags"], REQUIRED_FLAGS);
    }

    #[tokio::test]
    async fn hourly_forecast_with_default_query_omits_units() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        client_for(&server)
            .gridpoints()
            .forecast_hourly(&psr(), &ForecastQuery::default())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/gridpoints/PSR/159,100/forecast/hourly"
        );
        assert_eq!(requests[0].url.query(), None);
        assert_eq!(requests[0].headers["feature-flags"], REQUIRED_FLAGS);
    }

    #[tokio::test]
    async fn stations_preserve_limit_and_omit_feature_flags() {
        let server = MockServer::start().await;
        mount_geo_json(&server, COLLECTION).await;

        client_for(&server)
            .gridpoints()
            .stations(&psr(), &GridpointStationsQuery { limit: Some(25) })
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/gridpoints/PSR/159,100/stations");
        assert_eq!(requests[0].url.query(), Some("limit=25"));
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }
}
