use super::{
    piece::{Color, Piece, PieceType},
    square::{Square, SQUARES},
};
use rand::Rng;
pub struct ZobristHasher {
    pieces: [u64; 768], // 12 pieces * 64 squares
    pawns: [u64; 128],  // 2 colors * 64 squares
    en_passant: [u64; 8],
    castling_rights: [u64; 4],
    turn: u64,
}

impl ZobristHasher {
    pub fn new() -> ZobristHasher {
        let mut zobrist = ZobristHasher {
            pieces: [0; 768],
            pawns: [0; 128],
            en_passant: [0; 8],
            castling_rights: [0; 4],
            turn: 0,
        };
        zobrist.initialize();
        zobrist
    }

    fn initialize(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..768 {
            self.pieces[i] = rng.gen::<u64>();
        }

        for i in 0..128 {
            self.pawns[i] = rng.gen::<u64>();
        }

        for i in 0..8 {
            self.en_passant[i] = rng.gen::<u64>();
        }

        for i in 0..4 {
            self.castling_rights[i] = rng.gen::<u64>();
        }

        self.turn = rng.gen::<u64>();
    }

    pub fn init_hash(&self, board: &[Option<Piece>; 64], side: Color, en_passant: Option<Square>, castling_rights: u8) -> u64 {
        let mut hash = 0;

        for sq in SQUARES {
            if let Some(piece) = board[sq] {
                let piece_index = piece.to_index();
                let sq_index = sq.to_index();
                let index = 64 * piece_index + sq_index;
                hash ^= self.pieces[index];
            }
        }

        // Hash the side to move.
        if side == Color::White {
            hash ^= self.turn;
        }

        // Hash the en passant square, if any.
        if let Some(ep_square) = en_passant {
            let ep_file = ep_square.get_file() as usize;
            hash ^= self.en_passant[ep_file];
        }

        // Hash the castling rights.
        let castling_rights = castling_rights;
        for i in 0..4 {
            if (castling_rights >> i) & 1 != 0 {
                hash ^= self.castling_rights[i];
            }
        }

        hash
    }

    pub fn init_hash_pawns(&self, board: &[Option<Piece>; 64]) -> u64 {
        let mut hash = 0;

        for sq in SQUARES {
            if let Some(piece) = board[sq] {
                if piece.get_type() == PieceType::Pawn {
                    let color = piece.get_color() as usize;
                    let sq_index = sq.to_index();
                    let index = 64 * color + sq_index;
                    hash ^= self.pawns[index];
                }
            }
        }

        hash
    }

    pub fn update_zobrist_hash_move(&self, zobrist_hash: &mut u64, pawn_hash: &mut u64, piece: Piece, from: Square, to: Square) {
        // Update the hash for the moving piece
        *zobrist_hash ^= self.pieces[64 * piece.to_index() + from.to_index()];
        *zobrist_hash ^= self.pieces[64 * piece.to_index() + to.to_index()];

        // Update the turn hash
        *zobrist_hash ^= self.turn;

        // If the moving piece is a pawn, update the pawn hash too
        if piece.get_type() == PieceType::Pawn {
            *pawn_hash ^= self.pawns[64 * piece.get_color().to_index() + from.to_index()];
            *pawn_hash ^= self.pawns[64 * piece.get_color().to_index() + to.to_index()];
        }
    }

    pub fn update_zobrist_hash_capture(&self, zobrist_hash: &mut u64, pawn_hash: &mut u64, piece: Piece, square: Square) {
        // Update the hash for the captured piece
        *zobrist_hash ^= self.pieces[64 * piece.to_index() + square.to_index()];

        // If the captured piece is a pawn, update the pawn hash too
        if piece.get_type() == PieceType::Pawn {
            *pawn_hash ^= self.pawns[64 * piece.get_color().to_index() + square.to_index()];
        }
    }

    pub fn update_zobrist_hash_promotion(&self, zobrist_hash: &mut u64, pawn_hash: &mut u64, pawn: Piece, promoted: Piece, square: Square) {
        // Remove the pawn from the square
        *zobrist_hash ^= self.pieces[64 * pawn.to_index() + square.to_index()];
        *pawn_hash ^= self.pawns[64 * pawn.get_color().to_index() + square.to_index()];

        // Add the new promoted piece to the square
        *zobrist_hash ^= self.pieces[64 * promoted.to_index() + square.to_index()];
    }

    pub fn update_zobrist_hash_en_passant(&self, zobrist_hash: &mut u64, old_sq: Option<Square>, new_sq: Option<Square>) {
        // Update the hash for the old en passant file
        if let Some(sq) = old_sq {
            *zobrist_hash ^= self.en_passant[sq.get_file()];
        }

        // Update the hash for the new en passant file
        if let Some(sq) = new_sq {
            *zobrist_hash ^= self.en_passant[sq.get_file()];
        }
    }

    pub fn update_zobrist_hash_castling(&self, zobrist_hash: &mut u64, old_rights: u8, new_rights: u8) {
        // Update the hash for the changed castling rights
        for i in 0..4 {
            if (old_rights >> i & 1) != (new_rights >> i & 1) {
                *zobrist_hash ^= self.castling_rights[i];
            }
        }
    }
}
