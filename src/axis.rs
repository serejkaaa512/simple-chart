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


pub fn create_axis(max: f64,
                   min: f64,
                   size: usize,
                   inverse: bool,
                   opposite_size: usize)
                   -> Vec<DisplayPoint> {

    let (c, c_i, start_shift, start_value, k_i, kzc) = calculate_axis_ticks_params(max, min, size);
    let mut v: Vec<DisplayPoint> = vec![];
    let ticks = create_ticks_points(c,
                                    c_i,
                                    start_shift,
                                    start_value,
                                    k_i,
                                    inverse,
                                    opposite_size,
                                    kzc);
    let line = calculate_axis_line(size);
    let arrow = calculate_axis_arrow(size);
    v.extend(ticks);
    v.extend(line);
    v.extend(arrow);
    v
}


fn calculate_axis_line(size: usize) -> Vec<DisplayPoint> {
    let mut v = vec![];
    let start_shift = W_BORDER + H_NUMBER + W_NUMBER;
    for x in start_shift..size {
        v.push(DisplayPoint {
            x: x,
            y: start_shift,
        });
    }
    v
}

fn calculate_axis_arrow(size: usize) -> Vec<DisplayPoint> {

    vec![(4, 13), (3, 12), (2, 11), (4, 7), (3, 8), (2, 9)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: size - x,
                y: y,
            }
        })
        .collect()
}

fn calculate_axis_ticks_params(max: f64,
                               min: f64,
                               total_size: usize)
                               -> (f64, f64, usize, f64, u8, u8) {
    let available_size = total_size - 2 * W_BORDER - H_NUMBER - W_NUMBER - W_ARROW;
    let (s_max, kzc) = determine_max_numbers_count(max, min);
    let k_i = calculate_intervals_count(available_size, s_max);
    let (c, c_i) = calculate_scale_interval(max, min, kzc, k_i, available_size);
    let start_shift = W_BORDER + H_NUMBER + W_NUMBER;
    let start_value = round(min, kzc as i32);
    (c, c_i, start_shift, start_value, k_i, kzc)
}


fn create_ticks_points(c: f64,
                       c_i: f64,
                       start_shift: usize,
                       start_value: f64,
                       k_i: u8,
                       inverse: bool,
                       opposite_size: usize,
                       kzc: u8)
                       -> Vec<DisplayPoint> {
    let mut v: Vec<DisplayPoint> = vec![];
    for i in 0..k_i {
        let value = round((start_value + c * (i as f64)), kzc as i32);
        let value_s = &*value.to_string();
        let shift = (c_i * (i as f64)).round() as usize;
        v.extend(tick::create_tick_with_label(start_shift + shift,
                                              value_s,
                                              inverse,
                                              opposite_size));
    }
    v
}


fn calculate_scale_interval(max: f64,
                            min: f64,
                            kzc: u8,
                            k_i: u8,
                            available_size: usize)
                            -> (f64, f64) {
    let c = (max - min) / (k_i as f64);
    let c_round = round(c, kzc as i32);
    let c_i = (available_size as f64) * c / (max - min);
    (c_round, c_i)
}

fn round(value: f64, kzc: i32) -> f64 {
    let k = 10f64.powi(kzc);
    (value * k).ceil() / k
}

fn determine_max_numbers_count(max: f64, min: f64) -> (u8, u8) {
    let mut d = max - min;
    let c_max = get_numbers_count(max as i64);
    let c_min = get_numbers_count(min as i64);

    let maxc = if c_max >= c_min {
        c_max
    } else {
        c_min
    };

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


#[test]
fn get_numbers_count_test() {
    let val = 34234;
    let c = get_numbers_count(val);
    assert_eq!(c, 5);
}

#[test]
fn determine_max_numbers_count_test_diff_more_10() {
    let max = 13.54543;
    let min = 1.34;
    let (s_max, kzc) = determine_max_numbers_count(max, min);
    assert_eq!(s_max, 2);
    assert_eq!(kzc, 0);
}

#[test]
fn determine_max_numbers_count_test_diff_less_10() {
    let max = 1.54543;
    let min = 1.34;
    let (s_max, kzc) = determine_max_numbers_count(max, min);
    assert_eq!(s_max, 4);
    assert_eq!(kzc, 2);
}

#[test]
fn calculate_intervals_count_test_less_10() {
    let available_width = 100;
    let s_max = 5;
    let k_i = calculate_intervals_count(available_width, s_max);
    assert_eq!(k_i, 3);
}

#[test]
fn calculate_intervals_count_test_more_10() {
    let width = 1000;
    let s_max = 5;
    let k_i = calculate_intervals_count(width, s_max);
    assert_eq!(k_i, 10);
}

#[test]
fn calculate_scale_interval_test() {
    let (c, c_i) = calculate_scale_interval(100.0, 0.0, 0, 2, 89);
    assert_eq!(c, 50.0);
    assert_eq!(c_i, 44);
}



#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn create_axis_bench(b: &mut Bencher) {
        b.iter(|| create_axis(100.0, 0.0, 1000, false, 1000))
    }
}