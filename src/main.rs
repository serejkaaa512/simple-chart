#[macro_use]
extern crate simple_graph;
use simple_graph::{Chart, Serie, Axis};
use std::io::prelude::*;
use std::fs::File;


fn main() {

    let mut chart = Chart::new(740, 480, "#000000", "#ffffff")
        .unwrap()
        .add_axis_x(Axis::new(-2.0, 2.0, 7, 2))
        .add_axis_y(Axis::new(-2.0, 2.0, 7, 2));

    let v1: Vec<_> = formula!(y(x) = x.sin(), x = [-3.14, 3.14; 0.1]).collect();
    let v2: Vec<_> = formula!(y(x) = x.cos(), x = [-3.14, 3.14; 0.1]).collect();
    let serie1 = Serie::new(v1.into_iter(), "#ff0000").unwrap();
    let serie2 = Serie::new(v2.into_iter(), "#00ff00").unwrap();
    let series = vec![serie1, serie2];
    let bmp = chart.draw(series.into_iter());

    let mut file = File::create("graph.bmp").unwrap();
    file.write_all(&bmp).unwrap();
}
