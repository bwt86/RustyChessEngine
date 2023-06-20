use super::{bit_masks::*, bitboard::Bitboard};

// These are the square positions for a standard chess board.
// They are enumerated by ranks (rows) from 1 to 8 and files (columns) from A to H.
#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Square{
    A1, B1, C1, D1, E1, F1, G1, H1, 
    A2, B2, C2, D2, E2, F2, G2, H2, 
    A3, B3, C3, D3, E3, F3, G3, H3, 
    A4, B4, C4, D4, E4, F4, G4, H4, 
    A5, B5, C5, D5, E5, F5, G5, H5, 
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8, 
}
macro_rules! impl_from_for_square {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Square {
                fn from(value: $ty) -> Self {
                    SQUARES[value as usize]
                }
            }
        )*
    };
}

impl_from_for_square! { u8, u16, u32, u64, usize }

// Corresponding constant array to easily get a square by an index.
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

// Enums and array to represent and get the file part of a square.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum File {
    FA,
    FB,
    FC,
    FD,
    FE,
    FF,
    FG,
    FH,
}
macro_rules! impl_from_for_file {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for File {
                fn from(value: $ty) -> Self {
                    FILES[value as usize]
                }
            }
        )*
    };
}

impl_from_for_file! { u8, u16, u32, u64, usize }

// Enums and array to represent and get the rank part of a square.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}
macro_rules! impl_from_for_rank {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Rank {
                fn from(value: $ty) -> Self {
                    RANKS[value as usize]
                }
            }
        )*
    };
}

impl_from_for_rank! { u8, u16, u32, u64, usize }

pub const FILES: [File; 8] = [
    File::FA,
    File::FB,
    File::FC,
    File::FD,
    File::FE,
    File::FF,
    File::FG,
    File::FH,
];

pub const RANKS: [Rank; 8] = [
    Rank::R1,
    Rank::R2,
    Rank::R3,
    Rank::R4,
    Rank::R5,
    Rank::R6,
    Rank::R7,
    Rank::R8,
];

// The main functionalities of Square
impl Square {
    pub fn from_rank_file(rank: Rank, file: File) -> Square {
        SQUARES[rank as usize * 8 + file as usize]
    }

    pub fn get_rank(self) -> Rank {
        RANKS[self as usize / 8]
    }

    pub fn get_file(self) -> File {
        FILES[self as usize % 8]
    }

    pub fn get_rank_file(self) -> (Rank, File) {
        (self.get_rank(), self.get_file())
    }

    pub fn get_bb(self) -> Bitboard {
        Bitboard::init(self as u64)
    }

    pub fn get_rank_bb(self) -> Bitboard {
        RANKS_BB[self.get_rank()]
    }

    pub fn get_file_bb(self) -> Bitboard {
        FILES_BB[self.get_file()]
    }

    pub fn to_string(self) -> String {
        let rank: u8 = self.get_rank() as u8 + 1;
        let file: char = (self.get_file() as u8 + b'a') as char;

        format!("{}{}", file, rank)
    }

    pub fn get_square_algebraic(square: &str) -> Result<Square, &'static str> {
        if square.len() != 2 {
            return Err("Invalid square");
        }
        let mut chars = square.chars();
        let file = match chars.next().unwrap() {
            'a' => File::FA,
            'b' => File::FB,
            'c' => File::FC,
            'd' => File::FD,
            'e' => File::FE,
            'f' => File::FF,
            'g' => File::FG,
            'h' => File::FH,
            _ => return Err("Invalid file"),
        };

        let rank = match chars.next().unwrap() {
            '1' => Rank::R1,
            '2' => Rank::R2,
            '3' => Rank::R3,
            '4' => Rank::R4,
            '5' => Rank::R5,
            '6' => Rank::R6,
            '7' => Rank::R7,
            '8' => Rank::R8,
            _ => return Err("Invalid rank"),
        };

        Ok(Square::from_rank_file(rank, file))
    }
}

impl std::ops::Shl<Square> for u64 {
    type Output = u64;

    fn shl(self, rhs: Square) -> Self::Output {
        self << rhs as u64
    }
}

impl std::ops::Shr<Square> for u64 {
    type Output = u64;

    fn shr(self, rhs: Square) -> Self::Output {
        self >> rhs as u64
    }
}

impl std::ops::BitAnd<Square> for u64 {
    type Output = u64;

    fn bitand(self, rhs: Square) -> Self::Output {
        self & (1 << rhs as u64)
    }
}

impl std::ops::BitOr<Square> for u64 {
    type Output = u64;

    fn bitor(self, rhs: Square) -> Self::Output {
        self | (1 << rhs as u64)
    }
}

impl std::ops::BitXor<Square> for u64 {
    type Output = u64;

    fn bitxor(self, rhs: Square) -> Self::Output {
        self ^ (1 << rhs as u64)
    }
}

impl std::ops::Not for Square {
    type Output = u64;

    fn not(self) -> Self::Output {
        !(1 << self as u64)
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
    fn test_square_from() {
        // Test with minimum valid value
        let s = Square::from(0 as u64);
        assert_eq!(s, Square::A1);

        // Test with maximum valid value
        let s = Square::from(63 as u8);
        assert_eq!(s, Square::H8);

        // Test with a value in the middle
        let s = Square::from(32 as u16);
        assert_eq!(s, Square::A5);
    }

    #[test]
    fn test_square_from_rank_file() {
        // Test with minimum rank and file
        let s = Square::from_rank_file(Rank::R1, File::FA);
        assert_eq!(s, Square::A1);

        // Test with maximum rank and file
        let s = Square::from_rank_file(Rank::R8, File::FH);
        assert_eq!(s, Square::H8);

        // Test with a combination in the middle
        let s = Square::from_rank_file(Rank::R5, File::FD);
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
    fn test_get_square_algebraic() {
        // Test with valid input
        let s = Square::get_square_algebraic("a1");
        assert_eq!(s.unwrap(), Square::A1);

        // Test with invalid file
        let s = Square::get_square_algebraic("z1");
        assert!(s.is_err());

        // Test with invalid rank
        let s = Square::get_square_algebraic("a9");
        assert!(s.is_err());

        // Test with empty string
        let s = Square::get_square_algebraic("");
        assert!(s.is_err());

        // Test with extra characters
        let s = Square::get_square_algebraic("e2extra");
        assert!(s.is_err());
    }

    #[test]
    fn test_file_from_num() {
        assert_eq!(File::from(0 as u64), File::FA);
        assert_eq!(File::from(7 as usize), File::FH);
    }

    #[test]
    fn test_rank_from_num() {
        assert_eq!(Rank::from(0 as u32), Rank::R1);
        assert_eq!(Rank::from(7 as u64), Rank::R8);
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
    fn test_bitwise_operations() {
        // Test left shift
        assert_eq!(1_u64 << Square::A1, 1);
        assert_eq!(1_u64 << Square::C1, 4);

        // Test right shift
        assert_eq!((1_u64 << Square::C1) >> Square::B1, 2);

        // Test bitwise and
        assert_eq!(1_u64 & Square::A1, 1);
        assert_eq!(2_u64 & Square::A1, 0);

        // Test bitwise or
        assert_eq!(1_u64 | Square::B1, 3);

        // Test bitwise xor
        assert_eq!(3_u64 ^ Square::B1, 1);

        // Test bitwise not
        assert_eq!(!Square::A1, !1);
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
