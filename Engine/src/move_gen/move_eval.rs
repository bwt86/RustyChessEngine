use crate::{
    core::{
        attack_pregen::PregenAttacks,
        board_state::{self, BoardState},
        piece::{Color, Piece, PieceType},
    },
    game::Game,
};

use super::{move_encode::Move, move_gen};

pub fn find_best_move(game_state: &mut Game, depth: u8) -> Move {
    let mut best_score = std::i32::MIN + 1;
    let mut best_move = None;

    let mut moves = move_gen::get_sudo_moves(game_state);

    order_moves(&mut moves);

    for m in moves {
        game_state.make_move(m);

        if is_invalid(game_state) {
            game_state.unmake_move();
            continue;
        }

        let score = -alpha_beta(game_state, depth, (std::i32::MIN + 1), std::i32::MAX);
        game_state.unmake_move();

        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }
    }

    best_move.unwrap()
}

pub fn order_moves(moves: &mut Vec<Move>) {
    moves.sort_by(|a, b| b.get_score().cmp(&a.get_score()));
}

pub fn is_invalid(game_state: &mut Game) -> bool {
    let board_state = game_state.get_board_state();
    let pregen_attacks = game_state.get_pregen_attacks();

    let side = board_state.get_side().get_opposite();

    let both_bb = *board_state.get_position_bb(Color::Both);
    let enemy_bb = *board_state.get_position_bb(side.get_opposite());
    let king_bb = *board_state.get_piece_bb(Piece::from_type(PieceType::King, side));

    if king_bb.is_empty() {
        return true;
    }

    let king_square = king_bb.get_ls_square();
    let king_ray = pregen_attacks.get_queen_attacks(king_square, &both_bb);

    if !king_ray.intersect(enemy_bb).is_empty() {
        for piece in crate::PIECE_TYPES {
            for sq in board_state
                .get_piece_bb(Piece::from_type(piece, side.get_opposite()))
                .get_occupied_squares()
            {
                let piece_attack = pregen_attacks.get_piece_attacks(piece, side.get_opposite(), sq, &both_bb);

                if piece_attack.is_occupied(king_square) {
                    return true;
                }
            }
        }
    }
    false
}

fn alpha_beta(game_state: &mut Game, depth: u8, mut alpha: i32, beta: i32) -> i32 {
    let board_state = game_state.get_board_state();

    if depth == 0 {
        return board_state.evaluate_position(board_state.get_side());
    }

    let mut max_score = std::i32::MIN + 1;
    let mut moves = move_gen::get_sudo_moves(game_state);
    order_moves(&mut moves);

    for m in moves {
        game_state.make_move(m);

        if is_invalid(game_state) {
            game_state.unmake_move();
            continue;
        }

        let score = -alpha_beta(game_state, depth - 1, -beta, -alpha);
        game_state.unmake_move();

        max_score = max_score.max(score);
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }

    max_score
}
