//! Smoke tests for every endpoint handle through the public interface only.

use jiff::civil::date;
use noaa_weather_client::apis::alerts::ActiveAlertsQuery;
use noaa_weather_client::apis::aviation::SigmetsQuery;
use noaa_weather_client::apis::gridpoints::{ForecastQuery, ForecastUnits};
use noaa_weather_client::apis::products::ProductsQuery;
use noaa_weather_client::apis::radar::RadarQueueQuery;
use noaa_weather_client::apis::radio::TransmittersQuery;
use noaa_weather_client::apis::stations::ObservationsQuery;
use noaa_weather_client::apis::zones::{ZoneType, ZonesQuery};
use noaa_weather_client::models::{AlertSeverity, RadarQueueHost};
use noaa_weather_client::{
    AtsuId, CallSign, Client, Coordinates, CwsuId, Error, GridpointId, Interval, OfficeId,
    ProductTypeCode, RadarStationId, RetryPolicy, StationId, ZoneId,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

const USER_AGENT: &str = "noaa-weather-handles/1.0 (tests@example.com)";
const FEATURE: &str = r#"{"type":"Feature","geometry":null,"properties":{}}"#;
const COLLECTION: &str = r#"{"type":"FeatureCollection","features":[]}"#;
const GRAPH: &str = r#"{"@context":{},"@graph":[]}"#;

fn client_for(server: &MockServer) -> Client {
    Client::builder(USER_AGENT)
        .base_url(server.uri())
        .retry(RetryPolicy::none())
        .build()
        .expect("valid test client")
}

async fn mount(server: &MockServer, route: &str, body: &'static str, media: &'static str) {
    Mock::given(method("GET"))
        .and(path(route))
        .and(header("Accept", media))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body, media))
        .expect(1)
        .mount(server)
        .await;
}

async fn only_query(server: &MockServer) -> Option<String> {
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    requests[0].url.query().map(str::to_owned)
}

#[tokio::test]
async fn alerts_handle_encodes_filters_as_csv() {
    let server = MockServer::start().await;
    mount(
        &server,
        "/alerts/active",
        COLLECTION,
        "application/geo+json",
    )
    .await;

    let alerts = client_for(&server)
        .alerts()
        .active(&ActiveAlertsQuery {
            severity: vec![AlertSeverity::Severe, AlertSeverity::Extreme],
            zone: vec!["AZZ540".parse::<ZoneId>().unwrap()],
            ..Default::default()
        })
        .await
        .unwrap();

    assert!(alerts.is_empty());
    assert_eq!(alerts.next_cursor(), None);
    assert_eq!(
        only_query(&server).await.as_deref(),
        Some("zone=AZZ540&severity=Severe%2CExtreme")
    );
}

#[tokio::test]
async fn points_handle_sends_coordinates_as_one_segment() {
    let server = MockServer::start().await;
    mount(
        &server,
        "/points/39.7456,-97.0892",
        FEATURE,
        "application/geo+json",
    )
    .await;

    client_for(&server)
        .points()
        .get(Coordinates::new(39.7456, -97.0892).unwrap())
        .await
        .unwrap();
    assert_eq!(only_query(&server).await, None);
}

#[tokio::test]
async fn points_forecast_for_chains_point_lookup_into_gridpoint_forecast() {
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
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"type":"Feature","geometry":null,"properties":{"periods":[{"name":"Tonight"}]}}"#,
            "application/geo+json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let forecast = client_for(&server)
        .points()
        .forecast_for(Coordinates::new(39.7456, -97.0892).unwrap())
        .await
        .unwrap();
    let periods = forecast.properties.periods.unwrap();
    assert_eq!(periods[0].name.as_deref(), Some("Tonight"));
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[1].headers["feature-flags"],
        "forecast_temperature_qv,forecast_wind_speed_qv"
    );
}

#[tokio::test]
async fn points_forecast_for_rejects_a_point_without_a_grid() {
    let server = MockServer::start().await;
    mount(
        &server,
        "/points/39.7456,-97.0892",
        FEATURE,
        "application/geo+json",
    )
    .await;

    let error = client_for(&server)
        .points()
        .forecast_for(Coordinates::new(39.7456, -97.0892).unwrap())
        .await
        .unwrap_err();
    assert!(matches!(error, Error::Invalid(_)), "{error}");
}

#[tokio::test]
async fn gridpoints_handle_uses_the_grid_id_and_units_query() {
    let server = MockServer::start().await;
    mount(
        &server,
        "/gridpoints/TOP/31,80/forecast/hourly",
        FEATURE,
        "application/geo+json",
    )
    .await;

    let grid: GridpointId = "top/31,80".parse().unwrap();
    client_for(&server)
        .gridpoints()
        .forecast_hourly(
            &grid,
            &ForecastQuery {
                units: Some(ForecastUnits::Si),
            },
        )
        .await
        .unwrap();
    assert_eq!(only_query(&server).await.as_deref(), Some("units=si"));
}

#[tokio::test]
async fn stations_handle_encodes_timestamps_as_rfc_3339() {
    let server = MockServer::start().await;
    mount(
        &server,
        "/stations/KPHX/observations",
        COLLECTION,
        "application/geo+json",
    )
    .await;

    let station: StationId = "kphx".parse().unwrap();
    client_for(&server)
        .stations()
        .observations(
            &station,
            &ObservationsQuery {
                start: Some("2026-08-30T00:00:00-07:00".parse().unwrap()),
                limit: Some(3),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(
        only_query(&server).await.as_deref(),
        Some("start=2026-08-30T07%3A00%3A00Z&limit=3")
    );
}

#[tokio::test]
async fn zones_handle_places_type_and_id_in_the_path_and_point_in_the_query() {
    let server = MockServer::start().await;
    mount(&server, "/zones/county", COLLECTION, "application/geo+json").await;

    client_for(&server)
        .zones()
        .list_of_type(
            ZoneType::County,
            &ZonesQuery {
                point: Some(Coordinates::new(33.4484, -112.074).unwrap()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(
        only_query(&server).await.as_deref(),
        Some("point=33.4484%2C-112.074")
    );
}

#[tokio::test]
async fn offices_handle_returns_binary_payloads_for_documents() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/offices/PSR/briefing/download/latest"))
        .and(header("Accept", "application/pdf"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(b"%PDF-1.7")
                .insert_header("Content-Type", "application/pdf"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let office: OfficeId = "psr".parse().unwrap();
    let payload = client_for(&server)
        .offices()
        .latest_briefing_document(&office)
        .await
        .unwrap();
    assert_eq!(payload.as_bytes(), b"%PDF-1.7");
    assert_eq!(payload.content_type().essence_str(), "application/pdf");
}

#[tokio::test]
async fn products_handle_encodes_typed_lists_under_wire_names() {
    let server = MockServer::start().await;
    mount(&server, "/products", "{}", "application/ld+json").await;

    client_for(&server)
        .products()
        .search(&ProductsQuery {
            location_ids: vec!["PSR".parse::<OfficeId>().unwrap()],
            product_type_codes: vec!["afd".parse::<ProductTypeCode>().unwrap()],
            limit: Some(2),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_eq!(
        only_query(&server).await.as_deref(),
        Some("location=PSR&type=AFD&limit=2")
    );
}

#[tokio::test]
async fn aviation_handle_formats_dates_and_issue_instants() {
    let server = MockServer::start().await;
    mount(
        &server,
        "/aviation/cwsus/ZAB/cwas/2026-08-30/101",
        FEATURE,
        "application/geo+json",
    )
    .await;
    mount(
        &server,
        "/aviation/sigmets/KKCI/2026-08-30/1430",
        FEATURE,
        "application/geo+json",
    )
    .await;
    Mock::given(method("GET"))
        .and(path("/aviation/sigmets"))
        .and(query_param("atsu", "KKCI"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(COLLECTION, "application/geo+json"))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let zab: CwsuId = "zab".parse().unwrap();
    let kkci: AtsuId = "kkci".parse().unwrap();
    client
        .aviation()
        .cwa(&zab, date(2026, 8, 30), 101)
        .await
        .unwrap();
    client
        .aviation()
        .sigmet(&kkci, "2026-08-30T14:30:00Z".parse().unwrap())
        .await
        .unwrap();
    client
        .aviation()
        .sigmets(&SigmetsQuery {
            atsu: Some(kkci),
            ..Default::default()
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn radar_handle_encodes_intervals_and_station_ids() {
    let server = MockServer::start().await;
    mount(&server, "/radar/queues/tds", GRAPH, "application/ld+json").await;

    let interval: Interval = "2026-08-30T00:00:00Z/PT2H".parse().unwrap();
    client_for(&server)
        .radar()
        .queue(
            &RadarQueueHost::Tds,
            &RadarQueueQuery {
                limit: Some(5),
                published: Some(interval),
                station: Some("kabq".parse::<RadarStationId>().unwrap()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(
        only_query(&server).await.as_deref(),
        Some("limit=5&published=2026-08-30T00%3A00%3A00Z%2FPT2H&station=KABQ")
    );
}

#[tokio::test]
async fn radio_handle_pages_transmitters_and_decodes_broadcasts() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/radio"))
        .and(query_param("cursor", "abc"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(r#"{"@graph":[]}"#, "application/ld+json"),
        )
        .expect(1)
        .mount(&server)
        .await;
    mount(
        &server,
        "/radio/KEC94/broadcast",
        r#"<speak version="1.1" xml:lang="en-US"></speak>"#,
        "application/ssml+xml",
    )
    .await;

    let client = client_for(&server);
    client
        .radio()
        .transmitters(&TransmittersQuery {
            cursor: Some("abc".parse().unwrap()),
        })
        .await
        .unwrap();
    let call_sign: CallSign = "kec94".parse().unwrap();
    let broadcast = client.radio().broadcast(&call_sign).await.unwrap();
    assert_eq!(broadcast.lang, "en-US");
}

#[tokio::test]
async fn glossary_handle_requests_json_ld() {
    let server = MockServer::start().await;
    mount(
        &server,
        "/glossary",
        r#"{"glossary":[]}"#,
        "application/ld+json",
    )
    .await;

    let terms = client_for(&server).glossary().terms().await.unwrap();
    assert!(terms.glossary.is_empty());
}

#[test]
fn handles_are_copy_and_borrow_the_client() {
    fn assert_copy<T: Copy>(_: T) {}
    let client = Client::builder(USER_AGENT).build().unwrap();
    assert_copy(client.alerts());
    assert_copy(client.points());
    assert_copy(client.gridpoints());
    assert_copy(client.stations());
    assert_copy(client.zones());
    assert_copy(client.offices());
    assert_copy(client.products());
    assert_copy(client.aviation());
    assert_copy(client.radar());
    assert_copy(client.radio());
    assert_copy(client.glossary());
}

#[cfg(feature = "schemars")]
mod schema {
    use noaa_weather_client::apis::alerts::{ActiveAlertsQuery, AlertsQuery};
    use noaa_weather_client::apis::aviation::SigmetsQuery;
    use noaa_weather_client::apis::gridpoints::{ForecastQuery, GridpointStationsQuery};
    use noaa_weather_client::apis::products::ProductsQuery;
    use noaa_weather_client::apis::radar::{
        RadarQueueQuery, RadarServerQuery, RadarServersQuery, RadarStationQuery,
        RadarStationsQuery, SpgdsQuery, WindProfilerQuery,
    };
    use noaa_weather_client::apis::radio::TransmittersQuery;
    use noaa_weather_client::apis::stations::{
        LatestObservationQuery, ObservationsQuery, StationsQuery,
    };
    use noaa_weather_client::apis::zones::{
        ZoneObservationsQuery, ZoneQuery, ZoneStationsQuery, ZonesQuery,
    };

    fn assert_object_schema<T: schemars::JsonSchema>(properties: &[&str]) {
        let schema = schemars::schema_for!(T);
        let value = schema.as_value();
        assert_eq!(value["type"], "object", "{}: {value}", T::schema_name());
        let mut found = value["properties"]
            .as_object()
            .unwrap_or_else(|| panic!("{} has no properties", T::schema_name()))
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        found.sort();
        let mut expected = properties
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        expected.sort();
        assert_eq!(found, expected, "{}", T::schema_name());
    }

    const ALERT_FILTERS: [&str; 12] = [
        "status",
        "messageType",
        "event",
        "code",
        "area",
        "point",
        "region",
        "regionType",
        "zone",
        "urgency",
        "severity",
        "certainty",
    ];

    #[test]
    fn every_query_struct_is_an_object_schema_with_its_camel_case_properties() {
        assert_object_schema::<ActiveAlertsQuery>(&ALERT_FILTERS);
        let mut alerts = ALERT_FILTERS.to_vec();
        alerts.extend(["start", "end", "limit", "cursor"]);
        assert_object_schema::<AlertsQuery>(&alerts);
        assert_object_schema::<ForecastQuery>(&["units"]);
        assert_object_schema::<GridpointStationsQuery>(&["limit"]);
        assert_object_schema::<StationsQuery>(&["id", "state", "limit", "cursor"]);
        assert_object_schema::<LatestObservationQuery>(&["requireQc"]);
        assert_object_schema::<ObservationsQuery>(&["start", "end", "limit", "cursor"]);
        assert_object_schema::<ZoneQuery>(&["effective"]);
        assert_object_schema::<ZonesQuery>(&[
            "id",
            "area",
            "region",
            "type",
            "point",
            "includeGeometry",
            "limit",
            "effective",
        ]);
        assert_object_schema::<ZoneObservationsQuery>(&["start", "end", "limit"]);
        assert_object_schema::<ZoneStationsQuery>(&["limit", "cursor"]);
        assert_object_schema::<ProductsQuery>(&[
            "locationIds",
            "start",
            "end",
            "officeIds",
            "wmoIds",
            "productTypeCodes",
            "limit",
        ]);
        assert_object_schema::<SigmetsQuery>(&["start", "end", "date", "atsu", "sequence"]);
        assert_object_schema::<WindProfilerQuery>(&["time", "interval"]);
        assert_object_schema::<RadarQueueQuery>(&[
            "limit",
            "arrived",
            "created",
            "published",
            "station",
            "type",
            "feed",
            "resolution",
        ]);
        assert_object_schema::<RadarServersQuery>(&["reportingHost"]);
        assert_object_schema::<RadarServerQuery>(&["reportingHost"]);
        assert_object_schema::<RadarStationsQuery>(&["stationType", "reportingHost", "host"]);
        assert_object_schema::<RadarStationQuery>(&["reportingHost", "host"]);
        assert_object_schema::<SpgdsQuery>(&["published"]);
        assert_object_schema::<TransmittersQuery>(&["cursor"]);
    }

    #[test]
    fn limits_publish_their_range_and_typed_values_inline_as_strings() {
        let schema = schemars::schema_for!(AlertsQuery);
        let value = schema.as_value();
        let limit = &value["properties"]["limit"];
        assert_eq!(limit["minimum"], 1, "{limit}");
        assert_eq!(limit["maximum"], 500, "{limit}");
        let zone = &value["properties"]["zone"];
        assert_eq!(zone["type"], "array", "{zone}");
        assert_eq!(zone["items"]["type"], "string", "{zone}");
        assert!(zone["items"]["pattern"].is_string(), "{zone}");
        assert!(value.get("$defs").is_none(), "{value}");
    }
}
