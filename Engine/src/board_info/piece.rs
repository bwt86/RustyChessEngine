use super::color::{BLACK, WHITE};

pub const PIECE_CHARS: [char; 12] = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];
pub const PIECE_CHARS_FANCY: [char; 12] = ['♟', '♞', '♝', '♜', '♛', '♚', '♙', '♘', '♗', '♖', '♕', '♔'];

pub const WP: usize = 0;
pub const WB: usize = 1;
pub const WN: usize = 2;
pub const WR: usize = 3;
pub const WQ: usize = 4;
pub const WK: usize = 5;
pub const BP: usize = 6;
pub const BB: usize = 7;
pub const BN: usize = 8;
pub const BR: usize = 9;
pub const BQ: usize = 10;
pub const BK: usize = 11;

pub const PIECES: [usize; 12] = [WP, WB, WN, WR, WQ, WK, BP, BB, BN, BR, BQ, BK];

pub fn get_value(piece: usize) -> u8 {
    match piece {
        WP | BP => 1,
        WB | BB => 3,
        WN | BN => 3,
        WR | BR => 5,
        WQ | BQ => 10,
        WK | BK => 100,
        _ => todo!(),
    }
}

//Gets color of Piece
pub fn get_color(piece: usize) -> usize {
    if (piece as i8 - 5) > 0 {
        return BLACK;
    }
    return WHITE;
}

pub fn from_char(piece_char: char) -> usize {
    match piece_char {
        'p' => BP,
        'b' => BB,
        'n' => BN,
        'r' => BR,
        'q' => BQ,
        'k' => BK,
        'P' => WP,
        'B' => WB,
        'N' => WN,
        'R' => WR,
        'Q' => WQ,
        'K' => WK,
        _ => todo!(),
    }
}

// //Enum for Pieces
// // Formatted as first letter denoting white or black: W = white and B = Black
// // Formatted as second letter denoting Piece name: P = pawn, K = king, Q = queen, N = knight, B = bishop, R = rook
// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum Piece {
//     WP,
//     WB,
//     WN,
//     WR,
//     WQ,
//     WK,
//     BP,
//     BB,
//     BN,
//     BR,
//     BQ,
//     BK,
// }
// //fun implementation for Piece value
// impl Piece {
//     //Gets Piece values, values should be pawn = 1, knight =3, bishop = 3, rook = 5, queen = 10, king = 100
//     pub fn get_value(piece: Piece) -> u8 {
//         match piece {
//             Piece::WP | Piece::BP => 1,
//             Piece::WB | Piece::BB => 3,
//             Piece::WN | Piece::BN => 3,
//             Piece::WR | Piece::BR => 5,
//             Piece::WQ | Piece::BQ => 10,
//             Piece::WK | Piece::BK => 100,
//         }
//     }

//     //Gets color of Piece
//     pub fn get_color(&self) -> Color {
//         if (*self as i8 - 5) > 0 {
//             return Color::Black;
//         }
//         return Color::White;
//     }

//     pub fn from_char(piece_char: char) -> Piece {
//         match piece_char {
//             'p' => Piece::BP,
//             'b' => Piece::BB,
//             'n' => Piece::BN,
//             'r' => Piece::BR,
//             'q' => Piece::BQ,
//             'k' => Piece::BK,
//             'P' => Piece::WP,
//             'B' => Piece::WB,
//             'N' => Piece::WN,
//             'R' => Piece::WR,
//             'Q' => Piece::WQ,
//             'K' => Piece::WK,
//             _ => todo!(),
//         }
//     }
// }

// impl Index<Piece> for [u64] {
//     type Output = u64;

//     fn index(&self, piece: Piece) -> &u64 {
//         return &self[piece as usize];
//     }
// }

// impl IndexMut<Piece> for [u64] {
//     fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
//         return &mut self[piece as usize];
//     }
// }

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     //test values returned from get value funtion
//     #[test]
//     fn test_get_value() {
//         //test pawn white for value of 1
//         assert_eq!(Piece::get_value(Piece::WP), 1);
//         //test pawn black for value of 1
//         assert_eq!(Piece::get_value(Piece::BP), 1);
//         //test bishop white for value of 3
//         assert_eq!(Piece::get_value(Piece::WB), 3);
//         //test bishop black for value of 3
//         assert_eq!(Piece::get_value(Piece::BB), 3);
//         //test knight white for value of 3
//         assert_eq!(Piece::get_value(Piece::WN), 3);
//         //test knight black for value of 3
//         assert_eq!(Piece::get_value(Piece::BN), 3);
//         //test rook white for value of 3
//         assert_eq!(Piece::get_value(Piece::WR), 5);
//         //test rook black for value of 5
//         assert_eq!(Piece::get_value(Piece::BR), 5);
//         //test queen white for value of 10
//         assert_eq!(Piece::get_value(Piece::WQ), 10);
//         //test queen black for value of 10
//         assert_eq!(Piece::get_value(Piece::BQ), 10);
//         //test king white for value of 100
//         assert_eq!(Piece::get_value(Piece::WK), 100);
//         //test king black for value of 100
//         assert_eq!(Piece::get_value(Piece::BK), 100);
//     }
// }
