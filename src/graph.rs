use std::error::Error;
use std::f64;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use BitMap;

pub type GraphResult = Result<(), Box<Error>>;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for Point{
    fn from(t: (f64, f64)) -> Self {
        Point { x: t.0, y: t.1 }
    }
}

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

pub fn create<'a, T>(iter: T, path: &'a str, width: usize, height: usize) -> GraphResult
    where T: Iterator<Item = &'a Point> + Clone
{
    let graph = convert_to_display_points(iter, width, height);
    let mut bmp = BitMap::new();
    let picture = convert_display_points_to_array(graph, width, height);
    bmp = bmp.add_picture(picture, width, height);
    let byte_array = bmp.to_vec();
    try!(save_file_on_disc(byte_array, Path::new(&*path)));
    Ok(())
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


fn convert_to_display_points<'b, 'a: 'b, T>(iter: T,
                                            width: usize,
                                            height: usize)
                                            -> Box<Iterator<Item = DisplayPoint> + 'b>
    where T: 'b + Iterator<Item = &'a Point> + Clone
{

    let (mut min_x, mut max_x) = (f64::INFINITY, f64::NEG_INFINITY);
    let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);

    for p in iter.clone() {
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

    let resolution_x: f64 = (max_x - min_x) / (width as f64);
    let resolution_y: f64 = (max_y - min_y) / (height as f64);

    Box::new(iter.map(move |p| {
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

#[test]
fn it_works() {
    let p = vec![Point { x: 1f64, y: 1f64 },
                 Point { x: 2f64, y: 2f64 },
                 Point { x: 3f64, y: 3f64 }];
    let _ = create(p.iter(), "/example/graph.bmp", 740, 480);
}


#[test]
fn can_create_array() {
    let p = vec![Point { x: 1f64, y: 1f64 },
                 Point { x: 2f64, y: 2f64 },
                 Point { x: 3f64, y: 3f64 }];

    let width = 9;
    let height = 9;

    let display_points = convert_to_display_points(p.iter(), width, height);

    for p in display_points {
        println!("x: {}, y: {}", p.x, p.y);
    }
}
