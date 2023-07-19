use super::bitboard::Bitboard;

#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Square{
    A1 = 0,  B1 = 1,  C1 = 2,  D1 = 3,  E1 = 4,  F1 = 5,  G1 = 6,  H1 = 7, 
    A2 = 8,  B2 = 9,  C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum File {
    FA = 0,
    FB = 1,
    FC = 2,
    FD = 3,
    FE = 4,
    FF = 5,
    FG = 6,
    FH = 7,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
}

#[rustfmt::skip]
pub const SQUARES:[Square; 64] = [
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1, 
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2, 
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3, 
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4, 
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5, 
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
];
pub const FILES: [File; 8] = [File::FA, File::FB, File::FC, File::FD, File::FE, File::FF, File::FG, File::FH];

pub const RANKS: [Rank; 8] = [Rank::R1, Rank::R2, Rank::R3, Rank::R4, Rank::R5, Rank::R6, Rank::R7, Rank::R8];

impl Square {
    #[inline]
    pub fn from_file_rank(file: File, rank: Rank) -> Square {
        SQUARES[rank as usize * 8 + file as usize]
    }

    #[inline]
    pub fn from_index(index: usize) -> Square {
        SQUARES[index]
    }

    pub fn from_string(s: &str) -> Result<Square, &'static str> {
        let file = s.chars().nth(0).unwrap();
        let rank = s.chars().nth(1).unwrap();

        let file = file as u8 - b'a';
        let rank = rank as u8 - b'1';

        if file > 7 || rank > 7 {
            return Err("Invalid square");
        }

        Ok(Square::from_file_rank(FILES[file as usize], RANKS[rank as usize]))
    }

    #[inline]
    pub fn get_rank(self) -> Rank {
        RANKS[self as usize / 8]
    }

    #[inline]
    pub fn get_file(self) -> File {
        FILES[self as usize % 8]
    }

    pub fn get_rank_file(self) -> (Rank, File) {
        (self.get_rank(), self.get_file())
    }

    pub fn to_bitboard(&self) -> Bitboard {
        Bitboard::new_from_square(*self)
    }

    pub fn to_string(self) -> String {
        let rank: u8 = self.get_rank() as u8 + 1;
        let file: char = (self.get_file() as u8 + b'a') as char;

        format!("{}{}", file, rank)
    }

    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }

    pub fn is_light(self) -> bool {
        self.get_rank() as u8 + self.get_file() as u8 % 2 == 1
    }

    pub fn is_dark(self) -> bool {
        self.get_rank() as u8 + self.get_file() as u8 % 2 == 0
    }

    pub fn get_next(self) -> Square {
        SQUARES[(self.to_index() + 1) % 8]
    }

    pub fn get_prev(self) -> Square {
        SQUARES[((self.to_index() as i8 - 1) % 8) as usize]
    }

    pub fn move_up(self, n: u8) -> Square {
        SQUARES[(self.to_index() + n as usize * 8) % 64]
    }

    pub fn move_down(self, n: u8) -> Square {
        SQUARES[(self.to_index() - n as usize * 8) % 64]
    }

    pub fn move_left(self, n: u8) -> Square {
        SQUARES[(self.to_index() - n as usize) % 64]
    }

    pub fn move_right(self, n: u8) -> Square {
        SQUARES[(self.to_index() + n as usize) % 64]
    }

    pub fn flip(self) -> Square {
        SQUARES[63 - self.to_index()]
    }
}

impl File {
    pub fn from_index(index: usize) -> File {
        FILES[index]
    }

    pub fn get_next(self) -> File {
        FILES[(self as usize + 1) % 8]
    }

    pub fn get_next_n(self, n: u8) -> File {
        FILES[(self as u8 + n) as usize % 8]
    }

    pub fn get_prev(self) -> File {
        FILES[((self as i8 - 1) % 8) as usize]
    }

    pub fn get_prev_n(self, n: u8) -> File {
        FILES[((self as i8 - n as i8) % 8) as usize]
    }
}

impl Rank {
    pub fn from_index(index: usize) -> Rank {
        RANKS[index]
    }

    pub fn get_next(self) -> Rank {
        RANKS[(self as usize + 1) % 8]
    }

    pub fn get_next_n(self, n: u8) -> Rank {
        RANKS[(self as u8 + n) as usize % 8]
    }

    pub fn get_prev(self) -> Rank {
        RANKS[((self as i8 - 1) % 8) as usize]
    }

    pub fn get_prev_n(self, n: u8) -> Rank {
        RANKS[((self as i8 - n as i8) % 8) as usize]
    }
}

// These are implementations of Index and IndexMut for
// Square, Rank and File for both arrays and vectors.
impl<T> std::ops::Index<Square> for [T] {
    type Output = T;

    fn index(&self, index: Square) -> &T {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Square> for [T] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<Square> for Vec<T> {
    type Output = T;

    fn index(&self, index: Square) -> &T {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Square> for Vec<T> {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<Rank> for [T] {
    type Output = T;

    fn index(&self, index: Rank) -> &T {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Rank> for [T] {
    fn index_mut(&mut self, index: Rank) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<Rank> for Vec<T> {
    type Output = T;

    fn index(&self, index: Rank) -> &T {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<Rank> for Vec<T> {
    fn index_mut(&mut self, index: Rank) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<File> for [T] {
    type Output = T;

    fn index(&self, index: File) -> &T {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<File> for [T] {
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

impl<T> std::ops::Index<File> for Vec<T> {
    type Output = T;

    fn index(&self, index: File) -> &T {
        &self[index as usize]
    }
}

impl<T> std::ops::IndexMut<File> for Vec<T> {
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from_rank_file() {
        // Test with minimum rank and file
        let s = Square::from_file_rank(File::FA, Rank::R1);
        assert_eq!(s, Square::A1);

        // Test with maximum rank and file
        let s = Square::from_file_rank(File::FH, Rank::R8);
        assert_eq!(s, Square::H8);

        // Test with a combination in the middle
        let s = Square::from_file_rank(File::FD, Rank::R5);
        assert_eq!(s, Square::D5);
    }

    #[test]
    fn test_get_rank() {
        // Test with a square from the first rank
        let s = Square::A1;
        assert_eq!(s.get_rank(), Rank::R1);

        // Test with a square from the last rank
        let s = Square::H8;
        assert_eq!(s.get_rank(), Rank::R8);

        // Test with a square from a rank in the middle
        let s = Square::D5;
        assert_eq!(s.get_rank(), Rank::R5);
    }

    #[test]
    fn test_get_file() {
        // Test with a square from the first file
        let s = Square::A1;
        assert_eq!(s.get_file(), File::FA);

        // Test with a square from the last file
        let s = Square::H8;
        assert_eq!(s.get_file(), File::FH);

        // Test with a square from a file in the middle
        let s = Square::D5;
        assert_eq!(s.get_file(), File::FD);
    }

    #[test]
    fn test_square_get_rank_file() {
        let s = Square::A1;
        let (rank, file) = s.get_rank_file();
        assert_eq!(rank, Rank::R1);
        assert_eq!(file, File::FA);
    }

    #[test]
    fn test_square_to_string() {
        assert_eq!(Square::A1.to_string(), "a1");
        assert_eq!(Square::H8.to_string(), "h8");
    }

    #[test]
    fn test_index_operations() {
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let array = [0, 1, 2, 3, 4, 5, 6, 7];

        // Test indexing into vec with Square
        assert_eq!(vec[Square::A1], 0);
        assert_eq!(vec[Square::B1], 1);

        // Test indexing into array with Square
        assert_eq!(array[Square::A1], 0);
        assert_eq!(array[Square::B1], 1);

        // Test indexing into vec with Rank
        assert_eq!(vec[Rank::R1], 0);
        assert_eq!(vec[Rank::R2], 1);

        // Test indexing into array with Rank
        assert_eq!(array[Rank::R1], 0);
        assert_eq!(array[Rank::R2], 1);

        // Test indexing into vec with File
        assert_eq!(vec[File::FA], 0);
        assert_eq!(vec[File::FB], 1);

        // Test indexing into array with File
        assert_eq!(array[File::FA], 0);
        assert_eq!(array[File::FB], 1);
    }
}
