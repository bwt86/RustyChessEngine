use crate::core::{
    attack_pregen::PregenAttacks,
    bitboard::{FILE_A_BB, FILE_H_BB, RANK_1_BB, RANK_3_BB, RANK_6_BB, RANK_8_BB},
    board_state::*,
    piece::*,
    square::Rank,
};

use super::move_encode::Move;

/// Generates all pseudo-legal moves for the current position
///
/// # Arguments
/// * `board_state` - The current board state
/// * `pregen_attacks` - Pre-generated attack tables
/// * `moves` - Vector to store the generated moves
#[inline(always)]
pub fn get_pseudo_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    moves.clear(); // Clear existing moves
    moves.reserve(256); // Reserve space for typical number of moves

    gen_pawn_moves(board_state, pregen_attacks, moves);
    gen_knight_moves(board_state, pregen_attacks, moves);
    gen_bishop_moves(board_state, pregen_attacks, moves);
    gen_rook_moves(board_state, pregen_attacks, moves);
    gen_queen_moves(board_state, pregen_attacks, moves);
    gen_king_moves(board_state, pregen_attacks, moves);
}

/// Generates all pseudo-legal pawn moves
#[inline(always)]
fn gen_pawn_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let opposite_side = side.opposite();
    let piece = Piece::new(side, PieceType::Pawn);
    let enemy_pieces = board_state.get_position_bb(opposite_side);
    let empty_sqs = board_state.get_combined_bb().invert();
    let enpas = board_state.get_en_passant();

    for &pawn_sq in board_state.get_piece_squares(piece) {
        let pawn_sq_bb = pawn_sq.to_bitboard();
        let (single_move_forward, double_move_forward) = match side {
            Color::White => {
                let single = pawn_sq_bb.intersect(RANK_8_BB.invert()).shift_up(1).intersect(empty_sqs);
                let double = single.intersect(RANK_3_BB).shift_up(1).intersect(empty_sqs);
                (single, double)
            }
            Color::Black => {
                let single = pawn_sq_bb.intersect(RANK_1_BB.invert()).shift_down(1).intersect(empty_sqs);
                let double = single.intersect(RANK_6_BB).shift_down(1).intersect(empty_sqs);
                (single, double)
            }
        };

        // Handle en passant
        if let Some(enpas) = enpas {
            let enpas_attack = pregen_attacks.get_pawn_attacks(opposite_side, enpas).intersect(pawn_sq_bb);
            if !enpas_attack.is_empty() {
                moves.push(Move::new(
                    pawn_sq,
                    enpas,
                    piece,
                    Some(Piece::new(opposite_side, PieceType::Pawn)),
                    None,
                    false,
                    true,
                    false,
                ));
            }
        }

        // Handle single moves and promotions
        if !single_move_forward.is_empty() {
            let sq = single_move_forward.get_ls_square();
            if sq.get_rank() == Rank::R8 || sq.get_rank() == Rank::R1 {
                // Generate all promotion moves
                for promo_piece in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                    moves.push(Move::new(
                        pawn_sq,
                        sq,
                        piece,
                        None,
                        Some(Piece::new(side, promo_piece)),
                        false,
                        false,
                        false,
                    ));
                }
            } else {
                moves.push(Move::new(pawn_sq, sq, piece, None, None, false, false, false));
            }
        }

        // Handle double moves
        if !double_move_forward.is_empty() {
            moves.push(Move::new(
                pawn_sq,
                double_move_forward.get_ls_square(),
                piece,
                None,
                None,
                true,
                false,
                false,
            ));
        }

        // Handle captures
        let attacks = pregen_attacks.get_pawn_attacks(side, pawn_sq).intersect(enemy_pieces);
        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            if capture.is_some_and(|c| c.get_type() == PieceType::King) {
                continue;
            }

            let sq = att_sq;
            if sq.get_rank() == Rank::R8 || sq.get_rank() == Rank::R1 {
                // Generate all promotion captures
                for promo_piece in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                    moves.push(Move::new(
                        pawn_sq,
                        sq,
                        piece,
                        capture,
                        Some(Piece::new(side, promo_piece)),
                        false,
                        false,
                        false,
                    ));
                }
            } else {
                moves.push(Move::new(pawn_sq, sq, piece, capture, None, false, false, false));
            }
        }
    }
}

/// Generates all pseudo-legal knight moves
#[inline(always)]
fn gen_knight_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Knight);
    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    for &knight_sq in board_state.get_piece_squares(piece) {
        let knight_moves = pregen_attacks.get_knight_attacks(knight_sq);
        let attacks = knight_moves.intersect(enemy_pieces);
        let non_attacks = knight_moves.intersect(empty_sqs);

        // Handle captures
        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            if capture.is_some_and(|c| c.get_type() == PieceType::King) {
                continue;
            }
            moves.push(Move::new(knight_sq, att_sq, piece, capture, None, false, false, false));
        }

        // Handle non-captures
        for sq in non_attacks.get_occupied_squares() {
            moves.push(Move::new(knight_sq, sq, piece, None, None, false, false, false));
        }
    }
}

/// Generates all pseudo-legal bishop moves
#[inline(always)]
fn gen_bishop_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Bishop);
    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    for &bishop_sq in board_state.get_piece_squares(piece) {
        let bishop_moves = pregen_attacks.get_bishop_attacks(bishop_sq, &board_state.get_combined_bb());
        let attacks = bishop_moves.intersect(enemy_pieces);
        let non_attacks = bishop_moves.intersect(empty_sqs);

        // Handle captures
        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            if capture.is_some_and(|c| c.get_type() == PieceType::King) {
                continue;
            }
            moves.push(Move::new(bishop_sq, att_sq, piece, capture, None, false, false, false));
        }

        // Handle non-captures
        for sq in non_attacks.get_occupied_squares() {
            moves.push(Move::new(bishop_sq, sq, piece, None, None, false, false, false));
        }
    }
}

/// Generates all pseudo-legal rook moves
#[inline(always)]
fn gen_rook_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Rook);
    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    for &rook_sq in board_state.get_piece_squares(piece) {
        let rook_moves = pregen_attacks.get_rook_attacks(rook_sq, &board_state.get_combined_bb());
        let attacks = rook_moves.intersect(enemy_pieces);
        let non_attacks = rook_moves.intersect(empty_sqs);

        // Handle captures
        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            if capture.is_some_and(|c| c.get_type() == PieceType::King) {
                continue;
            }
            moves.push(Move::new(rook_sq, att_sq, piece, capture, None, false, false, false));
        }

        // Handle non-captures
        for sq in non_attacks.get_occupied_squares() {
            moves.push(Move::new(rook_sq, sq, piece, None, None, false, false, false));
        }
    }
}

/// Generates all pseudo-legal queen moves
#[inline(always)]
fn gen_queen_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Queen);
    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    for &queen_sq in board_state.get_piece_squares(piece) {
        let queen_moves = pregen_attacks.get_queen_attacks(queen_sq, &board_state.get_combined_bb());
        let attacks = queen_moves.intersect(enemy_pieces);
        let non_attacks = queen_moves.intersect(empty_sqs);

        // Handle captures
        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            if capture.is_some_and(|c| c.get_type() == PieceType::King) {
                continue;
            }
            moves.push(Move::new(queen_sq, att_sq, piece, capture, None, false, false, false));
        }

        // Handle non-captures
        for sq in non_attacks.get_occupied_squares() {
            moves.push(Move::new(queen_sq, sq, piece, None, None, false, false, false));
        }
    }
}

/// Generates all pseudo-legal king moves
#[inline(always)]
fn gen_king_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::King);
    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    let king_sq = board_state.get_piece_squares(piece)[0];
    let king_attacks = pregen_attacks.get_king_attacks(king_sq);
    let attacks = king_attacks.intersect(enemy_pieces);
    let non_attacks = king_attacks.intersect(empty_sqs);

    // Handle captures
    for att_sq in attacks.get_occupied_squares() {
        let capture = board_state.get_piece_on_square(att_sq);
        if capture.is_some_and(|c| c.get_type() == PieceType::King) {
            continue;
        }
        moves.push(Move::new(king_sq, att_sq, piece, capture, None, false, false, false));
    }

    // Handle non-captures
    for sq in non_attacks.get_occupied_squares() {
        moves.push(Move::new(king_sq, sq, piece, None, None, false, false, false));
    }

    // Handle castling
    let castle_rights = board_state.get_castling_rights();
    let castle_shift = 2 * side as u8;

    // King-side castling
    if castle_rights & ((CastlePerms::WKC as u8) << castle_shift) != 0 {
        let king_ray = pregen_attacks.get_rook_attacks(king_sq, &board_state.get_combined_bb());
        let rook_ks = board_state.get_piece_bb(Piece::new(side, PieceType::Rook)).intersect(FILE_H_BB);

        if !king_ray.intersect(rook_ks).is_empty() && !board_state.is_check(side, pregen_attacks) {
            moves.push(Move::new(king_sq, king_sq.move_right(2), piece, None, None, false, false, true));
        }
    }

    // Queen-side castling
    if castle_rights & ((CastlePerms::WQC as u8) << castle_shift) != 0 {
        let king_ray = pregen_attacks.get_rook_attacks(king_sq, &board_state.get_combined_bb());
        let rook_qs = board_state.get_piece_bb(Piece::new(side, PieceType::Rook)).intersect(FILE_A_BB);

        if !king_ray.intersect(rook_qs).is_empty() && !board_state.is_check(side, pregen_attacks) {
            moves.push(Move::new(king_sq, king_sq.move_left(2), piece, None, None, false, false, true));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::zobrist::ZobristHasher;

    fn setup_board(fen: &str) -> (BoardState, PregenAttacks) {
        let zobrist = ZobristHasher::new();
        let board_state = BoardState::new(Some(fen), &zobrist).unwrap();
        let pregen_attacks = PregenAttacks::init();
        (board_state, pregen_attacks)
    }

    #[test]
    fn test_initial_position() {
        let (board_state, pregen_attacks) = setup_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 20); // Initial position has 20 legal moves
    }

    #[test]
    fn test_pawn_moves() {
        // Test pawn pushes
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/8/8/PPPPPPPP/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 16); // 8 pawns * 2 moves each

        // Test pawn captures
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/8/1p6/1P6/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 3); // One push and two captures

        // Test pawn promotions
        let (board_state, pregen_attacks) = setup_board("8/P7/8/8/8/8/8/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 4); // Four promotion options
    }

    #[test]
    fn test_en_passant() {
        let (board_state, pregen_attacks) = setup_board("8/8/8/3pP3/8/8/8/8 w - d6 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert!(moves.iter().any(|m| m.is_en_passant()));
    }

    #[test]
    fn test_castling() {
        // Test white kingside castling
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/8/8/8/R3K2R w KQkq - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert!(moves.iter().any(|m| m.is_castling()));

        // Test black queenside castling
        let (board_state, pregen_attacks) = setup_board("r3k2r/8/8/8/8/8/8/8 b KQkq - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert!(moves.iter().any(|m| m.is_castling()));
    }

    #[test]
    fn test_knight_moves() {
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/4N3/8/8/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 8); // Knight in center has 8 moves
    }

    #[test]
    fn test_bishop_moves() {
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/4B3/8/8/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 13); // Bishop in center has 13 moves
    }

    #[test]
    fn test_rook_moves() {
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/4R3/8/8/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 14); // Rook in center has 14 moves
    }

    #[test]
    fn test_queen_moves() {
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/4Q3/8/8/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 27); // Queen in center has 27 moves
    }

    #[test]
    fn test_king_moves() {
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/4K3/8/8/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert_eq!(moves.len(), 8); // King in center has 8 moves
    }

    #[test]
    fn test_pinned_pieces() {
        // Test pinned piece (knight) that can't move
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/8/8/8/R1B1K2R w KQkq - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert!(moves.iter().all(|m| !m.get_piece().is_knight()));
    }

    #[test]
    fn test_check_evasion() {
        // Test position where king is in check
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/8/8/8/R1B1K2R w KQkq - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert!(moves.iter().any(|m| m.get_piece().is_king()));
    }

    #[test]
    fn test_promotion_captures() {
        let (board_state, pregen_attacks) = setup_board("8/P7/8/8/8/8/8/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert!(moves.iter().all(|m| m.get_promotion().is_some()));
    }

    #[test]
    fn test_double_pawn_push() {
        let (board_state, pregen_attacks) = setup_board("8/8/8/8/8/8/PPPPPPPP/8 w - - 0 1");
        let mut moves = Vec::new();
        get_pseudo_moves(&board_state, &pregen_attacks, &mut moves);
        assert!(moves.iter().any(|m| m.is_double_pawn_push()));
    }
}
