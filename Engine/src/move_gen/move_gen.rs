use crate::{
    core::{
        attack_pregen::{self, PregenAttacks},
        bitboard::{Bitboard, FILE_A_BB, FILE_H_BB, RANK_2_BB, RANK_3_BB, RANK_6_BB},
        board_state::*,
        piece::*,
        square::*,
    },
    game::Game,
    move_gen::move_encode::*,
};

use super::move_encode::Move;

pub fn get_sudo_moves(game_state: &Game) -> Vec<Move> {
    let mut moves = Vec::new();

    let board_state = game_state.get_board_state();
    let pregen_attacks = game_state.get_pregen_attacks();

    gen_pawn_moves(board_state, pregen_attacks, &mut moves);
    gen_knight_moves(board_state, pregen_attacks, &mut moves);
    gen_bishop_moves(board_state, pregen_attacks, &mut moves);
    gen_rook_moves(board_state, pregen_attacks, &mut moves);
    gen_queen_moves(board_state, pregen_attacks, &mut moves);
    gen_king_moves(board_state, pregen_attacks, &mut moves);

    moves
}

fn gen_pawn_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let color = board_state.get_side();
    let piece = Piece::from_type(PieceType::Pawn, color);

    let pawns = board_state.get_piece_bb(piece);
    let enemy_pieces = *board_state.get_position_bb(color.get_opposite());
    let empty_sqs = board_state.get_position_bb(Color::Both).invert();
    let enpas = *board_state.get_enpas().unwrap_or(&Square::None);

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

        if enpas != Square::None {
            let enpas_attack = pregen_attacks
                .get_pawn_attacks(color.get_opposite(), enpas)
                .intersect(pawn_sq.to_bitboard());
            if !enpas_attack.is_empty() {
                let m = Move::new(
                    pawn_sq,
                    enpas,
                    piece,
                    Some(Piece::from_type(PieceType::Pawn, color.get_opposite())),
                    None,
                    false,
                    true,
                    false,
                );
                moves.push(m);
            }
        }

        if !single_move_forward.is_empty() {
            let m = Move::new(pawn_sq, single_move_forward.get_ls_square(), piece, None, None, false, false, false);
            moves.push(m);
        }

        if !double_move_forward.is_empty() {
            let m = Move::new(pawn_sq, double_move_forward.get_ls_square(), piece, None, None, true, false, false);
            moves.push(m);
        }

        let attacks = pregen_attacks.get_pawn_attacks(color, pawn_sq).intersect(enemy_pieces);
        for att_sq in attacks.get_occupied_squares() {
            let m = Move::new(pawn_sq, att_sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_knight_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let color = board_state.get_side();
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
                    moves.push(m);
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(knight_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_king_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let color = board_state.get_side();
    let piece = Piece::from_type(PieceType::King, color);
    let enemy_pieces = board_state.get_position_bb(color.get_opposite());
    let empty_sqs = board_state.get_position_bb(Color::Both).invert();

    let king = board_state.get_piece_bb(piece);

    if king.is_empty() {
        return;
    }

    let king_sq = king.get_ls_square();

    let king_attack = pregen_attacks.get_king_attacks(king_sq).intersect(*enemy_pieces);
    let non_attack = pregen_attacks.get_king_attacks(king_sq).intersect(empty_sqs);

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
                moves.push(m);
            }
        }
    }

    for sq in non_attack.get_occupied_squares() {
        let m = Move::new(king_sq, sq, piece, None, None, false, false, false);
        moves.push(m);
    }

    let castle_rights = board_state.get_cast_perm();

    if castle_rights & (WKC << (2 * color as u8)) != 0 {
        let king_ray = pregen_attacks.get_queen_attacks(king_sq, board_state.get_position_bb(Color::Both));
        let rook_ks = board_state.get_piece_bb(Piece::from_type(PieceType::Rook, color)).intersect(FILE_H_BB);
        if !king_ray.intersect(rook_ks).is_empty() {
            let m = Move::new(king_sq, king_sq.move_right(2), piece, None, None, false, false, true);
            moves.push(m);
        }
    }

    if castle_rights & (WQC << (2 * color as u8)) != 0 {
        let king_ray = pregen_attacks.get_queen_attacks(king_sq, board_state.get_position_bb(Color::Both));
        let rook_ks = board_state.get_piece_bb(Piece::from_type(PieceType::Rook, color)).intersect(FILE_A_BB);
        if !king_ray.intersect(rook_ks).is_empty() {
            let m = Move::new(king_sq, king_sq.move_left(2), piece, None, None, false, false, true);
            moves.push(m);
        }
    }
}

fn gen_bishop_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let color = board_state.get_side();
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
                    moves.push(m);
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(bishop_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_rook_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let color = board_state.get_side();
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
                    moves.push(m);
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(rook_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}

fn gen_queen_moves(board_state: &BoardState, pregen_attacks: &PregenAttacks, moves: &mut Vec<Move>) {
    let color = board_state.get_side();
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
                    moves.push(m);
                }
            }
        }

        for sq in non_attacks.get_occupied_squares() {
            let m = Move::new(queen_sq, sq, piece, None, None, false, false, false);
            moves.push(m);
        }
    }
}
