use std::f64;

use BitMap;
use line;
use axis;

const W_ARROW: usize = 4;      //width of arrow
const W_NUMBER: usize = 4;     //number width in pixel
const H_NUMBER: usize = 5;     //number height in pixels
const W_BORDER: usize = 1;     //space around graph width

const LEFT_SHIFT: usize = W_BORDER + W_NUMBER + H_NUMBER;
const RIGHT_SHIFT: usize = W_ARROW;

const BACKGROUND_COLOR: &'static str = "#ffffff";
const POINTS_COLOR: &'static str = "#0000ff";
const AXIS_COLOR: &'static str = "#000000";


quick_error! {
    #[derive(Debug)]
    pub enum GraphError {
        NotEnoughPoints {
            description("There are not enough points to display on graph.")
        }
        NotEnoughSpace {
            description("There are not enough width and height to form graph with axis.")
        }
        NonUniquePoints {
            description("There are only one unique point. Can't construct line.")
        }
    }
}



pub type GraphResult = Result<Vec<u8>, GraphError>;

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

impl From<(f64, f64)> for Point {
    fn from(t: (f64, f64)) -> Point {
        Point { x: t.0, y: t.1 }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct DisplayPoint {
    pub x: usize,
    pub y: usize,
}


pub fn create<T, P>(iter: T, width: usize, height: usize) -> GraphResult
    where T: Iterator<Item = P> + Clone,
          P: Into<Point> + PartialEq
{

    if width < (H_NUMBER + W_NUMBER + W_ARROW + 2 * W_BORDER) ||
       height < (H_NUMBER + W_NUMBER + W_ARROW + 2 * W_BORDER) {
        return Err(GraphError::NotEnoughSpace);
    }


    if iter.clone().nth(1).is_none() {
        return Err(GraphError::NotEnoughPoints);
    }

    let first = iter.clone().nth(0).unwrap();
    if !iter.clone().skip(1).any(move |p| p != first) {
        return Err(GraphError::NonUniquePoints);
    }

    let (max_x, min_x, max_y, min_y) = calculate_max_min(iter.clone());

    let (axis_x, min_x_axis_value, max_x_axis_value) =
        axis::create_axis(max_x, min_x, width, false, height);

    let (axis_y_converted, min_y_axis_value, max_y_axis_value) =
        axis::create_axis(max_y, min_y, height, true, width);

    let axis_y: Vec<DisplayPoint> = axis_y_converted.into_iter()
        .map(|p| DisplayPoint { x: p.y, y: p.x })
        .collect();


    let function = convert_to_display_points(iter,
                                             width,
                                             height,
                                             min_x_axis_value,
                                             max_x_axis_value,
                                             min_y_axis_value,
                                             max_y_axis_value);

    let line: Vec<DisplayPoint> = line::extrapolate(function).collect();

    let mut bmp = BitMap::new(width, height);

    let background_color_number = bmp.add_color(BACKGROUND_COLOR);

    let points_color_number = bmp.add_color(POINTS_COLOR);

    let axis_color_number = bmp.add_color(AXIS_COLOR);

    let size = width * height;

    let mut pixs = vec![background_color_number;  size];

    draw_pixels(&mut pixs, width, axis_x, axis_color_number);

    draw_pixels(&mut pixs, width, axis_y, axis_color_number);

    draw_pixels(&mut pixs, width, line, points_color_number);

    bmp.add_pixels(pixs);

    Ok(bmp.to_vec())
}


fn draw_pixels(pixs: &mut Vec<u8>, width: usize, points: Vec<DisplayPoint>, color: u8) {
    for p in points {
        let i = p.y * width + p.x;
        pixs[i] = color;
    }
}




fn convert_to_display_points<'b, T, P>(iter: T,
                                       width: usize,
                                       height: usize,
                                       min_x: f64,
                                       max_x: f64,
                                       min_y: f64,
                                       max_y: f64)
                                       -> Box<Iterator<Item = DisplayPoint> + 'b>
    where T: 'b + Iterator<Item = P>,
          P: Into<Point>
{
    let width_available = width - LEFT_SHIFT - RIGHT_SHIFT;
    let height_available = height - LEFT_SHIFT - RIGHT_SHIFT;

    let resolution_x: f64 = (max_x - min_x) / (width_available as f64);
    let resolution_y: f64 = (max_y - min_y) / (height_available as f64);

    Box::new(iter.map(move |p| {
        let p = p.into();
        let mut id_x = ((p.x - min_x) / resolution_x).round() as usize;
        let mut id_y = ((p.y - min_y) / resolution_y).round() as usize;
        if id_x == width {
            id_x -= 1;
        }
        if id_y == height {
            id_y -= 1;
        }
        DisplayPoint {
            x: (id_x + LEFT_SHIFT),
            y: (id_y + LEFT_SHIFT),
        }
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
    (max_x, min_x, max_y, min_y)
}

#[test]
fn not_enough_space_test() {
    let p = vec![(1f64, 1f64), (2f64, 2f64), (3f64, 3f64)];
    let result = create(p.iter(), 10, 15);
    assert_eq!(result.unwrap_err().to_string(),
               "There are not enough width and height to form graph with axis.");
}

#[test]
fn not_enough_points_test() {
    let p = vec![];
    let result = create(p.iter(), 100, 150);
    assert_eq!(result.unwrap_err().to_string(),
               "There are not enough points to display on graph.");
}

#[test]
fn one_point_test() {
    let p = vec![(1f64, 1f64)];
    let result = create(p.iter(), 100, 150);
    assert_eq!(result.unwrap_err().to_string(),
               "There are not enough points to display on graph.");
}

#[test]
fn two_identical_point_test() {
    let p = vec![(1f64, 1f64), (1f64, 1f64)];
    let result = create(p.iter(), 100, 150);
    assert_eq!(result.unwrap_err().to_string(),
               "There are only one unique point. Can't construct line.");
}

#[test]
fn can_create_array() {
    let p = vec![(1f64, 1f64), (2f64, 2f64), (3f64, 3f64)];
    let display_points = convert_to_display_points(p.iter(), 19, 19, 0.0, 10.0, 0.0, 100.0);
    for p in display_points {
        println!("x: {}, y: {}", p.x, p.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn create_graph_2_points(b: &mut Bencher) {
        b.iter(|| {
            let p = vec![(1f64, 1f64), (2f64, 2f64)];
            let _ = create(p.iter(), 740, 480);
        })
    }

    #[bench]
    fn create_graph_1000_points(b: &mut Bencher) {
        b.iter(|| {
            let points: Vec<_> = formula!(y(x): f64 = {x*x}, x = [0f64, 1000f64; 1f64]).collect();
            let _ = create(points.into_iter(), 740, 480);
        })
    }

    #[bench]
    #[ignore]
    fn create_graph_1000000_points(b: &mut Bencher) {
        b.iter(|| {
            let points: Vec<_> = formula!(y(x): f64 = {x*x}, x = [0f64, 1000f64; 0.001f64])
                .collect();
            let _ = create(points.into_iter(), 740, 480);
        })
    }
}
