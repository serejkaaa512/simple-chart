use DisplayPoint;

const W_NUMBER: usize = 4;     //number width in pixels
const H_NUMBER: usize = 5;     //number height in pixels
const SPACE_BETWEEN_NUMBERS: usize = 1;     //space between numbers in pixels
const BORDER: usize = 1;     //space around graph width
const H_ARROW_HALF: usize = 3;      //half arrow height


pub fn create_tick_with_label(shift: usize,
                              value: &str,
                              inverse: bool,
                              opposite_size: usize)
                              -> Vec<DisplayPoint> {
    let mut v: Vec<DisplayPoint> = vec![];
    v.extend(create_mark(shift, opposite_size));
    v.extend(create_label(shift - W_NUMBER, value, inverse));
    v
}


pub fn create_mark(shift: usize, opposite_size: usize) -> Vec<DisplayPoint> {
    let mut v = vec![];
    let opposite_shift = BORDER + H_NUMBER + BORDER;

    for i in opposite_shift..(opposite_shift + H_ARROW_HALF) {
        v.push(DisplayPoint { x: shift, y: i })
    }

    for j in (opposite_shift + H_ARROW_HALF)..(opposite_size - opposite_shift - BORDER) {
        if j % 2 == 0 {
            continue;
        }
        v.push(DisplayPoint { x: shift, y: j })
    }
    v
}



pub fn create_label(shift: usize, value: &str, inverse: bool) -> Vec<DisplayPoint> {
    let mut char_position = shift;
    let mut v = vec![];
    if inverse {
        for char_ in value.chars() {
            let char_v = get_char_picture(char_).into_iter().map(move |p| {
                DisplayPoint {
                    x: char_position + p.0,
                    y: H_NUMBER + BORDER - p.1,
                }
            });
            char_position += W_NUMBER + SPACE_BETWEEN_NUMBERS;
            v.extend(char_v);
        }
    } else {
        for char_ in value.chars() {
            let char_v = get_char_picture(char_).into_iter().map(move |p| {
                DisplayPoint {
                    x: char_position + p.0,
                    y: p.1,
                }
            });
            char_position += W_NUMBER + SPACE_BETWEEN_NUMBERS;
            v.extend(char_v);
        }
    }
    v
}



fn get_char_picture(char: char) -> Vec<(usize, usize)> {
    match char {
        '1' => get_picture_of_1(),
        '2' => get_picture_of_2(),
        '3' => get_picture_of_3(),
        '4' => get_picture_of_4(),
        '5' => get_picture_of_5(),
        '6' => get_picture_of_6(),
        '7' => get_picture_of_7(),
        '8' => get_picture_of_8(),
        '9' => get_picture_of_9(),
        '0' => get_picture_of_0(),
        '.' => get_picture_of_point(),
        '-' => get_picture_of_minus(),
        _ => vec![],
    }
}


fn get_picture_of_1() -> Vec<(usize, usize)> {
    vec![(2, 5), (1, 4), (2, 4), (0, 3), (2, 3), (2, 2), (0, 1), (1, 1), (2, 1), (3, 1)]
}

fn get_picture_of_2() -> Vec<(usize, usize)> {
    vec![(1, 5), (2, 5), (0, 4), (3, 4), (2, 3), (1, 2), (0, 1), (1, 1), (2, 1), (3, 1)]
}

fn get_picture_of_3() -> Vec<(usize, usize)> {
    vec![(1, 5), (2, 5), (0, 4), (3, 4), (2, 3), (3, 2), (0, 2), (1, 1), (2, 1)]

}

fn get_picture_of_4() -> Vec<(usize, usize)> {
    vec![(0, 5), (3, 5), (0, 4), (3, 4), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1)]
}

fn get_picture_of_5() -> Vec<(usize, usize)> {
    vec![(3, 5), (2, 5), (1, 5), (0, 5), (0, 4), (2, 3), (1, 3), (0, 3), (3, 2), (2, 1), (1, 1),
         (0, 1)]
}

fn get_picture_of_6() -> Vec<(usize, usize)> {
    vec![(3, 5), (2, 5), (1, 5), (0, 4), (2, 3), (1, 3), (0, 3), (3, 2), (0, 2), (2, 1), (1, 1)]

}

fn get_picture_of_7() -> Vec<(usize, usize)> {
    vec![(3, 5), (2, 5), (1, 5), (0, 5), (3, 4), (2, 3), (1, 2), (0, 1)]

}

fn get_picture_of_8() -> Vec<(usize, usize)> {
    vec![(2, 5), (1, 5), (3, 4), (0, 4), (2, 3), (1, 3), (3, 2), (0, 2), (2, 1), (1, 1)]

}

fn get_picture_of_9() -> Vec<(usize, usize)> {
    vec![(2, 5), (1, 5), (3, 4), (0, 4), (3, 3), (2, 3), (1, 3), (3, 2), (2, 1), (1, 1), (0, 1)]

}

fn get_picture_of_0() -> Vec<(usize, usize)> {
    vec![(2, 5), (1, 5), (3, 4), (0, 4), (3, 3), (0, 3), (3, 2), (0, 2), (2, 1), (1, 1)]

}

fn get_picture_of_minus() -> Vec<(usize, usize)> {
    vec![(3, 3), (2, 3), (1, 3), (0, 3)]

}

fn get_picture_of_point() -> Vec<(usize, usize)> {
    vec![(2, 1), (1, 1)]

}
