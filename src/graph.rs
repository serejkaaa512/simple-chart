use std::error::Error;
use std::f64;


#[derive(Debug)]
pub enum GraphError {
    DataError,
    NegativeLogarithm,
    NegativeSquareRoot,
}

pub type GraphResult = Result<u64, GraphError>;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct DisplayPoint {
    pub x: usize,
    pub y: usize,
}


pub fn create<'a, T>(iter: T, path: &'a str, width: usize, height: usize) -> GraphResult
    where T: Iterator<Item = &'a Point> + Clone
{
    let graph = convert_to_display_points(iter, width, height);

    // let bmp = pack_into_bmp(graph);
    // save_file_on_disc(bmp, path)
    Ok(0)
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
        let mut id_y = ((max_y - p.y) / resolution_y).floor() as usize;
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
    create(p.iter(), "./example/graph.bmp", 740, 480);
}


#[test]
fn can_create_array() {
    let p = vec![Point { x: 1f64, y: 1f64 },
                 Point { x: 2f64, y: 2f64 },
                 Point { x: 3f64, y: 3f64 }];

    let width = 9;
    let height = 9;

    let display_points = convert_to_display_points(p.iter(), width, height).unwrap();

    for p in display_points {
        println!("x: {}, y: {}", p.x, p.y);
    }
}
