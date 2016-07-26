#![allow(dead_code, unused_variables)]

use std::f64;
use DisplayPoint;
use tick;

const W_POINT: u8 = 1;      //value point separator width
const W_BORDER: u8 = 1;     //space around graph width
const SPACE_BETWEEN_NUMBERS: u8 = 1;     //space between numbers in pixels
const W_ARROW: u8 = 4;      //width of arrow
const W_NUMBER: usize = 4;     //number width in pixel
const H_NUMBER: u8 = 5;     //number height in pixels
const MAX_INTERVALS: u8 = 10;   // maximum intervals count

struct Axis {
    max: f64,
    min: f64,
    kzc: u8,
    k_i: u8,
    c: f64,
    c_i: usize,
}

pub fn create_axis<'a>(x_max: f64,
                       x_min: f64,
                       y_max: f64,
                       y_min: f64,
                       width: usize,
                       height: usize)
                       -> Box<Iterator<Item = DisplayPoint> + 'a> {
    let w_available = width - 2 * (W_BORDER as usize) - (H_NUMBER as usize) - (W_ARROW as usize);
    let x_points = calculate_x_axis_points(x_max, x_min, w_available);
    x_points
}


fn calculate_x_axis_points<'a>(max: f64,
                               min: f64,
                               w_available: usize)
                               -> Box<Iterator<Item = DisplayPoint> + 'a> {

    let (s_max, kzc) = determine_max_numbers_count(max, min);
    let k_i = calculate_intervals_count(w_available, s_max);
    let (c, c_i) = calculate_scale_intervals(max, min, kzc, k_i, w_available);
    let mut v: Vec<DisplayPoint> = vec![];
    let mut x_coord = 10;
    for index in 0..k_i {
        x_coord = x_coord + (index as usize) * c_i;
        let value = round(min, kzc as i32) + (index as f64) * c;
        let value_s = &*value.to_string();
        v.extend(tick::create_tick_with_label(x_coord, value_s));
    }
    Box::new(v.into_iter())
}

fn calculate_scale_intervals(max: f64,
                             min: f64,
                             kzc: u8,
                             k_i: u8,
                             w_available: usize)
                             -> (f64, usize) {
    let c = (max - min) / (k_i as f64);
    let c_round = round(c, kzc as i32);
    let c_i = ((w_available as f64) * c / (max - min)) as usize;
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
            d = d * 10.0;
            kzc += 1;
        }
        (maxc + W_POINT + kzc, kzc)
    }
}

fn get_numbers_count(value: i64) -> u8 {
    value.to_string().len() as u8
}

fn calculate_intervals_count(w_available: usize, s_max: u8) -> u8 {
    let k = (w_available /
             (((W_NUMBER as usize) + (SPACE_BETWEEN_NUMBERS as usize)) * (s_max as usize))) -
            1;
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
    let (c, c_i) = calculate_scale_intervals(100.0, 0.0, 0, 2, 89);
    assert_eq!(c, 50.0);
    assert_eq!(c_i, 44);
}