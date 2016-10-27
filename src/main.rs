#[macro_use]
extern crate simple_chart;
use simple_chart::{Chart, Serie, Axis, Formula};
use std::io::prelude::*;
use std::fs::File;


fn main() {

    let mut chart = Chart::new(740, 740, "#000000", "#ffffff")
        .unwrap()
        .add_axis_x(Axis::new(-1.0, 1.0, 10, 2));

    let f1 = formula!(y(x) = (1f64 - x.powi(2)).abs().sqrt(), x = [-1f64, 1f64; 0.01]);
    let f2 = formula!(y(x) = -(1f64 - x.powi(2)).abs().sqrt(), x = [1f64, -1f64; 0.01]);
    let ff1 = f1.chain(f2);

    let f3 = formula!(y(x) = (0.64f64 - x.powi(2)).abs().sqrt(), x = [-0.8f64, 0.8f64; 0.01]);
    let f4 = formula!(y(x) = -(0.64f64 - x.powi(2)).abs().sqrt(), x = [0.8f64, -0.8f64; 0.01]);

    let ff2 = f3.chain(f4);

    let serie1 = Serie::new(ff1.into_iter(), "#ff0000").unwrap();
    let serie2 = Serie::new(ff2.into_iter(), "#00ff00").unwrap();
    let series = vec![serie1, serie2];
    let bmp = chart.draw(series.into_iter());

    let mut file = File::create("graph.bmp").unwrap();
    file.write_all(&bmp).unwrap();
}
