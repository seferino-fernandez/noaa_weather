#![doc = "[`alerts`]: crate::alerts"]
#![doc = "[`aviation`]: crate::aviation"]
#![doc = "[`glossary`]: crate::glossary"]
#![doc = "[`gridpoints`]: crate::gridpoints"]
#![doc = "[`offices`]: crate::offices"]
#![doc = "[`points`]: crate::points"]
#![doc = "[`products`]: crate::products"]
#![doc = "[`radar`]: crate::radar"]
#![doc = "[`radio`]: crate::radio"]
#![doc = "[`stations`]: crate::stations"]
#![doc = "[`zones`]: crate::zones"]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod macros;

pub mod alerts;
pub mod aviation;
pub mod client;
pub mod geo;
pub mod glossary;
pub mod gridpoints;
pub mod ids;
pub mod offices;
pub mod points;
pub mod prelude;
pub mod products;
pub mod radar;
pub mod radio;
pub mod stations;
pub mod time;
pub mod units;
pub mod zones;

pub use alerts::Alerts;
pub use aviation::Aviation;
pub use client::{
    BinaryPayload, BuildError, Client, ClientBuilder, Error, ProtocolError, RedirectReason,
    ResponseContent, RetryPolicy,
};
pub use geo::{
    AreaCode, Coordinates, Feature, FeatureCollection, Geometry, LandRegionCode, MarineAreaCode,
    MarineRegionCode, Pagination, Position, RegionCode, StateTerritoryCode,
};
pub use glossary::Glossary;
pub use gridpoints::Gridpoints;
pub use ids::{
    AlertId, AtsuId, CallSign, Cursor, CwsuId, GridpointId, InvalidValue,
    NwsCenterWeatherServiceUnitId, NwsForecastOfficeId, NwsNationalHqid, NwsOfficeId,
    NwsRegionalHqid, OfficeId, ParseNwsOfficeIdError, ProductId, ProductTypeCode, RadarStationId,
    StationId, ValueKind, ZoneId,
};
pub use offices::Offices;
pub use points::Points;
pub use products::Products;
pub use radar::Radar;
pub use radio::Radio;
pub use stations::Stations;
pub use time::{Interval, OffsetDateTime};
pub use units::{NwsUnitCode, Quantity, Unit, WmoUnitCode};
pub use zones::Zones;
