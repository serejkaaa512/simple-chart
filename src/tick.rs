// #![allow(dead_code, unused_variables)]
use DisplayPoint;

const W_NUMBER: usize = 4;     //number width in pixels


pub fn create_tick_with_label<'a>(w: usize,
                                  value: &'a str)
                                  -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(create_mark(w).chain(create_label(w - W_NUMBER, value)))
}

pub fn create_mark<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new((0..2).map(move |i| DisplayPoint { x: w, y: 7 + i }))
}

pub fn create_label<'a>(w: usize, value: &'a str) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    let mut char_position = w;
    Box::new(value.chars().flat_map(move |char| {
        let it = get_char_picture(char, char_position);
        char_position += W_NUMBER + 1;
        it
    }))
}




fn get_char_picture<'a>(char: char, w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    match char {
        '1' => get_picture_of_1(w),
        '2' => get_picture_of_2(w),
        '3' => get_picture_of_3(w),
        '4' => get_picture_of_4(w),
        '5' => get_picture_of_5(w),
        '6' => get_picture_of_6(w),
        '7' => get_picture_of_7(w),
        '8' => get_picture_of_8(w),
        '9' => get_picture_of_9(w),
        '0' => get_picture_of_0(w),
        '.' => get_picture_of_point(w),
        '-' => get_picture_of_minus(w),
        _ => Box::new(vec![].into_iter()),
    }
}



fn get_picture_of_1<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 4), (2, 4), (0, 3), (2, 3), (2, 2), (0, 1), (1, 1), (2, 1), (3, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_2<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(1, 5), (2, 5), (0, 4), (3, 4), (2, 3), (1, 2), (0, 1), (1, 1), (2, 1), (3, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_3<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(1, 5), (2, 5), (0, 4), (3, 4), (2, 3), (3, 2), (0, 2), (1, 1), (2, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_4<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(0, 5), (3, 5), (0, 4), (3, 4), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_5<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 5), (2, 5), (1, 5), (0, 5), (0, 4), (2, 3), (1, 3), (0, 3), (3, 2), (2, 1),
                  (1, 1), (0, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_6<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 5), (2, 5), (1, 5), (0, 4), (2, 3), (1, 3), (0, 3), (3, 2), (0, 2), (2, 1),
                  (1, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_7<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 5), (2, 5), (1, 5), (0, 5), (3, 4), (2, 3), (1, 2), (0, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_8<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 5), (3, 4), (0, 4), (2, 3), (1, 3), (3, 2), (0, 2), (2, 1), (1, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_9<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 5), (3, 4), (0, 4), (3, 3), (2, 3), (1, 3), (3, 2), (2, 1), (1, 1),
                  (0, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_0<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 5), (3, 4), (0, 4), (3, 3), (0, 3), (3, 2), (0, 2), (2, 1), (1, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_minus<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 3), (2, 3), (1, 3), (0, 3)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}

fn get_picture_of_point<'a>(w: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 1), (1, 1)]
        .into_iter()
        .map(move |(x, y)| DisplayPoint { x: w + x, y: y }))
}
