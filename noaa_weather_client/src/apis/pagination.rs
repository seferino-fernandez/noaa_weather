//! Bounded multi-page collection, shared by the `*_all` handle methods.
//!
//! NOAA pages `/alerts`, `/stations`, and `/stations/{id}/observations`
//! with an opaque cursor carried in `pagination.next`. The helper here walks
//! that chain up to a caller-chosen number of pages and merges the pages
//! into one [`FeatureCollection`].

use std::num::NonZeroU16;

use super::Error;
use crate::geo::FeatureCollection;
use crate::ids::Cursor;

/// A query struct whose `cursor` field selects the page to fetch.
pub(crate) trait Paged: Clone {
    /// Returns a copy of this query positioned at `cursor`.
    fn at_cursor(&self, cursor: Cursor) -> Self;
}

/// Fetches up to `max_pages` pages starting from `query` and merges them.
///
/// The first page is fetched with the caller's query as given, cursor
/// included. Each following page is fetched with the previous page's
/// [`next_cursor`](FeatureCollection::next_cursor) until a page offers no
/// usable cursor or `max_pages` pages have been fetched. Features are
/// concatenated in order; `title` and `updated` come from the first page;
/// `pagination` is the last page's link when the cap stopped the walk with
/// more available, and `None` when the collection was exhausted. An error on
/// any page is returned as is, with no partial result.
pub(crate) async fn collect<Q, T, F>(
    query: &Q,
    max_pages: NonZeroU16,
    fetch: impl Fn(Q) -> F,
) -> Result<FeatureCollection<T>, Error>
where
    Q: Paged,
    F: Future<Output = Result<FeatureCollection<T>, Error>>,
{
    let mut merged = fetch(query.clone()).await?;
    let mut fetched: u16 = 1;
    let mut cursor = merged.next_cursor();
    while let Some(next) = cursor.take() {
        if fetched >= max_pages.get() {
            return Ok(merged);
        }
        let page = fetch(query.at_cursor(next)).await?;
        fetched += 1;
        cursor = page.next_cursor();
        merged.features.extend(page.features);
        merged.pagination = page.pagination;
    }
    merged.pagination = None;
    Ok(merged)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path, query_param, query_param_is_missing},
    };

    use super::*;
    use crate::apis::alerts::AlertsQuery;
    use crate::apis::stations::{ObservationsQuery, StationsQuery};
    use crate::client::test_support::client_for;
    use crate::models::Alert;

    fn page(events: &[&str], next: Option<&str>) -> String {
        let features: Vec<_> = events
            .iter()
            .map(|event| {
                json!({
                    "id": format!("https://api.weather.gov/alerts/{event}"),
                    "type": "Feature",
                    "geometry": null,
                    "properties": {
                        "id": format!("urn:oid:2.49.0.1.840.0.{event}.001.1"),
                        "areaDesc": "Kent",
                        "sent": "2026-09-02T03:48:00-04:00",
                        "effective": "2026-09-02T03:48:00-04:00",
                        "expires": "2026-09-02T04:45:00-04:00",
                        "status": "Actual",
                        "messageType": "Alert",
                        "category": "Met",
                        "severity": "Moderate",
                        "certainty": "Observed",
                        "urgency": "Expected",
                        "event": event,
                        "sender": "w-nws.webmaster@noaa.gov",
                        "senderName": "NWS Grand Rapids MI",
                        "scope": "Public"
                    }
                })
            })
            .collect();
        let mut body = json!({
            "@context": [],
            "type": "FeatureCollection",
            "features": features,
            "title": "Alerts",
            "updated": "2026-09-02T02:05:00+00:00",
        });
        if let Some(next) = next {
            body["pagination"] = json!({"next": next});
        }
        body.to_string()
    }

    /// Mounts three pages: the first without a cursor, then `c1` and `c2`.
    async fn mount_three_pages(server: &MockServer, third_next: Option<&str>) {
        Mock::given(method("GET"))
            .and(path("/alerts"))
            .and(query_param_is_missing("cursor"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                page(
                    &["A"],
                    Some(&format!("{}/alerts?limit=1&cursor=c1", server.uri())),
                ),
                "application/geo+json",
            ))
            .mount(server)
            .await;
        Mock::given(method("GET"))
            .and(path("/alerts"))
            .and(query_param("cursor", "c1"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                page(
                    &["B", "C"],
                    Some("https://api.weather.gov/alerts?limit=1&cursor=c2%3D%3D"),
                ),
                "application/geo+json",
            ))
            .mount(server)
            .await;
        Mock::given(method("GET"))
            .and(path("/alerts"))
            .and(query_param("cursor", "c2=="))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(page(&["D"], third_next), "application/geo+json"),
            )
            .mount(server)
            .await;
    }

    fn events(collection: &FeatureCollection<Alert>) -> Vec<&str> {
        collection
            .iter()
            .map(|alert| alert.event.as_str())
            .collect()
    }

    #[tokio::test]
    async fn exhausts_three_pages_and_clears_pagination() {
        let server = MockServer::start().await;
        mount_three_pages(&server, None).await;

        let all = client_for(&server)
            .alerts()
            .list_all(
                &AlertsQuery {
                    limit: Some(1),
                    ..Default::default()
                },
                NonZeroU16::new(10).unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(events(&all), ["A", "B", "C", "D"]);
        assert_eq!(all.len(), 4);
        assert_eq!(all.title.as_deref(), Some("Alerts"));
        assert!(all.updated.is_some());
        assert_eq!(all.pagination, None);
        assert_eq!(all.next_cursor(), None);
        let queries: Vec<_> = server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .map(|request| request.url.query().unwrap().to_owned())
            .collect();
        assert_eq!(
            queries,
            ["limit=1", "limit=1&cursor=c1", "limit=1&cursor=c2%3D%3D"]
        );
    }

    #[tokio::test]
    async fn cap_stops_early_and_keeps_the_last_next_link() {
        let server = MockServer::start().await;
        mount_three_pages(&server, None).await;

        let two = client_for(&server)
            .alerts()
            .list_all(&AlertsQuery::default(), NonZeroU16::new(2).unwrap())
            .await
            .unwrap();

        assert_eq!(events(&two), ["A", "B", "C"]);
        assert_eq!(
            two.pagination.as_ref().unwrap().next,
            "https://api.weather.gov/alerts?limit=1&cursor=c2%3D%3D"
        );
        assert_eq!(two.next_cursor().unwrap().as_str(), "c2==");
        assert_eq!(server.received_requests().await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn a_dangling_final_next_link_is_dropped_when_the_walk_ends_by_cursor() {
        // The third page links onward, but its cursor is not valid, so the
        // walk ends there and reports the collection as exhausted.
        let server = MockServer::start().await;
        mount_three_pages(
            &server,
            Some("https://api.weather.gov/alerts?cursor=bad%20one"),
        )
        .await;

        let all = client_for(&server)
            .alerts()
            .list_all(&AlertsQuery::default(), NonZeroU16::new(10).unwrap())
            .await
            .unwrap();

        assert_eq!(events(&all), ["A", "B", "C", "D"]);
        assert_eq!(all.pagination, None);
    }

    #[tokio::test]
    async fn a_single_page_cap_returns_the_first_page_untouched() {
        let server = MockServer::start().await;
        mount_three_pages(&server, None).await;

        let first = client_for(&server)
            .alerts()
            .list_all(&AlertsQuery::default(), NonZeroU16::MIN)
            .await
            .unwrap();

        assert_eq!(events(&first), ["A"]);
        assert_eq!(first.next_cursor().unwrap().as_str(), "c1");
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn an_error_on_a_later_page_propagates_without_a_partial_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts"))
            .and(query_param_is_missing("cursor"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                page(&["A"], Some("https://api.weather.gov/alerts?cursor=c1")),
                "application/geo+json",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/alerts"))
            .and(query_param("cursor", "c1"))
            .respond_with(ResponseTemplate::new(400).set_body_raw(
                r#"{"type":"https://api.weather.gov/problems/InvalidParameter","title":"Bad Request","status":400,"detail":"Invalid cursor","instance":"https://api.weather.gov/requests/abc","correlationId":"abc"}"#,
                "application/problem+json",
            ))
            .mount(&server)
            .await;

        let error = client_for(&server)
            .alerts()
            .list_all(&AlertsQuery::default(), NonZeroU16::new(5).unwrap())
            .await
            .unwrap_err();

        assert_eq!(error.status().map(|status| status.as_u16()), Some(400));
        assert_eq!(error.problem().unwrap().detail, "Invalid cursor");
    }

    #[tokio::test]
    async fn the_callers_cursor_seeds_the_first_page_and_is_sent_decoded() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/alerts"))
            .and(query_param("cursor", "start=="))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(page(&["Z"], None), "application/geo+json"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let all = client_for(&server)
            .alerts()
            .list_all(
                &AlertsQuery {
                    cursor: Some("start==".parse().unwrap()),
                    ..Default::default()
                },
                NonZeroU16::new(3).unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(events(&all), ["Z"]);
        let requests = server.received_requests().await.unwrap();
        assert_eq!(requests[0].url.query(), Some("cursor=start%3D%3D"));
    }

    #[tokio::test]
    async fn station_helpers_page_their_own_routes() {
        let server = MockServer::start().await;
        let stations_page = |ids: &[&str], next: Option<String>| {
            let features: Vec<_> = ids
                .iter()
                .map(|id| {
                    json!({
                        "id": format!("https://api.weather.gov/stations/{id}"),
                        "type": "Feature",
                        "geometry": {"type": "Point", "coordinates": [-111.97, 40.77]},
                        "properties": {"stationIdentifier": id}
                    })
                })
                .collect();
            let mut body = json!({"type": "FeatureCollection", "features": features});
            if let Some(next) = next {
                body["pagination"] = json!({"next": next});
            }
            body.to_string()
        };
        Mock::given(method("GET"))
            .and(path("/stations"))
            .and(query_param_is_missing("cursor"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                stations_page(
                    &["KSLC"],
                    Some("https://api.weather.gov/stations?limit=1&cursor=s2".to_owned()),
                ),
                "application/geo+json",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations"))
            .and(query_param("cursor", "s2"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(stations_page(&["KDEN"], None), "application/geo+json"),
            )
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KSLC/observations"))
            .and(query_param_is_missing("cursor"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                page(
                    &["obs1"],
                    Some("https://api.weather.gov/stations/KSLC/observations?cursor=o2"),
                ),
                "application/geo+json",
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/stations/KSLC/observations"))
            .and(query_param("cursor", "o2"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(page(&["obs2"], None), "application/geo+json"),
            )
            .mount(&server)
            .await;

        let client = client_for(&server);
        let stations = client
            .stations()
            .list_all(
                &StationsQuery {
                    limit: Some(1),
                    ..Default::default()
                },
                NonZeroU16::new(5).unwrap(),
            )
            .await
            .unwrap();
        let ids: Vec<_> = stations
            .iter()
            .map(|station| station.station_identifier.as_deref().unwrap())
            .collect();
        assert_eq!(ids, ["KSLC", "KDEN"]);
        assert_eq!(stations.pagination, None);

        let observations = client
            .stations()
            .observations_all(
                &"kslc".parse().unwrap(),
                &ObservationsQuery::default(),
                NonZeroU16::new(5).unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(observations.len(), 2);
        assert_eq!(observations.pagination, None);
    }
}
