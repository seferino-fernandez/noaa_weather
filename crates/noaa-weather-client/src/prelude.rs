//! Convenient imports for building NOAA Weather API clients and requests.
//!
//! ```
//! use noaa_weather_client::prelude::*;
//!
//! let client = Client::builder("app/1.0 (contact@example.com)").build()?;
//! let point = Coordinates::new(39.7456, -97.0892)?;
//! let _request = (client.points(), point);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub use crate::alerts::{
    ActiveAlertsQuery, AlertCertainty, AlertMessageType, AlertSeverity, AlertStatus, AlertUrgency,
    AlertsQuery, RegionType,
};
pub use crate::aviation::SigmetsQuery;
pub use crate::gridpoints::{ForecastQuery, ForecastUnits, GridpointStationsQuery};
pub use crate::products::ProductsQuery;
pub use crate::radar::{
    RadarQueueQuery, RadarServerQuery, RadarServersQuery, RadarStationQuery, RadarStationsQuery,
    SpgdsQuery, WindProfilerQuery,
};
pub use crate::radio::TransmittersQuery;
pub use crate::stations::{LatestObservationQuery, ObservationsQuery, StationsQuery};
pub use crate::zones::{ZoneObservationsQuery, ZoneQuery, ZoneStationsQuery, ZoneType, ZonesQuery};

pub use crate::{
    AlertId, Alerts, AreaCode, AtsuId, Aviation, CallSign, Client, ClientBuilder, Coordinates,
    Cursor, CwsuId, Error, Feature, FeatureCollection, Geometry, Glossary, GridpointId, Gridpoints,
    Interval, InvalidValue, LandRegionCode, MarineAreaCode, MarineRegionCode, OfficeId, Offices,
    OffsetDateTime, Pagination, Points, Position, ProductId, ProductTypeCode, Products, Quantity,
    Radar, RadarStationId, Radio, RegionCode, RetryPolicy, StateTerritoryCode, StationId, Stations,
    Unit, ValueKind, ZoneId, Zones,
};
