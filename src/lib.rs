#![cfg_attr(feature = "dev", allow(unstable_features, unused_features))]
#![cfg_attr(feature = "dev", feature(plugin, test))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate byteorder;
#[macro_use]
extern crate quick_error;
#[macro_use]
pub mod macros;
pub mod chart;
pub mod bitmap;
mod line;
mod flatmappairs;
mod axis;
mod tick;


pub use self::bitmap::BitMap;
pub use self::bitmap::Color;
pub use self::chart::DisplayPoint;
pub use self::chart::Point;
pub use self::chart::Chart;
pub use self::chart::Serie;
pub use self::axis::Axis;
pub use self::macros::Formula;
