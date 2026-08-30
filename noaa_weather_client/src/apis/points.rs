//! Geographic point metadata lookup.
//!
//! Covers the `/points/{latitude},{longitude}` endpoints. A point lookup
//! returns the forecast office, grid coordinates, and zone identifiers for
//! any lat/lon pair — the starting point for most forecast workflows.

use super::{Error, configuration, http};
use crate::models;

/// Returns metadata about a specific latitude/longitude point.
///
/// Corresponds to the `/points/{latitude},{longitude}` endpoint.
/// This metadata includes the responsible forecast office, grid coordinates, and links to
/// relevant forecast endpoints for the location.
///
/// # Parameters
///
/// * `configuration`: The API client configuration.
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
    configuration: &configuration::Configuration,
    latitude: f64,
    longitude: f64,
) -> Result<models::PointGeoJson, Error> {
    let uri_str = format!(
        "/points/{latitude},{longitude}",
        latitude = latitude,
        longitude = longitude
    );
    let req_builder = http::get(configuration, &uri_str);

    req_builder.json().await
}
