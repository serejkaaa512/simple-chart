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
}
