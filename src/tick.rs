use DisplayPoint;

const W_NUMBER: usize = 4;     //number width in pixels
const H_NUMBER: usize = 5;     //number height in pixels
const SPACE_BETWEEN_NUMBERS: usize = 1;     //space between numbers in pixels
const BORDER: usize = 1;     //space around graph width


pub fn create_tick_with_label<'a>(shift: usize,
                                  value: &'a str,
                                  inverse: bool)
                                  -> Box<Iterator<Item = DisplayPoint> + 'a> {

    Box::new(create_mark(shift).chain(create_label(shift - W_NUMBER, value, inverse)))
}

pub fn create_mark<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new((0..2).map(move |i| {
        DisplayPoint {
            x: shift,
            y: 2 * BORDER + H_NUMBER + i,
        }
    }))
}

pub fn create_label<'a>(shift: usize,
                        value: &'a str,
                        inverse: bool)
                        -> Box<Iterator<Item = DisplayPoint> + 'a> {
    let mut char_position = shift;
    if inverse {
        Box::new(value.chars().flat_map(move |char| {
            let it = get_char_picture(char, char_position).map(|p| {
                DisplayPoint {
                    x: p.x,
                    y: H_NUMBER + BORDER - p.y,
                }
            });
            char_position += W_NUMBER + SPACE_BETWEEN_NUMBERS;
            it
        }))
    } else {
        Box::new(value.chars().flat_map(move |char| {
            let it = get_char_picture(char, char_position);
            char_position += W_NUMBER + SPACE_BETWEEN_NUMBERS;
            it
        }))
    }
}




fn get_char_picture<'a>(char: char, shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    match char {
        '1' => get_picture_of_1(shift),
        '2' => get_picture_of_2(shift),
        '3' => get_picture_of_3(shift),
        '4' => get_picture_of_4(shift),
        '5' => get_picture_of_5(shift),
        '6' => get_picture_of_6(shift),
        '7' => get_picture_of_7(shift),
        '8' => get_picture_of_8(shift),
        '9' => get_picture_of_9(shift),
        '0' => get_picture_of_0(shift),
        '.' => get_picture_of_point(shift),
        '-' => get_picture_of_minus(shift),
        _ => Box::new(vec![].into_iter()),
    }
}



fn get_picture_of_1<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 4), (2, 4), (0, 3), (2, 3), (2, 2), (0, 1), (1, 1), (2, 1), (3, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_2<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(1, 5), (2, 5), (0, 4), (3, 4), (2, 3), (1, 2), (0, 1), (1, 1), (2, 1), (3, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_3<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(1, 5), (2, 5), (0, 4), (3, 4), (2, 3), (3, 2), (0, 2), (1, 1), (2, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_4<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(0, 5), (3, 5), (0, 4), (3, 4), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_5<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 5), (2, 5), (1, 5), (0, 5), (0, 4), (2, 3), (1, 3), (0, 3), (3, 2), (2, 1),
                  (1, 1), (0, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_6<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 5), (2, 5), (1, 5), (0, 4), (2, 3), (1, 3), (0, 3), (3, 2), (0, 2), (2, 1),
                  (1, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_7<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 5), (2, 5), (1, 5), (0, 5), (3, 4), (2, 3), (1, 2), (0, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_8<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 5), (3, 4), (0, 4), (2, 3), (1, 3), (3, 2), (0, 2), (2, 1), (1, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_9<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 5), (3, 4), (0, 4), (3, 3), (2, 3), (1, 3), (3, 2), (2, 1), (1, 1),
                  (0, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_0<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 5), (1, 5), (3, 4), (0, 4), (3, 3), (0, 3), (3, 2), (0, 2), (2, 1), (1, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_minus<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(3, 3), (2, 3), (1, 3), (0, 3)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}

fn get_picture_of_point<'a>(shift: usize) -> Box<Iterator<Item = DisplayPoint> + 'a> {
    Box::new(vec![(2, 1), (1, 1)]
        .into_iter()
        .map(move |(x, y)| {
            DisplayPoint {
                x: shift + x,
                y: y,
            }
        }))
}
