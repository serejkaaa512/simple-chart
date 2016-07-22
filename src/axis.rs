use std::f64;

const W_POINT: u8 = 1;      //value point separator width
const W_BORDER: u8 = 1;     //space around graph width
const SPACE_BETWEEN_NUMBERS: u8 = 1;     //space between numbers in pixels
const W_ARROW: u8 = 4;      //width of arrow
const W_NUMBER: u8 = 4;     //number width in pixels
const H_NUMBER: u8 = 5;     //number height in pixels
const MAX_INTERVALS: u8 = 10;   // maximum intervals count

#[allow(dead_code, unused_variables)]
pub fn create_graph_with_axis(x_max: f64,
                              x_min: f64,
                              y_max: f64,
                              y_min: f64,
                              width: usize,
                              height: usize)
                              -> Vec<u8> {
    let mut pixs = vec![0xFFu8; width * height * 4 ];
    let (s_max, kzc) = determine_max_numbers_count(x_max, x_min);
    let k_i = calculate_intervals_count(width, s_max);
    // add_intervals_x
    add_axis_x(&pixs, width);
    pixs
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

fn calculate_intervals_count(w: usize, s_max: u8) -> u8 {
    let w_available = w - 2 * (W_BORDER as usize) - (H_NUMBER as usize) - (W_ARROW as usize);
    let k = (w_available /
             (((W_NUMBER as usize) + (SPACE_BETWEEN_NUMBERS as usize)) * (s_max as usize))) -
            1;
    if k > MAX_INTERVALS as usize {
        MAX_INTERVALS
    } else {
        k as u8
    }

}

fn add_axis_x<'a>(pixs: &'a Vec<u8>, width: usize) {}


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
    let width = 100;
    let s_max = 5;
    let k_i = calculate_intervals_count(width, s_max);
    assert_eq!(k_i, 2);
}

#[test]
fn calculate_intervals_count_test_more_10() {
    let width = 1000;
    let s_max = 5;
    let k_i = calculate_intervals_count(width, s_max);
    assert_eq!(k_i, 10);
}