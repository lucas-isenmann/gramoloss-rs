use std::time::{Instant, Duration};

use crate::graph::{Graph, stretch_genetic_maximizer};

mod vertex;
mod coord;
mod link;
mod graph;
mod math_utils;

fn main() {

    stretch_genetic_maximizer();
    
    // let mut mean = Duration::from_secs(0);
    // for _ in 0..1 {
    //     let t = Instant::now();
    //     let mut g = Graph::rand_points(20,100.);
    //     g.reset_edges_delaunay_v3();
    //     let d = Instant::now() -t;
    //     mean += d;
    //     println!("{:?}", Instant::now()-t);
    //     g.to_svg("test.svg");
    // }
    // println!("mean={:?}", mean/10);


    println!("Hello, world!");
}
