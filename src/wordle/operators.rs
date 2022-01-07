use super::*;

use std::ops;

impl<const N: usize> Word<N> {
    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.letters.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut char> {
        self.letters.iter_mut()
    }
}

impl<const N: usize> ops::Index<usize> for Word<N> {
    type Output = char;
    fn index(&self, i: usize) -> &char {
        &self.letters[i]
    }
}

impl<const N: usize> ops::IndexMut<usize> for Word<N> {
    fn index_mut(&mut self, i: usize) -> &mut char {
        &mut self.letters[i]
    }
}

impl<const N: usize> Clue<N> {
    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.tiles.iter_mut()
    }
}

impl<const N: usize> ops::Index<usize> for Clue<N> {
    type Output = Tile;
    fn index(&self, i: usize) -> &Tile {
        &self.tiles[i]
    }
}

impl<const N: usize> ops::IndexMut<usize> for Clue<N> {
    fn index_mut(&mut self, i: usize) -> &mut Tile {
        &mut self.tiles[i]
    }
}
