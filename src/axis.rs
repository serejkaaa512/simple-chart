use std::f64;
use DisplayPoint;
use tick;

const W_POINT: u8 = 1;      //value point separator width
const W_BORDER: usize = 1;     //space around graph width
const SPACE_BETWEEN_NUMBERS: usize = 1;     //space between numbers in pixels
const W_ARROW: usize = 4;      //width of arrow
const W_NUMBER: usize = 4;     //number width in pixel
const H_NUMBER: usize = 5;     //number height in pixels
const MAX_INTERVALS: u8 = 10;   // maximum intervals count
const START_SHIFT: usize = W_BORDER + H_NUMBER + W_NUMBER;
const DEFAULT_MIN_VALUE: f64 = 0f64;
const DEFAULT_MAX_VALUE: f64 = 1f64;
const DEFAULT_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct Axis {
    pub min_value: f64,
    pub max_value: f64,
    pub k_i: u8,
    pub c_i: f64,
    c: f64,
    kzc: u8,
    size: usize,
    rotated: bool,
}


impl Axis {
    pub fn new() -> Self {
        Axis {
            min_value: DEFAULT_MIN_VALUE,
            max_value: DEFAULT_MAX_VALUE,
            c: 0f64,
            c_i: 0f64,
            k_i: 0u8,
            kzc: 0u8,
            size: DEFAULT_SIZE,
            rotated: false,
        }
    }
    pub fn rotate(self) -> Self {
        Axis { rotated: true, ..self }
    }

    pub fn create_points(&self) -> Vec<DisplayPoint> {
        let mut v: Vec<DisplayPoint> = vec![];
        let ticks = self.create_ticks_points();
        let line = self.calculate_axis_line();
        let arrow = self.calculate_axis_arrow();
        v.extend(ticks);
        v.extend(line);
        v.extend(arrow);
        if self.rotated {
            v.into_iter()
                .map(|p| DisplayPoint { x: p.y, y: p.x })
                .collect::<Vec<DisplayPoint>>()
        } else {
            v
        }
    }

    pub fn calculate_axis(max: f64, min: f64, total_size: usize) -> Axis {
        let mut axis = Self::new();
        let available_size = total_size - 2 * W_BORDER - H_NUMBER - W_NUMBER - W_ARROW;
        let (s_max, kzc) = determine_max_numbers_count(max, min);
        axis.kzc = kzc;
        axis.k_i = calculate_intervals_count(available_size, s_max);
        axis.c_i = (available_size as f64) / (axis.k_i as f64);
        axis.size = total_size;

        axis.min_value = calc(f64::floor, min, axis.kzc as i32);
        axis.max_value = calc(f64::ceil, max, axis.kzc as i32);
        let c = (axis.max_value - axis.min_value) / (axis.k_i as f64);
        axis.c = calc(f64::ceil, c, axis.kzc as i32);
        axis.max_value = axis.min_value + axis.c * (axis.k_i as f64);
        axis
    }

    fn create_ticks_points(&self) -> Vec<DisplayPoint> {
        let mut v: Vec<DisplayPoint> = vec![];
        for i in 0..self.k_i {
            let value = round((self.min_value + self.c * (i as f64)), self.kzc as i32);
            let value_s = &*value.to_string();
            let shift = (self.c_i * (i as f64)).round() as usize;
            v.extend(tick::create_tick_with_label(START_SHIFT + shift, value_s, self.rotated));
        }
        v
    }

    fn calculate_axis_line(&self) -> Vec<DisplayPoint> {
        let mut v = vec![];
        for x in START_SHIFT..self.size {
            v.push(DisplayPoint {
                x: x,
                y: START_SHIFT,
            });
        }
        v
    }


    fn calculate_axis_arrow(&self) -> Vec<DisplayPoint> {
        vec![(4, 13), (3, 12), (2, 11), (4, 7), (3, 8), (2, 9)]
            .into_iter()
            .map(move |(x, y)| {
                DisplayPoint {
                    x: self.size - x,
                    y: y,
                }
            })
            .collect()
    }
}


fn round(value: f64, kzc: i32) -> f64 {
    let k = 10f64.powi(kzc);
    (value * k).round() / k
}

fn calc<F>(f: F, value: f64, kzc: i32) -> f64
    where F: Fn(f64) -> f64
{
    let k = 10f64.powi(kzc);
    let new_value = f(value * k);
    new_value / k
}

fn determine_max_numbers_count(max: f64, min: f64) -> (u8, u8) {
    let mut d = max - min;
    let c_max = get_numbers_count(max as i64);
    let c_min = get_numbers_count(min as i64);

    let maxc = if c_max >= c_min { c_max } else { c_min };

    if d > 10.0 {
        (maxc, 0)
    } else {
        let mut kzc = 0;
        while d < 10.0 {
            d *= 10.0;
            kzc += 1;
        }
        (maxc + W_POINT + kzc, kzc)
    }
}

fn get_numbers_count(value: i64) -> u8 {
    value.to_string().len() as u8
}

fn calculate_intervals_count(available_size: usize, s_max: u8) -> u8 {
    let k = (available_size / ((W_NUMBER + SPACE_BETWEEN_NUMBERS) * (s_max as usize))) - 1;
    if k > MAX_INTERVALS as usize {
        MAX_INTERVALS
    } else {
        k as u8
    }

}




#[cfg(test)]
mod tests {
    use super::*;
    use axis;
    use test::Bencher;

    #[test]
    fn get_numbers_count_test() {
        let val = 34234;
        let c = axis::get_numbers_count(val);
        assert_eq!(c, 5);
    }

    #[test]
    fn determine_max_numbers_count_test_diff_more_10() {
        let max = 13.54543;
        let min = 1.34;
        let (s_max, kzc) = axis::determine_max_numbers_count(max, min);
        assert_eq!(s_max, 2);
        assert_eq!(kzc, 0);
    }

    #[test]
    fn determine_max_numbers_count_test_diff_less_10() {
        let max = 1.54543;
        let min = 1.34;
        let (s_max, kzc) = axis::determine_max_numbers_count(max, min);
        assert_eq!(s_max, 4);
        assert_eq!(kzc, 2);
    }

    #[test]
    fn calculate_intervals_count_test_less_10() {
        let available_width = 100;
        let s_max = 5;
        let k_i = axis::calculate_intervals_count(available_width, s_max);
        assert_eq!(k_i, 3);
    }

    #[test]
    fn calculate_intervals_count_test_more_10() {
        let width = 1000;
        let s_max = 5;
        let k_i = axis::calculate_intervals_count(width, s_max);
        assert_eq!(k_i, 10);
    }


    #[bench]
    fn create_axis_bench(b: &mut Bencher) {
        b.iter(|| {
            let axis = Axis::calculate_axis(100.0, 0.0, 1000);
            let _ = axis.create_points();
        })
    }
}
