use std::fmt::Display;

mod char;

pub use char::CharTile;

pub trait Tile: Default {
    type Output: Display + Default;

    fn is_collapsed(&self) -> bool;

    fn value(&self) -> Self::Output;
}
