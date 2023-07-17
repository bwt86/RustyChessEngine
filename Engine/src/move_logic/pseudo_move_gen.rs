use crate::core::{
    attack_pregen::PregenAttacks,
    bitboard::{Bitboard, FILE_A_BB, FILE_H_BB, RANK_1_BB, RANK_3_BB, RANK_6_BB, RANK_8_BB},
    board_state::*,
    piece::*,
    square::Rank,
};

use super::move_encode::Move;

pub fn get_pseudo_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, sudo_moves: &mut Vec<Move>) {
    gen_pawn_moves(board_state, pregen_attacks, sudo_moves);
    gen_knight_moves(board_state, pregen_attacks, sudo_moves);
    gen_bishop_moves(board_state, pregen_attacks, sudo_moves);
    gen_rook_moves(board_state, pregen_attacks, sudo_moves);
    gen_queen_moves(board_state, pregen_attacks, sudo_moves);
    gen_king_moves(board_state, pregen_attacks, sudo_moves);
}

fn gen_pawn_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Pawn);

    let pawns = board_state.get_piece_bb(piece);
    let enemy_pieces = board_state.get_position_bb(side.opposite());

    let empty_sqs = board_state.get_combined_bb().invert();
    let enpas = board_state.get_en_passant();

    for pawn_sq in board_state.get_piece_squares(piece) {
        let single_move_forward;
        let double_move_forward;

        if side == Color::White {
            single_move_forward = pawn_sq.to_bitboard().intersect(RANK_8_BB.invert()).shift_up(1).intersect(empty_sqs);
            double_move_forward = single_move_forward.intersect(RANK_3_BB).shift_up(1).intersect(empty_sqs);
        } else {
            single_move_forward = pawn_sq.to_bitboard().intersect(RANK_1_BB.invert()).shift_down(1).intersect(empty_sqs);
            double_move_forward = single_move_forward.intersect(RANK_6_BB).shift_down(1).intersect(empty_sqs);
        }

        if let Some(enpas) = enpas {
            let enpas_attack = pregen_attacks.get_pawn_attacks(side.opposite(), enpas).intersect(pawn_sq.to_bitboard());
            if !enpas_attack.is_empty() {
                let m = Move::new(
                    *pawn_sq,
                    enpas,
                    piece,
                    Some(Piece::new(side.opposite(), PieceType::Pawn)),
                    None,
                    false,
                    true,
                    false,
                );
                moves.push(m);
            }
        }

        if !single_move_forward.is_empty() {
            let sq = single_move_forward.get_ls_square();
            if sq.get_rank() == Rank::R8 || sq.get_rank() == Rank::R1 {
                let m = Move::new(*pawn_sq, sq, piece, None, Some(Piece::new(side, PieceType::Queen)), false, false, false);
                moves.push(m);

                let m = Move::new(*pawn_sq, sq, piece, None, Some(Piece::new(side, PieceType::Rook)), false, false, false);
                moves.push(m);

                let m = Move::new(*pawn_sq, sq, piece, None, Some(Piece::new(side, PieceType::Bishop)), false, false, false);
                moves.push(m);

                let m = Move::new(*pawn_sq, sq, piece, None, Some(Piece::new(side, PieceType::Knight)), false, false, false);
                moves.push(m);
            } else {
                let m = Move::new(*pawn_sq, sq, piece, None, None, false, false, false);
                moves.push(m);
            }
        }

        if !double_move_forward.is_empty() {
            let m = Move::new(*pawn_sq, double_move_forward.get_ls_square(), piece, None, None, true, false, false);
            moves.push(m);
        }

        let attacks = pregen_attacks.get_pawn_attacks(side, *pawn_sq).intersect(enemy_pieces);
        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            let m = Move::new(*pawn_sq, att_sq, piece, capture, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_knight_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Knight);

    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();
    let knights = board_state.get_piece_bb(piece);

    for knight_sq in board_state.get_piece_squares(piece) {
        let knight_moves = pregen_attacks.get_knight_attacks(*knight_sq);

        let attacks = knight_moves.intersect(enemy_pieces);
        let non_attacks: Bitboard = knight_moves.intersect(empty_sqs);

        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            let m = Move::new(*knight_sq, att_sq, piece, capture, None, false, false, false);
            moves.push(m);
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(*knight_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_king_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::King);

    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    if board_state.get_piece_squares(piece).len() != 1 {
        return;
    }

    let king_sq = board_state.get_piece_squares(piece)[0];

    let king_attack = pregen_attacks.get_king_attacks(king_sq).intersect(enemy_pieces);
    let non_attack = pregen_attacks.get_king_attacks(king_sq).intersect(empty_sqs);

    for att_sq in king_attack.get_occupied_squares() {
        let capture = board_state.get_piece_on_square(att_sq);
        let m = Move::new(king_sq, att_sq, piece, capture, None, false, false, false);
        moves.push(m);
    }

    for sq in non_attack.get_occupied_squares() {
        let m = Move::new(king_sq, sq, piece, None, None, false, false, false);
        moves.push(m);
    }

    let castle_rights = board_state.get_castling_rights();

    if castle_rights & ((CastlePerms::WKC as u8) << (2 * side as u8)) != 0 {
        let king_ray = pregen_attacks.get_rook_attacks(king_sq, &board_state.get_combined_bb());

        let rook_ks = board_state.get_piece_bb(Piece::new(side, PieceType::Rook)).intersect(FILE_H_BB);

        if !king_ray.intersect(rook_ks).is_empty() {
            let m = Move::new(king_sq, king_sq.move_right(2), piece, None, None, false, false, true);
            moves.push(m);
        }
    }

    if castle_rights & ((CastlePerms::WQC as u8) << (2 * side as u8)) != 0 {
        let king_ray = pregen_attacks.get_rook_attacks(king_sq, &board_state.get_combined_bb());

        let rook_ks = board_state.get_piece_bb(Piece::new(side, PieceType::Rook)).intersect(FILE_A_BB);
        if !king_ray.intersect(rook_ks).is_empty() {
            let m = Move::new(king_sq, king_sq.move_left(2), piece, None, None, false, false, true);
            moves.push(m);
        }
    }
}

fn gen_bishop_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Bishop);

    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    for bishop_sq in board_state.get_piece_squares(piece) {
        let bis_moves = pregen_attacks.get_bishop_attacks(*bishop_sq, &board_state.get_combined_bb());

        let attacks = bis_moves.intersect(enemy_pieces);

        let non_attacks = bis_moves.intersect(empty_sqs);

        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            let m = Move::new(*bishop_sq, att_sq, piece, capture, None, false, false, false);
            moves.push(m);
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(*bishop_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_rook_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Rook);

    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    for rook_sq in board_state.get_piece_squares(piece) {
        let rook_moves = pregen_attacks.get_rook_attacks(*rook_sq, &board_state.get_combined_bb());

        let attacks = rook_moves.intersect(enemy_pieces);

        let non_attacks = rook_moves.intersect(empty_sqs);

        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            let m = Move::new(*rook_sq, att_sq, piece, capture, None, false, false, false);
            moves.push(m);
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(*rook_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_queen_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let side = board_state.get_side();
    let piece = Piece::new(side, PieceType::Queen);

    let enemy_pieces = board_state.get_position_bb(side.opposite());
    let empty_sqs = board_state.get_combined_bb().invert();

    for queen_sq in board_state.get_piece_squares(piece) {
        let queen_moves = pregen_attacks.get_queen_attacks(*queen_sq, &board_state.get_combined_bb());

        let attacks = queen_moves.intersect(enemy_pieces);

        let non_attacks = queen_moves.intersect(empty_sqs);

        for att_sq in attacks.get_occupied_squares() {
            let capture = board_state.get_piece_on_square(att_sq);
            let m = Move::new(*queen_sq, att_sq, piece, capture, None, false, false, false);
            moves.push(m);
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(*queen_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}
