// inspired by @SCareAngel's version

use std::iter::Peekable;


pub struct FlatMapPairs<I, B, F>
    where I: Iterator,
          B: Iterator,
          F: FnMut(I::Item, I::Item) -> B
{
    it: Peekable<I>,
    f: F,
    cur: Option<B>,
}

impl<I, B, F> FlatMapPairs<I, B, F>
    where I: Iterator,
          B: Iterator,
          F: FnMut(I::Item, I::Item) -> B
{
    pub fn new(it: I, f: F) -> Self {
        FlatMapPairs {
            it: it.peekable(),
            f: f,
            cur: None,
        }
    }
}



impl<I, B, F> Iterator for FlatMapPairs<I, B, F>
    where I: Iterator,
          B: Iterator,
          F: FnMut(I::Item, I::Item) -> B,
          I::Item: Copy
{
    type Item = B::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(res) = self.cur.as_mut().and_then(Iterator::next) {
                return Some(res);
            }
            if let Some(val) = self.it.next() {
                if let Some(next_val) = self.it.peek() {
                    self.cur = Some((self.f)(val, *next_val))
                } else {
                    self.cur = Some((self.f)(val, val))
                }
            } else {
                return None;
            }
        }
    }
}
