//Utility File 

// Enum for color on board
pub enum Color {WHITE, BLACK, BOTH}

//Enum for Piecess
// Formatted as first letter denoting white or black: W = white and B = Black
// Formatted as second letter denoting Pieces name: P = pawn, K = king, Q = queen, N = knight, B = bishop, R = rook
pub enum Pieces {WP, WB, WN, WR, WQ, WK, BP, BB, BN, BR, BQ, BK}

//Enum for File on chess board
pub enum File {FA, FB, FC, FD, FE, FF, FG, FH}

//Enum for Rank on chess board
pub enum Rank {R1, R2, R3, R4, R5, R6, R7, R8}

//Enum for Square, each value represents a unique Pieces on the board
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

//fun implementation for Pieces value
impl Pieces{
    //Gets Pieces values, values should be pawn = 1, knight =3, bishop = 3, rook = 5, queen = 10, king = 100
    pub fn get_value(Pieces:Pieces) -> u8 {
        match Pieces {
            Pieces::WP => 1,
            Pieces::BP => 1,
            Pieces::WB => 3,
            Pieces::BB => 3,
            Pieces::WN => 3,
            Pieces::BN => 3,
            Pieces::WR => 5,
            Pieces::BR => 5,
            Pieces::WQ => 10,
            Pieces::BQ => 10,
            Pieces::WK => 100,
            Pieces::BK => 100
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    //test values returned from get value funtion
    #[test]
    fn test_get_value() {
        //test pawn white for value of 1
        assert_eq!(Pieces::get_value(Pieces::WP), 1);
        //test pawn black for value of 1 
        assert_eq!(Pieces::get_value(Pieces::BP), 1);
        //test bishop white for value of 3
        assert_eq!(Pieces::get_value(Pieces::WB), 3);
        //test bishop black for value of 3
        assert_eq!(Pieces::get_value(Pieces::BB), 3);
        //test knight white for value of 3
        assert_eq!(Pieces::get_value(Pieces::WN), 3);
        //test knight black for value of 3
        assert_eq!(Pieces::get_value(Pieces::BN), 3);
        //test rook white for value of 3
        assert_eq!(Pieces::get_value(Pieces::WR), 5);
        //test rook black for value of 5
        assert_eq!(Pieces::get_value(Pieces::BR), 5);
        //test queen white for value of 10
        assert_eq!(Pieces::get_value(Pieces::WQ), 10);
        //test queen black for value of 10
        assert_eq!(Pieces::get_value(Pieces::BQ), 10);
        //test king white for value of 100
        assert_eq!(Pieces::get_value(Pieces::WK), 100);
        //test king black for value of 100
        assert_eq!(Pieces::get_value(Pieces::BK), 100);

    }
}