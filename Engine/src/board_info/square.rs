use std::ops::{Index, Shl, Shr};

//Enum for Square, each value represents a unique Piece on the board
#[derive(Copy, Clone, Debug)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

//implements Index for square
impl Index<Square> for [u64] {
    type Output = u64;

    fn index(&self, square: Square) -> &u64 {
        return &self[square as usize];
    }
}

//implements Index for square
impl Index<Square> for [u8] {
    type Output = u8;

    fn index(&self, square: Square) -> &u8 {
        return &self[square as usize];
    }
}

impl Shl<Square> for u64 {
    type Output = Self;

    fn shl(self, square: Square) -> Self::Output {
        self << square as u64
    }
}

impl Shr<Square> for u64 {
    type Output = Self;

    fn shr(self, square: Square) -> Self::Output {
        self >> square as u64
    }
}

impl Square {
    //Maps u8 to Square
    pub fn from_u8(value: u8) -> Square {
        match value {
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,
            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,
            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,
            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,
            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,
            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,
            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,
            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,
            _ => {
                println!("WRONG: {}", value);
                panic!("DID A BAD");
            }
        }
    }

    pub fn get_rank(&self) -> u8 {
        return *self as u8 / 8;
    }

    pub fn get_file(&self) -> u8 {
        return *self as u8 % 8;
    }

    pub fn to_bit_board(self) -> u64 {
        return 1 << self;
    }
}

//get square from rank and file
pub fn get_square(rank: u8, file: u8) -> Square {
    let num = (rank << 3) + file;
    return Square::from_u8(num);
}

//get square from rank and file
pub fn get_square_temp(rank: i8, file: i8) -> Square {
    let num = ((rank as u8) << 3) + file as u8;
    if num > 63 {
        print!("FUCK")
    }
    return Square::from_u8(num);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_rank_file() {
        assert_eq!(Square::A1.get_file(), 0);
        assert_eq!(Square::A1.get_rank(), 0);
        assert_eq!(Square::H8.get_file(), 7);
        assert_eq!(Square::H8.get_rank(), 7);
    }
}
