use std::time::{self, Duration};

use crate::{
    game_logic::game,
    move_logic::{
        move_encode::Move,
        pseudo_move_gen::{self, get_pseudo_moves},
    },
};

mod core;
mod game_logic;
mod move_logic;

fn main() {
    let mut game_state = game_logic::game::GameState::new(None, Duration::from_secs(300));

    game_state.run();
}
