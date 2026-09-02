use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use super::{InvalidValue, OfficeId, ValueKind};
use crate::models::Point;

const SHAPE: &str = "must be OFFICE/x,y (for example TOP/31,80)";
const OFFICE: &str = "office code must be 3 to 4 ASCII letters or digits";
const RANGE: &str = "grid x and y must be whole numbers from 0 to 65535";
const NO_GRID: &str = "point response has no grid coordinates";

/// A forecast grid cell: an office code plus `x,y` grid coordinates.
///
/// Used by `/gridpoints/{wfo}/{x},{y}` and its forecast and station
/// sub-resources. The text form is exactly `OFFICE/x,y`, the same shape
/// NOAA uses in URLs, and a [`Point`] response converts into one with
/// `TryFrom`.
///
/// ```
/// use std::str::FromStr;
///
/// use noaa_weather_client::{GridpointId, OfficeId};
///
/// let grid = GridpointId::from_str("TOP/31,80")?;
/// assert_eq!(grid.office().as_str(), "TOP");
/// assert_eq!((grid.x(), grid.y()), (31, 80));
/// assert_eq!(grid.to_string(), "TOP/31,80");
///
/// let same = GridpointId::new("top".parse::<OfficeId>()?, 31, 80);
/// assert_eq!(grid, same);
/// # Ok::<(), noaa_weather_client::InvalidValue>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct GridpointId {
    office: OfficeId,
    x: u16,
    y: u16,
}

impl GridpointId {
    /// Creates a gridpoint from an office code and grid coordinates.
    #[must_use]
    pub const fn new(office: OfficeId, x: u16, y: u16) -> Self {
        Self { office, x, y }
    }

    /// Returns the forecast office that owns the grid.
    #[must_use]
    pub const fn office(&self) -> &OfficeId {
        &self.office
    }

    /// Returns the grid x coordinate.
    #[must_use]
    pub const fn x(&self) -> u16 {
        self.x
    }

    /// Returns the grid y coordinate.
    #[must_use]
    pub const fn y(&self) -> u16 {
        self.y
    }
}

impl fmt::Display for GridpointId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{},{}", self.office, self.x, self.y)
    }
}

impl FromStr for GridpointId {
    type Err = InvalidValue;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reject = |reason| InvalidValue::new(ValueKind::GridpointId, input, reason);
        let Some((office, grid)) = input.split_once('/') else {
            return Err(reject(SHAPE));
        };
        let Some((x, y)) = grid.split_once(',') else {
            return Err(reject(SHAPE));
        };
        if y.contains('/') {
            return Err(reject(SHAPE));
        }
        let office = office.parse().map_err(|_| reject(OFFICE))?;
        let x = parse_coordinate(x).ok_or_else(|| reject(RANGE))?;
        let y = parse_coordinate(y).ok_or_else(|| reject(RANGE))?;
        Ok(Self { office, x, y })
    }
}

/// Parses a grid coordinate as 1 to 5 plain ASCII digits with a value that
/// fits `u16`; `u16::from_str` would also accept a leading `+` and unbounded
/// leading zeros, which NOAA URLs never carry. Matches the published schema
/// pattern exactly.
fn parse_coordinate(text: &str) -> Option<u16> {
    if !(1..=5).contains(&text.len()) || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

impl From<GridpointId> for String {
    fn from(value: GridpointId) -> Self {
        value.to_string()
    }
}

impl_try_from_str!(GridpointId);
impl_string_schema!(
    GridpointId,
    "Forecast grid cell as OFFICE/x,y (for example TOP/31,80).",
    concat!(
        "^[A-Za-z0-9]{3,4}/",
        "(6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{2}|6[0-4][0-9]{3}|[0-5][0-9]{4}|[0-9]{1,4})",
        ",",
        "(6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{2}|6[0-4][0-9]{3}|[0-5][0-9]{4}|[0-9]{1,4})",
        "$"
    )
);

impl TryFrom<&Point> for GridpointId {
    type Error = InvalidValue;

    fn try_from(point: &Point) -> Result<Self, Self::Error> {
        let describe = || {
            let field = |value: Option<i32>| value.map_or("?".to_owned(), |v| v.to_string());
            format!(
                "{}/{},{}",
                point.grid_id.map_or("?".to_owned(), |id| id.to_string()),
                field(point.grid_x),
                field(point.grid_y)
            )
        };
        let (Some(office), Some(x), Some(y)) = (point.grid_id, point.grid_x, point.grid_y) else {
            return Err(InvalidValue::new(
                ValueKind::GridpointId,
                describe(),
                NO_GRID,
            ));
        };
        let (Ok(x), Ok(y)) = (u16::try_from(x), u16::try_from(y)) else {
            return Err(InvalidValue::new(ValueKind::GridpointId, describe(), RANGE));
        };
        Ok(Self::new(OfficeId::from(office), x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NwsForecastOfficeId;

    fn top() -> OfficeId {
        "TOP".parse().unwrap()
    }

    #[test]
    fn parses_office_and_coordinates() {
        let grid: GridpointId = "TOP/31,80".parse().unwrap();
        assert_eq!(grid, GridpointId::new(top(), 31, 80));
        assert_eq!(grid.office(), &top());
        assert_eq!(grid.x(), 31);
        assert_eq!(grid.y(), 80);
        assert_eq!(
            "top/0,65535".parse::<GridpointId>().unwrap().to_string(),
            "TOP/0,65535"
        );
        assert_eq!(
            "TOP/00031,00080"
                .parse::<GridpointId>()
                .unwrap()
                .to_string(),
            "TOP/31,80"
        );
        assert_eq!(GridpointId::try_from("TOP/31,80").unwrap(), grid);
        assert_eq!(
            GridpointId::try_from(String::from("TOP/31,80")).unwrap(),
            grid
        );
    }

    #[test]
    fn display_round_trips() {
        let grid = GridpointId::new(top(), 31, 80);
        assert_eq!(grid.to_string(), "TOP/31,80");
        assert_eq!(grid.to_string().parse::<GridpointId>().unwrap(), grid);
        assert_eq!(String::from(grid), "TOP/31,80");
    }

    #[test]
    fn rejects_each_malformed_part() {
        let cases = [
            ("", SHAPE),
            ("TOP", SHAPE),
            ("TOP/31", SHAPE),
            ("TOP/31,80/1", SHAPE),
            ("TOP31,80", SHAPE),
            ("T/31,80", OFFICE),
            ("T*P/31,80", OFFICE),
            ("TOP/-1,80", RANGE),
            ("TOP/31,65536", RANGE),
            ("TOP/+31,80", RANGE),
            ("TOP/31, 80", RANGE),
            ("TOP/,80", RANGE),
            ("TOP/31,", RANGE),
            ("TOP/3a,80", RANGE),
            ("TOP/000031,80", RANGE),
            ("TOP/31,100000", RANGE),
        ];
        for (input, reason) in cases {
            let error = input.parse::<GridpointId>().unwrap_err();
            assert_eq!(error.kind(), ValueKind::GridpointId, "{input:?}");
            assert_eq!(error.input(), input);
            assert_eq!(error.reason(), reason, "{input:?}");
        }
    }

    #[test]
    fn converts_from_a_point_with_grid_fields() {
        let point = Point {
            grid_id: Some(NwsForecastOfficeId::Top),
            grid_x: Some(31),
            grid_y: Some(80),
            ..Point::default()
        };
        let grid = GridpointId::try_from(&point).unwrap();
        assert_eq!(grid, GridpointId::new(top(), 31, 80));
    }

    #[test]
    fn point_without_grid_fields_is_invalid() {
        let point = Point::default();
        let error = GridpointId::try_from(&point).unwrap_err();
        assert_eq!(error.kind(), ValueKind::GridpointId);
        assert_eq!(error.reason(), NO_GRID);
        assert_eq!(error.input(), "?/?,?");

        let partial = Point {
            grid_id: Some(NwsForecastOfficeId::Top),
            grid_x: Some(31),
            ..Point::default()
        };
        let error = GridpointId::try_from(&partial).unwrap_err();
        assert_eq!(error.reason(), NO_GRID);
        assert_eq!(error.input(), "TOP/31,?");
    }

    #[test]
    fn point_with_out_of_range_grid_is_invalid() {
        let point = Point {
            grid_id: Some(NwsForecastOfficeId::Top),
            grid_x: Some(-1),
            grid_y: Some(80),
            ..Point::default()
        };
        let error = GridpointId::try_from(&point).unwrap_err();
        assert_eq!(error.reason(), RANGE);
        assert_eq!(error.input(), "TOP/-1,80");
    }

    #[test]
    fn serde_round_trip() {
        let grid = GridpointId::new(top(), 31, 80);
        assert_eq!(serde_json::to_string(&grid).unwrap(), "\"TOP/31,80\"");
        assert_eq!(
            serde_json::from_str::<GridpointId>("\"top/31,80\"").unwrap(),
            grid
        );
        assert!(serde_json::from_str::<GridpointId>("\"TOP/31\"").is_err());
    }
}
