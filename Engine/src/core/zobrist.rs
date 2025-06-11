use super::{
    piece::{Color, Piece, PieceType},
    square::{Square, SQUARES},
};
use rand::Rng;

/// A Zobrist hashing implementation for chess positions.
///
/// This struct maintains random 64-bit numbers for:
/// - Each piece type and color on each square (12 pieces * 64 squares)
/// - Each pawn position (2 colors * 64 squares)
/// - Each possible en passant file (8 files)
/// - Each castling right (4 rights)
/// - Side to move
#[derive(Clone)]
pub struct ZobristHasher {
    pieces: [u64; 768], // 12 pieces * 64 squares
    pawns: [u64; 128],  // 2 colors * 64 squares
    en_passant: [u64; 8],
    castling_rights: [u64; 4],
    turn: u64,
}

impl ZobristHasher {
    /// Creates a new Zobrist hasher with randomly initialized values.
    #[inline]
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut hasher = Self {
            pieces: [0; 768],
            pawns: [0; 128],
            en_passant: [0; 8],
            castling_rights: [0; 4],
            turn: rng.gen(),
        };

        // Initialize all arrays with random values
        hasher.pieces.iter_mut().for_each(|x| *x = rng.gen());
        hasher.pawns.iter_mut().for_each(|x| *x = rng.gen());
        hasher.en_passant.iter_mut().for_each(|x| *x = rng.gen());
        hasher.castling_rights.iter_mut().for_each(|x| *x = rng.gen());

        hasher
    }

    /// Initializes a hash for a given board position.
    ///
    /// # Arguments
    /// * `board` - The current board state
    /// * `side` - The side to move
    /// * `en_passant` - The en passant square, if any
    /// * `castling_rights` - The current castling rights
    #[inline]
    pub fn init_hash(&self, board: &[Option<Piece>; 64], side: Color, en_passant: Option<Square>, castling_rights: u8) -> u64 {
        let mut hash = 0;

        // Hash pieces
        for sq in SQUARES {
            if let Some(piece) = board[sq] {
                hash ^= self.pieces[64 * piece.to_index() + sq.to_index()];
            }
        }

        // Hash side to move
        if side == Color::White {
            hash ^= self.turn;
        }

        // Hash en passant
        if let Some(ep_square) = en_passant {
            hash ^= self.en_passant[ep_square.get_file() as usize];
        }

        // Hash castling rights
        for i in 0..4 {
            if (castling_rights >> i) & 1 != 0 {
                hash ^= self.castling_rights[i];
            }
        }

        hash
    }

    /// Initializes a hash specifically for pawn positions.
    /// This is used for pawn structure evaluation.
    #[inline]
    pub fn init_hash_pawns(&self, board: &[Option<Piece>; 64]) -> u64 {
        let mut hash = 0;

        for sq in SQUARES {
            if let Some(piece) = board[sq] {
                if piece.get_type() == PieceType::Pawn {
                    hash ^= self.pawns[64 * piece.get_color().to_index() + sq.to_index()];
                }
            }
        }

        hash
    }

    /// Updates the Zobrist hash for a move.
    #[inline]
    pub fn update_zobrist_hash_move(&self, zobrist_hash: &mut u64, pawn_hash: &mut u64, piece: Piece, from: Square, to: Square) {
        let piece_idx = piece.to_index();
        let from_idx = from.to_index();
        let to_idx = to.to_index();

        // Update piece hash
        *zobrist_hash ^= self.pieces[64 * piece_idx + from_idx];
        *zobrist_hash ^= self.pieces[64 * piece_idx + to_idx];
        *zobrist_hash ^= self.turn;

        // Update pawn hash if needed
        if piece.get_type() == PieceType::Pawn {
            let color_idx = piece.get_color().to_index();
            *pawn_hash ^= self.pawns[64 * color_idx + from_idx];
            *pawn_hash ^= self.pawns[64 * color_idx + to_idx];
        }
    }

    /// Updates the Zobrist hash for a capture.
    #[inline]
    pub fn update_zobrist_hash_capture(&self, zobrist_hash: &mut u64, pawn_hash: &mut u64, piece: Piece, square: Square) {
        let piece_idx = piece.to_index();
        let sq_idx = square.to_index();

        *zobrist_hash ^= self.pieces[64 * piece_idx + sq_idx];

        if piece.get_type() == PieceType::Pawn {
            *pawn_hash ^= self.pawns[64 * piece.get_color().to_index() + sq_idx];
        }
    }

    /// Updates the Zobrist hash for a promotion.
    #[inline]
    pub fn update_zobrist_hash_promotion(&self, zobrist_hash: &mut u64, pawn_hash: &mut u64, pawn: Piece, promoted: Piece, square: Square) {
        let sq_idx = square.to_index();
        let color_idx = pawn.get_color().to_index();

        // Remove pawn
        *zobrist_hash ^= self.pieces[64 * pawn.to_index() + sq_idx];
        *pawn_hash ^= self.pawns[64 * color_idx + sq_idx];

        // Add promoted piece
        *zobrist_hash ^= self.pieces[64 * promoted.to_index() + sq_idx];
    }

    /// Updates the Zobrist hash for en passant changes.
    #[inline]
    pub fn update_zobrist_hash_en_passant(&self, zobrist_hash: &mut u64, old_sq: Option<Square>, new_sq: Option<Square>) {
        if let Some(sq) = old_sq {
            *zobrist_hash ^= self.en_passant[sq.get_file() as usize];
        }
        if let Some(sq) = new_sq {
            *zobrist_hash ^= self.en_passant[sq.get_file() as usize];
        }
    }

    /// Updates the Zobrist hash for castling rights changes.
    #[inline]
    pub fn update_zobrist_hash_castling(&self, zobrist_hash: &mut u64, old_rights: u8, new_rights: u8) {
        let changed = old_rights ^ new_rights;
        for i in 0..4 {
            if (changed >> i) & 1 != 0 {
                *zobrist_hash ^= self.castling_rights[i];
            }
        }
    }

    /// Updates the Zobrist hash for side to move changes.
    #[inline]
    pub fn update_zobrist_hash_side(&self, zobrist_hash: &mut u64) {
        *zobrist_hash ^= self.turn;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zobrist_initialization() {
        let hasher = ZobristHasher::new();
        assert_ne!(hasher.turn, 0);
        assert!(hasher.pieces.iter().all(|&x| x != 0));
        assert!(hasher.pawns.iter().all(|&x| x != 0));
        assert!(hasher.en_passant.iter().all(|&x| x != 0));
        assert!(hasher.castling_rights.iter().all(|&x| x != 0));
    }

    #[test]
    fn test_hash_consistency() {
        let hasher = ZobristHasher::new();
        let board = [None; 64];
        let hash1 = hasher.init_hash(&board, Color::White, None, 0);
        let hash2 = hasher.init_hash(&board, Color::White, None, 0);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_difference() {
        let hasher = ZobristHasher::new();
        let mut board1 = [None; 64];
        let mut board2 = [None; 64];
        board1[0] = Some(Piece::WPawn);
        board2[0] = Some(Piece::BPawn);

        let hash1 = hasher.init_hash(&board1, Color::White, None, 0);
        let hash2 = hasher.init_hash(&board2, Color::White, None, 0);
        assert_ne!(hash1, hash2);
    }
}
