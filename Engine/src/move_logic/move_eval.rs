use crate::game_logic::game::GameState;

use super::{move_encode::Move, pseudo_move_gen};

const NULL_MOVE_DEPTH: u8 = 3;
const NULL_MOVE_REDUCTION: u8 = 2;
const NULL_MOVE_PIECE_COUNT: u8 = 10;

pub fn find_best_move(game_state: &mut GameState, max_depth: u8) -> Option<Move> {
    let mut pseudo_moves: Vec<Move> = Vec::with_capacity(256);

    pseudo_move_gen::get_pseudo_moves(game_state.get_board_state(), game_state.get_pregen_attacks(), &mut pseudo_moves);
    order_moves(&mut pseudo_moves);

    let mut best_move = None;
    let mut best_score = i32::MIN + 1;

    for depth in 1..=max_depth {
        let mut alpha = i32::MIN + 1;
        let beta = i32::MAX - 1;

        for &m in &pseudo_moves {
            game_state.make_move(m);
            if game_state.is_check(game_state.get_board_state().get_opposite_side()) {
                game_state.unmake_move();
                continue;
            }

            let score = -alpha_beta(game_state, depth, -beta, -alpha, false);
            game_state.unmake_move();

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }

            if score > alpha {
                alpha = score;
            }
        }
        println!("Depth: {}, Best score: {}, Best move: {:?}", depth, -best_score, best_move);
    }
    best_move
}

pub fn order_moves(moves: &mut Vec<Move>) {
    moves.sort_by(|a, b| b.get_score().cmp(&a.get_score()));
}

pub fn alpha_beta(game_state: &mut GameState, depth: u8, mut alpha: i32, beta: i32, null_move: bool) -> i32 {
    //transposition_table
    let zobrist_key = game_state.get_board_state().get_zobrist_hash();
    if let Some((score, entry_depth)) = game_state.get_transposition_table().get(&zobrist_key) {
        if *entry_depth > depth {
            return *score;
        }
    }

    if game_state.is_checkmate() {
        return -100000;
    }

    // Null move pruning
    if !null_move
        && depth >= NULL_MOVE_DEPTH
        && game_state.get_board_state().get_num_pieces() > NULL_MOVE_PIECE_COUNT
        && !game_state.is_check(game_state.get_board_state().get_side())
    {
        game_state.make_null_move();
        let score = -alpha_beta(game_state, depth - 1 - NULL_MOVE_REDUCTION, -beta, -alpha, true);
        game_state.unmake_null_move();
        if score >= beta {
            return beta;
        }
    }

    if depth == 0 {
        return game_state.get_board_state().evaluate(game_state.get_pregen_attacks());
    }

    let mut pseudo_moves: Vec<Move> = Vec::with_capacity(256);
    pseudo_move_gen::get_pseudo_moves(game_state.get_board_state(), game_state.get_pregen_attacks(), &mut pseudo_moves);
    order_moves(&mut pseudo_moves);

    for m in pseudo_moves {
        game_state.make_move(m);
        if game_state.is_check(game_state.get_board_state().get_opposite_side()) {
            game_state.unmake_move();
            continue;
        }

        let score = -alpha_beta(game_state, depth - 1, -beta, -alpha, false);
        game_state.unmake_move();

        if score >= beta {
            return beta;
        }

        if score > alpha {
            alpha = score;
        }
    }

    game_state.get_transposition_table().insert(zobrist_key, (alpha, depth));
    alpha
}
