#[macro_use]
extern crate simple_graph;
use simple_graph::Chart;
use simple_graph::Serie;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let mut chart = Chart::new(740, 480, "#000000", "#00ffff").unwrap();

    let v1: Vec<_> = formula!(y(x) = x.sin(), x = [-10, 10; 0.1]).collect();
    let v2: Vec<_> = formula!(y(x) = x.cos(), x = [-10, 10; 0.1]).collect();

    // let v2 = vec![(1f64, -74.756),
    //              (2f64, -75.271),
    //              (3f64, -75.674),
    //              (4f64, -74.405),
    //              (5f64, -74.809),
    //              (6f64, -75.324),
    //              (7f64, -75.727),
    //              (8f64, -74.458),
    //              (9f64, -74.862),
    //              (10f64, -75.592)];

    let serie1 = Serie::new(v1.into_iter(), "#ff0000".to_string()).unwrap();
    let serie2 = Serie::new(v2.into_iter(), "#00ff00".to_string()).unwrap();
    let series = vec![serie1, serie2];
    let bmp = chart.draw(series.into_iter());
    let mut file = File::create("graph.bmp").unwrap();
    file.write_all(&bmp).unwrap();

}
