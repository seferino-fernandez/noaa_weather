//! Geographic point metadata lookup.
//!
//! Covers the `/points/{latitude},{longitude}` endpoints. A point lookup
//! returns the forecast office, grid coordinates, and zone identifiers for
//! any lat/lon pair — the starting point for most forecast workflows.

use std::fmt;

use super::Error;
use crate::client::{Client, http};
use crate::models;

struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{},{}", self.latitude, self.longitude)
    }
}

/// Returns metadata about a specific latitude/longitude point.
///
/// Corresponds to the `/points/{latitude},{longitude}` endpoint.
/// This metadata includes the responsible forecast office, grid coordinates, and links to
/// relevant forecast endpoints for the location.
///
/// # Parameters
///
/// * `client`: The API client.
/// * `latitude`: The latitude of the point (e.g., 39.7456).
/// * `longitude`: The longitude of the point (e.g., -97.0892).
///
/// # Returns
///
/// A `Result` containing a [`models::PointGeoJson`] on success, which includes the point metadata
/// in its `properties` field.
///
/// # Errors
///
/// Returns an [`Error`] if the request fails (e.g., invalid coordinates,
/// point outside CONUS) or the response cannot be parsed.
pub async fn get_point(
    client: &Client,
    latitude: f64,
    longitude: f64,
) -> Result<models::PointGeoJson, Error> {
    http::request(client, "/points")
        .path_segment(Coordinates {
            latitude,
            longitude,
        })
        .json(http::JsonMedia::GeoJson)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    use super::get_point;
    use crate::client::test_support::client_for;

    #[tokio::test]
    async fn typed_coordinates_are_one_geo_json_path_segment() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/points/39.7456,-97.0892"))
            .and(header("Accept", "application/geo+json"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"{"type":"Feature","geometry":null,"properties":{}}"#,
                "application/geo+json",
            ))
            .expect(1)
            .mount(&server)
            .await;
        let client = client_for(&server);

        get_point(&client, 39.7456, -97.0892).await.unwrap();
    }
}
