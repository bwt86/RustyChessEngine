use std::io::{self, Write};

use crate::{
    core::{attack_pregen::PregenAttacks, board_state::BoardState},
    move_gen::{move_encode::Move, move_eval},
};

pub struct Game {
    board_state: BoardState,
    pregen_attacks: PregenAttacks,
    history: Vec<BoardState>,
}

impl Game {
    pub fn new(fen_str: Option<&str>) -> Game {
        Game {
            board_state: BoardState::new(fen_str),
            pregen_attacks: PregenAttacks::init(),
            history: Vec::new(),
        }
    }

    pub fn get_board_state(&self) -> &BoardState {
        &self.board_state
    }

    pub fn get_pregen_attacks(&self) -> &PregenAttacks {
        &self.pregen_attacks
    }

    pub fn make_move(&mut self, m: Move) {
        self.history.push(self.board_state.clone());
        self.board_state.make_move(m);
    }

    pub fn unmake_move(&mut self) {
        self.board_state = self.history.pop().unwrap();
    }

    pub fn run(&mut self) {
        loop {
            self.get_board_state().display_info();

            let mut chess_move = String::new();

            print!("Enter move: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut chess_move).unwrap();

            let chess_move = chess_move.trim();

            if chess_move == "quit" {
                break;
            }

            match Move::move_from_algebraic(chess_move, &self.board_state) {
                Ok(m) => {
                    self.make_move(m);
                    m.print_move();
                }
                Err(e) => println!("Error: {:?}", e),
            }

            self.get_board_state().display_info();

            let m = move_eval::find_best_move(self, 5);
            println!("Move Selected: {:?}{:?}", m.get_from(), m.get_to());

            self.make_move(m);
        }
    }
}
