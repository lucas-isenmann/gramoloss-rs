
#[derive(Clone,Copy,Debug)]
pub struct Link {
    pub start_index: usize,
    pub end_index: usize,
    pub weight: Option<f64>
}

impl Link {
    pub fn new(start_index: usize, end_index: usize, weight: Option<f64>) -> Link{
        Link {
            start_index,
            end_index,
            weight
        }
    }
}