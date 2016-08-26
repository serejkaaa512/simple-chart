use std::f64;
use DisplayPoint;
use tick;

const W_POINT: u8 = 1;      //value point separator width
const W_BORDER: usize = 1;     //space around graph width
const W_DECIMAL_SEPARATOR: usize = 1;     //space between numbers in pixels
const W_ARROW: usize = 4;      //width of arrow
const W_NUMBER: usize = 4;     //number width in pixel
const H_NUMBER: usize = 5;     //number height in pixels
const MAX_INTERVALS: u8 = 10;   // maximum intervals count
const START_SHIFT: usize = W_BORDER + H_NUMBER + W_NUMBER;
const DEFAULT_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct Axis {
    pub min_value: f64,
    pub max_value: f64,
    pub interval_count: u8,
    pub scale_interval_pix: f64,
    scale_interval_value: f64,
    pub decimal_places: u8,
    size: usize,
    rotated: bool,
}


impl Axis {
    
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

    pub fn set_axis_auto(max: f64, min: f64, total_size: usize) -> Axis {
        let available_size = total_size - 2 * W_BORDER - H_NUMBER - W_NUMBER - W_ARROW;
        let (s_max, decimal_places) = determine_max_numbers_count(max, min);
        let interval_count = calculate_intervals_count(available_size, s_max);
        let scale_interval_pix = (available_size as f64) / (interval_count as f64);
        let min_value = calc(f64::floor, min, decimal_places as i32);
        let max_value = calc(f64::ceil, max, decimal_places as i32);
        let scale_interval_value = (max_value - min_value) / (interval_count as f64);
        let scale_interval_value =
            calc(f64::ceil, scale_interval_value, decimal_places as i32);

        Axis {
            min_value: min_value,
            max_value: max_value,
            scale_interval_value: scale_interval_value,
            scale_interval_pix: scale_interval_pix,
            interval_count: interval_count,
            decimal_places: decimal_places,
            size: total_size,
            rotated: false,
        }
    }


    pub fn set_axis_manual(min_value: f64,
                           max_value: f64,
                           interval_count: u8,
                           decimal_places: u8,
                           size: usize)
                           -> Axis {
        let available_size = size - 2 * W_BORDER - H_NUMBER - W_NUMBER - W_ARROW;
        let scale_interval_pix = (available_size as f64) / (interval_count as f64);
        let min = calc(f64::floor, min_value, decimal_places as i32);
        let max = calc(f64::ceil, max_value, decimal_places as i32);
        let mut scale_interval_value = (max - min) / (interval_count as f64);
        scale_interval_value = calc(f64::ceil, scale_interval_value, decimal_places as i32);

        Axis {
            min_value: min,
            max_value: max,
            scale_interval_value: scale_interval_value,
            scale_interval_pix: scale_interval_pix,
            interval_count: interval_count,
            decimal_places: decimal_places,
            size: size,
            rotated: false,
        }
    }

    pub fn create(min_value: f64, max_value: f64, interval_count: u8, decimal_places: u8) -> Axis {
        Axis {
            min_value: min_value,
            max_value: max_value,
            scale_interval_value: 0f64,
            scale_interval_pix: 0f64,
            interval_count: interval_count,
            decimal_places: decimal_places,
            size: DEFAULT_SIZE,
            rotated: false,
        }
    }

    fn create_ticks_points(&self) -> Vec<DisplayPoint> {
        let mut v: Vec<DisplayPoint> = vec![];
        for i in 0..self.interval_count {
            let value = round((self.min_value + self.scale_interval_value * (i as f64)),
                              self.decimal_places as i32);
            let value_s = &*value.to_string();
            let shift = (self.scale_interval_pix * (i as f64)).round() as usize;
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


fn round(value: f64, decimal_places: i32) -> f64 {
    let k = 10f64.powi(decimal_places);
    (value * k).round() / k
}

fn calc<F>(f: F, value: f64, decimal_places: i32) -> f64
    where F: Fn(f64) -> f64
{
    let k = 10f64.powi(decimal_places);
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
        let mut decimal_places = 0;
        while d < 10.0 {
            d *= 10.0;
            decimal_places += 1;
        }
        (maxc + W_POINT + decimal_places, decimal_places)
    }
}

fn get_numbers_count(value: i64) -> u8 {
    value.to_string().len() as u8
}

fn calculate_intervals_count(available_size: usize, s_max: u8) -> u8 {
    let k = (available_size / ((W_NUMBER + W_DECIMAL_SEPARATOR) * (s_max as usize))) - 1;
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
        let (s_max, decimal_places) = axis::determine_max_numbers_count(max, min);
        assert_eq!(s_max, 2);
        assert_eq!(decimal_places, 0);
    }

    #[test]
    fn determine_max_numbers_count_test_diff_less_10() {
        let max = 1.54543;
        let min = 1.34;
        let (s_max, decimal_places) = axis::determine_max_numbers_count(max, min);
        assert_eq!(s_max, 4);
        assert_eq!(decimal_places, 2);
    }

    #[test]
    fn calculate_intervals_count_test_less_10() {
        let available_width = 100;
        let s_max = 5;
        let interval_count = axis::calculate_intervals_count(available_width, s_max);
        assert_eq!(interval_count, 3);
    }

    #[test]
    fn calculate_intervals_count_test_more_10() {
        let width = 1000;
        let s_max = 5;
        let interval_count = axis::calculate_intervals_count(width, s_max);
        assert_eq!(interval_count, 10);
    }


    #[bench]
    fn create_axis_bench(b: &mut Bencher) {
        b.iter(|| {
            let axis = Axis::set_axis_auto(100.0, 0.0, 1000);
            let _ = axis.create_points();
        })
    }
}
