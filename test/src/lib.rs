#[derive(Debug, Default, Clone, Copy)]
pub struct Coordinate(f64);

impl<T> From<T> for Coordinate
where
    T: Into<f64>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Point(Coordinate, Coordinate);

impl<T, U> From<(T, U)> for Point
where
    T: Into<f64>,
    U: Into<f64>,
{
    fn from((x, y): (T, U)) -> Self {
        Self(Coordinate::from(x), Coordinate::from(y))
    }
}

#[derive(Debug, Default, Clone)]
pub struct Grid(Vec<Point>);

impl Grid {
    pub fn push(&mut self, point: Point) {
        self.0.push(point)
    }
}
