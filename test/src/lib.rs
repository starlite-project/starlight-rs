#[derive(Debug, Default, Clone, Copy)]
pub struct Coordinate(f64);

#[derive(Debug, Default, Clone, Copy)]
pub struct Point(Coordinate, Coordinate);

#[derive(Debug, Default, Clone)]
pub struct Grid(Vec<Point>);

impl Grid {
    pub fn push(&mut self, point: Point) {
        self.0.push(point)
    }
}
