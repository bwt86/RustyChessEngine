use super::{
    piece::{Color, Piece, PieceType, PIECES},
    square::{Square, SQUARES},
};

/// Represents the game phase for piece-square table evaluation
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Phase {
    Opening,
    Endgame,
}

/// Piece-square tables for both opening and endgame phases
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PieceSquareTable {
    /// Opening phase piece-square tables for each piece type
    openings: [[i32; 64]; 6],
    /// Endgame phase piece-square tables for each piece type
    endgames: [[i32; 64]; 6],
}

impl PieceSquareTable {
    /// Creates a new piece-square table with default values
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            openings: [PAWN_OPENING, KNIGHT_OPENING, BISHOP_OPENING, ROOK_OPENING, QUEEN_OPENING, KING_OPENING],
            endgames: [PAWN_ENDGAME, KNIGHT_ENDGAME, BISHOP_ENDGAME, ROOK_ENDGAME, QUEEN_ENDGAME, KING_ENDGAME],
        }
    }

    /// Gets the piece-square table value for a given piece, square, and phase
    #[inline(always)]
    pub fn get_value(&self, phase: Phase, piece: Piece, square: Square) -> i32 {
        let piece_type = piece.get_type();
        let color = piece.get_color();
        let table = match phase {
            Phase::Opening => &self.openings[piece_type.to_index()],
            Phase::Endgame => &self.endgames[piece_type.to_index()],
        };

        match color {
            Color::White => table[square.flip()],
            Color::Black => table[square],
        }
    }
}

// Piece-square tables for the opening phase
#[rustfmt::skip]
pub const PAWN_OPENING: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     0, -5,-10,  0,  0,-10, -5,  0,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
pub const KNIGHT_OPENING: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

#[rustfmt::skip]
pub const BISHOP_OPENING: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20
];

#[rustfmt::skip]
pub const ROOK_OPENING: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     5,  0,  0,  0,  0,  0,  0,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0
];

#[rustfmt::skip]
pub const QUEEN_OPENING: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[rustfmt::skip]
pub const KING_OPENING: [i32; 64] = [
     20, 30, 10,  0,  0, 10, 30, 20,
     20, 20,  0,  0,  0,  0, 20, 20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30
];

// Piece-square tables for the endgame phase
#[rustfmt::skip]
pub const PAWN_ENDGAME: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    80, 80, 80, 80, 80, 80, 80, 80,
    60, 60, 60, 60, 60, 60, 60, 60,
    40, 40, 40, 40, 40, 40, 40, 40,
    20, 20, 20, 20, 20, 20, 20, 20,
     0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
pub const KNIGHT_ENDGAME: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 20, 25, 25, 20,  0,-30,
    -30,  5, 25, 30, 30, 25,  5,-30,
    -30,  0, 25, 30, 30, 25,  0,-30,
    -30,  5, 20, 25, 25, 20,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

#[rustfmt::skip]
pub const BISHOP_ENDGAME: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0, 10, 15, 15, 10,  0,-10,
    -10,  5, 15, 20, 20, 15,  5,-10,
    -10,  0, 15, 20, 20, 15,  0,-10,
    -10, 10, 15, 15, 15, 15, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20
];

#[rustfmt::skip]
pub const ROOK_ENDGAME: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0
];

#[rustfmt::skip]
pub const QUEEN_ENDGAME: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[rustfmt::skip]
pub const KING_ENDGAME: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_square_table_creation() {
        let pst = PieceSquareTable::new();
        assert_eq!(pst.openings.len(), 6);
        assert_eq!(pst.endgames.len(), 6);
    }

    #[test]
    fn test_piece_square_table_values() {
        let pst = PieceSquareTable::new();

        // Test white pawn in opening
        assert_eq!(pst.get_value(Phase::Opening, Piece::WPawn, Square::A2), 50);

        // Test black pawn in opening
        assert_eq!(pst.get_value(Phase::Opening, Piece::BPawn, Square::A7), 50);

        // Test white king in endgame
        assert_eq!(pst.get_value(Phase::Endgame, Piece::WKing, Square::E4), 40);

        // Test black king in endgame
        assert_eq!(pst.get_value(Phase::Endgame, Piece::BKing, Square::E5), 40);
    }
}
