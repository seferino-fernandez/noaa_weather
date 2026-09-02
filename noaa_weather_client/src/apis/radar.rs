//! Radar infrastructure: stations, servers, data queues, wind profilers, and
//! SPGDS telemetry, the `/radar` family.
//!
//! Obtain the handle with [`Client::radar`]. Radar stations are addressed by
//! [`RadarStationId`]; server and profiler ids are opaque strings NOAA
//! issues. Time-range filters take an [`Interval`].
//!
//! ```no_run
//! use noaa_weather_client::{Client, apis::radar::RadarStationsQuery};
//!
//! # async fn run() -> Result<(), noaa_weather_client::Error> {
//! let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
//! let stations = client
//!     .radar()
//!     .stations(&RadarStationsQuery {
//!         station_type: vec!["WSR-88D".to_owned()],
//!         ..Default::default()
//!     })
//!     .await?;
//! # let _ = stations;
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

use super::Error;
use crate::client::{Client, http};
use crate::ids::RadarStationId;
use crate::models::{self, RadarQueueHost};
use crate::time::Interval;

/// Filters for [`Radar::wind_profiler`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct WindProfilerQuery {
    /// Time range of the data to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<Interval>,
    /// Sampling interval of the data to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
}

impl http::QueryParams for WindProfilerQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("time", self.time.as_ref());
        request.scalar("interval", self.interval.as_ref());
    }
}

/// Filters for [`Radar::queue`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct RadarQueueQuery {
    /// Maximum number of queue entries to return (1 to 50,000). NOAA
    /// rejects requests without a limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 50_000)))]
    pub limit: Option<u16>,
    /// Only entries that arrived within this interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrived: Option<Interval>,
    /// Only entries created within this interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Interval>,
    /// Only entries published within this interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<Interval>,
    /// Only entries from this radar station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub station: Option<RadarStationId>,
    /// Only entries of this data type (`type` on the wire).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// Only entries from this feed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<String>,
    /// Only entries at this resolution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<u32>,
}

impl http::QueryParams for RadarQueueQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("limit", self.limit.as_ref());
        request.scalar("arrived", self.arrived.as_ref());
        request.scalar("created", self.created.as_ref());
        request.scalar("published", self.published.as_ref());
        request.scalar("station", self.station.as_ref());
        request.scalar("type", self.data_type.as_ref());
        request.scalar("feed", self.feed.as_ref());
        request.scalar("resolution", self.resolution.as_ref());
    }
}

/// Filters for [`Radar::servers`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct RadarServersQuery {
    /// Only servers reporting through this host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
}

impl http::QueryParams for RadarServersQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("reportingHost", self.reporting_host.as_ref());
    }
}

/// Filters for [`Radar::server`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct RadarServerQuery {
    /// Report the server as seen from this host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
}

impl http::QueryParams for RadarServerQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("reportingHost", self.reporting_host.as_ref());
    }
}

/// Filters for [`Radar::stations`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct RadarStationsQuery {
    /// Station types to include, such as `WSR-88D` or `TDWR`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub station_type: Vec<String>,
    /// Only stations reporting through this host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
    /// Only stations served by this queue host.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub host: Option<RadarQueueHost>,
}

impl http::QueryParams for RadarStationsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.list("stationType", &self.station_type);
        request.scalar("reportingHost", self.reporting_host.as_ref());
        request.scalar("host", self.host.as_ref());
    }
}

/// Filters for [`Radar::station`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct RadarStationQuery {
    /// Report the station as seen from this host.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_host: Option<String>,
    /// Report the station as served by this queue host.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schemars", schemars(with = "Option<String>"))]
    pub host: Option<RadarQueueHost>,
}

impl http::QueryParams for RadarStationQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("reportingHost", self.reporting_host.as_ref());
        request.scalar("host", self.host.as_ref());
    }
}

/// Filters for [`Radar::spgds`].
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default)]
pub struct SpgdsQuery {
    /// Only telemetry published within this interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<Interval>,
}

impl http::QueryParams for SpgdsQuery {
    fn append_to(&self, request: &mut http::ContractRequest<'_>) {
        request.scalar("published", self.published.as_ref());
    }
}

/// The `/radar` endpoints, obtained from [`Client::radar`].
#[derive(Clone, Copy, Debug)]
pub struct Radar<'a> {
    client: &'a Client,
}

impl Client {
    /// Returns the handle for the `/radar` endpoints.
    #[must_use]
    pub fn radar(&self) -> Radar<'_> {
        Radar { client: self }
    }
}

impl Radar<'_> {
    /// Returns metadata for one radar wind profiler. NOAA publishes no
    /// schema for this response, so it is returned as raw JSON.
    ///
    /// `GET /radar/profilers/{stationId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::radar::WindProfilerQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let profiler = client
    ///     .radar()
    ///     .wind_profiler("HWPA2", &WindProfilerQuery::default())
    ///     .await?;
    /// # let _ = profiler;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response is not JSON.
    pub async fn wind_profiler(
        &self,
        profiler_id: &str,
        query: &WindProfilerQuery,
    ) -> Result<serde_json::Value, Error> {
        http::request(self.client, "/radar/profilers")
            .path_segment(profiler_id)
            .query(query)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns the data queue of one host.
    ///
    /// `GET /radar/queues/{host}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::radar::RadarQueueQuery};
    /// use noaa_weather_client::models::RadarQueueHost;
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let queue = client
    ///     .radar()
    ///     .queue(&RadarQueueHost::Rds, &RadarQueueQuery { limit: Some(10), ..Default::default() })
    ///     .await?;
    /// # let _ = queue;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn queue(
        &self,
        host: &RadarQueueHost,
        query: &RadarQueueQuery,
    ) -> Result<models::RadarQueuesResponse, Error> {
        http::request(self.client, "/radar/queues")
            .path_segment(host)
            .query(query)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns every radar server.
    ///
    /// `GET /radar/servers`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::radar::RadarServersQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let servers = client.radar().servers(&RadarServersQuery::default()).await?;
    /// # let _ = servers;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn servers(
        &self,
        query: &RadarServersQuery,
    ) -> Result<models::RadarServersResponse, Error> {
        http::request(self.client, "/radar/servers")
            .query(query)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns one radar server by its server-issued id.
    ///
    /// `GET /radar/servers/{id}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::radar::RadarServerQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let server = client.radar().server("ldm1", &RadarServerQuery::default()).await?;
    /// # let _ = server;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn server(
        &self,
        server_id: &str,
        query: &RadarServerQuery,
    ) -> Result<models::RadarServer, Error> {
        http::request(self.client, "/radar/servers")
            .path_segment(server_id)
            .query(query)
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns radar stations matching `query`.
    ///
    /// `GET /radar/stations`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, apis::radar::RadarStationsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let stations = client.radar().stations(&RadarStationsQuery::default()).await?;
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
        query: &RadarStationsQuery,
    ) -> Result<models::RadarStationsResponse, Error> {
        http::request(self.client, "/radar/stations")
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns one radar station.
    ///
    /// `GET /radar/stations/{stationId}`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, RadarStationId, apis::radar::RadarStationQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let kabq: RadarStationId = "KABQ".parse()?;
    /// let station = client.radar().station(&kabq, &RadarStationQuery::default()).await?;
    /// # let _ = station;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn station(
        &self,
        station: &RadarStationId,
        query: &RadarStationQuery,
    ) -> Result<models::RadarStationFeature, Error> {
        http::request(self.client, "/radar/stations")
            .path_segment(station)
            .query(query)
            .json(http::JsonMedia::GeoJson)
            .await
    }

    /// Returns the alarms raised by one radar station.
    ///
    /// `GET /radar/stations/{stationId}/alarms`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, RadarStationId};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let kabq: RadarStationId = "KABQ".parse()?;
    /// let alarms = client.radar().station_alarms(&kabq).await?;
    /// # let _ = alarms;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn station_alarms(
        &self,
        station: &RadarStationId,
    ) -> Result<models::RadarStationAlarmsResponse, Error> {
        http::request(self.client, "/radar/stations")
            .path_segment(station)
            .literal_path("alarms")
            .json(http::JsonMedia::JsonLd)
            .await
    }

    /// Returns SPGDS host telemetry.
    ///
    /// `GET /radar/spgds`
    ///
    /// ```no_run
    /// use noaa_weather_client::{Client, Interval, apis::radar::SpgdsQuery};
    ///
    /// # async fn run() -> Result<(), noaa_weather_client::Error> {
    /// let client = Client::builder("app/1.0 (contact@example.com)").build().unwrap();
    /// let last_hour: Interval = "2026-08-30T00:00:00Z/PT1H".parse()?;
    /// let telemetry = client
    ///     .radar()
    ///     .spgds(&SpgdsQuery { published: Some(last_hour) })
    ///     .await?;
    /// # let _ = telemetry;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the request fails or the response cannot be
    /// decoded.
    pub async fn spgds(&self, query: &SpgdsQuery) -> Result<models::RadarSpgdsResponse, Error> {
        http::request(self.client, "/radar/spgds")
            .query(query)
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

    use super::{
        RadarQueueQuery, RadarServerQuery, RadarServersQuery, RadarStationQuery,
        RadarStationsQuery, SpgdsQuery, WindProfilerQuery,
    };
    use crate::{Error, client::test_support::client_for, models::RadarQueueHost};

    async fn mount(server: &MockServer, route: &str, body: &'static str, media: &'static str) {
        Mock::given(method("GET"))
            .and(path(route))
            .and(header("Accept", media))
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, media))
            .expect(1)
            .mount(server)
            .await;
    }

    async fn first_query(server: &MockServer) -> Option<String> {
        let requests = server.received_requests().await.unwrap();
        requests[0].url.query().map(str::to_owned)
    }

    #[tokio::test]
    async fn stations_encode_types_as_csv_and_request_geo_json() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/radar/stations",
            r#"{"type":"FeatureCollection","features":[]}"#,
            "application/geo+json",
        )
        .await;

        client_for(&server)
            .radar()
            .stations(&RadarStationsQuery {
                station_type: vec!["WSR-88D".to_owned(), "TD/WR".to_owned()],
                reporting_host: Some("report/host".to_owned()),
                host: Some(RadarQueueHost::Rds),
            })
            .await
            .unwrap();

        assert_eq!(
            first_query(&server).await.as_deref(),
            Some("stationType=WSR-88D%2CTD%2FWR&reportingHost=report%2Fhost&host=rds")
        );
    }

    #[tokio::test]
    async fn station_and_alarms_normalize_the_station_id() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/radar/stations/KABQ",
            r#"{"type":"Feature","geometry":null,"properties":{}}"#,
            "application/geo+json",
        )
        .await;
        mount(
            &server,
            "/radar/stations/KABQ/alarms",
            r#"{"@context":{},"@graph":[]}"#,
            "application/ld+json",
        )
        .await;

        let client = client_for(&server);
        let kabq = "kabq".parse().unwrap();
        client
            .radar()
            .station(
                &kabq,
                &RadarStationQuery {
                    reporting_host: Some("report/host".to_owned()),
                    host: Some(RadarQueueHost::Tds),
                },
            )
            .await
            .unwrap();
        client.radar().station_alarms(&kabq).await.unwrap();

        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests[0].url.query(),
            Some("reportingHost=report%2Fhost&host=tds")
        );
        assert_eq!(requests[1].url.query(), None);
    }

    #[tokio::test]
    async fn queue_encodes_intervals_scalars_and_wire_names_in_order() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/radar/queues/rds",
            r#"{"@context":{},"@graph":[]}"#,
            "application/ld+json",
        )
        .await;

        client_for(&server)
            .radar()
            .queue(
                &RadarQueueHost::Rds,
                &RadarQueueQuery {
                    limit: Some(10),
                    arrived: Some("2026-08-30T12:00:00Z/PT1H".parse().unwrap()),
                    created: Some("PT15M/2026-08-30T12:00:00Z".parse().unwrap()),
                    published: Some("PT30M".parse().unwrap()),
                    station: Some("kiwa".parse().unwrap()),
                    data_type: Some("LEVEL2".to_owned()),
                    feed: Some("level2".to_owned()),
                    resolution: Some(1),
                },
            )
            .await
            .unwrap();

        assert_eq!(
            first_query(&server).await.as_deref(),
            Some(
                "limit=10&arrived=2026-08-30T12%3A00%3A00Z%2FPT1H\
                 &created=PT15M%2F2026-08-30T12%3A00%3A00Z&published=PT30M\
                 &station=KIWA&type=LEVEL2&feed=level2&resolution=1"
            )
        );
    }

    #[tokio::test]
    async fn queue_intervals_built_from_fractional_instants_send_whole_seconds() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/radar/queues/tds",
            r#"{"@context":{},"@graph":[]}"#,
            "application/ld+json",
        )
        .await;

        let arrived = crate::Interval::between(
            "2026-08-30T00:00:00.123456789Z".parse().unwrap(),
            "2026-08-30T01:00:00.999Z".parse().unwrap(),
        )
        .unwrap();
        client_for(&server)
            .radar()
            .queue(
                &RadarQueueHost::Tds,
                &RadarQueueQuery {
                    arrived: Some(arrived),
                    ..Default::default()
                },
            )
            .await
            .unwrap();

        assert_eq!(
            first_query(&server).await.as_deref(),
            Some("arrived=2026-08-30T00%3A00%3A00Z%2F2026-08-30T01%3A00%3A00Z")
        );
    }

    #[tokio::test]
    async fn servers_and_server_use_the_reporting_host_wire_name() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/radar/servers",
            r#"{"@context":{},"@graph":[]}"#,
            "application/ld+json",
        )
        .await;
        mount(
            &server,
            "/radar/servers/ldm%2Fone%20host",
            r#"{"id":"ldm/one host"}"#,
            "application/ld+json",
        )
        .await;

        let client = client_for(&server);
        client
            .radar()
            .servers(&RadarServersQuery {
                reporting_host: Some("report/host".to_owned()),
            })
            .await
            .unwrap();
        let response = client
            .radar()
            .server(
                "ldm/one host",
                &RadarServerQuery {
                    reporting_host: Some("report/host".to_owned()),
                },
            )
            .await
            .unwrap();
        assert_eq!(response.id.as_deref(), Some("ldm/one host"));

        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), Some("reportingHost=report%2Fhost"));
        assert_eq!(requests[1].url.query(), Some("reportingHost=report%2Fhost"));
    }

    #[tokio::test]
    async fn wind_profiler_encodes_time_and_interval() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/radar/profilers/HWPA2",
            "{}",
            "application/ld+json",
        )
        .await;

        client_for(&server)
            .radar()
            .wind_profiler(
                "HWPA2",
                &WindProfilerQuery {
                    time: Some("2026-08-30T00:00:00Z/2026-08-30T01:00:00Z".parse().unwrap()),
                    interval: Some("PT1H".parse().unwrap()),
                },
            )
            .await
            .unwrap();

        assert_eq!(
            first_query(&server).await.as_deref(),
            Some("time=2026-08-30T00%3A00%3A00Z%2F2026-08-30T01%3A00%3A00Z&interval=PT1H")
        );
    }

    #[tokio::test]
    async fn spgds_omits_query_by_default_and_decodes_tolerant_telemetry() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radar/spgds"))
            .and(header("Accept", "application/ld+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{
                    "@context": {},
                    "@graph": [{
                        "@type": "SPGDS",
                        "id": 7,
                        "timestamp": true,
                        "dataflow": {"state": 1, "unknown": []},
                        "ldm": {"conns": 47.5},
                        "throughput": {"in": false, "out": "42"},
                        "spg": {"TXYZ": {"swimDataState": 0, "ldmPingState": true}},
                        "unknown": {"nested": "ignored"}
                    }]
                }"#,
                "application/ld+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let response = client_for(&server)
            .radar()
            .spgds(&SpgdsQuery::default())
            .await
            .unwrap();

        assert_eq!(response.spgds.len(), 1);
        let entry = &response.spgds[0];
        assert_eq!(entry.id.as_deref(), Some("7"));
        assert_eq!(entry.ldm.as_ref().unwrap().conns.as_deref(), Some("47.5"));
        assert_eq!(entry.spg["TXYZ"].swim_data_state.as_deref(), Some("0"));
        assert_eq!(first_query(&server).await, None);
    }

    #[tokio::test]
    async fn spgds_sends_published_interval_once_with_exact_percent_encoding() {
        let server = MockServer::start().await;
        mount(
            &server,
            "/radar/spgds",
            r#"{"@graph":[]}"#,
            "application/ld+json",
        )
        .await;

        client_for(&server)
            .radar()
            .spgds(&SpgdsQuery {
                published: Some(
                    "2026-01-01T00:00:00+00:00/2026-01-01T01:30:00+00:00"
                        .parse()
                        .unwrap(),
                ),
            })
            .await
            .unwrap();

        assert_eq!(
            first_query(&server).await.as_deref(),
            Some("published=2026-01-01T00%3A00%3A00Z%2F2026-01-01T01%3A30%3A00Z")
        );
    }

    #[tokio::test]
    async fn retains_typed_problem_detail_for_non_success_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/radar/spgds"))
            .respond_with(ResponseTemplate::new(503).set_body_raw(
                r#"{"type":"https://api.weather.gov/problems/unavailable","title":"Unavailable","status":503,"detail":"Try later","instance":"urn:test","correlationId":"test-correlation"}"#,
                "application/problem+json",
            ))
            .expect(1)
            .mount(&server)
            .await;

        let error = client_for(&server)
            .radar()
            .spgds(&SpgdsQuery::default())
            .await
            .unwrap_err();
        let Error::Response(response) = error else {
            panic!("expected response error");
        };
        let problem = response.problem_detail().expect("typed problem detail");
        assert_eq!(problem.title, "Unavailable");
        assert_eq!(problem.status, 503.0);
    }

    #[test]
    fn queue_query_serializes_data_type_under_the_wire_name() {
        let json = serde_json::to_value(RadarQueueQuery {
            data_type: Some("LEVEL2".to_owned()),
            published: Some("PT1H".parse().unwrap()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(
            json,
            serde_json::json!({"type": "LEVEL2", "published": "PT1H"})
        );
    }
}
