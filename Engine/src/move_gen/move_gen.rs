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
    let piece_moved = sudo_move.get_piece();
    let from = sudo_move.get_from();
    let to = sudo_move.get_to();
    let side = piece_moved.get_color();

    let mut both_bb = *board_state.get_position_bb(Color::Both);
    let mut self_bb = *board_state.get_position_bb(side);
    let enemy_bb = *board_state.get_position_bb(side.get_opposite());
    let mut king_bb = *board_state.get_piece_bb(Piece::from_type(PieceType::King, side));

    both_bb.make_move(from, to);
    self_bb.make_move(from, to);
    if piece_moved == Piece::from_type(PieceType::King, side) {
        king_bb.make_move(from, to)
    }

    let king_square = king_bb.get_ls_square();

    let king_ray = pregen_attacks.get_queen_attacks(king_square, &both_bb);
    if !king_ray.intersect(enemy_bb).is_empty() {
        for piece in PIECE_TYPES {
            for sq in board_state
                .get_piece_bb(Piece::from_type(piece, side.get_opposite()))
                .get_occupied_squares()
            {
                let piece_attack = pregen_attacks.get_piece_attacks(piece, side.get_opposite(), sq, &both_bb);

                if piece_attack.is_occupied(king_square) {
                    return false;
                }
            }
        }
    }
    true
}

pub fn get_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks) -> Vec<Vec<Move>> {
    let mut moves = Vec::new();

    moves.push(gen_pawn_moves(board_state, pregen_attacks));
    moves.push(gen_knight_moves(board_state, pregen_attacks));
    moves.push(gen_bishop_moves(board_state, pregen_attacks));
    moves.push(gen_rook_moves(board_state, pregen_attacks));
    moves.push(gen_queen_moves(board_state, pregen_attacks));
    moves.push(gen_king_moves(board_state, pregen_attacks));

    moves
}

pub fn gen_pawn_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks) -> Vec<Move> {
    let mut moves = Vec::new();

    let color = *board_state.get_side();
    let piece = Piece::from_type(PieceType::Pawn, color);

    let pawns = *board_state.get_piece_bb(piece);
    let enemy_pieces = *board_state.get_position_bb(color.get_opposite());
    let empty_sqs = board_state.get_position_bb(Color::Both).invert();
    let enpas = *board_state.get_enpas();

    for pawn_sq in pawns.get_occupied_squares() {
        let single_move_forward;
        let double_move_forward;

        if color == Color::White {
            single_move_forward = pawn_sq.to_bitboard().move_up(1).intersect(empty_sqs);
            double_move_forward = single_move_forward.intersect(RANK_3_BB).move_up(1).intersect(empty_sqs);
        } else {
            single_move_forward = pawn_sq.to_bitboard().move_down(1).intersect(empty_sqs);
            double_move_forward = single_move_forward.intersect(RANK_6_BB).move_down(1).intersect(empty_sqs);
        }

        if enpas.is_some() {
            let enpas_attack = pregen_attacks
                .get_pawn_attacks(color.get_opposite(), enpas.unwrap())
                .intersect(pawn_sq.to_bitboard());
            if !enpas_attack.is_empty() {
                let m = Move::new(
                    pawn_sq,
                    enpas.unwrap(),
                    piece,
                    Some(Piece::from_type(PieceType::Pawn, color.get_opposite())),
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

        if !single_move_forward.is_empty() {
            let m = Move::new(pawn_sq, single_move_forward.get_ls_square(), piece, None, None, false, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }

        if !double_move_forward.is_empty() {
            let m = Move::new(pawn_sq, double_move_forward.get_ls_square(), piece, None, None, true, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }

        let attacks = pregen_attacks.get_pawn_attacks(color, pawn_sq).intersect(enemy_pieces);
        for att_sq in attacks.get_occupied_squares() {
            let m = Move::new(pawn_sq, att_sq, piece, None, None, false, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }
    }

    moves
}

pub fn gen_knight_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = *board_state.get_side();
    let piece = Piece::from_type(PieceType::Knight, color);
    let enemy_pieces = board_state.get_position_bb(color.get_opposite());
    let knights = board_state.get_piece_bb(piece);

    for knight_sq in knights.get_occupied_squares() {
        let attacks = pregen_attacks.get_knight_attacks(knight_sq).intersect(*enemy_pieces);
        let non_attacks = pregen_attacks
            .get_knight_attacks(knight_sq)
            .intersect(board_state.get_position_bb(Color::Both).invert());

        for att_sq in attacks.get_occupied_squares() {
            for pt in PIECE_TYPES {
                if board_state.get_piece_bb(Piece::from_type(pt, color.get_opposite())).is_occupied(att_sq) {
                    let m = Move::new(
                        knight_sq,
                        att_sq,
                        piece,
                        Some(Piece::from_type(pt, color.get_opposite())),
                        None,
                        false,
                        false,
                        false,
                    );
                    if is_legal(&m, &board_state, &pregen_attacks) {
                        moves.push(m);
                    }
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(knight_sq, sq, piece, None, None, false, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }
    }

    moves
}

pub fn gen_king_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = *board_state.get_side();
    let piece = Piece::from_type(PieceType::King, color);
    let enemy_pieces = board_state.get_position_bb(color.get_opposite());

    let king = board_state.get_piece_bb(piece);
    let king_sq = king.get_ls_square();

    let king_attack = pregen_attacks.get_king_attacks(king_sq).intersect(*enemy_pieces);
    let non_attack = pregen_attacks
        .get_king_attacks(king_sq)
        .intersect(board_state.get_position_bb(Color::Both).invert());

    for att_sq in king_attack.get_occupied_squares() {
        for pt in PIECE_TYPES {
            if board_state.get_piece_bb(Piece::from_type(pt, color.get_opposite())).is_occupied(att_sq) {
                let m = Move::new(
                    king_sq,
                    att_sq,
                    piece,
                    Some(Piece::from_type(pt, color.get_opposite())),
                    None,
                    false,
                    false,
                    false,
                );
                if is_legal(&m, &board_state, &pregen_attacks) {
                    moves.push(m);
                }
            }
        }
    }

    for sq in non_attack.get_occupied_squares() {
        let m = Move::new(king_sq, sq, piece, None, None, false, false, false);
        if is_legal(&m, &board_state, &pregen_attacks) {
            moves.push(m);
        }
    }

    moves
}

pub fn gen_bishop_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = *board_state.get_side();
    let piece = Piece::from_type(PieceType::Bishop, color);

    let enemy_pieces = board_state.get_position_bb(color.get_opposite());
    let bishops = board_state.get_piece_bb(piece);

    for bishop_sq in bishops.get_occupied_squares() {
        let attacks = pregen_attacks
            .get_bishop_attacks(bishop_sq, board_state.get_position_bb(Color::Both))
            .intersect(*enemy_pieces);
        let non_attacks = pregen_attacks
            .get_bishop_attacks(bishop_sq, board_state.get_position_bb(Color::Both))
            .intersect(board_state.get_position_bb(Color::Both).invert());

        for att_sq in attacks.get_occupied_squares() {
            for pt in PIECE_TYPES {
                if board_state.get_piece_bb(Piece::from_type(pt, color.get_opposite())).is_occupied(att_sq) {
                    let m = Move::new(
                        bishop_sq,
                        att_sq,
                        piece,
                        Some(Piece::from_type(pt, color.get_opposite())),
                        None,
                        false,
                        false,
                        false,
                    );
                    if is_legal(&m, &board_state, &pregen_attacks) {
                        moves.push(m);
                    }
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(bishop_sq, sq, piece, None, None, false, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }
    }

    moves
}

pub fn gen_rook_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = *board_state.get_side();
    let piece = Piece::from_type(PieceType::Rook, color);

    let enemy_pieces = board_state.get_position_bb(color.get_opposite());
    let rooks = board_state.get_piece_bb(piece);

    for rook_sq in rooks.get_occupied_squares() {
        let attacks = pregen_attacks
            .get_rook_attacks(rook_sq, board_state.get_position_bb(Color::Both))
            .intersect(*enemy_pieces);
        let non_attacks = pregen_attacks
            .get_rook_attacks(rook_sq, board_state.get_position_bb(Color::Both))
            .intersect(board_state.get_position_bb(Color::Both).invert());

        for att_sq in attacks.get_occupied_squares() {
            for pt in PIECE_TYPES {
                if board_state.get_piece_bb(Piece::from_type(pt, color.get_opposite())).is_occupied(att_sq) {
                    let m = Move::new(
                        rook_sq,
                        att_sq,
                        piece,
                        Some(Piece::from_type(pt, color.get_opposite())),
                        None,
                        false,
                        false,
                        false,
                    );
                    if is_legal(&m, &board_state, &pregen_attacks) {
                        moves.push(m);
                    }
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(rook_sq, sq, piece, None, None, false, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }
    }

    moves
}

pub fn gen_queen_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks) -> Vec<Move> {
    let mut moves = Vec::new();
    let color = *board_state.get_side();
    let piece = Piece::from_type(PieceType::Queen, color);

    let enemy_pieces = board_state.get_position_bb(color.get_opposite());
    let queens = board_state.get_piece_bb(piece);

    for queen_sq in queens.get_occupied_squares() {
        let attacks = pregen_attacks
            .get_queen_attacks(queen_sq, board_state.get_position_bb(Color::Both))
            .intersect(*enemy_pieces);
        let non_attacks = pregen_attacks
            .get_queen_attacks(queen_sq, board_state.get_position_bb(Color::Both))
            .intersect(board_state.get_position_bb(Color::Both).invert());

        for att_sq in attacks.get_occupied_squares() {
            for pt in PIECE_TYPES {
                if board_state.get_piece_bb(Piece::from_type(pt, color.get_opposite())).is_occupied(att_sq) {
                    let m = Move::new(
                        queen_sq,
                        att_sq,
                        piece,
                        Some(Piece::from_type(pt, color.get_opposite())),
                        None,
                        false,
                        false,
                        false,
                    );
                    if is_legal(&m, &board_state, &pregen_attacks) {
                        moves.push(m);
                    }
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(queen_sq, sq, piece, None, None, false, false, false);
            if is_legal(&m, &board_state, &pregen_attacks) {
                moves.push(m);
            }
        }
    }

    moves
}
