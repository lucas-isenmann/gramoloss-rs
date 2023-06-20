#[derive(Clone,Copy,Debug)]
pub struct Coord{
    pub x: f64,
    pub y: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Coord {
        Coord { x, y}
    }

    pub fn distance_to(&self, other: &Coord) -> f64{
        f64::sqrt(f64::powf(other.x-self.x, 2.) + f64::powf(other.y -self.y, 2.))
    }

    pub fn sub(&self, other: &Coord) -> Coord{
        Coord{x: self.x- other.x, y: self.y -other.y}
    }
}