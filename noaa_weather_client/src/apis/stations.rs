//! Observation stations, surface observations, and Terminal Aerodrome
//! Forecasts: the `/stations` family.
//!
//! Obtain the handle with [`Client::stations`]. Every station operation
//! takes a [`StationId`]; list and history operations take one query struct
//! each.
//!
//! ```no_run
//! use noaa_weather_client::{Client, StationId, apis::stations::LatestObservationQuery};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let station: StationId = "KPHX".parse()?;
//! let latest = client
//!     .stations()
//!     .latest_observation(&station, &LatestObservationQuery::default())
//!     .await?;
//! println!("{:?}", latest.properties.text_description);
//! # Ok(())
//! # }
//! ```

use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::ids::StationId;
use crate::models::{self, AreaCode};

/// Filters and paging for [`Stations::list`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct StationsQuery {
    /// Station identifiers to include.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub id: Vec<StationId>,
    /// State/territory or marine area codes to include.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "schemars", schemars(with = "Vec<String>"))]
    pub state: Vec<AreaCode>,
    /// Maximum number of stations per page (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
    /// Opaque pagination cursor from a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl http::QueryParams for StationsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.list("id", &self.id);
        request.list("state", &self.state);
        request.scalar("limit", self.limit.as_ref());
        request.scalar("cursor", self.cursor.as_ref());
    }
}

/// Options for [`Stations::latest_observation`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct LatestObservationQuery {
    /// Only return quality-controlled data. NOAA defaults to `false`;
    /// non-QC observations are preliminary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_qc: Option<bool>,
}

impl http::QueryParams for LatestObservationQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("require_qc", self.require_qc.as_ref());
    }
}

/// Filters and paging for [`Stations::observations`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct ObservationsQuery {
    /// Earliest observation time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub start: Option<Timestamp>,
    /// Latest observation time to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub end: Option<Timestamp>,
    /// Maximum number of observations per page (1 to 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 500)))]
    pub limit: Option<u16>,
    /// Opaque pagination cursor from a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl http::QueryParams for ObservationsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.instant("start", self.start.as_ref());
        request.instant("end", self.end.as_ref());
        request.scalar("limit", self.limit.as_ref());
        request.scalar("cursor", self.cursor.as_ref());
    }
}

/// The `/stations` endpoints, obtained from [`Client::stations`].
#[derive(Clone, Copy, Debug)]
pub struct Stations<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/stations` endpoints.
    #[must_use]
    pub fn stations(&self) -> Stations<'_> {
        Stations { client: self }
    }
}

impl Stations<'_> {
    fn station(&self, station: &StationId) -> http::ContractRequest<'_> {
        http::request(self.client, "/stations").path_segment(station)
    }

    /// Returns metadata for one observation station.
    ///
    /// `GET /stations/{stationId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, StationId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let station: StationId = "KPHX".parse()?;
    /// let metadata = client.stations().get(&station).await?;
    /// # let _ = metadata;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the station is unknown, or
    /// the response cannot be decoded.
    pub async fn get(
        &self,
        station: &StationId,
    ) -> Result<models::ObservationStationGeoJson, Error> {
        self.station(station).json(http::JsonMedia::GeoJson).await
    }

    /// Returns a page of observation stations matching `query`.
    ///
    /// `GET /stations`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::stations::StationsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let stations = client
    ///     .stations()
    ///     .list(&StationsQuery {
    ///         state: vec!["AZ".parse().unwrap()],
    ///         limit: Some(20),
    ///         ..Default::default()
    ///     })
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
    pub async fn list(
        &self,
        query: &StationsQuery,
    ) -> Result<models::ObservationStationCollectionGeoJson, Error> {
        http::request(self.client, "/stations")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the most recent observation from one station.
    ///
    /// `GET /stations/{stationId}/observations/latest`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, StationId, apis::stations::LatestObservationQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let station: StationId = "KPHX".parse()?;
    /// let latest = client
    ///     .stations()
    ///     .latest_observation(&station, &LatestObservationQuery { require_qc: Some(true) })
    ///     .await?;
    /// # let _ = latest;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, no observation is
    /// available, or the response cannot be decoded.
    pub async fn latest_observation(
        &self,
        station: &StationId,
        query: &LatestObservationQuery,
    ) -> Result<models::ObservationGeoJson, Error> {
        self.station(station)
            .literal_path("observations/latest")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns a page of observations from one station.
    ///
    /// `GET /stations/{stationId}/observations`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, StationId, apis::stations::ObservationsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let station: StationId = "KPHX".parse()?;
    /// let recent = client
    ///     .stations()
    ///     .observations(&station, &ObservationsQuery { limit: Some(12), ..Default::default() })
    ///     .await?;
    /// # let _ = recent;
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
        station: &StationId,
        query: &ObservationsQuery,
    ) -> Result<models::ObservationCollectionGeoJson, Error> {
        self.station(station)
            .literal_path("observations")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the observation recorded at exactly `time`, sent as an
    /// RFC 3339 UTC timestamp with whole seconds (NOAA rejects fractions).
    ///
    /// `GET /stations/{stationId}/observations/{time}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, StationId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let station: StationId = "KPHX".parse()?;
    /// let at = "2026-08-30T18:53:00Z".parse().unwrap();
    /// let observation = client.stations().observation_at(&station, at).await?;
    /// # let _ = observation;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, no observation exists at
    /// that instant, or the response cannot be decoded.
    pub async fn observation_at(
        &self,
        station: &StationId,
        time: Timestamp,
    ) -> Result<models::ObservationGeoJson, Error> {
        self.station(station)
            .literal_path("observations")
            .path_segment(time.strftime(http::RFC3339_SECONDS))
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns metadata for the current Terminal Aerodrome Forecasts of one
    /// airport station.
    ///
    /// `GET /stations/{stationId}/tafs`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, StationId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let station: StationId = "KPHX".parse()?;
    /// let tafs = client.stations().tafs(&station).await?;
    /// # let _ = tafs;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn tafs(
        &self,
        station: &StationId,
    ) -> Result<models::TerminalAerodromeForecastsResponse, Error> {
        self.station(station)
            .literal_path("tafs")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns one Terminal Aerodrome Forecast, decoded from IWXXM XML into
    /// forecast meaning.
    ///
    /// `GET /stations/{stationId}/tafs/{date}/{time}`
    ///
    /// NOAA addresses a TAF by its issue date and `HHMM` time in UTC, so
    /// `issued` is split into those two segments in UTC and any seconds are
    /// dropped (minute precision).
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, StationId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let station: StationId = "KPHX".parse()?;
    /// let issued = "2026-08-30T22:54:00Z".parse().unwrap();
    /// let taf = client.stations().taf(&station, issued).await?;
    /// println!("{}", taf.bulletin_identifier());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails, the body is not IWXXM, or
    /// the forecast cannot be normalized
    /// ([`Error::TerminalAerodromeForecast`]).
    pub async fn taf(
        &self,
        station: &StationId,
        issued: Timestamp,
    ) -> Result<models::TerminalAerodromeForecast, Error> {
        let bytes = self
            .station(station)
            .literal_path("tafs")
            .path_segment(issued.strftime("%Y-%m-%d"))
            .path_segment(issued.strftime("%H%M"))
            .xml_bytes(http::XmlMedia::Iwxxm)
            .await?;
        models::terminal_aerodrome_forecast::decode_iwxxm(&bytes).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use super::{LatestObservationQuery, ObservationsQuery, StationsQuery};
    use crate::{StationId, client::test_support::client_for};

    const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
    const COLLECTION: &str = r#"{"type":"FeatureCollection","features":[]}"#;

    fn kphx() -> StationId {
        "kphx".parse().unwrap()
    }

    async fn mount_geo_json(server: &MockServer, body: &'static str) {
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/geo+json"))
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn list_encodes_ids_and_states_as_csv_then_paging() {
        let server = MockServer::start().await;
        mount_geo_json(&server, COLLECTION).await;

        client_for(&server)
            .stations()
            .list(&StationsQuery {
                id: vec!["KPHX".parse().unwrap(), "kiwa".parse().unwrap()],
                state: vec!["AZ".parse().unwrap(), "CA".parse().unwrap()],
                limit: Some(20),
                cursor: Some("next-page".to_owned()),
            })
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/stations");
        assert_eq!(
            requests[0].url.query(),
            Some("id=KPHX%2CKIWA&state=AZ%2CCA&limit=20&cursor=next-page")
        );
        assert!(!requests[0].headers.contains_key("feature-flags"));
    }

    #[tokio::test]
    async fn list_with_default_query_sends_nothing() {
        let server = MockServer::start().await;
        mount_geo_json(&server, COLLECTION).await;

        client_for(&server)
            .stations()
            .list(&StationsQuery::default())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), None);
    }

    #[tokio::test]
    async fn station_id_is_normalized_into_the_path() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        client_for(&server).stations().get(&kphx()).await.unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.path(), "/stations/KPHX");
        assert_eq!(requests[0].headers["accept"], "application/geo+json");
    }

    #[tokio::test]
    async fn observation_at_formats_the_instant_as_rfc_3339_utc() {
        let server = MockServer::start().await;
        mount_geo_json(&server, FEATURE).await;

        client_for(&server)
            .stations()
            .observation_at(&kphx(), "2026-08-30T07:34:56.789-05:00".parse().unwrap())
            .await
            .unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.path(),
            "/stations/KPHX/observations/2026-08-30T12:34:56Z"
        );
    }

    #[tokio::test]
    async fn observation_queries_keep_their_wire_names_and_order() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/observations/latest"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(FEATURE, "application/geo+json"))
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/observations"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(COLLECTION, "application/geo+json"),
            )
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KPHX/tafs"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(r#"{"@graph":[]}"#, "application/ld+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        client
            .stations()
            .latest_observation(
                &kphx(),
                &LatestObservationQuery {
                    require_qc: Some(false),
                },
            )
            .await
            .unwrap();
        client
            .stations()
            .observations(
                &kphx(),
                &ObservationsQuery {
                    start: Some("2026-08-30T00:00:00.5Z".parse().unwrap()),
                    end: Some("2026-08-30T06:00:00Z".parse().unwrap()),
                    limit: Some(1),
                    cursor: Some("next page".to_owned()),
                },
            )
            .await
            .unwrap();
        client.stations().tafs(&kphx()).await.unwrap();

        let requests = server.received_requests().await.unwrap();
        let contracts = requests
            .iter()
            .map(|request| {
                (
                    request.url.path().to_owned(),
                    request.url.query().map(str::to_owned),
                    request.headers["accept"].to_str().unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            contracts,
            [
                (
                    "/stations/KPHX/observations/latest".to_owned(),
                    Some("require_qc=false".to_owned()),
                    "application/geo+json".to_owned(),
                ),
                (
                    "/stations/KPHX/observations".to_owned(),
                    Some(
                        "start=2026-08-30T00%3A00%3A00Z&end=2026-08-30T06%3A00%3A00Z\
                         &limit=1&cursor=next+page"
                            .to_owned()
                    ),
                    "application/geo+json".to_owned(),
                ),
                (
                    "/stations/KPHX/tafs".to_owned(),
                    None,
                    "application/ld+json".to_owned(),
                ),
            ]
        );
    }

    #[tokio::test]
    async fn taf_splits_the_issue_instant_into_utc_date_and_hhmm_segments() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KCXL/tafs/2026-08-30/1500"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/cancellation.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .expect(1)
            .mount(&server)
            .await;

        // 10:00:45 in UTC-5 is 15:00 UTC; seconds are dropped.
        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KCXL".parse().unwrap(),
                "2026-08-30T10:00:45-05:00".parse().unwrap(),
            )
            .await
            .unwrap();
        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].headers["accept"],
            "application/vnd.wmo.iwxxm+xml"
        );
        assert_eq!(
            serde_json::to_value(forecast).unwrap()["report"]["kind"],
            "cancellation"
        );
    }

    #[tokio::test]
    async fn taf_document_serializes_semantic_json_without_wire_artifacts() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KCXL/tafs/2026-08-30/1500"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/cancellation.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KCXL".parse().unwrap(),
                "2026-08-30T15:00:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        let json = serde_json::to_value(forecast).unwrap();

        assert_eq!(
            json,
            serde_json::json!({
                "bulletinIdentifier": "A_LTUS99KCXL301500_C_KCXL_20260830150000.xml",
                "reportMetadata": {
                    "status": { "kind": "amendment" },
                    "permissibleUsage": { "kind": "operational" }
                },
                "issuedAt": "2026-08-30T15:00:00Z",
                "aerodrome": {
                    "designator": "KCXL",
                    "icaoIdentifier": "KCXL"
                },
                "report": {
                    "kind": "cancellation",
                    "cancelledPeriod": {
                        "start": "2026-08-30T12:00:00Z",
                        "end": "2026-08-31T12:00:00Z"
                    }
                }
            }),
        );
        let encoded = json.to_string();
        for wire_artifact in ["ns0", "ns1", "xlink", "xmlns", "meteorologicalInformation"] {
            assert!(
                !encoded.contains(wire_artifact),
                "found {wire_artifact} in {encoded}"
            );
        }
    }

    #[tokio::test]
    async fn taf_document_serializes_forecast_states_as_semantic_json() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KXYZ/tafs/2026-08-30/1200"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/semantic_edges.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KXYZ".parse().unwrap(),
                "2026-08-30T12:00:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        let json = serde_json::to_value(forecast).unwrap();

        assert_eq!(
            json.pointer("/report/groups/0/conditions/visibility/state"),
            Some(&serde_json::json!("value")),
        );
        assert_eq!(
            json.pointer("/report/groups/0/conditions/visibility/value/meters"),
            Some(&serde_json::json!(800.0)),
        );
        assert_eq!(
            json.pointer("/report/groups/2/conditions/visibility/state"),
            Some(&serde_json::json!("unavailable")),
        );
        assert_eq!(
            json.pointer("/report/groups/2/conditions/visibility/value/reason/kind"),
            Some(&serde_json::json!("notObservable")),
        );
    }

    #[tokio::test]
    async fn taf_document_errors_preserve_semantic_context_and_sources() {
        use std::error::Error as _;

        use crate::{apis::Error, models::terminal_aerodrome_forecast::TafDecodeErrorKind};

        let fixture = include_str!("../../tests/fixtures/taf/kflg_normal.xml");
        let cases = [
            (
                "KUNT",
                fixture.replacen("uom=\"m\"", "uom=\"sm\"", 1),
                TafDecodeErrorKind::UnsupportedUnit,
                "TAF.forecastGroup.visibility",
                false,
            ),
            (
                "KNUM",
                fixture.replacen(
                    ">10000</prevailingVisibility>",
                    ">not-a-number</prevailingVisibility>",
                    1,
                ),
                TafDecodeErrorKind::InvalidNumber,
                "TAF.forecastGroup.visibility",
                true,
            ),
            (
                "KCAV",
                fixture.replacen(
                    "cloudAndVisibilityOK=\"false\"",
                    "cloudAndVisibilityOK=\"true\"",
                    1,
                ),
                TafDecodeErrorKind::InvalidCombination,
                "TAF.forecastGroup.cloudAndVisibilityOK",
                false,
            ),
            (
                "KXML",
                fixture.replacen("</TAF>", "", 1),
                TafDecodeErrorKind::MalformedXml,
                "TAF",
                true,
            ),
            (
                "KNSP",
                fixture.replacen(
                    "xmlns=\"http://icao.int/iwxxm/2021-2\"",
                    "xmlns=\"urn:unsupported:iwxxm\"",
                    1,
                ),
                TafDecodeErrorKind::InvalidValue,
                "MeteorologicalBulletin.meteorologicalInformation.TAF",
                false,
            ),
            (
                "KPER",
                fixture.replacen(
                    "2026-08-31T18:00:00Z</ns1:endPosition>",
                    "2026-08-29T18:00:00Z</ns1:endPosition>",
                    1,
                ),
                TafDecodeErrorKind::InvalidPeriod,
                "TAF.validPeriod",
                false,
            ),
            (
                "KPOS",
                fixture.replacen("35.14 -111.67", "95 -111.67", 1),
                TafDecodeErrorKind::InvalidCoordinate,
                "TAF.aerodrome.position",
                false,
            ),
            (
                "KUSE",
                fixture.replacen(
                    "permissibleUsage=\"OPERATIONAL\"",
                    "permissibleUsage=\"OPERATIONAL\" permissibleUsageReason=\"TEST\"",
                    1,
                ),
                TafDecodeErrorKind::InvalidCombination,
                "TAF.permissibleUsageReason",
                false,
            ),
            (
                "KCVK",
                fixture.replacen(" cloudAndVisibilityOK=\"false\"", "", 1),
                TafDecodeErrorKind::MissingRequiredField,
                "TAF.forecastGroup.cloudAndVisibilityOK",
                false,
            ),
            (
                "KVOP",
                fixture.replacen(
                    "<prevailingVisibility uom=\"m\">10000</prevailingVisibility>",
                    "",
                    1,
                ),
                TafDecodeErrorKind::InvalidCombination,
                "TAF.forecastGroup.visibilityOperator",
                false,
            ),
        ];
        let server = MockServer::start().await;
        for (station, body, _, _, _) in &cases {
            Mock::given(method("GET"))
                .and(path(format!("/stations/{station}/tafs/2026-08-30/2257")))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(body.clone(), "application/vnd.wmo.iwxxm+xml"),
                )
                .mount(&server)
                .await;
        }

        let issued = "2026-08-30T22:57:00Z".parse().unwrap();
        for (station, _, expected_kind, expected_path, has_decode_source) in cases {
            let error = client_for(&server)
                .stations()
                .taf(&station.parse().unwrap(), issued)
                .await
                .unwrap_err();
            let Error::TerminalAerodromeForecast(decode) = &error else {
                panic!("expected semantic TAF decode error, got {error}");
            };

            assert_eq!(decode.kind(), expected_kind);
            assert_eq!(decode.path(), expected_path);
            assert_eq!(decode.source().is_some(), has_decode_source);
            assert!(error.source().is_some());
            assert!(error.to_string().contains(expected_path));
        }
    }

    #[tokio::test]
    async fn taf_document_distinguishes_cancellation_and_translation_failure() {
        use jiff::Timestamp;

        use crate::models::terminal_aerodrome_forecast::{
            ForecastReport, MissingForecastReason, ReportStatus,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KCXL/tafs/2026-08-30/1500"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/cancellation.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KERR/tafs/2026-08-30/1600"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/translation_failed.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let cancelled = client_for(&server)
            .stations()
            .taf(
                &"KCXL".parse().unwrap(),
                "2026-08-30T15:00:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        let ForecastReport::Cancellation { cancelled_period } = cancelled.report() else {
            panic!("expected cancellation");
        };
        assert_eq!(
            cancelled.report_metadata().status(),
            &ReportStatus::Amendment
        );
        assert_eq!(
            (cancelled_period.start(), cancelled_period.end()),
            (
                "2026-08-30T12:00:00Z".parse::<Timestamp>().unwrap(),
                "2026-08-31T12:00:00Z".parse::<Timestamp>().unwrap(),
            ),
        );

        let missing = client_for(&server)
            .stations()
            .taf(
                &"KERR".parse().unwrap(),
                "2026-08-30T16:00:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            missing.report(),
            &ForecastReport::Missing {
                reason: MissingForecastReason::TranslationFailed {
                    tac: "TAF KERR malformed source".into(),
                },
            },
        );
        let translation = missing.report_metadata().translation().unwrap();
        assert_eq!(
            translation.source_bulletin_identifier(),
            Some("FTUS99KERR301600")
        );
        assert_eq!(translation.centre_designator(), Some("KERR"));
        assert_eq!(translation.centre_name(), Some("Fixture Translator"));
        assert_eq!(
            translation.source_bulletin_received_at(),
            Some("2026-08-30T16:01:00Z".parse::<Timestamp>().unwrap()),
        );
        assert_eq!(
            translation.translated_at(),
            Some("2026-08-30T16:02:00Z".parse::<Timestamp>().unwrap()),
        );
    }

    #[tokio::test]
    async fn taf_document_normalizes_temperature_vertical_visibility_and_nil_meaning() {
        use jiff::Timestamp;

        use crate::models::terminal_aerodrome_forecast::{
            ForecastClouds, ForecastGroupKind, ForecastValue, ForecastVisibility, ForecastWeather,
            ForecastWind, MissingReason,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KXYZ/tafs/2026-08-30/1200"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/semantic_edges.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KXYZ".parse().unwrap(),
                "2026-08-30T12:00:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        let base = forecast.base_forecast().unwrap();
        let temperatures = base.conditions().temperatures();

        assert_eq!(base.conditions().weather(), &ForecastWeather::NoSignificant);
        assert_eq!(
            base.conditions().clouds(),
            &ForecastClouds::VerticalVisibility {
                feet: ForecastValue::Value(300.0),
            },
        );
        assert_eq!(temperatures.len(), 1);
        assert_eq!(temperatures[0].maximum().celsius(), 7.0);
        assert_eq!(
            temperatures[0].maximum().occurs_at(),
            "2026-08-30T21:00:00Z".parse::<Timestamp>().unwrap(),
        );
        assert_eq!(temperatures[0].minimum().celsius(), -5.0);
        assert_eq!(
            temperatures[0].minimum().occurs_at(),
            "2026-08-31T10:00:00Z".parse::<Timestamp>().unwrap(),
        );

        let changes = forecast.change_forecasts();
        assert_eq!(
            changes[0].conditions().weather(),
            &ForecastWeather::NoSignificant
        );
        assert_eq!(
            changes[0].conditions().clouds(),
            &ForecastClouds::NoSignificant
        );
        assert_eq!(
            changes[1].conditions().clouds(),
            &ForecastClouds::VerticalVisibility {
                feet: ForecastValue::Unavailable {
                    reason: MissingReason::NotObservable,
                },
            },
        );
        assert_eq!(
            changes[1].conditions().wind(),
            &ForecastWind::Unavailable {
                reason: MissingReason::NotObservable,
            },
        );
        assert_eq!(
            changes[1].conditions().visibility(),
            &ForecastVisibility::Unavailable {
                reason: MissingReason::NotObservable,
            },
        );
        assert_eq!(
            changes.iter().map(|group| group.kind()).collect::<Vec<_>>(),
            [
                &ForecastGroupKind::Becoming,
                &ForecastGroupKind::From,
                &ForecastGroupKind::Probability {
                    percent: 40,
                    temporary: false,
                },
                &ForecastGroupKind::Probability {
                    percent: 30,
                    temporary: true,
                },
                &ForecastGroupKind::Probability {
                    percent: 40,
                    temporary: true,
                },
            ],
        );
    }

    #[tokio::test]
    async fn taf_document_preserves_weather_qualifiers_and_cloud_types() {
        use crate::models::terminal_aerodrome_forecast::{
            CloudAmount, CloudType, ForecastClouds, ForecastValue, ForecastWeather,
            WeatherDescriptor, WeatherIntensity, WeatherPhenomenon,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KFLG".parse().unwrap(),
                "2026-08-30T22:57:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        let conditions = forecast.change_forecasts()[0].conditions();
        let ForecastWeather::Phenomena { items } = conditions.weather() else {
            panic!("expected significant weather");
        };
        let ForecastClouds::Layers { layers } = conditions.clouds() else {
            panic!("expected cloud layers");
        };

        assert_eq!(
            (
                items[0].code(),
                items[0].intensity(),
                items[0].is_in_vicinity(),
                items[0].descriptor(),
                items[0].phenomena(),
                layers[0].amount(),
                layers[0].base_feet(),
                layers[0].cloud_type(),
            ),
            (
                "-TSRA",
                WeatherIntensity::Light,
                false,
                Some(&WeatherDescriptor::Thunderstorm),
                &[WeatherPhenomenon::Rain][..],
                &ForecastValue::Value(CloudAmount::Broken),
                &ForecastValue::Value(5_000.0),
                Some(&ForecastValue::Value(CloudType::Cumulonimbus)),
            ),
        );
    }

    #[tokio::test]
    async fn taf_document_normalizes_period_visibility_and_wind() {
        use crate::models::terminal_aerodrome_forecast::{Comparison, WindDirection};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KFLG".parse().unwrap(),
                "2026-08-30T22:57:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        let base = forecast.base_forecast().unwrap();
        let visibility = base.conditions().visibility().value().unwrap();
        let wind = base.conditions().wind().value().unwrap();

        assert_eq!(
            (
                base.valid_period().start().to_string(),
                base.valid_period().end().to_string(),
                base.conditions().is_cavok(),
                visibility.meters(),
                visibility.comparison(),
                wind.direction(),
                wind.speed().knots(),
                wind.speed().comparison(),
                wind.gust().map(|gust| gust.knots()),
            ),
            (
                "2026-08-30T17:39:00Z".to_owned(),
                "2026-08-31T02:00:00Z".to_owned(),
                false,
                10_000.0,
                &Comparison::Above,
                WindDirection::Degrees(220.0),
                15.0,
                &Comparison::Exact,
                Some(25.0),
            ),
        );
    }

    #[tokio::test]
    async fn taf_document_normalizes_report_metadata_times_and_position() {
        use crate::models::terminal_aerodrome_forecast::{
            ForecastReport, PermissibleUsage, ReportStatus,
        };

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .mount(&server)
            .await;

        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KFLG".parse().unwrap(),
                "2026-08-30T22:57:00Z".parse().unwrap(),
            )
            .await
            .unwrap();
        let position = forecast.aerodrome().position().unwrap();
        let ForecastReport::Forecast { valid_period, .. } = forecast.report() else {
            panic!("expected an ordinary forecast report");
        };

        assert_eq!(
            (
                forecast.bulletin_identifier(),
                forecast.report_metadata().status(),
                forecast.report_metadata().permissible_usage(),
                forecast.issued_at().to_string(),
                position.latitude(),
                position.longitude(),
                valid_period.start().to_string(),
                valid_period.end().to_string(),
            ),
            (
                "A_LTUS45KFGZ301700_C_KFGZ_20260830173930.xml",
                &ReportStatus::Normal,
                &PermissibleUsage::Operational,
                "2026-08-30T17:39:00Z".to_owned(),
                35.14,
                -111.67,
                "2026-08-30T18:00:00Z".to_owned(),
                "2026-08-31T18:00:00Z".to_owned(),
            ),
        );
    }

    #[tokio::test]
    async fn taf_document_normalizes_identity_and_forecast_groups() {
        use crate::models::terminal_aerodrome_forecast::ForecastGroupKind;

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/stations/KFLG/tafs/2026-08-30/2257"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                include_str!("../../tests/fixtures/taf/kflg_normal.xml"),
                "application/vnd.wmo.iwxxm+xml",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let forecast = client_for(&server)
            .stations()
            .taf(
                &"KFLG".parse().unwrap(),
                "2026-08-30T22:57:00Z".parse().unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            (
                forecast.aerodrome().icao_identifier(),
                forecast.groups().len(),
                forecast.base_forecast().map(|group| group.kind()),
            ),
            ("KFLG", 6, Some(&ForecastGroupKind::Base)),
        );
    }
}
