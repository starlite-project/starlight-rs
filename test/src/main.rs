use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Coordinate(f64);

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct Point(Coordinate, Coordinate);

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Grid(Vec<Point>);

impl Default for Grid {
    fn default() -> Self {
        let mut vec = Vec::with_capacity(10000);
        for i in 0..10000 {
            let coord = Coordinate(i.into());
            let point = Point(coord, coord);
            vec.push(point);
        }

        Self(vec)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}..{}", self.0.first().unwrap(), self.0.last().unwrap())
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let grid = Grid::default();

    let bincode_grid = bincode::serialize(&grid).unwrap();
    let cbor_grid = serde_cbor::to_vec(&grid).unwrap();

    let bincode_deserialized: Grid = bincode::deserialize(&bincode_grid[..]).unwrap();
    let cbor_deserialized: Grid = serde_cbor::from_slice(&cbor_grid[..]).unwrap();

    dbg!(bincode_deserialized == cbor_deserialized);

    Ok(())
}
