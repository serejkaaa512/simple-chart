#[macro_use]
extern crate simple_graph;
use simple_graph::graph;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let points: Vec<_> = formula!(y(x): f64 = {x*x}, x = [-150f64, 150f64; 1f64]).collect();



    let v = vec![(1f64	,-74.343),
    (1.1	,-74.858),
    (1.2	,-75.261),
    (1.3	,-75.992),
    (1.4	,-74.396),
    (1.5	,-74.91),
    (1.6	,-75.641),
    (1.7	,-74.045),
    (1.8	,-74.56),
    (1.9	,-74.963),
(    2f64	,-75.694)];

    let bmp = graph::create(points.into_iter(), 740, 480).unwrap();

    let mut file = File::create("graph.bmp").unwrap();
    file.write_all(&bmp).unwrap();



}
