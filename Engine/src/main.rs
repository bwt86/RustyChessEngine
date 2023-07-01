use move_gen::move_eval;

use crate::core::attack_pregen;
use crate::core::bitboard::Bitboard;
use crate::core::piece::*;
use crate::core::square::*;
use crate::move_gen::move_encode::Move;

mod core;
mod game;
mod move_gen;
fn main() {
    let mut game = game::Game::new(Some("r1bqkb1r/ppp2ppp/2n2n2/1B1pp3/3PP3/2N2N2/PPP2PPP/R1BQK2R w KQkq - 0 1"));

    game.run();

    // print!("{:?}", Move::move_from_algebraic("Kb1c3", Color::White).get_from());

    // print!("{:?}", -(-(std::i32::MIN + 1)))
}
