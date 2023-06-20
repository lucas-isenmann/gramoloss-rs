use rand::Rng;

use crate::coord::Coord;

#[derive(Clone,Copy,Debug)]
pub struct Vertex{
    pub pos: Coord
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            pos: Coord::new(0.,0.)
        }
    }

    pub fn from_coord(x: f64, y: f64) -> Vertex{
        Vertex {
            pos: Coord { x, y}

        }
    }

    /// Generate a vertex with random position between 0 and w.
    pub fn rand_range(w: f64) -> Vertex {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..w);
        let y = rng.gen_range(0.0..w);
        Vertex {
            pos: Coord::new(x,y)
        }
    }

    pub fn distance_to(&self, other: &Vertex) -> f64 {
        self.pos.distance_to(&other.pos)
    }
}