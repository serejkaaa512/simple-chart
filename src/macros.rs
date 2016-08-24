#[macro_export]
macro_rules! formula {
    ( y($x:ident) = $form:expr, x = [$min:expr, $max:expr; $step:expr] )
    =>
    {
        {
            struct Formula {
               current_x: f64,
               max_x: f64,
               step_x: f64,
               f: Box <Fn(f64)->f64>,
           }

           impl Iterator for Formula {
            type Item = (f64,f64);

            #[inline]
            fn next(&mut self) -> Option<(f64,f64)> {
                if self.current_x <= self.max_x {
                    let x = self.current_x;
                    self.current_x += self.step_x;
                    Some((x, (self.f)(x)))
                } else {
                    None
                }
            }
        }

        let min_x = $min as f64;
        let max_x = $max as f64;
        let step_x = $step as f64;
        Formula {current_x:min_x, max_x: max_x, step_x: step_x, f: Box::new(|$x| $form) }
    }
};
}




#[test]
fn macros_test() {
    let points = formula!(y(x) = x*2.0, x = [0, 2; 1]);
    let pp: Vec<_> = points.collect();
    assert_eq!(pp, vec![(0f64, 0f64), (1f64, 2f64), (2f64, 4f64)]);
}
