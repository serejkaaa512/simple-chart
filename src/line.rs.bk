use DisplayPoint;
use flatmappairs::FlatMapPairs;
use std::iter::once;

pub struct Line {
    first: DisplayPoint,
    last: DisplayPoint,
    cur: DisplayPoint,
    dx: i32,
    dy: i32,
    x_sign: i32,
    y_sign: i32,
    step: f32,
}

impl Line {
    pub fn new(f: DisplayPoint, l: DisplayPoint) -> Self {
        let dx = (l.x as i32 - f.x as i32).abs() + 1;
        let dy = (l.y as i32 - f.y as i32).abs() + 1;
        let x_sign = if l.x > f.x {
            1i32
        } else {
            -1i32
        };
        let y_sign = if l.y > f.y {
            1i32
        } else {
            -1i32
        };
        let step = if dx >= dy {
            dx as f32 / dy as f32
        } else {
            dy as f32 / dx as f32
        };

        Line {
            first: f,
            last: l,
            cur: f,
            dx: dx,
            dy: dy,
            x_sign: x_sign,
            y_sign: y_sign,
            step: step,
        }
    }
}

impl Iterator for Line {
    type Item = DisplayPoint;

    fn next(&mut self) -> Option<Self::Item> {

        if self.first == self.last {
            return None;
        }

        let (x, y) = if self.dx >= self.dy {

            let old_cur = &self.cur;
            let old_x = old_cur.x;
            let old_delta = (old_x as f32 - self.first.x as f32).abs() % self.step;

            let x = old_x as i32 + self.x_sign;
            let new_delta = (x as f32 - self.first.x as f32).abs() % self.step;

            let y = if new_delta > old_delta {
                old_cur.y as i32
            } else {
                old_cur.y as i32 + self.y_sign
            };
            (x, y)
        } else {

            let old_cur = &self.cur;
            let old_y = old_cur.y;
            let old_delta = (old_y as f32 - self.first.y as f32).abs() % self.step;

            let y = old_y as i32 + self.y_sign;
            let new_delta = (y as f32 - self.first.y as f32).abs() % self.step;

            let x = if new_delta > old_delta {
                old_cur.x as i32
            } else {
                old_cur.x as i32 + self.x_sign
            };
            (x, y)
        };

        self.cur = DisplayPoint {
            x: x as usize,
            y: y as usize,
        };

        if self.cur == self.last {
            None
        } else {
            Some(self.cur)
        }
    }
}

pub fn extrapolate<'a>(points: Box<Iterator<Item = DisplayPoint> + 'a>)
                       -> Box<Iterator<Item = DisplayPoint> + 'a> {

    let it = FlatMapPairs::new(points,
                               |a: DisplayPoint, b: DisplayPoint| once(a).chain(Line::new(a, b)));
    Box::new(it)
}