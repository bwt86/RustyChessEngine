// use std::ops::{Index, IndexMut};

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;
pub const BOTH: usize = 2;

pub const COLORS: [usize; 3] = [WHITE, BLACK, BOTH];

// // Enum for color on board
// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum Color {
//     White,
//     Black,
//     Both,
// }

// //Implements index for Color enum
// impl Index<Color> for [u64] {
//     type Output = u64;

//     fn index(&self, color: Color) -> &u64 {
//         return &self[color as usize];
//     }
// }

// impl IndexMut<Color> for [u64] {
//     fn index_mut(&mut self, color: Color) -> &mut Self::Output {
//         return &mut self[color as usize];
//     }
// }

// impl Index<Color> for [[u64; 64]] {
//     type Output = [u64; 64];

//     fn index(&self, color: Color) -> &[u64; 64] {
//         return &self[color as usize];
//     }
// }

// impl IndexMut<Color> for [[u64; 64]] {
//     fn index_mut(&mut self, color: Color) -> &mut Self::Output {
//         return &mut self[color as usize];
//     }
// }
