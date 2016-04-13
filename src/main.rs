#[macro_use]
extern crate simple_graph;
use simple_graph::graph;


fn main() {
    let points: Vec<_> = formula!(y(x): f64 = {x/2f64}, x = [-10f64, 10f64; 1f64]).collect();
    let _ = graph::create(points.iter(), "graph.bmp", 740, 480);
}
