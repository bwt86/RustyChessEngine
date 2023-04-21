//Utility File 
use std::ops::Index;
pub const INT_TO_CHAR:[char;12] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];

// Enum for color on board
#[derive(Copy, Clone, Debug)]
pub enum Color {WHITE, BLACK, BOTH}
//Implements index for Color enum
impl Index<Color> for [u64]{
    type Output = u64;
    
    fn index(&self, color:Color) -> &u64{
        return &self[color as usize];
    }
}

//Enum for Pieces
// Formatted as first letter denoting white or black: W = white and B = Black
// Formatted as second letter denoting Piece name: P = pawn, K = king, Q = queen, N = knight, B = bishop, R = rook
#[derive(Copy, Clone, Debug)]
pub enum Piece {WP, WB, WN, WR, WQ, WK, BP, BB, BN, BR, BQ, BK}
//fun implementation for Piece value
impl Piece{
    //Gets Piece values, values should be pawn = 1, knight =3, bishop = 3, rook = 5, queen = 10, king = 100
    pub fn get_value(piece:Piece) -> u8 {
        match piece {
            Piece::WP => 1,
            Piece::BP => 1,
            Piece::WB => 3,
            Piece::BB => 3,
            Piece::WN => 3,
            Piece::BN => 3,
            Piece::WR => 5,
            Piece::BR => 5,
            Piece::WQ => 10,
            Piece::BQ => 10,
            Piece::WK => 100,
            Piece::BK => 100
        }
    }
}
//Implements index for Piece enum 
impl Index<Piece> for [u64]{
    type Output = u64;
    
    fn index(&self, piece:Piece) -> &u64{
        return &self[piece as usize];
    }
}


//Enum for File on chess board
#[derive(Copy, Clone)]
pub enum File {FA, FB, FC, FD, FE, FF, FG, FH}

//Enum for Rank on chess board
#[derive(Copy, Clone, Debug)]
pub enum Rank {R1, R2, R3, R4, R5, R6, R7, R8}

//Enum for Square, each value represents a unique Piece on the board
#[derive(Copy, Clone, Debug)]
pub enum Square{
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8

}

//implements Index for square 
impl Index<Square> for [u64]{
    type Output = u64;
    
    fn index(&self, square:Square) -> &u64{
        return &self[square as usize];
    }
}

impl Square{
    //Maps u8 to Square
    fn from_u8(value:u8) -> Square{
        match value {
            0 => Square:: A1,
            1 => Square:: B1,
            2 => Square:: C1,
            3 => Square:: D1,
            4 => Square:: E1,
            5 => Square:: F1,
            6 => Square:: G1,
            7 => Square:: H1,
            8 => Square:: A2,
            9 => Square:: B2,
            10 => Square:: C2,
            11 => Square:: D2,
            12 => Square:: E2,
            13 => Square:: F2,
            14 => Square:: G2,
            15 => Square:: H2,
            16 => Square:: A3,
            17 => Square:: B3,
            18 => Square:: C3,
            19 => Square:: D3,
            20 => Square:: E3,
            21 => Square:: F3,
            22 => Square:: G3,
            23 => Square:: H3,
            24 => Square:: A4,
            25 => Square:: B4,
            26 => Square:: C4,
            27 => Square:: D4,
            28 => Square:: E4,
            29 => Square:: F4,
            30 => Square:: G4,
            31 => Square:: H4,
            32 => Square:: A5,
            33 => Square:: B5,
            34 => Square:: C5,
            35 => Square:: D5,
            36 => Square:: E5,
            37 => Square:: F5,
            38 => Square:: G5,
            39 => Square:: H5,
            40 => Square:: A6,
            41 => Square:: B6,
            42 => Square:: C6,
            43 => Square:: D6,
            44 => Square:: E6,
            45 => Square:: F6,
            46 => Square:: G6,
            47 => Square:: H6,
            48 => Square:: A7,
            49 => Square:: B7,
            50 => Square:: C7,
            51 => Square:: D7,
            52 => Square:: E7,
            53 => Square:: F7,
            54 => Square:: G7,
            55 => Square:: H7,
            56 => Square:: A8,
            57 => Square:: B8,
            58 => Square:: C8,
            59 => Square:: D8,
            60 => Square:: E8,
            61 => Square:: F8,
            62 => Square:: G8,
            63 => Square:: G8,
            _ => todo!()  
        }
    }
}

//get square from rank and file
pub fn get_square(rank:u8, file:u8) -> Square{
    let num = ((rank as u8) << 3) + file as u8;
    return Square::from_u8(num);
}

pub enum CastlingRights{WKC = 1, WQC = 2, BKC = 4, BQC = 8}

//Checks if a square is occupied
pub fn is_occupied(bit_board:u64, square:Square) -> bool{
    return bit_board & (1 << square as u64) != 0;
}

//Prints out bit_board unformated
pub fn print_bb(bit_board: u64){
    print!("\n");
    for rank in (0..8).rev(){
        for file in 0..8{
            if is_occupied(bit_board, get_square(rank, file)){
                print!("| 1 |");
            }
            else{
                print!("| 0 |");
            }
        }
        print!("\n");
    }
    print!("\n");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    //test values returned from get value funtion
    #[test]
    fn test_get_value() {
        //test pawn white for value of 1
        assert_eq!(Piece::get_value(Piece::WP), 1);
        //test pawn black for value of 1 
        assert_eq!(Piece::get_value(Piece::BP), 1);
        //test bishop white for value of 3
        assert_eq!(Piece::get_value(Piece::WB), 3);
        //test bishop black for value of 3
        assert_eq!(Piece::get_value(Piece::BB), 3);
        //test knight white for value of 3
        assert_eq!(Piece::get_value(Piece::WN), 3);
        //test knight black for value of 3
        assert_eq!(Piece::get_value(Piece::BN), 3);
        //test rook white for value of 3
        assert_eq!(Piece::get_value(Piece::WR), 5);
        //test rook black for value of 5
        assert_eq!(Piece::get_value(Piece::BR), 5);
        //test queen white for value of 10
        assert_eq!(Piece::get_value(Piece::WQ), 10);
        //test queen black for value of 10
        assert_eq!(Piece::get_value(Piece::BQ), 10);
        //test king white for value of 100
        assert_eq!(Piece::get_value(Piece::WK), 100);
        //test king black for value of 100
        assert_eq!(Piece::get_value(Piece::BK), 100);

    }
}