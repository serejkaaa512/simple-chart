use std::error::Error;
use std::f64;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::iter::once;
use BitMap;
use FlatMapPairs;
use Line;
use axis;

pub type GraphResult = Result<(), Box<Error>>;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl<'a> From<&'a (f64, f64)> for Point {
    fn from(t: &'a (f64, f64)) -> Point {
        Point { x: t.0, y: t.1 }
    }
}



#[derive(PartialEq, Clone, Copy)]
pub struct DisplayPoint {
    pub x: usize,
    pub y: usize,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const POINTS_COLOR: Color = Color {
    r: 0x00,
    g: 0x00,
    b: 0xff,
    a: 0x00,
};

pub fn create<T, P>(iter: T, path: &str, width: usize, height: usize) -> GraphResult
    where T: Iterator<Item = P> + Clone,
          P: Into<Point>
{
    let (min_x, max_x, min_y, max_y) = calculate_max_min(iter.clone());
    let axis = axis::create_axis(max_x, min_x, max_y, min_y, width, height);
    let points = convert_to_display_points(iter, width, height, min_x, max_x, min_y, max_y);
    let line = convert_points_to_line(points);
    let line_picture = convert_display_points_to_array(Box::new(line.chain(axis)), width, height);
    let bmp = BitMap::new().add_picture(line_picture, width, height);
    let byte_array = bmp.to_vec();
    try!(save_file_on_disc(byte_array, Path::new(&*path)));
    Ok(())
}

fn convert_points_to_line<'a>(points: Box<Iterator<Item = DisplayPoint> + 'a>)
                              -> Box<Iterator<Item = DisplayPoint> + 'a> {

    let it1 = FlatMapPairs::new(points,
                                |a: DisplayPoint, b: DisplayPoint| once(a).chain(Line::new(a, b)));
    Box::new(it1)
}

fn convert_display_points_to_array<'a>(points: Box<Iterator<Item = DisplayPoint> + 'a>,
                                       width: usize,
                                       height: usize)
                                       -> Vec<u8> {
    let mut pixs = vec![0xFFu8; width * height * 4 ];
    for p in points {
        let i = (p.y * width + p.x) * 4;
        pixs[i + 0] = POINTS_COLOR.b;
        pixs[i + 1] = POINTS_COLOR.g;
        pixs[i + 2] = POINTS_COLOR.r;
        pixs[i + 3] = POINTS_COLOR.a;
    }
    pixs
}

fn save_file_on_disc<'a>(bmp: Vec<u8>, path: &Path) -> GraphResult {
    let mut file = try!(File::create(&path));
    try!(file.write_all(&bmp));
    Ok(())
}




fn convert_to_display_points<'b, T, P>(iter: T,
                                       width: usize,
                                       height: usize,
                                       min_x: f64,
                                       max_x: f64,
                                       min_y: f64,
                                       max_y: f64)
                                       -> Box<Iterator<Item = DisplayPoint> + 'b>
    where T: 'b + Iterator<Item = P> + Clone,
          P: Into<Point>
{
    let resolution_x: f64 = (max_x - min_x) / (width as f64);
    let resolution_y: f64 = (max_y - min_y) / (height as f64);

    Box::new(iter.map(move |p| {
        let p = p.into();
        let mut id_x = ((p.x - min_x) / resolution_x).floor() as usize;
        let mut id_y = ((p.y - min_y) / resolution_y).floor() as usize;
        if id_x == width {
            id_x -= 1;
        }
        if id_y == height {
            id_y -= 1;
        }
        DisplayPoint { x: id_x, y: id_y }
    }))

}

fn calculate_max_min<'b, T, P>(iter: T) -> (f64, f64, f64, f64)
    where T: 'b + Iterator<Item = P>,
          P: Into<Point>
{
    let (mut min_x, mut max_x) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);

    for p in iter {
        let p = p.into();
        if p.x > max_x {
            max_x = p.x;
        }
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

#[test]
fn it_works() {
    let p = vec![(1f64, 1f64), (2f64, 2f64), (3f64, 3f64)];
    let _ = create(p.iter(), "/example/graph.bmp", 740, 480);
}


#[test]
fn can_create_array() {
    let p = vec![(1f64, 1f64), (2f64, 2f64), (3f64, 3f64)];
    let display_points = convert_to_display_points(p.iter(), 9, 9);
    for p in display_points {
        println!("x: {}, y: {}", p.x, p.y);
    }
}
