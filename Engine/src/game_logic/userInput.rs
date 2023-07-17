use std::io::{self, Write};

use crate::move_logic::{move_encode::Move, pseudo_move_gen};

use super::game::GameState;

pub fn get_user_move(game_state: &mut GameState) -> Move {
    let mut input = String::new();
    print!("Enter move: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let m = Move::move_from_algebraic(input.trim(), game_state.get_board_state());

    match m {
        Ok(m) => {
            if is_valid(m, game_state) {
                return m;
            } else {
                println!("Invalid move");
                get_user_move(game_state)
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            get_user_move(game_state)
        }
    }
}

fn is_valid(c_move: Move, game_state: &mut GameState) -> bool {
    let mut pseudo_moves: Vec<Move> = Vec::new();
    pseudo_move_gen::get_pseudo_moves(game_state.get_board_state(), game_state.get_pregen_attacks(), &mut pseudo_moves);

    if !pseudo_moves.contains(&c_move) {
        return false;
    }

    game_state.make_move(c_move);
    if game_state
        .get_board_state()
        .is_check(game_state.get_board_state().get_side(), game_state.get_pregen_attacks())
    {
        game_state.unmake_move();
        return false;
    }
    game_state.unmake_move();

    true
}
