use crate::{
    core::{attack_pregen::PregenAttacks, board_state::BoardState},
    move_gen::move_encode::Move,
};

pub struct Game {
    board_state: BoardState,
    pregen_attacks: PregenAttacks,
    move_stack: Vec<Move>,
    move_stack_index: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board_state: BoardState::new(None),
            pregen_attacks: PregenAttacks::init(),
            move_stack: Vec::new(),
            move_stack_index: 0,
        }
    }

    pub fn get_board_state(&self) -> &BoardState {
        &self.board_state
    }

    pub fn get_pregen_attacks(&self) -> &PregenAttacks {
        &self.pregen_attacks
    }

    pub fn get_move_stack(&self) -> &Vec<Move> {
        &self.move_stack
    }

    pub fn get_move_stack_index(&self) -> usize {
        self.move_stack_index
    }
}
