use std::fmt::{Debug, Display};

use super::Tile;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CharTile(char);

impl Default for CharTile {
    fn default() -> Self {
        Self(' ')
    }
}

impl From<char> for CharTile {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl Tile for CharTile {
    type Output = char;

    fn is_collapsed(&self) -> bool {
        self.0 != ' '
    }

    fn value(&self) -> Self::Output {
        self.0
    }
}

impl Display for CharTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for CharTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
