use std::ops::{Index, IndexMut};

// Enum for color on board
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    WHITE,
    BLACK,
    BOTH,
}

//Implements index for Color enum
impl Index<Color> for [u64] {
    type Output = u64;

    fn index(&self, color: Color) -> &u64 {
        return &self[color as usize];
    }
}

impl IndexMut<Color> for [u64] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        return &mut self[color as usize];
    }
}

impl Index<Color> for [[u64; 64]] {
    type Output = [u64; 64];

    fn index(&self, color: Color) -> &[u64; 64] {
        return &self[color as usize];
    }
}

impl IndexMut<Color> for [[u64; 64]] {
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        return &mut self[color as usize];
    }
}
