#![feature(test)]
extern crate test;
extern crate byteorder;
#[macro_use]
extern crate quick_error;
#[macro_use]
pub mod macros;
pub mod graph;
pub mod bitmap;
mod line;
mod flatmappairs;
mod axis;
mod tick;

pub use self::bitmap::BitMap;
pub use self::bitmap::Color;
pub use self::graph::DisplayPoint;
