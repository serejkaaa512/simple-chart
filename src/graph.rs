use std::f64;

use BitMap;
use line;
use Axis;

const W_ARROW: usize = 4;      //width of arrow
const W_NUMBER: usize = 4;     //number width in pixel
const H_NUMBER: usize = 5;     //number height in pixels
const W_BORDER: usize = 1;     //space around graph width
const H_ARROW_HALF: usize = 3;

const LEFT_SHIFT: usize = W_BORDER + W_NUMBER + H_NUMBER;
const RIGHT_SHIFT: usize = W_ARROW;


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


pub trait InPoint: Into<Point> + PartialEq {}
impl<T: Into<Point> + PartialEq> InPoint for T {}

pub trait IterInPoint<P: InPoint>: Iterator<Item = P> + Clone {}
impl<T, P> IterInPoint<P> for T
    where T: Iterator<Item = P> + Clone,
          P: InPoint
{
}

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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DisplayPoint {
    pub x: usize,
    pub y: usize,
}


#[derive(Debug, Clone)]
pub struct Serie<T: IterInPoint<P, Item = P>, P: InPoint> {
    pub iter: T,
    color: String,
    max_x: f64,
    max_y: f64,
    min_x: f64,
    min_y: f64,
}

impl<P: InPoint, T: IterInPoint<P>> Serie<T, P> {
    pub fn new(iter: T, color: String) -> Result<Self, GraphError> {

        if iter.clone().nth(1).is_none() {
            return Err(GraphError::NotEnoughPoints);
        }

        let first = iter.clone().nth(0).unwrap();
        if !iter.clone().skip(1).any(move |p| p != first) {
            return Err(GraphError::NonUniquePoints);
        }

        let (max_x, min_x, max_y, min_y) = Self::calculate_max_min(iter.clone());

        Ok(Serie {
            iter: iter,
            color: color,
            max_x: max_x,
            max_y: max_y,
            min_x: min_x,
            min_y: min_y,
        })
    }

    fn calculate_max_min(iter: T) -> (f64, f64, f64, f64) {
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
}


#[derive(Debug)]
pub struct Chart {
    width: usize,
    height: usize,
    background_color: u8,
    axis_color: u8,
    pixs: Vec<u8>,
    picture: BitMap,
    axis_x: Option<Axis>,
    axis_y: Option<Axis>,
}

impl Chart {
    pub fn new(width: usize,
               height: usize,
               background_color: &str,
               axis_color: &str,
               axis_x: Option<Axis>,
               axis_y: Option<Axis>)
               -> Result<Self, GraphError> {

        if width < (2 * H_NUMBER + 2 * W_NUMBER + W_ARROW + 2 * W_BORDER) ||
           height < (2 * H_NUMBER + 2 * W_NUMBER + W_ARROW + 2 * W_BORDER) {
            return Err(GraphError::NotEnoughSpace);
        };

        let mut picture = BitMap::new(width, height);

        let background_color_number = picture.add_color(background_color);

        let axis_color_number = picture.add_color(axis_color);

        let size = width * height;

        let pixs = vec![background_color_number;  size];


        let new_axis_x = if let Some(x) = axis_x {
            Some(Axis::set_axis_manual(x.min_value,
                                       x.max_value,
                                       x.interval_count,
                                       x.decimal_places,
                                       width))
        } else {
            None
        };

        let new_axis_y = if let Some(y) = axis_y {
            Some(Axis::set_axis_manual(y.min_value,
                                       y.max_value,
                                       y.interval_count,
                                       y.decimal_places,
                                       width)
                .rotate())
        } else {
            None
        };

        Ok(Chart {
            width: width,
            height: height,
            background_color: background_color_number,
            axis_color: axis_color_number,
            pixs: pixs,
            picture: picture,
            axis_x: new_axis_x,
            axis_y: new_axis_y,
        })
    }


    fn draw_serie<P: InPoint, T: IterInPoint<P>>(&mut self, serie: Serie<T, P>) {

        let func_points = {

            let max_width = self.width - RIGHT_SHIFT;

            let max_height = self.height - RIGHT_SHIFT;

            let function = self.serie_to_points(&serie);

            line::extrapolate(function)
                .filter(|p| {
                    p.x > LEFT_SHIFT && p.x < max_width && p.y > LEFT_SHIFT && p.y < max_height
                })
                .collect::<Vec<DisplayPoint>>()

        };

        let points_color_number = self.picture.add_color(&*serie.color);

        self.draw_pixels(func_points, points_color_number);
    }


    fn calc_axis<S, T, P>(&mut self, series: S)
        where S: Iterator<Item = Serie<T, P>>,
              T: IterInPoint<P>,
              P: InPoint
    {
        let (mut min_x, mut max_x) = (f64::INFINITY, f64::NEG_INFINITY);
        let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);

        for s in series {
            if s.max_x > max_x {
                max_x = s.max_x;
            }
            if s.min_x < min_x {
                min_x = s.min_x;
            }

            if s.max_y > max_y {
                max_y = s.max_y;
            }
            if s.min_y < min_y {
                min_y = s.min_y;
            }
        }

        if self.axis_x.is_none() {
            self.axis_x = Some(Axis::set_axis_auto(max_x, min_x, self.width));
        }

        if self.axis_y.is_none() {
            self.axis_y = Some(Axis::set_axis_auto(max_y, min_y, self.height).rotate());
        }
    }

    pub fn draw<S, T, P>(&mut self, series: S) -> Vec<u8>
        where S: Iterator<Item = Serie<T, P>> + Clone,
              T: IterInPoint<P>,
              P: InPoint
    {

        if self.axis_x.is_none() || self.axis_y.is_none() {
            self.calc_axis(series.clone());
        }

        self.draw_axis();

        for serie in series {
            self.draw_serie(serie);
        }

        self.picture.add_pixels(&self.pixs);

        self.picture.to_vec()
    }

    fn draw_axis(&mut self) {

        let axis_x = self.axis_x.clone().unwrap();

        let axis_y = self.axis_y.clone().unwrap();

        let minor_net = self.get_minor_net(&axis_x, &axis_y);

        let axis_color = self.axis_color;

        self.draw_pixels(axis_x.create_points(), axis_color);

        self.draw_pixels(axis_y.create_points(), axis_color);

        self.draw_pixels(minor_net, axis_color);
    }

    fn get_minor_net(&self, axis_x: &Axis, axis_y: &Axis) -> Vec<DisplayPoint> {
        let mut v: Vec<DisplayPoint> = vec![];
        for i in 0..axis_x.interval_count {
            let shift = LEFT_SHIFT + ((axis_x.scale_interval_pix * (i as f64)).round() as usize);
            for j in LEFT_SHIFT..(self.height - H_ARROW_HALF) {
                if j % 2 != 0 {
                    v.push(DisplayPoint { x: shift, y: j });
                }
            }
        }

        for i in 0..axis_y.interval_count {
            let shift = LEFT_SHIFT + ((axis_y.scale_interval_pix * (i as f64)).round() as usize);
            for j in LEFT_SHIFT..(self.width - H_ARROW_HALF) {
                if j % 2 != 0 {
                    v.push(DisplayPoint { x: j, y: shift });
                }
            }
        }
        v
    }

    fn serie_to_points<'b, P: InPoint, T: IterInPoint<P>>
        (&'b mut self,
         serie: &'b Serie<T, P>)
         -> Box<Iterator<Item = DisplayPoint> + 'b> {

        let width_available = self.width - LEFT_SHIFT - RIGHT_SHIFT;

        let height_available = self.height - LEFT_SHIFT - RIGHT_SHIFT;

        let axis_x = self.axis_x.clone().unwrap();

        let axis_y = self.axis_y.clone().unwrap();

        let resolution_x: f64 = (axis_x.max_value - axis_x.min_value) / (width_available as f64);
        let resolution_y: f64 = (axis_y.max_value - axis_y.min_value) / (height_available as f64);

        let serie_iter = serie.iter.clone();

        Box::new(serie_iter.map(move |p| {
            let p = p.into();
            let id_x = ((p.x - axis_x.min_value) / resolution_x).round();
            let id_y = ((p.y - axis_y.min_value) / resolution_y).round();

            let id_x = if id_x < 0f64 {
                0 as usize
            } else if id_x > (width_available as f64) {
                width_available as usize
            } else {
                id_x as usize
            };

            let id_y = if id_y < 0f64 {
                0 as usize
            } else if id_y > (height_available as f64) {
                height_available as usize
            } else {
                id_y as usize
            };

            DisplayPoint {
                x: (id_x + LEFT_SHIFT),
                y: (id_y + LEFT_SHIFT),
            }
        }))

    }


    fn draw_pixels(&mut self, points: Vec<DisplayPoint>, color: u8) {
        for p in points {
            let i = p.y * self.width + p.x;
            self.pixs[i] = color;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use Axis;

    #[test]
    fn not_enough_space_test() {
        let result = Chart::new(10, 15, "#ffffff", "#000000", None, None);
        assert_eq!(result.unwrap_err().to_string(),
                   "There are not enough width and height to form graph with axis.");
    }

    #[test]
    fn not_enough_points_test() {
        let v: Vec<(f64, f64)> = vec![];
        let result = Serie::new(v.into_iter(), "#0000ff".to_string());
        assert_eq!(result.unwrap_err().to_string(),
                   "There are not enough points to display on graph.");
    }

    #[test]
    fn one_point_test() {
        let p = vec![(1f64, 1f64)];
        let result = Serie::new(p.into_iter(), "#0000ff".to_string());
        assert_eq!(result.unwrap_err().to_string(),
                   "There are not enough points to display on graph.");
    }

    #[test]
    fn two_identical_point_test() {
        let p = vec![(1f64, 1f64), (1f64, 1f64)];
        let result = Serie::new(p.into_iter(), "#0000ff".to_string());
        assert_eq!(result.unwrap_err().to_string(),
                   "There are only one unique point. Can't construct line.");
    }

    #[test]
    fn can_draw_array() {
        let p = vec![(1f64, 1f64), (2f64, 2f64), (3f64, 3f64)];
        let serie = Serie::new(p.into_iter(), "#0000ff".to_string()).unwrap();
        let mut chart = Chart::new(100, 100, "#ffffff", "#000000", None, None).unwrap();
        let series = vec![serie];
        let bmp = chart.draw(series.into_iter());
        for p in bmp {
            println!("{}", p);
        }
    }

    #[test]
    fn can_draw_axis_manual() {
        let p = vec![(1f64, 1f64), (2f64, 2f64), (3f64, 3f64)];
        let serie = Serie::new(p.into_iter(), "#0000ff".to_string()).unwrap();
        let axis_x = Some(Axis::create(0f64, 2f64, 7, 2));
        let mut chart = Chart::new(100, 100, "#ffffff", "#000000", axis_x, None).unwrap();
        let series = vec![serie];
        let _ = chart.draw(series.into_iter());
    }

    #[bench]
    fn create_graph_2_points(b: &mut Bencher) {
        b.iter(|| {
            let p = vec![(1f64, 1f64), (2f64, 2f64), (3f64, 3f64)];
            let serie = Serie::new(p.into_iter(), "#0000ff".to_string()).unwrap();
            let mut chart = Chart::new(740, 480, "#ffffff", "#000000", None, None).unwrap();
            let series = vec![serie];
            let _ = chart.draw(series.into_iter());
        })
    }

    #[bench]
    fn create_graph_1000_points(b: &mut Bencher) {
        b.iter(|| {
            let p: Vec<_> = formula!(y(x) = {x*x}, x = [0, 1000; 1]).collect();
            let serie = Serie::new(p.into_iter(), "#0000ff".to_string()).unwrap();
            let mut chart = Chart::new(740, 480, "#ffffff", "#000000", None, None).unwrap();
            let series = vec![serie];
            let _ = chart.draw(series.into_iter());
        })
    }

    #[bench]
    #[ignore]
    fn create_graph_1000000_points(b: &mut Bencher) {
        b.iter(|| {
            let p: Vec<_> = formula!(y(x) = {x*x}, x = [0, 1000; 0.001]).collect();
            let serie = Serie::new(p.into_iter(), "#0000ff".to_string()).unwrap();
            let mut chart = Chart::new(740, 480, "#ffffff", "#000000", None, None).unwrap();
            let series = vec![serie];
            let _ = chart.draw(series.into_iter());
        })
    }
}
