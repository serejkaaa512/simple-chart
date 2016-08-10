#[macro_use]
extern crate simple_graph;
use simple_graph::graph;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let points: Vec<_> = formula!(y(x): f64 = {x*x}, x = [-150f64, 150f64; 1f64]).collect();

    let bmp = graph::create(points.into_iter(), 740, 480).unwrap();

    let mut file = File::create("graph.bmp").unwrap();
    file.write_all(&bmp).unwrap();

    // let p = vec![(1f64, -74.343f64),
    //              (1.1f64, -74.858f64),
    //              (1.2f64, -75.261f64),
    //              (1.3f64, -75.992f64),
    //              (1.4f64, -74.396f64),
    //              (1.5f64, -74.91f64),
    //              (1.6f64, -75.641f64),
    //              (1.7f64, -74.045f64),
    //              (1.8f64, -74.56f64),
    //              (1.9f64, -74.963f64),
    //              (2f64, -75.694f64)];
}
