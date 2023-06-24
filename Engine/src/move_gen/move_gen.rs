use std::ops::Deref;

use crate::core::{
    attack_pregen::{self, PregenAttacks},
    bitboard::{Bitboard, RANK_2_BB, RANK_3_BB, RANK_6_BB},
    board_state::*,
    piece::*,
};

use super::move_encode::Move;

fn is_legal(sudo_move: &Move, board_state: &BoardState, pregen_attacks: &PregenAttacks) -> bool {
    check_check(sudo_move, board_state, pregen_attacks)
}

fn check_check(sudo_move: &Move, board_state: &BoardState, pregen_attacks: &PregenAttacks) -> bool {
    let side = *board_state.get_side();
    let piece_moved = sudo_move.get_piece();
    let from = sudo_move.get_from();
    let to = sudo_move.get_to();

    let mut both_bb = *board_state.get_position_bb(Color::Both);
    let mut side_bb = *board_state.get_position_bb(side);
    let op_side_bb = *board_state.get_position_bb(side.get_opposite());
    let mut king_bb = *board_state.get_piece_bb(Piece::from_type(PieceType::King, side));

    both_bb.make_move(from, to);
    side_bb.make_move(from, to);
    if piece_moved == Piece::from_type(PieceType::King, side) {
        king_bb.make_move(from, to)
    }

    let king_square = king_bb.get_ls_square();

    let king_ray = pregen_attacks.get_queen_attacks(king_square, both_bb);
    if !king_ray.combine(op_side_bb).is_empty() {
        for piece in PIECE_TYPES {
            for sq in board_state
                .get_piece_bb(Piece::from_type(piece, side.get_opposite()))
                .get_occupied_squares()
            {
                let piece_attack =
                    pregen_attacks.get_piece_attacks(piece, side.get_opposite(), sq, both_bb);

                if !king_bb.combine(piece_attack).is_empty() {
                    return false;
                }
            }
        }
    }
    true
}

pub fn gen_pawn_moves(
    board_state: &BoardState,
    pregen_attacks: &PregenAttacks,
    color: Color,
) -> Vec<Move> {
    let mut moves = Vec::new();
    let piece = Piece::from_type(PieceType::Pawn, color);
    let enemy_piece = Piece::from_type(PieceType::Pawn, color.get_opposite());

    let pawns = board_state.get_piece_bb(piece).clone();
    let enemy_pieces = board_state.get_position_bb(color.get_opposite());
    let empty_sqs = board_state.get_position_bb(Color::Both).invert();
    let enpas = board_state.get_enpas();
    for pawn_sq in pawns.get_occupied_squares() {
        let mut single_moves_forward = Bitboard::new_empty();
        let mut double_moves_forward = Bitboard::new_empty();

        if color == Color::White {
            single_moves_forward = pawn_sq.move_up(1).to_bitboard().intersect(empty_sqs);
            double_moves_forward = (single_moves_forward & RANK_3_BB)
                .move_up(1)
                .combine(empty_sqs);
        } else {
            single_moves_forward = pawn_sq.move_down(1).to_bitboard().combine(empty_sqs);
            double_moves_forward = (single_moves_forward & RANK_6_BB)
                .move_down(1)
                .combine(empty_sqs);
        }

        if enpas.is_some() {
            let enpas_attack = pregen_attacks
                .get_pawn_attacks(color.get_opposite(), enpas.unwrap())
                & pawn_sq.to_bitboard();
            if !enpas_attack.is_empty() {
                let m = Move::new(
                    pawn_sq,
                    enpas.unwrap(),
                    piece,
                    Some(enemy_piece),
                    None,
                    false,
                    true,
                    false,
                );
                if is_legal(&m, &board_state, &pregen_attacks) {
                    moves.push(m);
                }
            }
        }

        if !single_moves_forward.is_empty() {
            let m = Move::new(
                pawn_sq,
                single_moves_forward.get_ls_square(),
                piece,
                None,
                None,
                false,
                false,
                false,
            );
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }

        if !double_moves_forward.is_empty() {
            let m = Move::new(
                pawn_sq,
                double_moves_forward.get_ls_square(),
                piece,
                None,
                None,
                true,
                false,
                false,
            );
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }

        let attacks = pregen_attacks
            .get_pawn_attacks(color, pawn_sq)
            .intersect(*enemy_pieces);
        for att_sq in attacks.get_occupied_squares() {
            let m = Move::new(pawn_sq, att_sq, piece, None, None, false, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }
    }

    moves
}
