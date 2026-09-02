//! Weather alerts, warnings, and watches: the `/alerts` family.
//!
//! Obtain the handle with [`Client::alerts`]. Filtering operations take one
//! query struct each ([`ActiveAlertsQuery`], [`AlertsQuery`]); build them with
//! struct-update syntax so unset filters stay absent:
//!
//! ```no_run
//! use noaa_weather_client::{Client, apis::alerts::ActiveAlertsQuery};
//! use noaa_weather_client::models::AlertSeverity;
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let severe = client
//!     .alerts()
//!     .active(&ActiveAlertsQuery {
//!         severity: vec![AlertSeverity::Severe, AlertSeverity::Extreme],
//!         ..Default::default()
//!     })
//!     .await?;
//! println!("{} severe alerts", severe.features.len());
//! # Ok(())
//! # }
//! ```

use std::{fmt, str::FromStr};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::geo::Coordinates;
use crate::ids::{AlertId, ZoneId};
use crate::models::{
    self, AlertCertainty, AlertMessageType, AlertSeverity, AlertStatus, AlertUrgency, AreaCode,
    MarineRegionCode,
};

/// The land or marine half of the alert system, for the `region_type`
/// filter.
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum RegionType {
    /// Alerts for land areas.
    #[serde(rename = "land")]
    #[default]
    Land,
    /// Alerts for marine areas.
    #[serde(rename = "marine")]
    Marine,
}

impl fmt::Display for RegionType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Land => "land",
            Self::Marine => "marine",
        })
    }
}

impl FromStr for RegionType {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.to_ascii_lowercase().as_str() {
            "land" => Ok(Self::Land),
            "marine" => Ok(Self::Marine),
            _ => Err(format!("Invalid region type: {text}")),
        }
    }
}

/// Filters for [`Alerts::active`] (`GET /alerts/active`).
///
/// NOAA treats `area`, `point`, `region`, `region_type`, and `zone` as
/// mutually exclusive location filters; the server rejects combinations.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ActiveAlertsQuery {
    /// Alert statuses (actual, exercise, system, test, draft).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub status: Vec<AlertStatus>,
    /// Message types (alert, update, cancel).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub message_type: Vec<AlertMessageType>,
    /// Event names such as `Tornado Warning` or `Flood Watch`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<String>,
    /// NWS public zone/county codes or SAME codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<String>,
    /// State/territory or marine area codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub area: Vec<AreaCode>,
    /// A point whose alerts to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<Coordinates>,
    /// Marine region codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub region: Vec<MarineRegionCode>,
    /// Land or marine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<RegionType>,
    /// NWS public zone or county identifiers.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zone: Vec<ZoneId>,
    /// Alert urgencies.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub urgency: Vec<AlertUrgency>,
    /// Alert severities.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub severity: Vec<AlertSeverity>,
    /// Alert certainties.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub certainty: Vec<AlertCertainty>,
}

impl http::QueryParams for ActiveAlertsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.list("status", &self.status);
        request.list("message_type", &self.message_type);
        request.list("event", &self.event);
        request.list("code", &self.code);
        request.list("area", &self.area);
        request.scalar("point", self.point.as_ref());
        request.list("region", &self.region);
        request.scalar("region_type", self.region_type.as_ref());
        request.list("zone", &self.zone);
        request.list("urgency", &self.urgency);
        request.list("severity", &self.severity);
        request.list("certainty", &self.certainty);
    }
}

/// Filters and paging for [`Alerts::search`] (`GET /alerts`).
///
/// The same location-filter exclusivity as [`ActiveAlertsQuery`] applies.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct AlertsQuery {
    /// Earliest alert time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub start: Option<Timestamp>,
    /// Latest alert time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub end: Option<Timestamp>,
    /// Alert statuses (actual, exercise, system, test, draft).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub status: Vec<AlertStatus>,
    /// Message types (alert, update, cancel).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub message_type: Vec<AlertMessageType>,
    /// Event names such as `Tornado Warning`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event: Vec<String>,
    /// NWS public zone/county codes or SAME codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub code: Vec<String>,
    /// State/territory or marine area codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub area: Vec<AreaCode>,
    /// A point whose alerts to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<Coordinates>,
    /// Marine region codes.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub region: Vec<MarineRegionCode>,
    /// Land or marine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_type: Option<RegionType>,
    /// NWS public zone or county identifiers.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub zone: Vec<ZoneId>,
    /// Alert urgencies.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub urgency: Vec<AlertUrgency>,
    /// Alert severities.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub severity: Vec<AlertSeverity>,
    /// Alert certainties.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub certainty: Vec<AlertCertainty>,
    /// Maximum number of alerts per page (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
    /// Opaque pagination cursor from a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl http::QueryParams for AlertsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.instant("start", self.start.as_ref());
        request.instant("end", self.end.as_ref());
        request.list("status", &self.status);
        request.list("message_type", &self.message_type);
        request.list("event", &self.event);
        request.list("code", &self.code);
        request.list("area", &self.area);
        request.scalar("point", self.point.as_ref());
        request.list("region", &self.region);
        request.scalar("region_type", self.region_type.as_ref());
        request.list("zone", &self.zone);
        request.list("urgency", &self.urgency);
        request.list("severity", &self.severity);
        request.list("certainty", &self.certainty);
        request.scalar("limit", self.limit.as_ref());
        request.scalar("cursor", self.cursor.as_ref());
    }
}

/// The `/alerts` endpoints, obtained from [`Client::alerts`].
#[derive(Clone, Copy, Debug)]
pub struct Alerts<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/alerts` endpoints.
    #[must_use]
    pub fn alerts(&self) -> Alerts<'_> {
        Alerts { client: self }
    }
}

impl Alerts<'_> {
    /// Returns currently active alerts matching `query`.
    ///
    /// `GET /alerts/active`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::alerts::ActiveAlertsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let alerts = client
    ///     .alerts()
    ///     .active(&ActiveAlertsQuery {
    ///         area: vec!["CA".parse().unwrap()],
    ///         ..Default::default()
    ///     })
    ///     .await?;
    /// # let _ = alerts;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn active(
        &self,
        query: &ActiveAlertsQuery,
    ) -> Result<models::AlertCollectionGeoJson, Error> {
        http::request(self.client, "/alerts/active")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns active alerts for one state, territory, or marine area.
    ///
    /// `GET /alerts/active/area/{area}`
    ///
    /// ```no_run
    /// use noaa_weather_client::Client;
    /// use noaa_weather_client::models::AreaCode;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let area: AreaCode = "AZ".parse().unwrap();
    /// let alerts = client.alerts().active_for_area(&area).await?;
    /// # let _ = alerts;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn active_for_area(
        &self,
        area: &AreaCode,
    ) -> Result<models::AlertCollectionGeoJson, Error> {
        http::request(self.client, "/alerts/active/area")
            .path_segment(area)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns active alerts for one marine region.
    ///
    /// `GET /alerts/active/region/{region}`
    ///
    /// ```no_run
    /// use noaa_weather_client::Client;
    /// use noaa_weather_client::models::MarineRegionCode;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let alerts = client
    ///     .alerts()
    ///     .active_for_marine_region(MarineRegionCode::Gm)
    ///     .await?;
    /// # let _ = alerts;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn active_for_marine_region(
        &self,
        region: MarineRegionCode,
    ) -> Result<models::AlertCollectionGeoJson, Error> {
        http::request(self.client, "/alerts/active/region")
            .path_segment(region)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns active alerts for one NWS public zone or county.
    ///
    /// `GET /alerts/active/zone/{zoneId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, ZoneId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let zone: ZoneId = "CAZ043".parse()?;
    /// let alerts = client.alerts().active_for_zone(&zone).await?;
    /// # let _ = alerts;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn active_for_zone(
        &self,
        zone: &ZoneId,
    ) -> Result<models::AlertCollectionGeoJson, Error> {
        http::request(self.client, "/alerts/active/zone")
            .path_segment(zone)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns counts of active alerts by area, region, and zone.
    ///
    /// `GET /alerts/active/count`
    ///
    /// ```no_run
    /// use noaa_weather_client::Client;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let count = client.alerts().active_count().await?;
    /// println!("{:?} active alerts", count.total);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn active_count(&self) -> Result<models::ActiveAlertsCountResponse, Error> {
        http::request(self.client, "/alerts/active/count")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns alerts, including past ones, matching `query`.
    ///
    /// `GET /alerts`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::alerts::AlertsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let page = client
    ///     .alerts()
    ///     .search(&AlertsQuery {
    ///         start: Some("2026-08-30T00:00:00Z".parse().unwrap()),
    ///         limit: Some(50),
    ///         ..Default::default()
    ///     })
    ///     .await?;
    /// # let _ = page;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn search(
        &self,
        query: &AlertsQuery,
    ) -> Result<models::AlertCollectionGeoJson, Error> {
        http::request(self.client, "/alerts")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns one alert by its identifier.
    ///
    /// `GET /alerts/{id}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{AlertId, Client};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let id: AlertId = "urn:oid:2.49.0.1.840.0.1234".parse()?;
    /// let alert = client.alerts().get(&id).await?;
    /// # let _ = alert;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the alert is not found, or
    /// the response cannot be decoded.
    pub async fn get(&self, id: &AlertId) -> Result<models::AlertGeoJson, Error> {
        http::request(self.client, "/alerts")
            .path_segment(id)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the event types the alert system recognizes.
    ///
    /// `GET /alerts/types`
    ///
    /// ```no_run
    /// use noaa_weather_client::Client;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let types = client.alerts().types().await?;
    /// # let _ = types;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn types(&self) -> Result<models::AlertTypesResponse, Error> {
        http::request(self.client, "/alerts/types")
            .json(http::JsonMedia::JsonLd)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::{ActiveAlertsQuery, AlertsQuery, RegionType};
    use crate::client::test_support::client_for;
    use crate::models::{
        AlertCertainty, AlertMessageType, AlertSeverity, AlertStatus, AlertUrgency, AreaCode,
        MarineRegionCode, StateTerritoryCode,
    };

    const COLLECTION: &str = r#"{"type":"FeatureCollection","features":[]}"#;
    const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;

    async fn mount(server: &MockServer, route: &str, body: &'static str, media: &'static str) {
        Mock::given(method("GET"))
            .and(path(route))
            .and(header("Accept", media))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, media))
            .expect(1)
            .mount(server)
            .await;
    }

    async fn query_of(server: &MockServer) -> Option<String> {
        let requests = server.received_requests().await.unwrap();
        requests[0].url.query().map(str::to_owned)
    }

    #[tokio::test]
    async fn active_encodes_every_filter_once_in_declaration_order() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/alerts/active",
            COLLECTION,
            "application/geo+json",
        )
        .await;

        client_for(&server)
            .alerts()
            .active(&ActiveAlertsQuery {
                status: vec![AlertStatus::Actual, AlertStatus::Test],
                message_type: vec![AlertMessageType::Alert],
                event: vec!["Flood Watch".to_owned(), "Wind/Warning".to_owned()],
                code: vec!["AZC013".to_owned()],
                area: vec![AreaCode::StateTerritoryCode(StateTerritoryCode::Az)],
                point: Some("39.7456,-97.0892".parse().unwrap()),
                region: vec![MarineRegionCode::Gm],
                region_type: Some(RegionType::Marine),
                zone: vec!["AZZ540".parse().unwrap(), "azc013".parse().unwrap()],
                urgency: vec![AlertUrgency::Immediate],
                severity: vec![AlertSeverity::Severe, AlertSeverity::Extreme],
                certainty: vec![AlertCertainty::Observed],
            })
            .await
            .unwrap();

        assert_eq!(
            query_of(&server).await.as_deref(),
            Some(
                "status=actual%2Ctest&message_type=Alert&event=Flood+Watch%2CWind%2FWarning\
                 &code=AZC013&area=AZ&point=39.7456%2C-97.0892&region=GM&region_type=marine\
                 &zone=AZZ540%2CAZC013&urgency=Immediate&severity=Severe%2CExtreme\
                 &certainty=Observed"
            )
        );
    }

    #[tokio::test]
    async fn active_with_default_query_sends_no_query_string() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/alerts/active",
            COLLECTION,
            "application/geo+json",
        )
        .await;

        client_for(&server)
            .alerts()
            .active(&ActiveAlertsQuery::default())
            .await
            .unwrap();

        assert_eq!(query_of(&server).await, None);
    }

    #[tokio::test]
    async fn search_encodes_timestamps_as_whole_second_rfc_3339_and_paging_last() {
        let server = MockServer::start().await;
        mount(&server, "/alerts", COLLECTION, "application/geo+json").await;

        client_for(&server)
            .alerts()
            .search(&AlertsQuery {
                start: Some("2026-08-30T00:00:00.123456789Z".parse().unwrap()),
                end: Some("2026-08-30T06:30:00-05:00".parse().unwrap()),
                event: vec!["Tornado Warning".to_owned()],
                limit: Some(25),
                cursor: Some("next page".to_owned()),
                ..Default::default()
            })
            .await
            .unwrap();

        let query = query_of(&server).await.unwrap();
        assert_eq!(
            query,
            "start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T11%3A30%3A00Z\
             &event=Tornado+Warning&limit=25&cursor=next+page"
        );
        assert!(!query.contains("active"));
    }

    #[tokio::test]
    async fn search_encodes_every_field_once_in_declaration_order() {
        let server = MockServer::start().await;
        mount(&server, "/alerts", COLLECTION, "application/geo+json").await;

        client_for(&server)
            .alerts()
            .search(&AlertsQuery {
                start: Some("2026-08-30T00:00:00Z".parse().unwrap()),
                end: Some("2026-08-30T06:30:00-05:00".parse().unwrap()),
                status: vec![AlertStatus::Actual, AlertStatus::Test],
                message_type: vec![AlertMessageType::Alert],
                event: vec!["Flood Watch".to_owned(), "Wind/Warning".to_owned()],
                code: vec!["AZC013".to_owned()],
                area: vec![AreaCode::StateTerritoryCode(StateTerritoryCode::Az)],
                point: Some("39.7456,-97.0892".parse().unwrap()),
                region: vec![MarineRegionCode::Gm],
                region_type: Some(RegionType::Marine),
                zone: vec!["AZZ540".parse().unwrap(), "azc013".parse().unwrap()],
                urgency: vec![AlertUrgency::Immediate],
                severity: vec![AlertSeverity::Severe, AlertSeverity::Extreme],
                certainty: vec![AlertCertainty::Observed],
                limit: Some(25),
                cursor: Some("next page".to_owned()),
            })
            .await
            .unwrap();

        assert_eq!(
            query_of(&server).await.as_deref(),
            Some(
                "start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T11%3A30%3A00Z\
                 &status=actual%2Ctest&message_type=Alert&event=Flood+Watch%2CWind%2FWarning\
                 &code=AZC013&area=AZ&point=39.7456%2C-97.0892&region=GM&region_type=marine\
                 &zone=AZZ540%2CAZC013&urgency=Immediate&severity=Severe%2CExtreme\
                 &certainty=Observed&limit=25&cursor=next+page"
            )
        );
    }

    #[tokio::test]
    async fn alert_id_is_one_encoded_path_segment_requested_as_geo_json() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/alerts/urn:oid:2.49.0.1.840.0.abc.001.1",
            FEATURE,
            "application/geo+json",
        )
        .await;

        client_for(&server)
            .alerts()
            .get(&"urn:oid:2.49.0.1.840.0.abc.001.1".parse().unwrap())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn count_and_types_are_requested_as_json_ld() {
        let server = MockServer::start().await;
        mount(&server, "/alerts/active/count", "{}", "application/ld+json").await;
        mount(&server, "/alerts/types", "{}", "application/ld+json").await;

        let client = client_for(&server);
        client.alerts().active_count().await.unwrap();
        client.alerts().types().await.unwrap();
    }

    #[tokio::test]
    async fn scoped_active_routes_place_typed_values_in_the_path() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/alerts/active/area/CA",
            COLLECTION,
            "application/geo+json",
        )
        .await;
        mount(
            &server,
            "/alerts/active/region/GM",
            COLLECTION,
            "application/geo+json",
        )
        .await;
        mount(
            &server,
            "/alerts/active/zone/CAZ043",
            COLLECTION,
            "application/geo+json",
        )
        .await;

        let client = client_for(&server);
        let alerts = client.alerts();
        alerts
            .active_for_area(&AreaCode::StateTerritoryCode(StateTerritoryCode::Ca))
            .await
            .unwrap();
        alerts
            .active_for_marine_region(MarineRegionCode::Gm)
            .await
            .unwrap();
        alerts
            .active_for_zone(&"caz043".parse().unwrap())
            .await
            .unwrap();
        for request in server.received_requests().await.unwrap() {
            assert_eq!(request.url.query(), None);
        }
    }

    #[test]
    fn region_type_round_trips_text_and_json() {
        assert_eq!("Marine".parse::<RegionType>().unwrap(), RegionType::Marine);
        assert_eq!(RegionType::Land.to_string(), "land");
        assert_eq!(
            serde_json::to_string(&RegionType::Marine).unwrap(),
            "\"marine\""
        );
        assert!("ocean".parse::<RegionType>().is_err());
    }

    #[test]
    fn query_json_omits_unset_filters() {
        let json = serde_json::to_value(AlertsQuery {
            limit: Some(10),
            zone: vec!["AZZ540".parse().unwrap()],
            ..Default::default()
        })
        .unwrap();
        assert_eq!(json, serde_json::json!({"limit": 10, "zone": ["AZZ540"]}));
        let parsed: AlertsQuery = serde_json::from_value(json).unwrap();
        assert_eq!(parsed.limit, Some(10));
        assert_eq!(parsed.zone[0].as_str(), "AZZ540");
    }
}
