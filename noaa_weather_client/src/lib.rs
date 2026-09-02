#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

pub mod apis;
pub mod client;
pub mod geo;
pub mod ids;
pub mod models;
pub mod time;
pub mod utils;

pub use apis::{
    Alerts, Aviation, BinaryPayload, Error, Glossary, Gridpoints, Offices, Points, Products,
    ProtocolError, Radar, Radio, RedirectReason, ResponseContent, Stations, Zones,
};
pub use client::{BuildError, Client, ClientBuilder, RetryPolicy};
pub use geo::{Coordinates, Feature, FeatureCollection, Geometry, Pagination, Position};
pub use ids::{
    AlertId, AtsuId, CallSign, Cursor, CwsuId, GridpointId, InvalidValue, OfficeId, ProductId,
    ProductTypeCode, RadarStationId, StationId, ValueKind, ZoneId,
};
pub use time::Interval;
