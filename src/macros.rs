
const EPSILON: f64 = 0.0000001;

type TypeFn = fn(x: f64) -> f64;

pub struct Formula {
    pub current_x: f64,
    pub stop: f64,
    pub start: f64,
    pub step: f64,
    pub f: TypeFn,
}

impl Formula {
    pub fn new(start: f64, stop: f64, step: f64, f: TypeFn) -> Self {
        Formula {
            current_x: start,
            stop: stop,
            start: start,
            step: step,
            f: f,
        }
    }
}


impl Iterator for Formula {
    type Item = (f64, f64);

    #[inline]
    fn next(&mut self) -> Option<(f64, f64)> {
        if self.start < self.stop {
            if self.current_x < self.stop {
                let x = self.current_x;
                self.current_x += self.step;
                Some((x, (self.f)(x)))
            } else if (self.stop - self.current_x).abs() < EPSILON {
                let x = self.stop;
                self.current_x += self.step;
                Some((x, (self.f)(x)))
            } else {
                None
            }
        } else if self.current_x > self.stop {
            let x = self.current_x;
            self.current_x -= self.step;
            Some((x, (self.f)(x)))
        } else if (self.stop - self.current_x).abs() < EPSILON {
            let x = self.stop;
            self.current_x -= self.step;
            Some((x, (self.f)(x)))
        } else {
            None
        }
    }
}

impl Copy for Formula {}

impl Clone for Formula {
    fn clone(&self) -> Self {
        *self
    }
}


#[macro_export]
macro_rules! formula {
    ( y($x:ident) = $form:expr, x = [$start:expr, $stop:expr; $step:expr] )
    =>
    {
        {
        fn f($x: f64) -> f64 { $form }
        let start = $start as f64;
        let stop = $stop as f64;
        let step = $step as f64;
        Formula::new(start, stop, step, f)
    }};
}




#[test]
fn macros_test() {
    let points = formula!(y(x) = -(0.64f64 - x.powi(2)).sqrt(), x = [0.8f64, 0.8f64; 0.01]);

    let pp: Vec<_> = points.collect();
    assert_eq!(pp, vec![(0f64, 0f64)]);
}
