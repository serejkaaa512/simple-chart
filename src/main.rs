#[macro_use]
extern crate simple_graph;
use simple_graph::graph;


fn main() {
    let mut p = vec![];
    for pp in 0..100 {
        p.push(graph::Point {
            x: pp as f64,
            y: pp as f64,
        });
    }
    let _ = graph::create(p.iter(), "graph.bmp", 740, 480);
}
