use std::collections::HashMap;

use crate::{
    core::{attack_pregen::PregenAttacks, board_state::BoardState, piece::Color, zobrist::ZobristHasher},
    move_logic::{move_encode::Move, move_eval, pseudo_move_gen},
};

use super::user_input;

pub struct GameState {
    board_state: BoardState,
    pregen_attacks: PregenAttacks,
    zobrist: ZobristHasher,
    transposition_table: HashMap<u64, (i32, u8)>,
    history: Vec<BoardState>,
}

impl GameState {
    pub fn new(fen_str: Option<&str>) -> GameState {
        let zobrist = ZobristHasher::new();
        GameState {
            board_state: BoardState::new(fen_str, &zobrist),
            pregen_attacks: PregenAttacks::init(),
            zobrist,
            transposition_table: HashMap::new(),
            history: Vec::new(),
        }
    }

    pub fn get_board_state(&self) -> &BoardState {
        &self.board_state
    }

    pub fn get_pregen_attacks(&self) -> &PregenAttacks {
        &self.pregen_attacks
    }

    pub fn get_transposition_table(&mut self) -> &mut HashMap<u64, (i32, u8)> {
        &mut self.transposition_table
    }

    pub fn make_move(&mut self, m: Move) {
        self.history.push(self.board_state.clone());
        self.board_state.make_move(m, &self.zobrist);
    }

    pub fn unmake_move(&mut self) {
        self.board_state = self.history.pop().unwrap();
    }

    pub fn is_check(&mut self, side: Color) -> bool {
        self.board_state.is_check(side, &self.pregen_attacks)
    }

    pub fn is_checkmate(&mut self) -> bool {
        let side = self.board_state.get_side();

        if self.is_check(side) {
            let mut pseudo_moves: Vec<Move> = Vec::new();
            pseudo_move_gen::get_pseudo_moves(&self.board_state, &self.pregen_attacks, &mut pseudo_moves);

            for m in pseudo_moves {
                self.make_move(m);
                if !self.is_check(side) {
                    self.unmake_move();
                    return false;
                }
                self.unmake_move();
            }
            return true;
        }
        false
    }

    pub fn run(&mut self) {
        loop {
            self.get_board_state().display_info(&self.pregen_attacks);

            let c_move = user_input::get_user_move(self);
            self.make_move(c_move);

            self.get_board_state().display_info(&self.pregen_attacks);

            let now = std::time::Instant::now();

            let c_move = move_eval::find_best_move(self, 5);

            if let Some(m) = c_move {
                println!("Time: {}s", now.elapsed().as_secs());
                println!("Move Selected: {:?}{:?}", m.get_from(), m.get_to());

                self.make_move(m);
                if self.is_checkmate() {
                    println!("Checkmate!");
                    break;
                }
            } else {
                println!("Time: {}s", now.elapsed().as_secs());
                println!("No move found");
                println!("Checkmate!");
                break;
            }
        }
    }
}
