#[macro_export]
macro_rules! formula {
    ( y($x:ident): $sty:ty = $form:expr, x = [$min:expr, $max:expr; $step:expr] ) 
    => 
    { 
        {
            struct Formula {
               current_x: $sty,
               max_x: $sty,
               step_x: $sty,
               f: Box <Fn($sty)->$sty>,
           }

           impl Iterator for Formula {
            type Item = ($sty,$sty);

            #[inline]
            fn next(&mut self) -> Option<($sty,$sty)> {
                if self.current_x <= self.max_x {
                    let x = self.current_x;
                    self.current_x += self.step_x;
                    Some((x, (self.f)(x)))
                } else {
                    None
                }
            }
        }

        Formula {current_x:$min, max_x: $max, step_x: $step, f: Box::new(|$x| $form) }
    }
};
}




#[test]
fn macros_test() {
    let points = formula!(y(x): i32 = {x*2}, x = [0, 2; 1]);
    let pp: Vec<_> = points.collect();
    assert_eq!(pp, vec![(0, 0), (1, 2), (2, 4)]);
}
