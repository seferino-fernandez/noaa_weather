//! Forecast, public, fire, county, and marine zones: the `/zones` family.
//!
//! Obtain the handle with [`Client::zones`]. Zone metadata and forecasts are
//! addressed by a [`ZoneType`] plus a [`ZoneId`]; observations and stations
//! are only defined for forecast zones and take the id alone.
//!
//! ```no_run
//! use noaa_weather_client::{Client, ZoneId, apis::zones::ZoneType};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let zone: ZoneId = "AZZ540".parse()?;
//! let forecast = client.zones().forecast(ZoneType::Forecast, &zone).await?;
//! # let _ = forecast;
//! # Ok(())
//! # }
//! ```

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::geo::Coordinates;
use crate::ids::ZoneId;
use crate::models::{self, AreaCode, RegionCode};

/// The kind of zone a `/zones/{type}/...` path addresses.
///
/// This is the response model's zone type under its request-side name,
/// since zone responses report the same value.
pub use crate::models::NwsZoneType as ZoneType;

/// Options for [`Zones::get`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ZoneQuery {
    /// Return the zone boundaries in effect at this instant.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub effective: Option<Timestamp>,
}

impl http::QueryParams for ZoneQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.instant("effective", self.effective.as_ref());
    }
}

/// Filters for [`Zones::list`] and [`Zones::list_of_type`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ZonesQuery {
    /// Zone identifiers to include.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub id: Vec<ZoneId>,
    /// State/territory or marine area codes to include.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub area: Vec<AreaCode>,
    /// Land or marine region codes to include.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub region: Vec<RegionCode>,
    /// Zone types to include (`type` on the wire).
    #[serde(rename = "type", skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub types: Vec<ZoneType>,
    /// Only zones containing this point.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<Coordinates>,
    /// Include zone geometry, which can be large. NOAA defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_geometry: Option<bool>,
    /// Maximum number of zones to return (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
    /// Only zones in effect at this instant.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub effective: Option<Timestamp>,
}

impl http::QueryParams for ZonesQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.list("id", &self.id);
        request.list("area", &self.area);
        request.list("region", &self.region);
        request.list("type", &self.types);
        request.scalar("point", self.point.as_ref());
        request.scalar("include_geometry", self.include_geometry.as_ref());
        request.scalar("limit", self.limit.as_ref());
        request.instant("effective", self.effective.as_ref());
    }
}

/// Filters for [`Zones::observations`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ZoneObservationsQuery {
    /// Earliest observation time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub start: Option<Timestamp>,
    /// Latest observation time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub end: Option<Timestamp>,
    /// Maximum number of observations to return (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
}

impl http::QueryParams for ZoneObservationsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.instant("start", self.start.as_ref());
        request.instant("end", self.end.as_ref());
        request.scalar("limit", self.limit.as_ref());
    }
}

/// Paging for [`Zones::stations`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ZoneStationsQuery {
    /// Maximum number of stations per page (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
    /// Opaque pagination cursor from a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl http::QueryParams for ZoneStationsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("limit", self.limit.as_ref());
        request.scalar("cursor", self.cursor.as_ref());
    }
}

/// The `/zones` endpoints, obtained from [`Client::zones`].
#[derive(Clone, Copy, Debug)]
pub struct Zones<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/zones` endpoints.
    #[must_use]
    pub fn zones(&self) -> Zones<'_> {
        Zones { client: self }
    }
}

impl Zones<'_> {
    fn forecast_zone(&self, zone: &ZoneId) -> http::ContractRequest<'_> {
        http::request(self.client, "/zones/forecast").path_segment(zone)
    }

    /// Returns metadata for one zone.
    ///
    /// `GET /zones/{type}/{zoneId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ZoneId, apis::zones::{ZoneQuery, ZoneType}};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zone: ZoneId = "AZZ540".parse()?;
    /// let metadata = client
    ///     .zones()
    ///     .get(ZoneType::Public, &zone, &ZoneQuery::default())
    ///     .await?;
    /// # let _ = metadata;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the zone is unknown, or the
    /// response cannot be decoded.
    pub async fn get(
        &self,
        zone_type: ZoneType,
        zone: &ZoneId,
        query: &ZoneQuery,
    ) -> Result<models::ZoneGeoJson, Error> {
        http::request(self.client, "/zones")
            .path_segment(zone_type)
            .path_segment(zone)
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the current text forecast for one zone.
    ///
    /// `GET /zones/{type}/{zoneId}/forecast`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ZoneId, apis::zones::ZoneType};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zone: ZoneId = "AZZ540".parse()?;
    /// let forecast = client.zones().forecast(ZoneType::Forecast, &zone).await?;
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
        zone_type: ZoneType,
        zone: &ZoneId,
    ) -> Result<models::ZoneForecastGeoJson, Error> {
        http::request(self.client, "/zones")
            .path_segment(zone_type)
            .path_segment(zone)
            .literal_path("forecast")
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns zones of every type matching `query`.
    ///
    /// `GET /zones`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::zones::{ZoneType, ZonesQuery}};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zones = client
    ///     .zones()
    ///     .list(&ZonesQuery {
    ///         area: vec!["AZ".parse().unwrap()],
    ///         types: vec![ZoneType::Forecast, ZoneType::Fire],
    ///         ..Default::default()
    ///     })
    ///     .await?;
    /// # let _ = zones;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn list(&self, query: &ZonesQuery) -> Result<models::ZoneCollectionGeoJson, Error> {
        http::request(self.client, "/zones")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns zones of one type matching `query`. The `types` filter in
    /// `query` further narrows within that type, as NOAA allows.
    ///
    /// `GET /zones/{type}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::zones::{ZoneType, ZonesQuery}};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let counties = client
    ///     .zones()
    ///     .list_of_type(ZoneType::County, &ZonesQuery {
    ///         area: vec!["AZ".parse().unwrap()],
    ///         ..Default::default()
    ///     })
    ///     .await?;
    /// # let _ = counties;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn list_of_type(
        &self,
        zone_type: ZoneType,
        query: &ZonesQuery,
    ) -> Result<models::ZoneCollectionGeoJson, Error> {
        http::request(self.client, "/zones")
            .path_segment(zone_type)
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns recent observations from stations in one forecast zone.
    ///
    /// `GET /zones/forecast/{zoneId}/observations`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ZoneId, apis::zones::ZoneObservationsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zone: ZoneId = "AZZ540".parse()?;
    /// let observations = client
    ///     .zones()
    ///     .observations(&zone, &ZoneObservationsQuery { limit: Some(20), ..Default::default() })
    ///     .await?;
    /// # let _ = observations;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn observations(
        &self,
        zone: &ZoneId,
        query: &ZoneObservationsQuery,
    ) -> Result<models::ObservationCollectionGeoJson, Error> {
        self.forecast_zone(zone)
            .literal_path("observations")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the observation stations in one forecast zone.
    ///
    /// `GET /zones/forecast/{zoneId}/stations`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ZoneId, apis::zones::ZoneStationsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zone: ZoneId = "AZZ540".parse()?;
    /// let stations = client
    ///     .zones()
    ///     .stations(&zone, &ZoneStationsQuery::default())
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
        zone: &ZoneId,
        query: &ZoneStationsQuery,
    ) -> Result<models::ObservationStationCollectionGeoJson, Error> {
        self.forecast_zone(zone)
            .literal_path("stations")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::method};

    use super::{ZoneObservationsQuery, ZoneQuery, ZoneStationsQuery, ZoneType, ZonesQuery};
    use crate::{
        ZoneId,
        client::test_support::client_for,
        models::{LandRegionCode, RegionCode},
    };

    const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
    const COLLECTION: &str = r#"{"type":"FeatureCollection","features":[]}"#;

    async fn mount_geo_json(server: &MockServer, body: &'static str) {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/geo+json"))
            .mount(server)
            .await;
    }

    fn azz540() -> ZoneId {
        "azz540".parse().unwrap()
    }

    #[tokio::test]
    async fn forecast_places_type_and_normalized_id_in_the_path() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        client_for(&server)
            .zones()
            .forecast(ZoneType::Public, &azz540())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/zones/public/AZZ540/forecast");
        assert_eq!(requests[0].url.query(), None);
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
    }

    #[tokio::test]
    async fn get_encodes_effective_as_rfc_3339_or_nothing() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;
        let client = client_for(&server);

        client
            .zones()
            .get(
                ZoneType::Forecast,
                &azz540(),
                &ZoneQuery {
                    effective: Some("2026-08-30T00:00:00Z".parse().unwrap()),
                },
            )
            .await
            .unwrap();
        client
            .zones()
            .get(ZoneType::Forecast, &azz540(), &ZoneQuery::default())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/zones/forecast/AZZ540");
        assert_eq!(
            requests[0].url.query(),
            Some("effective=2026-08-30T00%3A00%3A00Z")
        );
        assert_eq!(requests[1].url.query(), None);
    }

    #[tokio::test]
    async fn list_encodes_every_filter_in_declaration_order() {
        let server = MockServer::start().await;
        mount_geo_json(&server, COLLECTION).await;

        client_for(&server)
            .zones()
            .list(&ZonesQuery {
                id: vec!["AZZ540".parse().unwrap(), "AZC013".parse().unwrap()],
                area: vec!["AZ".parse().unwrap()],
                region: vec![RegionCode::Land(LandRegionCode::Wr)],
                types: vec![ZoneType::Forecast, ZoneType::Public],
                point: Some("33.4484,-112.074".parse().unwrap()),
                include_geometry: Some(false),
                limit: Some(10),
                effective: Some("2026-08-30T00:00:00Z".parse().unwrap()),
            })
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/zones");
        assert_eq!(
            requests[0].url.query(),
            Some(
                "id=AZZ540%2CAZC013&area=AZ&region=WR&type=forecast%2Cpublic\
                 &point=33.4484%2C-112.074&include_geometry=false&limit=10\
                 &effective=2026-08-30T00%3A00%3A00Z"
            )
        );
    }

    #[tokio::test]
    async fn list_of_type_keeps_the_type_path_and_optional_type_filter() {
        let server = MockServer::start().await;
        mount_geo_json(&server, COLLECTION).await;
        let client = client_for(&server);

        client
            .zones()
            .list_of_type(
                ZoneType::Public,
                &ZonesQuery {
                    types: vec![ZoneType::Fire, ZoneType::County],
                    limit: Some(1),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        client
            .zones()
            .list_of_type(ZoneType::Public, &ZonesQuery::default())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/zones/public");
        assert_eq!(requests[0].url.query(), Some("type=fire%2Ccounty&limit=1"));
        assert_eq!(requests[1].url.query(), None);
    }

    #[tokio::test]
    async fn observations_and_stations_use_the_forecast_literal() {
        let server = MockServer::start().await;
        mount_geo_json(&server, COLLECTION).await;
        let client = client_for(&server);

        client
            .zones()
            .observations(
                &azz540(),
                &ZoneObservationsQuery {
                    start: Some("2026-08-30T00:00:00Z".parse().unwrap()),
                    end: Some("2026-08-30T06:00:00Z".parse().unwrap()),
                    limit: Some(12),
                },
            )
            .await
            .unwrap();
        client
            .zones()
            .stations(
                &azz540(),
                &ZoneStationsQuery {
                    limit: Some(15),
                    cursor: Some("abc".to_owned()),
                },
            )
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/zones/forecast/AZZ540/observations"
        );
        assert_eq!(
            requests[0].url.query(),
            Some("start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T06%3A00%3A00Z&limit=12")
        );
        assert_eq!(requests[1].url.path(), "/zones/forecast/AZZ540/stations");
        assert_eq!(requests[1].url.query(), Some("limit=15&cursor=abc"));
        assert!(!requests[1].headers.contains_key("feature-flags"));
    }

    #[test]
    fn zones_query_serializes_types_under_the_wire_name() {
        let json = serde_json::to_value(ZonesQuery {
            types: vec![ZoneType::Fire],
            include_geometry: Some(true),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            json,
            serde_json::json!({"type": ["fire"], "includeGeometry": true})
        );
    }
}
